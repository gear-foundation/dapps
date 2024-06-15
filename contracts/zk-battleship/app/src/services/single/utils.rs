use gstd::{collections::HashMap, prelude::*, ActorId, Decode, Encode, TypeInfo};

pub type SingleGamesMap = HashMap<ActorId, SingleGame>;
pub(crate) type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rtl::scale_codec)]
#[scale_info(crate = sails_rtl::scale_info)]
pub enum Error {
    WrongStep,
    NoSuchGame,
    GameIsAlreadyOver,
    StatusIsPendingVerification,
    WrongStatusOrHit,
    WrongShipHash,
    WrongOut,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rtl::scale_codec)]
#[scale_info(crate = sails_rtl::scale_info)]
pub struct SingleGame {
    pub player_board: Vec<Entity>,
    pub ship_hash: Vec<u8>,
    pub bot_ships: Ships,
    pub start_time: u64,
    pub status: Status,
    pub total_shots: u8,
    pub succesfull_shots: u8,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rtl::scale_codec)]
#[scale_info(crate = sails_rtl::scale_info)]
pub struct SingleGameState {
    pub player_board: Vec<Entity>,
    pub ship_hash: Vec<u8>,
    pub start_time: u64,
    pub status: Status,
    pub total_shots: u8,
    pub succesfull_shots: u8,
}

impl SingleGame {
    pub fn check_end_game(&self) -> bool {
        let count_boom_ships = self
            .player_board
            .iter()
            .filter(|&entity| *entity == Entity::BoomShip)
            .count();
        count_boom_ships == 8
    }
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
