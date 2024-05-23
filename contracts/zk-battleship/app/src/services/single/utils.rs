use gstd::{collections::HashMap, prelude::*, ActorId, Decode, Encode, TypeInfo};

pub type SingleGamesMap = HashMap<ActorId, SingleGame>;
pub type SessionMap = HashMap<ActorId, Session>;
pub(crate) type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rtl::scale_codec)]
#[scale_info(crate = sails_rtl::scale_info)]
pub enum Error {
    AlreadyHaveActiveSession,
    NoActiveSession,
    SeveralGames,
    WrongStep,
    NoSuchGame,
    GameIsAlreadyOver,
    StatusIsPendingVerification,
    StatusIsNotPendingVerification,
    AllowedActionsIsEmpty,
    ErrorZkVerify,
    InvalidVerificationKey,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rtl::scale_codec)]
#[scale_info(crate = sails_rtl::scale_info)]
pub struct SingleGame {
    pub player_board: Vec<Entity>,
    pub bot_ships: Ships,
    pub start_time: u64,
    pub status: Status,
    pub end_time: Option<u64>,
    pub total_shots: u64,
    pub result: Option<BattleshipParticipants>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rtl::scale_codec)]
#[scale_info(crate = sails_rtl::scale_info)]
pub struct SingleGameState {
    pub player_board: Vec<Entity>,
    pub start_time: u64,
    pub status: Status,
    pub end_time: Option<u64>,
    pub total_shots: u64,
    pub result: Option<BattleshipParticipants>,
}

impl SingleGame {
    pub fn check_end_game(&self) -> bool {
        let count_dead_ships = self
            .player_board
            .iter()
            .filter(|&entity| *entity == Entity::DeadShip)
            .count();
        count_dead_ships == 8
    }
    pub fn dead_ship(&mut self, step: u8) {
        self.player_board[step as usize] = Entity::DeadShip;
        Self::auto_boom(self.player_board.as_mut(), step);
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
                if self.player_board[position as usize] == Entity::BoomShip {
                    self.player_board[position as usize] = Entity::DeadShip;
                    Self::auto_boom(self.player_board.as_mut(), position as u8);
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
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rtl::scale_codec)]
#[scale_info(crate = sails_rtl::scale_info)]
pub struct Session {
    // the address of the player who will play on behalf of the user
    pub key: ActorId,
    // until what time the session is valid
    pub expires: u64,
    // what messages are allowed to be sent by the account (key)
    pub allowed_actions: Vec<ActionsForSession>,
}
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rtl::scale_codec)]
#[scale_info(crate = sails_rtl::scale_info)]
pub enum ActionsForSession {
    StartSingleGame,
    StartMultipleGame,
    Move,
}
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rtl::scale_codec)]
#[scale_info(crate = sails_rtl::scale_info)]
pub enum Status {
    PendingVerificationOfTheMove(u8),
    PendingMove,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rtl::scale_codec)]
#[scale_info(crate = sails_rtl::scale_info)]
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

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rtl::scale_codec)]
#[scale_info(crate = sails_rtl::scale_info)]
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
    pub fn bang(&mut self, step: u8) -> StepResult {
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
                    StepResult::Killed
                } else {
                    StepResult::Injured
                };
            }
        }
        StepResult::Missed
    }
    pub fn check_end_game(&self) -> bool {
        let vectors = [&self.ship_1, &self.ship_2, &self.ship_3, &self.ship_4];
        let has_non_empty = !vectors.iter().any(|vector: &&Vec<u8>| !vector.is_empty());
        has_non_empty
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rtl::scale_codec)]
#[scale_info(crate = sails_rtl::scale_info)]
pub enum StepResult {
    Missed,
    Injured,
    Killed,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rtl::scale_codec)]
#[scale_info(crate = sails_rtl::scale_info)]
pub enum BattleshipParticipants {
    Player,
    Bot,
}
