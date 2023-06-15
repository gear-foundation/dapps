#![no_std]

use codec::{Decode, Encode};
use gmeta::{In, InOut, Metadata};
use gstd::{prelude::*, ActorId};

pub struct ProgramMetadata;

impl Metadata for ProgramMetadata {
    type Init = In<Initialize>;
    type Handle = InOut<Action, Event>;
    type Reply = InOut<(), ()>;
    type Others = InOut<(), ()>;
    type Signal = ();
    type State = LaunchSite;
}

#[derive(Encode, Decode, TypeInfo, Debug)]
pub struct Initialize {
    pub name: String,
    pub period_sec: u32,
}

#[derive(Encode, Decode, TypeInfo, Debug)]
pub enum Action {
    Info,
    // RegisterParticipant(String),
    ChangeParticipantName(String),
    StartNewSession,
    // RegisterOnLaunch {
    //     fuel_amount: u32,
    //     payload_amount: u32,
    // },
    RegisterParticipantOnLaunch {
        name: String,
        fuel_amount: u32,
        payload_amount: u32,
    },
    ExecuteSession,
    ReserveGas,
}

#[derive(Encode, Debug, PartialEq, Eq, Decode, TypeInfo)]
pub enum Event {
    Info {
        owner: ActorId,
        name: String,
        has_current_session: bool,
    },
    NewParticipant {
        id: ActorId,
        name: String,
    },
    ParticipantNameChange {
        id: ActorId,
        name: String,
    },
    NewLaunch {
        id: u32,
        name: String,
        weather: u32,
        altitude: u32,
        fuel_price: u32,
        payload_value: u32,
    },
    LaunchRegistration {
        id: u32,
        participant: ActorId,
    },
    LaunchStarted {
        id: u32,
    },
    LaunchFinished {
        id: u32,
        stats: Vec<(ActorId, bool, u32, u128)>, // participant id, success, final altitude, earnings
    },
    SessionInfo {
        weather: u32,
        altitude: u32,
        fuel_price: u32,
        payload_value: u32,
    },
    NoCurrentSession,
    GasReserved,
}

#[derive(Default, Encode, Decode, TypeInfo)]
pub struct CurrentSesionInfo {
    pub name: String,
    pub weather: u32,
    pub altitude: u32,
    pub fuel_price: u32,
    pub payload_value: u32,
}

#[derive(Default, Encode, Decode, TypeInfo, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct CurrentStat {
    pub participant: ActorId,
    pub alive: bool,
    pub fuel_left: u32,
    pub fuel_capacity: u32,
    pub last_altitude: u32,
    pub payload: u32,
    pub halt: Option<RocketHalt>,
}

#[derive(Default, Encode, Decode, TypeInfo, Debug)]
pub struct LaunchSite {
    pub name: String,
    pub owner: ActorId,
    pub participants: BTreeMap<ActorId, Participant>,
    pub current_session: Option<CurrentSession>,
    pub events: BTreeMap<u32, BTreeSet<CurrentStat>>,
    pub state: SessionState,
    pub session_id: u32,
}

#[derive(Default, Encode, Decode, TypeInfo, Debug)]
pub struct SessionStrategy {
    pub fuel: u32,
    pub payload: u32,
}

#[derive(Default, Encode, Decode, TypeInfo, Debug)]
pub struct CurrentSession {
    pub altitude: u32,
    pub weather: u32,
    pub fuel_price: u32,
    pub reward: u128,
    pub registered: BTreeMap<ActorId, (SessionStrategy, Participant)>,
    pub bet: Option<u128>,
}

#[derive(Default, Encode, Decode, TypeInfo, Debug, Clone)]
pub struct Participant {
    pub name: String,
    pub score: u128,
    pub balance: u128,
}

#[derive(Encode, Decode, TypeInfo, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum RocketHalt {
    Overfilled,
    Overfuelled,
    SeparationFailure,
    Asteroid,
    NotEnoughFuel,
    EngineError,
}

#[derive(Default, Encode, Decode, TypeInfo, Debug, Clone, PartialEq, Eq)]
pub enum SessionState {
    SessionIsOver,
    #[default]
    NoSession,
    Registration,
}
