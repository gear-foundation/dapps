#![no_std]
use gmeta::{InOut, Metadata};
use gstd::{prelude::*, ActorId};
pub type Price = u32;
pub type Rent = u32;
pub type Gears = Vec<Gear>;
#[derive(Encode, Decode, TypeInfo)]
pub struct YourTurn {
    pub players: Vec<(ActorId, PlayerInfo)>,
    pub properties: Vec<Option<(ActorId, Gears, Price, Rent)>>,
}

pub struct SynMetadata;

impl Metadata for SynMetadata {
    type Init = ();
    type Handle = InOut<GameAction, GameEvent>;
    type Reply = ();
    type Others = ();
    type Signal = ();
    type State = GameState;
}

#[derive(Clone, Default, Encode, Decode, TypeInfo)]
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

#[derive(Encode, Decode, TypeInfo)]
pub enum GameAction {
    StartRegistration,
    Register {
        player: ActorId,
    },
    ReserveGas,
    Play,
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
    ChangeAdmin(ActorId),
}

#[derive(Encode, Decode, TypeInfo)]
pub enum GameEvent {
    Registered,
    StartRegistration,
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
}
#[derive(Default, Debug, Clone, Encode, Decode, TypeInfo)]
pub struct PlayerInfo {
    pub position: u8,
    pub balance: u32,
    pub debt: u32,
    pub in_jail: bool,
    pub round: u128,
    pub cells: BTreeSet<u8>,
    pub penalty: u8,
    pub lost: bool,
}

#[derive(PartialEq, Eq, Encode, Decode, Clone, TypeInfo, Copy)]
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
pub enum GameStatus {
    Registration,
    Play,
    Finished,
}

impl Default for GameStatus {
    fn default() -> Self {
        Self::Registration
    }
}
