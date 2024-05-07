//! Data types for the contract input/output.

#![no_std]

use gmeta::{In, InOut,Out, Metadata};
use gstd::{prelude::*, ActorId};

pub type Cell = Option<Mark>;

/// Time deadline for player turn(30_000ms).
pub const TURN_DEADLINE_MS: u64 = 30_000;

/// Time after which the game instance must be removed
/// 1 block = 3s (1 minutes)
pub const TIME_INTERVAL: u32 = 20;

/// 1 block = 3s
pub const SEC_PER_BLOCK: u32 = 3;

/// Gas for deleting the game instance
pub const GAS_TO_REMOVE_GAME: u64 = 20_000_000_000;

/// Contract metadata. This is the contract's interface description.
///
/// It defines the types of messages that can be sent to the contract.
pub struct ContractMetadata;

impl Metadata for ContractMetadata {
    /// Init message type.
    ///
    /// Describes incoming/outgoing types for the `init()` function.
    ///
    /// The [`GameInit`] type is passed for initial smart-contract data(i.e config..) if any.
    type Init = In<GameInit>;
    /// Handle message type.
    ///
    /// Describes incoming/outgoing types for the `handle()` function.
    ///
    /// We use the [`GameAction`] type for incoming and [`GameReply`] for outgoing
    /// messages.
    type Handle = InOut<GameAction, Result<GameReply, GameError>>;
    /// Asynchronous handle message type.
    ///
    /// Describes incoming/outgoing types for the `main()` function in case of
    /// asynchronous interaction.
    ///
    /// The unit tuple is used as we don't use asynchronous interaction in this
    /// contract.
    type Others = Out<SignatureData>;
    /// Reply message type.
    ///
    /// Describes incoming/outgoing types of messages performed using the
    /// `handle_reply()` function.
    ///
    /// The unit tuple is used as we don't process any replies in this contract.
    type Reply = ();
    /// Signal message type.
    ///
    /// Describes only the outgoing type from the program while processing the
    /// system signal.
    ///
    /// The unit tuple is used as we don't process any signals in this contract.
    type Signal = ();
    /// State message type.
    ///
    /// Describes the type for the queried state returned by the `state()`
    /// function.
    ///
    /// We use a [`StateQuery`] and [`StateReply`]struct.
    type State = InOut<StateQuery, StateReply>;
}

#[derive(Encode, Decode, TypeInfo)]
pub enum StateQuery {
    Admins,
    Game { player_id: ActorId },
    AllGames,
    Config,
    MessagesAllowed,
    SessionForTheAccount(ActorId),
}

#[derive(Encode, Decode, TypeInfo)]
pub struct SignatureData {
    pub key: ActorId,
    pub duration: u64,
    pub allowed_actions: Vec<ActionsForSession>,
}

#[derive(Encode, Decode, TypeInfo)]
pub enum StateReply {
    Admins(Vec<ActorId>),
    Game(Option<GameInstance>),
    AllGames(Vec<(ActorId, GameInstance)>),
    Config(Config),
    MessagesAllowed(bool),
    SessionForTheAccount(Option<Session>),
}

// This structure is for creating a gaming session, which allows players to predefine certain actions for an account that will play the game on their behalf for a certain period of time.
// Sessions can be used to send transactions from a dApp on behalf of a user without requiring their confirmation with a wallet.
// The user is guaranteed that the dApp can only execute transactions that comply with the allowed_actions of the session until the session expires.
#[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
pub struct Session {
    // the address of the player who will play on behalf of the user
    pub key: ActorId,
    // until what time the session is valid
    pub expires: u64,
    // what messages are allowed to be sent by the account (key)
    pub allowed_actions: Vec<ActionsForSession>,

    pub expires_at_block: u32,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
pub enum ActionsForSession {
    StartGame,
    Move, 
    Skip
}

/// Smart-contract input data structure, for example can contain configuration.
#[derive(Encode, Decode, TypeInfo)]
pub struct GameInit {
    pub config: Config,
}

/// The main type used as an input message.
#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum GameAction {
    AddAdmin(ActorId),
    RemoveAdmin(ActorId),
    StartGame {
        session_for_account: Option<ActorId>,
    },
    Turn {
        step: u8,
        session_for_account: Option<ActorId>,
    },
    Skip {
        session_for_account: Option<ActorId>,
    },
    RemoveGameInstance {
        account_id: ActorId,
    },
    RemoveGameInstances {
        accounts: Option<Vec<ActorId>>,
    },
    CreateSession {
        key: ActorId,
        duration: u64,
        allowed_actions: Vec<ActionsForSession>,
        signature: Option<Vec<u8>>,
    },
    DeleteSessionFromProgram {
        account: ActorId,
    },
    DeleteSessionFromAccount,
    UpdateConfig {
        s_per_block: Option<u64>,
        gas_to_remove_game: Option<u64>,
        time_interval: Option<u32>,
        turn_deadline_ms: Option<u64>,
        block_duration_ms: Option<u64>,
        gas_to_delete_session: Option<u64>,
    },
    AllowMessages(bool),
}

/// The main type used as an output message.
#[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
pub enum GameReply {
    GameFinished { game: GameInstance, player_address: ActorId },
    GameStarted { game: GameInstance },
    MoveMade { game: GameInstance },
    GameInstanceRemoved,
    ConfigUpdated,
    AdminRemoved,
    AdminAdded,
    StatusMessagesUpdated,
    SessionCreated,
    SessionDeleted,
}
#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum GameError {
    GameIsAlreadyStarted,
    CellIsAlreadyOccupied,
    GameIsAlreadyOver,
    MissedYourTurn,
    NotMissedTurnMakeMove,
    GameIsNotStarted,
    MessageOnlyForProgram,
    NotAdmin,
    MessageProcessingSuspended,
}

/// Represent game instance status.
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub enum GameResult {
    Player,
    Bot,
    Draw,
}

/// Represent concrete game instance.
#[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
pub struct GameInstance {
    pub board: Vec<Option<Mark>>,
    pub player_mark: Mark,
    pub bot_mark: Mark,
    pub last_time: u64,
    pub game_over: bool,
    pub game_result: Option<GameResult>,
}

/// Indicates tic-tac-toe board mark-state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub enum Mark {
    X,
    O,
}

#[derive(Debug, Default, Encode, Clone, Decode, TypeInfo)]
pub struct Config {
    pub s_per_block: u64,
    pub gas_to_remove_game: u64,
    pub time_interval: u32,
    pub turn_deadline_ms: u64,
    pub block_duration_ms: u64,
    pub gas_to_delete_session: u64
}
