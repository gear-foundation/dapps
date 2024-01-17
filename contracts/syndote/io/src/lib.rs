#![no_std]

use gmeta::{InOut, Metadata, Out, In};
use gstd::{collections::BTreeSet,MessageId, prelude::*, ActorId, ReservationId};

pub type Price = u32;
pub type Rent = u32;
pub type Gears = Vec<Gear>;

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct YourTurn {
    pub game_info: GameInfo,
}

pub struct SynMetadata;

impl Metadata for SynMetadata {
    type Init = In<Config>;
    type Handle = InOut<GameAction, Result<GameReply, GameError>>;
    type Reply = ();
    type Others = ();
    type Signal = ();
    type State = Out<GameState>;
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct GameInfo {
    pub properties_in_bank: Vec<u8>,
    pub players: Vec<(ActorId, PlayerInfo)>,
    pub players_queue: Vec<ActorId>,
    // mapping from cells to built properties,
    pub properties: Vec<Option<(ActorId, Gears, u32, u32)>>,
    // mapping from cells to accounts who have properties on it
    pub ownership: Vec<ActorId>,
}

#[derive(Clone, Default, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct GameState {
    pub admin: ActorId,
    pub properties_in_bank: Vec<u8>,
    pub round: u128,
    pub players: Vec<(ActorId, PlayerInfo)>,
    pub players_queue: Vec<ActorId>,
    pub current_player: ActorId,
    pub current_step: u64,
    // mapping from cells to built properties,
    pub properties: Vec<Option<(ActorId, Gears, u32, u32)>>,
    // mapping from cells to accounts who have properties on it
    pub ownership: Vec<ActorId>,
    pub game_status: GameStatus,
    pub winner: ActorId,
}

#[derive(Debug, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum GameAction {
    StartRegistration,
    MakeReservation,
    Register { player: ActorId },
    Play,
    ChangeAdmin(ActorId),
}

#[derive(Debug, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum StrategicAction {
    ThrowRoll {
        pay_fine: bool,
        properties_for_sale: Option<Vec<u8>>,
    },
    AddGear {
        properties_for_sale: Option<Vec<u8>>,
    },
    Upgrade {
        properties_for_sale: Option<Vec<u8>>,
    },
    BuyCell {
        properties_for_sale: Option<Vec<u8>>,
    },
    PayRent {
        properties_for_sale: Option<Vec<u8>>,
    },
    Skip,
}

#[derive(PartialEq, Eq,Debug, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum GameReply {
    Registered,
    RegistrationStarted,
    GameFinished {
        winner: ActorId,
    },
    StrategicError,
    StrategicSuccess,
    Step {
        players: Vec<(ActorId, PlayerInfo)>,
        properties: Vec<Option<(ActorId, Gears, Price, Rent)>>,
        current_player: ActorId,
        ownership: Vec<ActorId>,
        current_step: u64,
    },
    Jail {
        in_jail: bool,
        position: u8,
    },
    GasReserved,
    NextRoundFromReservation,
    AdminChanged,
    ReservationMade,
}

#[derive(Debug, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum GameError {
    AlreadyReistered,
    ReservationError,
    WrongGameStatus,
    OnlyAdmin,
    NotInTheGame,
    StrategicError,
    PlayerDoesNotExist,
    NoGasForPlaying,
}
#[derive(PartialEq, Eq, Debug, Clone, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct PlayerInfo {
    pub position: u8,
    pub balance: u32,
    pub debt: u32,
    pub in_jail: bool,
    pub round: u128,
    pub cells: BTreeSet<u8>,
    pub penalty: u8,
    pub lost: bool,
    pub reservation_id: ReservationId,
}

impl Default for PlayerInfo {
    fn default() -> Self {
        PlayerInfo {
            position: 0,
            balance: 0,
            debt: 0,
            in_jail: false,
            round: 0,
            cells: BTreeSet::new(),
            penalty: 0,
            lost: false,
            reservation_id: gcore::ReservationId::from([0; 32]).into(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Encode, Decode, Clone, TypeInfo, Copy)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum Gear {
    Bronze,
    Silver,
    Gold,
}

impl Gear {
    pub fn upgrade(&self) -> Self {
        match *self {
            Self::Bronze => Self::Silver,
            Self::Silver => Self::Gold,
            Self::Gold => Self::Gold,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, TypeInfo, Encode, Decode)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum GameStatus {
    Registration,
    Play,
    Finished,
    Wait,
}

impl Default for GameStatus {
    fn default() -> Self {
        Self::Registration
    }
}

#[derive(Default, Clone, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct Config {
    pub reservation_amount: u64,
    pub reservation_duration: u32,
    pub time_for_step: u32,
    pub min_gas_limit: u64,
    pub wait_duration: u32,   
}

#[derive(Debug, PartialEq, Eq, Clone, TypeInfo, Encode, Decode)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum StateQuery {
    MessageId,
}

#[derive(Debug, PartialEq, Eq, Clone, TypeInfo, Encode, Decode)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum StateReply {
    MessageId(MessageId),
}