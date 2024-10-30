use gstd::ReservationId;
use gstd::debug;
use sails_rs::{
    collections::{HashMap, HashSet, BTreeSet},
    gstd::{exec, msg},
    prelude::*,
    ActorId,
};
use crate::services::game::Game;
use crate::services::game::game::PENALTY;
pub type Price = u32;
pub type Rent = u32;
pub type Gears = Vec<Gear>;
pub type AdminId = ActorId;

#[derive(Default, Clone, Encode, Decode, TypeInfo, PartialEq, Eq, Debug)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct Config {
    pub reservation_amount: u64,
    pub reservation_duration_in_block: u32,
    pub time_for_step: u32,
    pub min_gas_limit: u64,
    pub gas_refill_timeout: u32,
    pub gas_for_step: u64,
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct YourTurn {
    pub players: Vec<(ActorId, PlayerInfo)>,
    pub properties: Vec<Option<(ActorId, Gears, Price, Rent)>>,
}

#[derive(Default, Debug, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct PlayerInfo {
    pub owner_id: ActorId,
    pub name: String,
    pub position: u8,
    pub balance: u32,
    pub debt: u32,
    pub in_jail: bool,
    pub round: u128,
    pub cells: BTreeSet<u8>,
    pub penalty: u8,
    pub lost: bool,
    pub reservation_id: Option<ReservationId>,
}

#[derive(Default, Debug, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct PlayerInfoState {
    pub owner_id: ActorId,
    pub name: String,
    pub position: u8,
    pub balance: u32,
    pub debt: u32,
    pub in_jail: bool,
    pub round: u128,
    pub cells: Vec<u8>,
    pub penalty: u8,
    pub lost: bool,
    pub reservation_id: Option<ReservationId>,
}

impl From<PlayerInfo> for PlayerInfoState {
    fn from(player_info: PlayerInfo) -> Self {
        PlayerInfoState {
            owner_id: player_info.owner_id,
            name: player_info.name,
            position: player_info.position,
            balance: player_info.balance,
            debt: player_info.debt,
            in_jail: player_info.in_jail,
            round: player_info.round,
            cells: player_info.cells.into_iter().collect(),
            penalty: player_info.penalty,
            lost: player_info.lost,
            reservation_id: player_info.reservation_id,
        }
    }
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct GameInfo {
    pub admin_id: AdminId,
    pub properties_in_bank: Vec<u8>,
    pub players: Vec<(ActorId, PlayerInfo)>,
    pub players_queue: Vec<ActorId>,
    // mapping from cells to built properties,
    pub properties: Vec<Option<(ActorId, Gears, u32, u32)>>,
    // mapping from cells to accounts who have properties on it
    pub ownership: Vec<ActorId>,
}


#[derive(Debug, PartialEq, Eq, Encode, Decode, Clone, TypeInfo, Copy)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum Gear {
    Bronze,
    Silver,
    Gold,
}

impl Gear {
    pub fn upgrade(&self) -> Self {
        match *self {
            Self::Bronze => Self::Silver,
            Self::Silver => Self::Gold,
            Self::Gold => Self::Gold,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, TypeInfo, Encode, Decode)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum GameStatus {
    Registration,
    Play,
    Finished,
    Wait,
    WaitingForGasForGameContract,
    WaitingForGasForStrategy(ActorId),
}

impl Default for GameStatus {
    fn default() -> Self {
        Self::Registration
    }
}

pub fn get_player_info<'a>(
    player: &'a ActorId,
    players: &'a mut HashMap<ActorId, PlayerInfo>,
    current_round: u128,
) -> Result<&'a mut PlayerInfo, GameError> {
    if &msg::source() != player {
        //        debug!("PENALTY: WRONG MSG::SOURCE()");
        players.entry(msg::source()).and_modify(|player_info| {
            player_info.penalty += 1;
        });
        return Err(GameError::StrategicError);
    }
    let player_info = players.get_mut(player).expect("Cant be None: Get Player");
    if player_info.round >= current_round {
        //   debug!("PENALTY: MOVE ALREADY MADE");
        player_info.penalty += 1;
        return Err(GameError::StrategicError);
    }
    Ok(player_info)
}

pub fn sell_property(
    admin: &ActorId,
    ownership: &mut [ActorId],
    properties_for_sale: &Vec<u8>,
    properties_in_bank: &mut HashSet<u8>,
    properties: &[Option<(ActorId, Gears, u32, u32)>],
    player_info: &mut PlayerInfo,
) -> Result<(), GameError> {
    for property in properties_for_sale {
        if ownership[*property as usize] != msg::source() {
            player_info.penalty += 1;
            return Err(GameError::StrategicError);
        }
    }

    for property in properties_for_sale {
        if let Some((_, _, price, _)) = properties[*property as usize] {
            player_info.cells.remove(property);
            player_info.balance += price / 2;
            ownership[*property as usize] = *admin;
            properties_in_bank.insert(*property);
        }
    }
    Ok(())
}

static mut SEED: u8 = 0;
pub fn get_rolls() -> (u8, u8) {
    let seed = unsafe {
        SEED = SEED.wrapping_add(1);
        SEED
    };
    let random_input: [u8; 32] = [seed; 32];
    let (random, _) = exec::random(random_input).expect("Error in getting random number");
    let r1: u8 = random[0] % 6 + 1;
    let r2: u8 = random[1] % 6 + 1;
    (r1, r2)
}

pub fn bankrupt_and_penalty(
    admin: &ActorId,
    players: &mut HashMap<ActorId, PlayerInfo>,
    players_queue: &mut Vec<ActorId>,
    properties: &[Option<(ActorId, Gears, Price, Rent)>],
    properties_in_bank: &mut HashSet<u8>,
    ownership: &mut [ActorId],
    current_turn: &mut u8,
) {
    for (player, mut player_info) in players.clone() {
        if player_info.debt > 0 {
            for cell in &player_info.cells.clone() {
                if player_info.balance >= player_info.debt {
                    player_info.balance -= player_info.debt;
                    player_info.debt = 0;
                    player_info.penalty += 1;
                    players.insert(player, player_info);
                    break;
                }
                if let Some((_, _, price, _)) = &properties[*cell as usize] {
                    player_info.balance += price / 2;
                    player_info.cells.remove(cell);
                    ownership[*cell as usize] = *admin;
                    properties_in_bank.insert(*cell);
                }
            }
        }
    }

    for (player, mut player_info) in players.clone() {
        if (player_info.penalty >= PENALTY || player_info.debt > 0) && players_queue.len() > 1 {
            player_info.lost = true;
            player_info.balance = 0;
            players_queue.retain(|&p| p != player);
            for cell in &player_info.cells.clone() {
                ownership[*cell as usize] = *admin;
                properties_in_bank.insert(*cell);
            }
            players.insert(player, player_info);
            *current_turn = current_turn.saturating_sub(1);
        }
    }
}


pub fn init_properties(
    properties: &mut Vec<Option<(ActorId, Gears, Price, Rent)>>,
    ownership: &mut Vec<ActorId>,
) {
    // 0
    properties.push(None);
    // 1
    properties.push(Some((ActorId::zero(), Vec::new(), 1_000, 100)));
    // 2
    properties.push(None);
    // 3
    properties.push(Some((ActorId::zero(), Vec::new(), 1_050, 105)));
    // 4
    properties.push(None);
    // 5
    properties.push(Some((ActorId::zero(), Vec::new(), 1_100, 110)));
    // 6
    properties.push(Some((ActorId::zero(), Vec::new(), 1_500, 150)));
    // 7
    properties.push(None);
    // 8
    properties.push(Some((ActorId::zero(), Vec::new(), 1_550, 155)));
    // 9
    properties.push(Some((ActorId::zero(), Vec::new(), 1_700, 170)));

    // 10
    properties.push(None);
    // 11
    properties.push(Some((ActorId::zero(), Vec::new(), 2_000, 200)));
    // 12
    properties.push(Some((ActorId::zero(), Vec::new(), 2_050, 205)));
    // 13
    properties.push(Some((ActorId::zero(), Vec::new(), 2_100, 210)));
    // 14
    properties.push(Some((ActorId::zero(), Vec::new(), 2_200, 220)));
    // 15
    properties.push(Some((ActorId::zero(), Vec::new(), 2_300, 230)));
    // 16
    properties.push(None);
    // 17
    properties.push(Some((ActorId::zero(), Vec::new(), 2_400, 240)));
    // 18
    properties.push(Some((ActorId::zero(), Vec::new(), 2_450, 245)));
    // 19
    properties.push(Some((ActorId::zero(), Vec::new(), 2_500, 250)));

    // 20
    properties.push(None);
    // 21
    properties.push(Some((ActorId::zero(), Vec::new(), 3_000, 300)));
    // 22
    properties.push(None);
    // 23
    properties.push(Some((ActorId::zero(), Vec::new(), 3_100, 310)));
    // 24
    properties.push(Some((ActorId::zero(), Vec::new(), 3_150, 315)));
    // 25
    properties.push(Some((ActorId::zero(), Vec::new(), 3_200, 320)));
    // 26
    properties.push(Some((ActorId::zero(), Vec::new(), 3_250, 325)));
    // 27
    properties.push(Some((ActorId::zero(), Vec::new(), 3_300, 330)));
    // 28
    properties.push(Some((ActorId::zero(), Vec::new(), 3_350, 334)));
    // 29
    properties.push(Some((ActorId::zero(), Vec::new(), 3_400, 340)));

    // 30
    properties.push(None);
    // 31
    properties.push(Some((ActorId::zero(), Vec::new(), 4_000, 400)));
    // 32
    properties.push(Some((ActorId::zero(), Vec::new(), 4_050, 405)));
    // 33
    properties.push(None);
    // 34
    properties.push(Some((ActorId::zero(), Vec::new(), 4_100, 410)));
    // 35
    properties.push(Some((ActorId::zero(), Vec::new(), 4_150, 415)));
    // 36
    properties.push(None);
    // 37
    properties.push(Some((ActorId::zero(), Vec::new(), 4_200, 420)));
    // 38
    properties.push(None);
    // 39
    properties.push(Some((ActorId::zero(), Vec::new(), 4_500, 450)));

    for _i in 0..40 {
        ownership.push(ActorId::zero());
    }
}

#[derive(Debug)]
pub enum GameError {
    /// Error reply on `Register`
    /// In case if this strategy is already registered
    StrategyAlreadyReistered,

    /// Error reply on `Register`
    /// In case if the account is already registered in the game
    AccountAlreadyRegistered,

    /// Error reply on `ExitGame`
    /// In case if strategy for this account doesn't exist
    StrategyDoesNotExist,

    /// Error reply during making reservation
    ReservationError,

    /// Error reply in case `msg::source()` is not an admin
    OnlyAdmin,

    /// Error reply in case the player does not exist
    PlayerDoesNotExist,

    /// Error reply on case the
    WrongGameStatus,

    /// Error reply in case `msg::source()` is neither admin nor the program
    MsgSourceMustBeAdminOrProgram,

    /// Error reply in case game does not exist
    GameDoesNotExist,

    /// Error reply on case the reservation is no more valid
    ReservationNotValid,

    /// Error reply in case of insufficient gas
    /// for the game contract during the game.
    AddGasToGameContract,

    /// Error reply on `Play` message
    /// in case of insufficient gas for strategy
    AddGasForStrategy(ActorId),

    /// Error reply on `CreateGameSession`
    /// In case a game session has already been created for the specified account.
    GameSessionAlreadyExists,

    /// Error reply on `CreateGameSession`
    /// In case if indicated fee is less than ED
    FeeIsLessThanED,

    /// Error reply on `Register`
    /// In case a player didn't attach the required amount of value
    WrongValueAmount,

    /// Error reply on wrong move
    StrategicError,

    AccessDenied
}


pub fn take_your_turn(
    reservation_id: ReservationId,
    player: &ActorId,
    game_info: GameInfo,
) -> Result<MessageId, GameError> {
    debug!("take_your_turn");
    let request = [
        "Player".encode(),
        "YourTurn".to_string().encode(),
        (game_info).encode(),
    ]
    .concat();
    msg::send_bytes_from_reservation(reservation_id, *player, request, 0)
        .map_err(|_| GameError::ReservationNotValid)
}

pub fn msg_to_play_game(
    reservation_id: ReservationId,
    program_id: &ActorId,
    admin_id: &ActorId,
) -> Result<MessageId, GameError> {

    let request = [
        "Syndote".encode(),
        "Play".to_string().encode(),
        (admin_id).encode(),
    ]
    .concat();

    msg::send_bytes_from_reservation(
        reservation_id,
        *program_id,
        request,
        0,
    )
    .map_err(|_| GameError::ReservationNotValid)
}

#[derive(Debug, PartialEq, Eq, Clone, TypeInfo, Encode, Decode, Default)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct GameState {
    pub admin_id: ActorId,
    pub properties_in_bank: Vec<u8>,
    pub round: u128,
    pub players: Vec<(ActorId, PlayerInfoState)>,
    pub owners_to_strategy_ids: Vec<(ActorId, ActorId)>,
    pub players_queue: Vec<ActorId>,
    pub current_turn: u8,
    pub current_player: ActorId,
    pub current_step: u64,
    // mapping from cells to built properties,
    pub properties: Vec<Option<(ActorId, Gears, Price, Rent)>>,
    // mapping from cells to accounts who have properties on it
    pub ownership: Vec<ActorId>,
    pub game_status: GameStatus,
    pub winner: ActorId,
    pub reservations: Vec<ReservationId>,
    pub entry_fee: Option<u128>,
    pub prize_pool: u128,
}

impl From<Game> for GameState {
    fn from(game: Game) -> Self {
        GameState {
            admin_id: game.admin_id,
            properties_in_bank: game.properties_in_bank.into_iter().collect(),
            round: game.round,
            players: game.players.into_iter().map(|(id, player_info)| (id, player_info.clone().into())).collect(),
            owners_to_strategy_ids: game.owners_to_strategy_ids.into_iter().collect(),
            players_queue: game.players_queue,
            current_turn: game.current_turn,
            current_player: game.current_player,
            current_step: game.current_step,
            properties: game.properties,
            ownership: game.ownership,
            game_status: game.game_status,
            winner: game.winner,
            reservations: game.reservations,
            entry_fee: game.entry_fee,
            prize_pool: game.prize_pool,
        }
    }
}
