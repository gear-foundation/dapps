use crate::single::Entity;
use gstd::{collections::HashMap, prelude::*, ActorId, Decode, Encode, TypeInfo};

pub type MultipleGamesMap = HashMap<ActorId, MultipleGame>;
pub type GamePairsMap = HashMap<ActorId, ActorId>;
pub(crate) type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rtl::scale_codec)]
#[scale_info(crate = sails_rtl::scale_info)]
pub enum Error {
    SeveralGames,
    NoSuchGame,
    WrongStep,
    AccessDenied,
    WrongStatus,
    NotPlayer,
    AlreadyVerified,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rtl::scale_codec)]
#[scale_info(crate = sails_rtl::scale_info)]
pub struct MultipleGame {
    pub first_player_board: (ActorId, Vec<Entity>),
    pub second_player_board: Option<(ActorId, Vec<Entity>)>,
    pub participants: (ActorId, ActorId),
    pub start_time: Option<u64>,
    pub end_time: Option<u64>,
    pub status: Status,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rtl::scale_codec)]
#[scale_info(crate = sails_rtl::scale_info)]
pub enum Status {
    Registration,
    VerificationPlacement(Option<ActorId>),
    PendingVerificationOfTheMove((ActorId, u8)),
    Turn(ActorId),
    GameOver(ActorId),
}

impl MultipleGame {
    pub fn get_opponent(&self, player: &ActorId) -> ActorId {
        match player {
            p if *p == self.participants.0 => self.participants.1,
            _ => self.participants.0,
        }
    }

    pub fn shot(&mut self, player: &ActorId, step: u8, res: u8) {
        let board: &mut Vec<Entity> = match player {
            p if *p == self.first_player_board.0 => self.first_player_board.1.as_mut(),
            _ => self.second_player_board.as_mut().unwrap().1.as_mut(),
        };
        match res {
            0 => board[step as usize] = Entity::Boom,
            1 => board[step as usize] = Entity::BoomShip,
            _ => unimplemented!(),
        }
    }
    pub fn check_end_game(&self, player: &ActorId) -> bool {
        let board: &Vec<Entity> = match player {
            p if *p == self.first_player_board.0 => self.first_player_board.1.as_ref(),
            _ => self.second_player_board.as_ref().unwrap().1.as_ref(),
        };
        let count_dead_ships = board
            .iter()
            .filter(|&entity| *entity == Entity::DeadShip)
            .count();
        count_dead_ships == 8
    }

    // fn dead_ship(step: u8, player_board: &mut Vec<Entity>) {
    //     player_board[step as usize] = Entity::DeadShip;
    //     Self::auto_boom(player_board.as_mut(), step);
    //     let mut current_step = step as i8;
    //     'stop: loop {
    //         let directions: Vec<i8> = match current_step {
    //             0 => vec![5, 1],
    //             4 => vec![5, -1],
    //             20 => vec![1, -5],
    //             24 => vec![-1, -5],
    //             p if p % 5 == 0 => vec![-5, 1, 5],
    //             p if (p + 1) % 5 == 0 => vec![-5, -1, 5],
    //             _ => vec![-5, -1, 1, 5],
    //         };
    //         for direction in directions {
    //             let position = current_step + direction;
    //             if !(0..=24).contains(&position) {
    //                 continue;
    //             }
    //             if player_board[position as usize] == Entity::BoomShip {
    //                 player_board[position as usize] = Entity::DeadShip;
    //                 Self::auto_boom(player_board.as_mut(), position as u8);
    //                 current_step += direction;
    //                 continue 'stop;
    //             }
    //         }
    //         break;
    //     }
    // }

    // fn auto_boom(board: &mut [Entity], position: u8) {
    //     let cells = match position {
    //         0 => vec![1, 5, 6],
    //         4 => vec![-1, 4, 5],
    //         20 => vec![1, -4, -5],
    //         24 => vec![-1, -5, -6],
    //         p if p % 5 == 0 => vec![-4, -5, 1, 5, 6],
    //         p if (p + 1) % 5 == 0 => vec![-1, -5, -6, 4, 5],
    //         _ => vec![-1, -4, -5, -6, 1, 4, 5, 6],
    //     };

    //     for cell in &cells {
    //         let current_position = position as i8 + *cell;
    //         if !(0..=24).contains(&current_position) {
    //             continue;
    //         }
    //         if board[current_position as usize] == Entity::Empty {
    //             board[current_position as usize] = Entity::Boom;
    //         }
    //     }
    // }
}
