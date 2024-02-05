#![no_std]

use gmeta::{InOut, Metadata, In};
use gstd::{collections::BTreeSet,MessageId, prelude::*, ActorId, ReservationId};

pub type Price = u32;
pub type Rent = u32;
pub type Gears = Vec<Gear>;
pub type ValidUntilBlock = u32;
pub type SessionId = u32;
#[derive(Encode, Decode, TypeInfo)]
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
    type State = InOut<StateQuery, StateReply>;
}

#[derive(Encode, Decode, TypeInfo)]
pub struct GameInfo {
    pub properties_in_bank: Vec<u8>,
    pub players: Vec<(ActorId, PlayerInfo)>,
    pub players_queue: Vec<ActorId>,
    // mapping from cells to built properties,
    pub properties: Vec<Option<(ActorId, Gears, u32, u32)>>,
    // mapping from cells to accounts who have properties on it
    pub ownership: Vec<ActorId>,
}

/// Type that should be used to send a message to the contract.
#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum GameAction {
    /// Message to create a game session.
    /// Following this, a Game structure is created, 
    /// where the account that sent the message becomes the admin of the game. 
    /// An ID is assigned to the Game structure to uniquely identify the session.
    CreateGameSession,

    /// Message to reserve gas for a specific game session. 
    /// During the game, which takes place entirely on-chain, the contract sends messages to game strategies in sequence, 
    /// processes the replies, and monitors the correctness of the strategy replies. 
    /// Gas is reserved for these actions.
    /// 
    /// - `session_id`: the ID of the session.
    MakeReservation {
        session_id: SessionId,
    },

    /// Message for player registration. 
    /// The player specifies the address of their strategy (the strategy must be preloaded in advance) 
    /// and the session ID in which they wish to register.
    /// 
    /// - `session_id`: the ID of the session.
    /// - `strategy_id`: the address of the player strategy.
    Register {  session_id: SessionId, strategy_id: ActorId },

    /// Message to start the game. 
    /// It must be sent by the game's admin (the account that created the game session).
    /// 
    /// - `session_id`: the ID of the session.
    Play {
        session_id: SessionId,
    },

    /// Message to add gas to the player's strategy to continue the game.
    /// 
    /// - `session_id`: the ID of the session.
    AddGasToPlayerStrategy {
        session_id: SessionId,
    }
}

#[derive(Debug, Encode, Decode, TypeInfo)]
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
pub enum GameReply {
    // Reply on `CreateGameSession` message
    GameSessionCreated {
        session_id: SessionId,
    },

    // Reply on `MakeReservation` message
    ReservationMade,

    // Reply on `Register` message
    StrategyRegistered,

    // Reply on `Play` message
    // in case of successful completion of the game
    GameFinished {
        session_id: SessionId,
        winner: ActorId,
    },

    // Reply on `AddGasToPlayerStrategy`
    GasForPlayerStrategyAdded,

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
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum GameError {
    AlreadyReistered,
    ReservationError,
    OnlyAdmin,
    NotInTheGame,
    StrategicError,
    PlayerDoesNotExist,
    NoGasForPlaying,
    WrongGameStatus,
    MsgSourceMustBeAdminOrProgram,
    GameDoesNotExist,
    ReservationNotValid,
    
    // Error reply in case of insufficient gas 
    // for the game contract during the game.
    AddGasToGameContract,
}

#[derive(PartialEq, Eq, Debug, Clone, Encode, Decode, TypeInfo)]
pub struct PlayerInfo {
    pub position: u8,
    pub balance: u32,
    pub debt: u32,
    pub in_jail: bool,
    pub round: u128,
    pub cells: BTreeSet<u8>,
    pub penalty: u8,
    pub lost: bool,
    pub reservation_id: (ReservationId, ValidUntilBlock),
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
            reservation_id: (gcore::ReservationId::from([0; 32]).into(), 0),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Encode, Decode, Clone, TypeInfo, Copy)]
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
    Wait,
    WaitingForGasForGameContract,
    WaitingForGasForStrategy(ActorId),
}

impl Default for GameStatus {
    fn default() -> Self {
        Self::Registration
    }
}

#[derive(Default, Clone, Encode, Decode, TypeInfo)]
pub struct Config {
    pub reservation_amount: u64,
    pub reservation_duration_in_block: u32,
    pub time_for_step: u32,
    pub min_gas_limit: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, TypeInfo, Encode, Decode)]
pub enum StateQuery {
    MessageId,
    GameState,
}

#[derive(Debug, PartialEq, Eq, Clone, TypeInfo, Encode, Decode)]
pub enum StateReply {
    MessageId(MessageId),
    GameState(GameState),
}

#[derive(Debug, PartialEq, Eq, Clone, TypeInfo, Encode, Decode)]
pub struct GameState {
    pub admin: ActorId,
    pub properties_in_bank: Vec<u8>,
    pub round: u128,
    pub players: Vec<(ActorId, PlayerInfo)>,
    pub players_queue: Vec<ActorId>,
    pub current_turn: u8,
    pub current_player: ActorId,
    pub current_step: u64,
    // mapping from cells to built properties,
    pub properties: Vec<Option<(ActorId, Gears, Price, Rent)>>,
    // mapping from cells to accounts who have properties on it
    pub  ownership: Vec<ActorId>,
    pub game_status: GameStatus,
    pub winner: ActorId,
}
