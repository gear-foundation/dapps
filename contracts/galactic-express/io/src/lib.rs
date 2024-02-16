#![no_std]

use gear_lib::tx_manager::TransactionManagerError;
use gmeta::{InOut, Metadata, Out};
use gstd::{errors::Error as GstdError, prelude::*, ActorId};

pub struct ContractMetadata;

impl Metadata for ContractMetadata {
    type Init = Out<Result<(), Error>>;
    type Handle = InOut<Action, Result<Event, Error>>;
    type Reply = ();
    type Others = ();
    type Signal = ();
    type State = Out<State>;
}

pub const PARTICIPANTS: usize = 4;
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

#[derive(Encode, Decode, TypeInfo, Debug)]
pub struct State {
    pub admin: ActorId,
    pub session: Session,
    pub is_session_ended: bool,
    pub participants: Vec<(ActorId, Participant)>,
    pub turns: Vec<Vec<(ActorId, Turn)>>,
    pub rankings: Vec<(ActorId, u128)>,
}

#[derive(Encode, Decode, TypeInfo, Debug, PartialEq, Eq)]
pub struct Session {
    pub session_id: u128,
    pub altitude: u16,
    pub weather: Weather,
    pub reward: u128,
}

#[derive(Encode, Decode, TypeInfo, Debug)]
pub enum Stage {
    Registration(Vec<(ActorId, Participant)>),
    Results(Results),
}

#[derive(Encode, Decode, TypeInfo, Default, Clone, Debug, PartialEq, Eq)]
pub struct Results {
    pub turns: Vec<Vec<(ActorId, Turn)>>,
    pub rankings: Vec<(ActorId, u128)>,
}

#[derive(Encode, Decode, TypeInfo)]
pub enum Action {
    ChangeAdmin(ActorId),
    CreateNewSession,
    Register(Participant),
    StartGame(Participant),
}

#[derive(Encode, Decode, TypeInfo, Debug, PartialEq, Eq)]
pub enum Event {
    GameFinished(Results),
    AdminChanged(ActorId, ActorId),
    NewSession(Session),
    Registered(ActorId, Participant),
}

#[derive(Encode, Decode, TypeInfo, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Participant {
    pub fuel_amount: u8,
    pub payload_amount: u8,
}

impl Participant {
    pub fn check(&self) -> Result<(), Error> {
        if self.fuel_amount > MAX_FUEL || self.payload_amount > MAX_PAYLOAD {
            Err(Error::FuelOrPayloadOverload)
        } else {
            Ok(())
        }
    }
}

#[derive(Encode, Decode, TypeInfo, Clone, Copy, Debug, PartialEq, Eq)]
pub enum HaltReason {
    PayloadOverload,
    FuelOverload,
    SeparationFailure,
    AsteroidCollision,
    FuelShortage,
    EngineFailure,
}

#[derive(Encode, Decode, TypeInfo, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Turn {
    Alive { fuel_left: u8, payload_amount: u8 },
    Destroyed(HaltReason),
}

#[derive(Encode, Decode, TypeInfo, Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Weather {
    #[default]
    Clear,
    Cloudy,
    Rainy,
    Stormy,
    Thunder,
    Tornado,
}

#[derive(Encode, Decode, TypeInfo, Debug, PartialEq, Eq)]
pub enum Error {
    StateUninitaliazed,
    GstdError(String),
    AccessDenied,
    SessionEnded,
    FuelOrPayloadOverload,
    SessionFull,
    NotEnoughParticipants,
    TxManager(TransactionManagerError),
}

impl From<GstdError> for Error {
    fn from(error: GstdError) -> Self {
        Error::GstdError(error.to_string())
    }
}

impl From<TransactionManagerError> for Error {
    fn from(error: TransactionManagerError) -> Self {
        Self::TxManager(error)
    }
}
