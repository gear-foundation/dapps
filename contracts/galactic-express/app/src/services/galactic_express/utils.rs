use crate::services::galactic_express::Storage;
use num_traits::FromBytes;
use sails_rs::{
    collections::HashMap,
    errors::Error as GstdError,
    gstd::exec,
    ops::{Add, Rem, Sub},
    prelude::*,
};

pub const MAX_PARTICIPANTS: usize = 4;
pub const TURNS: usize = 3;

/// Represents a range of the minimum & the maximum reward for a session.
pub const REWARD: (u128, u128) = (80, 360);
/// Represents a range of the minimum & the maximum turn altitude.
pub const TURN_ALTITUDE: (u16, u16) = (500, 1_000);
/// Dangerous level for high fuel and payload values
/// This is to account for the scenario where a player specifies a significant amount of fuel
/// or a large payload, resulting in a greater likelihood of mission failure.
pub const PENALTY_LEVEL: u8 = 80;
// maximum fuel value that can be entered by the user
pub const MAX_FUEL: u8 = 100;
// maximum payload value that can be entered by the user
pub const MAX_PAYLOAD: u8 = 100;

pub struct Random {
    index: usize,
    random: [u8; 32],
}

impl Random {
    pub fn new() -> Result<Self, GameError> {
        exec::random([0; 32])
            .map(|(random, _)| Self { index: 0, random })
            .map_err(|error| GstdError::from(error).into())
    }

    pub fn next(&mut self) -> u8 {
        let next = *self
            .random
            .get(self.index)
            .expect("index for the random array traversing must'n overflow");

        self.index += 1;

        next
    }

    pub fn generate<T, const N: usize>(&mut self, min: T, max: T) -> T
    where
        T: FromBytes<Bytes = [u8; N]>
            + Add<T, Output = T>
            + Sub<T, Output = T>
            + Rem<T, Output = T>
            + Copy,
    {
        min + T::from_le_bytes(&array::from_fn(|_| self.next())) % (max - min)
    }

    pub fn chance(&mut self, probability: u8) -> bool {
        assert!(probability < 101, "probability can't be more than 100");

        self.next() % 100 < probability
    }
}

#[derive(Encode, Decode, TypeInfo, Debug, PartialEq, Eq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum GameError {
    StateUninitaliazed,
    GstdError(String),
    SessionEnded,
    FuelOrPayloadOverload,
    SessionFull,
    NotEnoughParticipants,
    NoSuchGame,
    WrongBid,
    NoSuchPlayer,
    Unregistered,
    AlreadyRegistered,
    SeveralRegistrations,
    NotForAdmin,
    DeniedAccess,
}

impl From<GstdError> for GameError {
    fn from(error: GstdError) -> Self {
        GameError::GstdError(error.to_string())
    }
}

#[derive(Encode, Decode, TypeInfo, Default, Clone, Copy, Debug, PartialEq, Eq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum Weather {
    #[default]
    Clear,
    Cloudy,
    Rainy,
    Stormy,
    Thunder,
    Tornado,
}

pub enum Stage {
    Registration(HashMap<ActorId, Participant>),
    Results(Results),
}

impl Stage {
    pub fn mut_participants(&mut self) -> Result<&mut HashMap<ActorId, Participant>, GameError> {
        if let Stage::Registration(participants) = self {
            Ok(participants)
        } else {
            Err(GameError::SessionEnded)
        }
    }
}

impl Default for Stage {
    fn default() -> Self {
        Self::Results(Results::default())
    }
}

#[derive(Encode, Decode, TypeInfo, Default, Clone, Debug, PartialEq, Eq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct Results {
    pub turns: Vec<Vec<(ActorId, Turn)>>,
    pub rankings: Vec<(ActorId, u128)>,
    pub participants: Vec<(ActorId, Participant)>,
}

#[derive(Encode, Decode, TypeInfo, Clone, Debug, Default, PartialEq, Eq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct Participant {
    pub id: ActorId,
    pub name: String,
    pub fuel_amount: u8,
    pub payload_amount: u8,
}

impl Participant {
    pub fn check(&self) -> Result<(), GameError> {
        if self.fuel_amount > MAX_FUEL || self.payload_amount > MAX_PAYLOAD {
            Err(GameError::FuelOrPayloadOverload)
        } else {
            Ok(())
        }
    }
}

#[derive(Encode, Decode, TypeInfo, Clone, Copy, Debug, PartialEq, Eq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum Turn {
    Alive { fuel_left: u8, payload_amount: u8 },
    Destroyed(HaltReason),
}

#[derive(Encode, Decode, TypeInfo, Clone, Copy, Debug, PartialEq, Eq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum HaltReason {
    PayloadOverload,
    FuelOverload,
    SeparationFailure,
    AsteroidCollision,
    FuelShortage,
    EngineFailure,
}

#[derive(Encode, Decode, TypeInfo, Debug)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct State {
    pub games: Vec<(ActorId, GameState)>,
    pub player_to_game_id: Vec<(ActorId, ActorId)>,
    pub dns_info: Option<(ActorId, String)>,
    pub admin: ActorId,
}

#[derive(Encode, Decode, TypeInfo, Debug)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct GameState {
    pub admin: ActorId,
    pub admin_name: String,
    pub altitude: u16,
    pub weather: Weather,
    pub reward: u128,
    pub stage: StageState,
    pub bid: u128,
}

#[derive(Encode, Decode, TypeInfo, Debug)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum StageState {
    Registration(Vec<(ActorId, Participant)>),
    Results(Results),
}

impl From<&Storage> for State {
    fn from(value: &Storage) -> Self {
        let Storage {
            games,
            player_to_game_id,
            dns_info,
            admin,
        } = value;

        let games = games
            .iter()
            .map(|(id, game)| {
                let stage = match &game.stage {
                    Stage::Registration(participants_data) => StageState::Registration(
                        participants_data
                            .iter()
                            .map(|(actor_id, participant)| (*actor_id, participant.clone()))
                            .collect(),
                    ),
                    Stage::Results(results) => StageState::Results(results.clone()),
                };

                let game_state = GameState {
                    admin: game.admin,
                    admin_name: game.admin_name.clone(),
                    altitude: game.altitude,
                    weather: game.weather,
                    reward: game.reward,
                    stage,
                    bid: game.bid,
                };
                (*id, game_state)
            })
            .collect();

        let player_to_game_id = player_to_game_id.iter().map(|(k, v)| (*k, *v)).collect();

        Self {
            games,
            player_to_game_id,
            dns_info: dns_info.clone(),
            admin: *admin,
        }
    }
}
