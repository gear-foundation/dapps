#![no_std]

use gstd::{
    collections::{HashMap, HashSet},
    exec, msg,
    prelude::*,
    ActorId, MessageId, ReservationId,
};
use messages::*;
use syndote_io::*;
use utils::*;

pub mod messages;
pub mod strategic_actions;
pub mod utils;

pub const NUMBER_OF_CELLS: u8 = 40;
pub const NUMBER_OF_PLAYERS: u8 = 4;
pub const JAIL_POSITION: u8 = 10;
pub const LOTTERY_POSITION: u8 = 20;
pub const COST_FOR_UPGRADE: u32 = 500;
pub const FINE: u32 = 1_000;
pub const PENALTY: u8 = 5;
pub const INITIAL_BALANCE: u32 = 15_000;
pub const NEW_CIRCLE: u32 = 2_000;
pub const WAIT_DURATION: u32 = 5;

#[derive(Clone, Default)]
pub struct Game {
    admin: ActorId,
    properties_in_bank: HashSet<u8>,
    round: u128,
    players: HashMap<ActorId, PlayerInfo>,
    players_queue: Vec<ActorId>,
    current_turn: u8,
    current_player: ActorId,
    current_step: u64,
    // mapping from cells to built properties,
    properties: Vec<Option<(ActorId, Gears, u32, u32)>>,
    // mapping from cells to accounts who have properties on it
    ownership: Vec<ActorId>,
    game_status: GameStatus,
    winner: ActorId,
    config: Config,
    current_msg_id: MessageId,
    awaiting_reply_msg_id: MessageId,
}

static mut GAME: Option<Game> = None;

impl Game {
    fn change_admin(&mut self, admin: &ActorId) -> Result<GameReply, GameError> {
        assert_eq!(msg::source(), self.admin);
        self.admin = *admin;
        Ok(GameReply::AdminChanged)
    }

    fn start_registration(&mut self) -> Result<GameReply, GameError> {
        self.check_status(GameStatus::Finished);
        self.only_admin();
        let mut game: Game = Game {
            admin: self.admin,
            ..Default::default()
        };

        init_properties(&mut game.properties, &mut game.ownership);
        *self = game;
        Ok(GameReply::RegistrationStarted)
    }

    fn register(&mut self, player: &ActorId) -> Result<GameReply, GameError> {
        self.check_status(GameStatus::Registration);
        assert!(
            !self.players.contains_key(player),
            "You have already registered"
        );
        let reservation_id = match ReservationId::reserve(
            self.config.reservation_amount,
            self.config.reservation_duration,
        ) {
            Ok(id) => id,
            Err(_) => return Err(GameError::ReservationError),
        };
        self.players.insert(
            *player,
            PlayerInfo {
                balance: INITIAL_BALANCE,
                reservation_id,
                ..Default::default()
            },
        );
        self.players_queue.push(*player);
        if self.players_queue.len() == NUMBER_OF_PLAYERS as usize {
            self.game_status = GameStatus::Play;
        }
        Ok(GameReply::Registered)
    }

    fn play(&mut self) -> Result<GameReply, GameError> {
        //self.check_status(GameStatus::Play);
        assert!(
            msg::source() == self.admin || msg::source() == exec::program_id(),
            "Only admin or the program can send that message"
        );

        match self.game_status {
            GameStatus::Play | GameStatus::Wait => {
                if self.game_status == GameStatus::Wait {
                    // This status means that the player has missed their turn or their strategy did not manage to make a move within the allotted time.
                    // The player is removed from the game.
                    self.exclude_player_from_game(self.current_player);

                    // If the value of current_turn was 0 (meaning the player who missed their turn and was removed was the last in the array),
                    // then this value remains the same.
                    // If the value was 1, 2, or 3, then it is properly decreased by one.
                    self.current_turn = self.current_turn.saturating_sub(1);
                }
                let current_player: ActorId = self.players_queue[self.current_turn as usize];
                self.current_player = current_player;
                self.current_step += 1;

                let mut player_info = self
                    .players
                    .get(&current_player)
                    .expect("Cant be None: Get Player")
                    .clone();

                let position = if player_info.in_jail {
                    player_info.position
                } else {
                    let (r1, r2) = get_rolls();
                    //     debug!("ROOLS {:?} {:?}", r1, r2);
                    let roll_sum = r1 + r2;
                    (player_info.position + roll_sum) % NUMBER_OF_CELLS
                };

                // If a player is on a cell that belongs to another player
                // we write down a debt on him in the amount of the rent.
                // This is done in order to penalize the participant's contract
                // if he misses the rent
                let account = self.ownership[position as usize];
                if account != current_player && !account.is_zero() {
                    if let Some((_, _, _, rent)) = self.properties[position as usize] {
                        player_info.debt = rent;
                    }
                }
                // If the new position is behind the previous one, it indicates that the player has completed a circuit around the board,
                // and his balance should be accordingly updated.
                if position <= player_info.position {
                    player_info.balance += NEW_CIRCLE;
                }
                player_info.position = position;
                player_info.in_jail = position == JAIL_POSITION;
                player_info.round = self.round;

                self.players.insert(current_player, player_info.clone());
                self.current_turn = (self.current_turn + 1) % self.players.len() as u8;
                match position {
                    // free cells (it can be lottery or penalty): TODO as a task on hackathon
                    0 | 2 | 4 | 7 | 16 | 20 | 30 | 33 | 36 | 38 => {}
                    _ => {
                        let game_info = self.get_game_info();
                        self.awaiting_reply_msg_id =
                            take_your_turn(player_info.reservation_id, &current_player, game_info);
                    }
                }
                if self.current_msg_id != MessageId::zero() {
                    self.current_msg_id = msg::id();
                }
                self.game_status = GameStatus::Wait;
                exec::wait_for(self.config.time_for_step);
            }
            GameStatus::Finished => {
                return Ok(GameReply::GameFinished {
                    winner: self.winner,
                });
            }
            GameStatus::Registration => return Err(GameError::WrongGameStatus),
        }
    }
}

#[no_mangle]
extern fn handle() {
    let action: GameAction = msg::load().expect("Could not load `GameAction`");
    let game: &mut Game = unsafe {
        GAME.as_mut()
            .expect("Unexpected: Contract is not initialized")
    };
    let reply = match action {
        GameAction::Register { player } => game.register(&player),
        GameAction::StartRegistration => game.start_registration(),
        GameAction::Play => game.play(),
        GameAction::ChangeAdmin(admin) => game.change_admin(&admin),
    };
    msg::reply(reply, 0).expect("Error during sending a reply");
}

#[no_mangle]
unsafe extern fn init() {
    let mut game = Game {
        admin: msg::source(),
        ..Default::default()
    };
    init_properties(&mut game.properties, &mut game.ownership);
    GAME = Some(game);
}

#[no_mangle]
extern fn state() {
    let game = unsafe { GAME.take().expect("Game is not initialized") };
    let game_state: GameState = game.into();
    msg::reply(game_state, 0).expect("Failed to share state");
}

#[no_mangle]
extern fn handle_reply() {
    let reply_to = msg::reply_to().expect("Unable to get the msg id");

    let game: &mut Game = unsafe {
        GAME.as_mut()
            .expect("Unexpected: Contract is not initialized")
    };
    let current_player = game.current_player;
    if game.awaiting_reply_msg_id == reply_to {
        let reply: Result<StrategicAction, gstd::errors::Error> = msg::load();
        match reply {
            Ok(strategic_action) => match strategic_action {
                StrategicAction::AddGear {
                    properties_for_sale,
                } => game.add_gear(properties_for_sale),
                StrategicAction::BuyCell {
                    properties_for_sale,
                } => game.buy_cell(properties_for_sale),
                StrategicAction::PayRent {
                    properties_for_sale,
                } => game.pay_rent(properties_for_sale),
                StrategicAction::ThrowRoll {
                    pay_fine,
                    properties_for_sale,
                } => game.throw_roll(pay_fine, properties_for_sale),
                StrategicAction::Upgrade {
                    properties_for_sale,
                } => game.upgrade(properties_for_sale),
                StrategicAction::Skip => {},
            },
            _ => {
                game.exclude_player_from_game(game.current_player);
            }
        };
    }

    // check penalty and debt of the players for the previous round
    // if penalty is equal to 5 points we remove the player from the game
    // if a player has a debt and he has not enough balance to pay it
    // he is also removed from the game
    bankrupt_and_penalty(
        &game.admin,
        &mut game.players,
        &mut game.players_queue,
        &game.properties,
        &mut game.properties_in_bank,
        &mut game.ownership,
    );

    match game.players_queue.len() {
        0 => {
            // All players have been removed from the game (either penalized or bankrupt)
            game.game_status = GameStatus::Finished;
        }
        1 => {
            game.winner = game.players_queue[0];
            game.game_status = GameStatus::Finished;
        }
        _ => {
            game.game_status = GameStatus::Play;
            let gas_available = exec::gas_available();

            if gas_available > game.config.min_gas_limit {
                let reservation_id = ReservationId::reserve(
                    gas_available - game.config.min_gas_limit / 10,
                    game.config.reservation_duration,
                )
                .expect("Error during reservation");
                game.players
                    .entry(current_player)
                    .and_modify(|info| info.reservation_id = reservation_id);
            }
        }
    }

    msg::send_with_gas(
        game.admin,
        GameReply::Step {
            players: game
                .players
                .iter()
                .map(|(key, value)| (*key, value.clone()))
                .collect(),
            properties: game.properties.clone(),
            current_player: game.current_player,
            current_step: game.current_step,
            ownership: game.ownership.clone(),
        },
        0,
        0,
    )
    .expect("Error in sending a message `GameEvent::Step`");

    exec::wake(game.current_msg_id).expect("Unable to wake the msg");
}
