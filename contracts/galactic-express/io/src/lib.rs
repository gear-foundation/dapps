#![no_std]

use gear_lib::tx_manager::TransactionManagerError;
use gmeta::{InOut, Metadata, Out};
use gstd::{errors::Error as GstdError, prelude::*, ActorId};

pub struct ContractMetadata;

impl Metadata for ContractMetadata {
    type Init = InOut<Initialize, Result<(), Error>>;
    type Handle = InOut<Action, Result<Event, Error>>;
    type Reply = ();
    type Others = ();
    type Signal = ();
    type State = Out<State>;
}

pub const PARTICIPANTS: usize = 4;
pub const TURNS: usize = 3;

/// Represents a range of the minimum & the maximum fuel price.
pub const FUEL_PRICE: (u8, u8) = (80, 120);
/// Represents a range of the minimum & the maximum reward for a session.
pub const REWARD: (u128, u128) = (80, 360);
/// Represents a range of the minimum & the maximum turn altitude.
pub const TURN_ALTITUDE: (u16, u16) = (2_600, 5_000);

#[derive(Encode, Decode, TypeInfo, Debug)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct State {
    /// Can be changed with [`Action::ChangeAdmin`].
    pub admin: ActorId,
    /// The address of the Sharded fungible token contract.
    ///
    /// Can be changed with [`Action::ChangeSft`].
    pub sft: ActorId,

    pub session: Session,
    pub stage: Stage,
}

#[derive(Encode, Decode, TypeInfo, Debug, PartialEq, Eq)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct Session {
    pub id: u128,
    pub altitude: u16,
    pub weather: Weather,
    pub fuel_price: u8,
    pub reward: u128,
}

#[derive(Encode, Decode, TypeInfo, Debug)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum Stage {
    Registration(Vec<(ActorId, Participant)>),
    Results(Results),
}

#[derive(Encode, Decode, TypeInfo, Default, Clone, Debug, PartialEq, Eq)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct Results {
    pub turns: Vec<Vec<(ActorId, TurnOutcome)>>,
    pub rankings: Vec<(ActorId, u128)>,
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct Initialize {
    pub admin: ActorId,
    pub sft: ActorId,
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum Action {
    ChangeAdmin(ActorId),
    ChangeSft(ActorId),
    CreateNewSession,
    Register(Participant),
    StartGame(Participant),
}

#[derive(Encode, Decode, TypeInfo, Debug, PartialEq, Eq)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum Event {
    AdminChanged(ActorId, ActorId),
    SftChanged(ActorId, ActorId),
    NewSession(Session),
    Registered(ActorId, Participant),
    GameFinished(Results),
}

#[derive(Encode, Decode, TypeInfo, Clone, Copy, Debug, Default, PartialEq, Eq)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct Participant {
    pub fuel: u8,
    pub payload: u8,
}

impl Participant {
    pub fn check(&self) -> Result<(), Error> {
        if self.fuel > 100 || self.payload > 100 {
            Err(Error::FuelOrPayloadOverload)
        } else {
            Ok(())
        }
    }
}

#[derive(Encode, Decode, TypeInfo, Clone, Copy, Debug, PartialEq, Eq)]
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

#[derive(Encode, Decode, TypeInfo, Clone, Copy, Debug, PartialEq, Eq)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum TurnOutcome {
    Alive { fuel_left: u8, payload: u8 },
    Destroyed(HaltReason),
}

#[derive(Encode, Decode, TypeInfo, Default, Clone, Copy, Debug, PartialEq, Eq)]
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

#[derive(Encode, Decode, TypeInfo, Debug, PartialEq, Eq)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum Error {
    StateUninitaliazed,
    GstdError(String),
    AccessDenied,
    SessionEnded,
    FuelOrPayloadOverload,
    SessionFull,
    NotEnoughParticipants,
    TransferFailed,
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
