use crate::single::Entity;
use gstd::{ActorId, Decode, Encode, TypeInfo, collections::HashMap, prelude::*};

pub type MultipleGamesMap = HashMap<ActorId, MultipleGame>;
pub type GamePairsMap = HashMap<ActorId, ActorId>;
pub(crate) type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum Error {
    SeveralGames,
    NoSuchGame,
    WrongStep,
    AccessDenied,
    WrongStatus,
    WrongShipsHash,
    NotPlayer,
    AlreadyVerified,
    WrongBid,
    WrongOut,
    StepIsNotTaken,
}

pub struct MultipleGame {
    pub admin: ActorId,
    pub participants_data: HashMap<ActorId, ParticipantInfo>,
    pub create_time: u64,
    pub start_time: Option<u64>,
    pub last_move_time: u64,
    pub status: Status,
    pub bid: u128,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct MultipleGameState {
    pub admin: ActorId,
    pub participants_data: Vec<(ActorId, ParticipantInfo)>,
    pub create_time: u64,
    pub start_time: Option<u64>,
    pub last_move_time: u64,
    pub status: Status,
    pub bid: u128,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum Status {
    Registration,
    VerificationPlacement(Option<ActorId>),
    PendingVerificationOfTheMove((ActorId, u8)),
    Turn(ActorId),
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct ParticipantInfo {
    pub name: String,
    pub board: Vec<Entity>,
    pub ship_hash: Vec<u8>,
    pub total_shots: u8,
    pub succesfull_shots: u8,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum StepResult {
    Missed,
    Injured,
    Killed,
}

impl MultipleGame {
    pub fn get_opponent(&self, player: &ActorId) -> ActorId {
        let (id, _) = self
            .participants_data
            .iter()
            .find(|&(&id, _)| id != *player)
            .expect("The opponent must exist");
        *id
    }

    pub fn shot(&mut self, player: &ActorId, step: u8, res: u8) {
        let data = self
            .participants_data
            .get_mut(player)
            .expect("The player must exist");

        match res {
            0 => data.board[step as usize] = Entity::Boom,
            1 => {
                data.board[step as usize] = Entity::BoomShip;
                let opponent = self.get_opponent(player);
                let opponent_data = self
                    .participants_data
                    .get_mut(&opponent)
                    .expect("The player must exist");

                opponent_data.succesfull_shots += 1;
            }
            2 => {
                Self::dead_ship(step, &mut data.board);
                let opponent = self.get_opponent(player);
                let opponent_data = self
                    .participants_data
                    .get_mut(&opponent)
                    .expect("The player must exist");
                opponent_data.succesfull_shots += 1;
            }
            _ => unimplemented!(),
        }
    }
    pub fn check_end_game(&self, player: &ActorId) -> bool {
        let data = self
            .participants_data
            .get(player)
            .expect("The player must exist");
        let count_dead_ships = data
            .board
            .iter()
            .filter(|&entity| *entity == Entity::DeadShip)
            .count();
        count_dead_ships == 8
    }

    fn dead_ship(step: u8, player_board: &mut Vec<Entity>) {
        player_board[step as usize] = Entity::DeadShip;
        Self::auto_boom(player_board.as_mut(), step);
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
                if player_board[position as usize] == Entity::BoomShip {
                    player_board[position as usize] = Entity::DeadShip;
                    Self::auto_boom(player_board.as_mut(), position as u8);
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
            if board[current_position as usize] == Entity::Unknown {
                board[current_position as usize] = Entity::Boom;
            }
        }
    }
}
