#![no_std]

use gmeta::{In, InOut, Metadata};
use gstd::{
    fmt::{self, Display},
    prelude::*,
    ActorId,
};

// Minimum duration of session: 3 mins = 180_000 ms = 60 blocks
pub const MINIMUM_SESSION_SURATION_MS: u64 = 180_000;

pub struct BattleshipMetadata;

impl Metadata for BattleshipMetadata {
    type Init = In<BattleshipInit>;
    type Handle = InOut<BattleshipAction, Result<BattleshipReply, BattleshipError>>;
    type Others = ();
    type Reply = ();
    type Signal = ();
    type State = InOut<StateQuery, StateReply>;
}

#[derive(Encode, Decode, TypeInfo)]
pub enum StateQuery {
    All,
    Game(ActorId),
    BotContractId,
    SessionForTheAccount(ActorId),
}

#[derive(Encode, Decode, TypeInfo)]
pub enum StateReply {
    All(BattleshipState),
    Game(Option<GameState>),
    BotContractId(ActorId),
    SessionForTheAccount(Option<Session>),
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct BattleshipState {
    pub games: Vec<(ActorId, GameState)>,
    pub bot_address: ActorId,
    pub admin: ActorId,
}

// This structure is for creating a gaming session, which allows players to predefine certain actions for an account that will play the game on their behalf for a certain period of time.
// Sessions can be used to send transactions from a dApp on behalf of a user without requiring their confirmation with a wallet.
// The user is guaranteed that the dApp can only execute transactions that comply with the allowed_actions of the session until the session expires.
#[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct Session {
    // the address of the player who will play on behalf of the user
    pub key: ActorId,
    // until what time the session is valid
    pub expires: u64,
    // what messages are allowed to be sent by the account (key)
    pub allowed_actions: Vec<ActionsForSession>,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum ActionsForSession {
    StartGame,
    Turn,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub enum Entity {
    Empty,
    Unknown,
    Occupied,
    Ship,
    Boom,
    BoomShip,
    DeadShip,
}

impl Entity {
    pub fn is_empty(&self) -> bool {
        matches!(self, Entity::Empty)
    }
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum BattleshipAction {
    StartGame {
        ships: Ships,
        session_for_account: Option<ActorId>,
    },
    Turn {
        step: u8,
        session_for_account: Option<ActorId>,
    },
    ChangeBot {
        bot: ActorId,
    },
    ClearState {
        leave_active_games: bool,
    },
    DeleteGame {
        player_address: ActorId,
    },
    CreateSession {
        key: ActorId,
        duration: u64,
        allowed_actions: Vec<ActionsForSession>,
    },
    DeleteSessionFromProgram {
        account: ActorId,
    },
    DeleteSessionFromAccount,
    UpdateConfig {
        gas_for_start: Option<u64>,
        gas_for_move: Option<u64>,
        gas_to_delete_session: Option<u64>,
        block_duration_ms: Option<u64>,
    },
}

#[derive(Debug, Clone, Default, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct Config {
    pub gas_for_start: u64,
    pub gas_for_move: u64,
    pub gas_to_delete_session: u64,
    pub block_duration_ms: u64,
}
#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum BattleshipReply {
    GameFinished(BattleshipParticipants),
    MessageSentToBot,
    BotChanged(ActorId),
    SessionCreated,
    SessionDeleted,
    ConfigUpdated,
    StateCleared,
    GameDeleted,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum BattleshipError {
    GameIsAlreadyStarted,
    GameIsNotStarted,
    IncorrectLocationShips,
    OutOfBounds,
    GameIsAlreadyOver,
    ThisCellAlreadyKnown,
    BotDidNotInitializeBoard,
    NotYourTurn,
    NotAdmin,
    WrongLength,
    AccessDenied,
    AlreadyHaveActiveSession,
    NoMessagesForApprovalWerePassed,
    DurationIsSmall,
    HasNotValidSession,
    SessionHasAlreadyExpired,
    MessageIsNotAllowed,
    NotApproved,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum Step {
    Missed,
    Injured,
    Killed,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct BattleshipInit {
    pub bot_address: ActorId,
    pub config: Config,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo, Default)]
pub struct Game {
    pub player_board: Vec<Entity>,
    pub bot_board: Vec<Entity>,
    pub player_ships: Ships,
    pub bot_ships: Ships,
    pub turn: Option<BattleshipParticipants>,
    pub start_time: u64,
    pub end_time: u64,
    pub total_shots: u64,
    pub game_over: bool,
    pub game_result: Option<BattleshipParticipants>,
}

impl Game {
    pub fn start_bot(&mut self, mut ships: Ships) {
        let bot_board = ships.get_field().unwrap();
        self.bot_board = bot_board;
        ships.sort_by_length();
        self.bot_ships = ships;
    }
    pub fn turn(&mut self, step: u8) {
        let res = self.player_ships.bang(step);
        match res {
            Step::Missed => self.player_board[step as usize] = Entity::Boom,
            Step::Injured => self.player_board[step as usize] = Entity::BoomShip,
            Step::Killed => self.dead_ship(step, 0),
        }
    }
    pub fn get_hidden_field(&self) -> Vec<Entity> {
        let mut board = self.player_board.clone();
        board.iter_mut().for_each(|position| {
            if let Entity::Empty | Entity::Ship = position {
                *position = Entity::Unknown;
            }
        });
        board
    }

    // if the ship is killed, we must change the BoomShip cells to DeadShip.
    pub fn dead_ship(&mut self, step: u8, ship_id: u8) {
        let (board, _) = if ship_id == 0 {
            (&mut self.player_board, &mut self.bot_board)
        } else {
            (&mut self.bot_board, &mut self.player_board)
        };
        board[step as usize] = Entity::DeadShip;
        Self::auto_boom(board, step);
        let mut current_step = step as i8;
        'stop: loop {
            let directions: Vec<i8> = match current_step {
                0 => vec![5, 1],
                4 => vec![5, -1],
                20 => vec![1, -5],
                24 => vec![-1, -5],
                p if p % 5 == 0 => vec![-5, 1, 5],
                p if (p + 1) % 5 == 0 => vec![-5, -1, 5],
                _ => vec![-5, -1, 1, 5],
            };
            for direction in directions {
                let position = current_step + direction;
                if !(0..=24).contains(&position) {
                    continue;
                }
                if board[position as usize] == Entity::BoomShip {
                    board[position as usize] = Entity::DeadShip;
                    Self::auto_boom(board, position as u8);
                    current_step += direction;
                    continue 'stop;
                }
            }
            break;
        }
    }

    fn auto_boom(board: &mut [Entity], position: u8) {
        let cells = match position {
            0 => vec![1, 5, 6],
            4 => vec![-1, 4, 5],
            20 => vec![1, -4, -5],
            24 => vec![-1, -5, -6],
            p if p % 5 == 0 => vec![-4, -5, 1, 5, 6],
            p if (p + 1) % 5 == 0 => vec![-1, -5, -6, 4, 5],
            _ => vec![-1, -4, -5, -6, 1, 4, 5, 6],
        };

        for cell in &cells {
            let current_position = position as i8 + *cell;
            if !(0..=24).contains(&current_position) {
                continue;
            }
            if board[current_position as usize] == Entity::Empty {
                board[current_position as usize] = Entity::Boom;
            }
        }
    }
}

impl Display for Ships {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ships = self.to_vec();
        for (index, inner_vec) in ships.iter().enumerate() {
            f.write_str(&format!("Ship {}: ", index + 1))?;

            for &value in inner_vec {
                f.write_str(&value.to_string())?;
                f.write_str(", ")?;
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo, Default)]
pub struct Ships {
    pub ship_1: Vec<u8>,
    pub ship_2: Vec<u8>,
    pub ship_3: Vec<u8>,
    pub ship_4: Vec<u8>,
}

impl Ships {
    pub fn iter(&self) -> impl Iterator<Item = &u8> {
        self.ship_1
            .iter()
            .chain(&self.ship_2)
            .chain(&self.ship_3)
            .chain(&self.ship_4)
    }
    pub fn to_vec(&self) -> Vec<Vec<u8>> {
        vec![
            self.ship_1.clone(),
            self.ship_2.clone(),
            self.ship_3.clone(),
            self.ship_4.clone(),
        ]
    }

    pub fn sort_by_length(&mut self) {
        let mut vectors: Vec<Vec<u8>> = vec![
            self.ship_1.clone(),
            self.ship_2.clone(),
            self.ship_3.clone(),
            self.ship_4.clone(),
        ];

        vectors.sort_by_key(|a| a.len());

        self.ship_1 = vectors[0].clone();
        self.ship_2 = vectors[1].clone();
        self.ship_3 = vectors[2].clone();
        self.ship_4 = vectors[3].clone();
    }
    pub fn bang(&mut self, step: u8) -> Step {
        for ship in [
            &mut self.ship_1,
            &mut self.ship_2,
            &mut self.ship_3,
            &mut self.ship_4,
        ]
        .iter_mut()
        {
            if let Some(pos) = ship.iter().position(|&x| x == step) {
                ship.remove(pos);
                return if ship.is_empty() {
                    Step::Killed
                } else {
                    Step::Injured
                };
            }
        }
        Step::Missed
    }
    pub fn check_end_game(&self) -> bool {
        let vectors = [&self.ship_1, &self.ship_2, &self.ship_3, &self.ship_4];
        let has_non_empty = !vectors.iter().any(|vector: &&Vec<u8>| !vector.is_empty());
        has_non_empty
    }
    pub fn get_field(&self) -> Result<Vec<Entity>, BattleshipError> {
        let mut board = vec![Entity::Empty; 25];
        for position in self.iter() {
            if board[*position as usize] == Entity::Ship {
                return Err(BattleshipError::IncorrectLocationShips);
            }
            board[*position as usize] = Entity::Ship;
        }
        Ok(board)
    }
    pub fn check_correct_location(&self) -> Result<(), BattleshipError> {
        if self.iter().any(|&position| position > 24) {
            return Err(BattleshipError::OutOfBounds);
        }
        // ship size check
        let mut vec_len = vec![
            self.ship_1.len(),
            self.ship_2.len(),
            self.ship_3.len(),
            self.ship_4.len(),
        ];
        vec_len.sort();
        if vec_len != vec![1, 2, 2, 3] {
            return Err(BattleshipError::WrongLength);
        }
        let mut field = self.get_field()?;
        let mut ships = [
            self.ship_1.clone(),
            self.ship_2.clone(),
            self.ship_3.clone(),
            self.ship_4.clone(),
        ];
        for ship in ships.iter_mut() {
            // ship's integrity check
            ship.sort();
            let distance = ship[ship.len() - 1] - ship[0];
            match (ship.len(), distance) {
                (1, 0) | (2, 1) | (2, 5) => (),
                (3, 2) | (3, 10) if (ship[2] + ship[0]) % ship[1] == 0 => (),
                _ => return Err(BattleshipError::IncorrectLocationShips),
            }
            // checking the distance between ships
            let mut occupy_cells = vec![];
            for position in ship {
                if field[*position as usize] == Entity::Occupied {
                    return Err(BattleshipError::IncorrectLocationShips);
                }
                let cells = match *position {
                    0 => vec![1, 5, 6],
                    4 => vec![-1, 4, 5],
                    20 => vec![1, -4, -5],
                    24 => vec![-1, -5, -6],
                    p if p % 5 == 0 => vec![-4, -5, 1, 5, 6],
                    p if (p + 1) % 5 == 0 => vec![-1, -5, -6, 4, 5],
                    _ => vec![-1, -4, -5, -6, 1, 4, 5, 6],
                };
                for &cell in &cells {
                    let current_position = *position as isize + cell;
                    if !(0..=24).contains(&current_position) {
                        continue;
                    }
                    occupy_cells.push(current_position)
                }
            }

            for occupy_cell in occupy_cells {
                field[occupy_cell as usize] = Entity::Occupied;
            }
        }

        Ok(())
    }

    pub fn count_alive_ships(&self) -> Vec<(u8, u8)> {
        let alive_ship_1 = !self.ship_1.is_empty() as u8;
        let alive_ship_2 = !self.ship_2.is_empty() as u8 + !self.ship_3.is_empty() as u8;
        let alive_ship_3 = !self.ship_4.is_empty() as u8;
        vec![(1, alive_ship_1), (2, alive_ship_2), (3, alive_ship_3)]
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub enum BattleshipParticipants {
    Player,
    Bot,
}

#[derive(Encode, Decode, TypeInfo, PartialEq)]
pub enum BotBattleshipAction {
    Start,
    Turn(Vec<Entity>),
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo, Default)]
pub struct GameState {
    pub player_board: Vec<Entity>,
    pub bot_board: Vec<Entity>,
    pub player_ships: Vec<(u8, u8)>,
    pub bot_ships: Vec<(u8, u8)>,
    pub turn: Option<BattleshipParticipants>,
    pub start_time: u64,
    pub end_time: u64,
    pub total_shots: u64,
    pub game_over: bool,
    pub game_result: Option<BattleshipParticipants>,
}
