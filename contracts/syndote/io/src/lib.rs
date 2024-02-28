#![no_std]

use gmeta::{In, InOut, Metadata};
use gstd::{
    collections::{BTreeSet, HashMap, HashSet},
    prelude::*,
    ActorId, MessageId, ReservationId,
};

pub type Price = u32;
pub type Rent = u32;
pub type Gears = Vec<Gear>;
pub type AdminId = ActorId;
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
    /// The game's IS is the admin's address.
    ///
    /// - `entry_fee`: participation fee for the game
    CreateGameSession { entry_fee: Option<u128> },

    /// Message to reserve gas for a specific game session.
    /// During the game, which takes place entirely on-chain, the contract sends messages to game strategies in sequence,
    /// processes the replies, and monitors the correctness of the strategy replies.
    /// Gas is reserved for these actions.
    ///
    /// - `admin_id`: the admin of the game.
    MakeReservation { admin_id: AdminId },

    /// Message for player registration.
    /// The player specifies the address of their strategy (the strategy must be preloaded in advance)
    /// and the session ID in which they wish to register.
    ///
    /// - `admin_id`: the admin of the game.
    /// - `strategy_id`: the address of the player strategy.
    Register {
        admin_id: AdminId,
        strategy_id: ActorId,
    },

    /// Message to start the game.
    /// It must be sent by the game's admin (the account that created the game session).
    ///
    /// - `admin_id`: the admin of the game.
    Play { admin_id: AdminId },

    /// Message to add gas to the player's strategy to continue the game.
    ///
    /// - `admin_id`: the admin of the game.
    AddGasToPlayerStrategy { admin_id: AdminId },

    /// Message to cancel game session.
    ///
    /// - `admin_id`: the admin of the game.
    CancelGameSession { admin_id: AdminId },

    /// Message to leave game
    /// Can be called before the start of the game (for example, if the admin takes too long to start the game)
    /// and during the game, if the game has stopped due to a lack of gas and the admin does not add more.
    ///
    /// - `admin_id`: the admin of the game.
    ExitGame { admin_id: AdminId },
}

#[derive(PartialEq, Eq, Debug, Encode, Decode, TypeInfo)]
pub enum GameReply {
    /// Reply on `CreateGameSession` message
    GameSessionCreated {
        admin_id: AdminId,
    },

    /// Reply on `MakeReservation` message
    ReservationMade,

    /// Reply on `Register` message
    StrategyRegistered,

    /// Reply on `Play` message
    /// in case of successful completion of the game
    GameFinished {
        admin_id: AdminId,
        winner: ActorId,
    },

    /// Reply on `AddGasToPlayerStrategy`
    GasForPlayerStrategyAdded,

    /// Reply on `CancelGame`
    GameWasCancelled,

    /// Reply on `ExitGame`
    PlayerLeftGame,

    /// Event for the front-end app
    Step {
        players: Vec<(ActorId, PlayerInfo)>,
        properties: Vec<Option<(ActorId, Gears, Price, Rent)>>,
        current_player: ActorId,
        ownership: Vec<ActorId>,
        current_step: u64,
    }, 

    /// Reply on `Play`` message, in case when the current gas runs out, 
    /// the next reservation is taken and the next game cycle is started from the new reservation
    NextRoundFromReservation,
}

#[derive(Debug, Encode, Decode, TypeInfo, PartialEq, Eq)]
pub enum GameError {
    /// Error reply on `Register`
    /// In case if this strategy is already registered
    StrategyAlreadyReistered,

    /// Error reply on `Register`
    /// In case if the account is already registered in the game
    AccountAlreadyRegistered,

    /// Error reply on `ExitGame`
    /// In case if strategy for this account doesn't exist
    StrategyDoesNotExist,

    /// Error reply during making reservation
    ReservationError,

    /// Error reply in case `msg::source()` is not an admin
    OnlyAdmin,

    /// Error reply in case the player does not exist
    PlayerDoesNotExist,

    NoGasForPlaying,

    /// Error reply on case the
    WrongGameStatus,

    /// Error reply in case `msg::source()` is neither admin nor the program
    MsgSourceMustBeAdminOrProgram,

    /// Error reply in case game does not exist
    GameDoesNotExist,

    /// Error reply on case the reservation is no more valid
    ReservationNotValid,

    /// Error reply in case of insufficient gas
    /// for the game contract during the game.
    AddGasToGameContract,

    /// Error reply on `Play` message
    /// in case of insufficient gas for strategy
    AddGasForStrategy(ActorId),

    /// Error reply on `CreateGameSession`
    /// In case a game session has already been created for the specified account.
    GameSessionAlreadyExists,

    /// Error reply on `CreateGameSession`
    /// In case if indicated fee is less than ED
    FeeIsLessThanED,

    /// Error reply on `Register`
    /// In case a player didn't attach the required amount of value
    WrongValueAmount,
}

/// Type that should be used to query the state of contract.
#[derive(Debug, PartialEq, Eq, Clone, TypeInfo, Encode, Decode)]
pub enum StateQuery {
    /// Query to get the game session
    GetGameSession { admin_id: AdminId },

    /// Query to get the player info from the game session
    GetPlayerInfo {
        admin_id: AdminId,
        account_id: ActorId,
    },

    /// Query to get the owner address of the indicated strategy
    GetOwnerId {
        admin_id: AdminId,
        strategy_id: ActorId,
    },
}

#[derive(Debug, PartialEq, Eq, Clone, TypeInfo, Encode, Decode)]
pub enum StateReply {
    /// Reply on query `GetGameSession`
    GameSession { game_session: Option<GameState> },

    /// Reply on query `GetPlayerInfo`
    PlayerInfo { player_info: Option<PlayerInfo> },

    /// Reply on query  `GetOwnerId`
    OwnerId { owner_id: Option<ActorId> },
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

#[derive(PartialEq, Eq, Debug, Clone, Encode, Decode, TypeInfo, Default)]
pub struct PlayerInfo {
    pub owner_id: ActorId,
    pub position: u8,
    pub balance: u32,
    pub debt: u32,
    pub in_jail: bool,
    pub round: u128,
    pub cells: BTreeSet<u8>,
    pub penalty: u8,
    pub lost: bool,
    pub reservation_id: Option<ReservationId>,
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
    pub gas_refill_timeout: u32,
    pub gas_for_step: u64,
}

impl From<Game> for GameState {
    fn from(game: Game) -> Self {
        GameState {
            admin_id: game.admin_id,
            properties_in_bank: game.properties_in_bank.into_iter().collect(),
            round: game.round,
            players: game.players.into_iter().collect(),
            owners_to_strategy_ids: game.owners_to_strategy_ids.into_iter().collect(),
            players_queue: game.players_queue,
            current_turn: game.current_turn,
            current_player: game.current_player,
            current_step: game.current_step,
            properties: game.properties,
            ownership: game.ownership,
            game_status: game.game_status,
            winner: game.winner,
            reservations: game.reservations,
            entry_fee: game.entry_fee,
            prize_pool: game.prize_pool,
        }
    }
}
#[derive(Clone, Default, Debug)]
pub struct Game {
    pub admin_id: AdminId,
    pub properties_in_bank: HashSet<u8>,
    pub round: u128,
    // strategy ID to PlayerInfo
    pub players: HashMap<ActorId, PlayerInfo>,
    pub owners_to_strategy_ids: HashMap<ActorId, ActorId>,
    pub players_queue: Vec<ActorId>,
    pub current_turn: u8,
    pub current_player: ActorId,
    pub current_step: u64,
    // mapping from cells to built properties,
    pub properties: Vec<Option<(ActorId, Gears, Price, Rent)>>,
    // mapping from cells to accounts who have properties on it
    pub ownership: Vec<ActorId>,
    pub game_status: GameStatus,
    pub winner: ActorId,
    pub current_msg_id: MessageId,
    pub reservations: Vec<ReservationId>,
    pub entry_fee: Option<u128>,
    pub prize_pool: u128,
}

#[derive(Debug, PartialEq, Eq, Clone, TypeInfo, Encode, Decode, Default)]
pub struct GameState {
    pub admin_id: ActorId,
    pub properties_in_bank: Vec<u8>,
    pub round: u128,
    pub players: Vec<(ActorId, PlayerInfo)>,
    pub owners_to_strategy_ids: Vec<(ActorId, ActorId)>,
    pub players_queue: Vec<ActorId>,
    pub current_turn: u8,
    pub current_player: ActorId,
    pub current_step: u64,
    // mapping from cells to built properties,
    pub properties: Vec<Option<(ActorId, Gears, Price, Rent)>>,
    // mapping from cells to accounts who have properties on it
    pub ownership: Vec<ActorId>,
    pub game_status: GameStatus,
    pub winner: ActorId,
    pub reservations: Vec<ReservationId>,
    pub entry_fee: Option<u128>,
    pub prize_pool: u128,
}
