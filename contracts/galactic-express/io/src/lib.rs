#![no_std]

use gmeta::{InOut, Metadata, Out};
use gstd::{errors::Error as GstdError, prelude::*, ActorId};

pub const MIN_FUEL_PRICE: u8 = 80;
pub const MAX_FUEL_PRICE: u8 = 120;
pub const MIN_REWARD: u128 = 80;
pub const MAX_REWARD: u128 = 360;
pub const MIN_TURN_ALTITUDE: u16 = 2_600;
pub const MAX_TURN_ALTITUDE: u16 = 5_000;
pub const TOTAL_TURNS: usize = 3;

pub struct ContractMetadata;

impl Metadata for ContractMetadata {
    type Init = InOut<(), Result<(), Error>>;
    type Handle = InOut<Action, Result<Event, Error>>;
    type Reply = InOut<(), ()>;
    type Others = InOut<(), ()>;
    type Signal = ();
    type State = Out<State>;
}

#[derive(Encode, Decode, TypeInfo, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct State {
    pub admin: ActorId,
    pub session: Session,
    pub is_session_ended: bool,
    pub participants: Vec<(ActorId, Participant)>,
    pub turns: Vec<Vec<(ActorId, Turn)>>,
    pub rankings: Vec<(ActorId, u128)>,
}

#[derive(Encode, Decode, TypeInfo, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct Session {
    pub session_id: u128,
    pub altitude: u16,
    pub weather: Weather,
    pub fuel_price: u8,
    pub reward: u128,
}

impl Default for Session {
    fn default() -> Self {
        Self {
            session_id: 0,

            altitude: MIN_TURN_ALTITUDE * TOTAL_TURNS as u16,
            weather: Weather::default(),
            fuel_price: MIN_FUEL_PRICE,
            reward: MIN_REWARD,
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            admin: ActorId::zero(),
            session: Session::default(),
            is_session_ended: true,
            participants: vec![],
            turns: vec![],
            rankings: vec![],
        }
    }
}

#[derive(Encode, Decode, TypeInfo, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum Action {
    ChangeAdmin(ActorId),
    CreateNewSession,
    Register(Participant),
    StartGame(Participant),
}

#[derive(Encode, Decode, TypeInfo, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum Event {
    AdminChanged(ActorId, ActorId),
    NewSession(Session),
    Registered(ActorId, Participant),
    GameFinished(Vec<Vec<(ActorId, Turn)>>),
}

#[derive(Encode, Decode, TypeInfo, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum Turn {
    Alive { fuel_left: u8, payload_amount: u8 },
    Destroyed(HaltReason),
}

#[derive(Encode, Decode, TypeInfo, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum Error {
    AccessDenied,
    FullSession,
    EndedSession,
    FuelOrPayloadOverload,
    NotEnoughParticipants,
    GstdError(String),
}

impl From<GstdError> for Error {
    fn from(error: GstdError) -> Self {
        Self::GstdError(error.to_string())
    }
}

#[derive(
    Default, Encode, Decode, TypeInfo, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash,
)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum Weather {
    #[default]
    Clear,
    Cloudy,
    Rainy,
    Stormy,
    Thunder,
    Tornado,
}

#[derive(
    Default, Encode, Decode, TypeInfo, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash,
)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct Participant {
    pub fuel_amount: u8,
    pub payload_amount: u8,
}

#[derive(Encode, Decode, TypeInfo, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum HaltReason {
    PayloadOverload,
    FuelOverload,
    SeparationFailure,
    AsteroidCollision,
    FuelShortage,
    EngineFailure,
}
