#![no_std]
use gstd::{prelude::*, ActorId};
pub type Price = u32;
pub type Rent = u32;
pub type Gears = Vec<Gear>;
#[derive(Encode, Decode, TypeInfo)]
pub struct YourTurn {
    pub players: BTreeMap<ActorId, PlayerInfo>,
    pub properties: Vec<Option<(ActorId, Gears, Price, Rent)>>,
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
        players: BTreeMap<ActorId, PlayerInfo>,
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
