//! Data types for the contract input/output.

#![no_std]

use gmeta::{In, InOut, Metadata};
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
    /// We use the [`GameAction`] type for incoming and [`GameEvent`] for outgoing
    /// messages.
    type Handle = InOut<GameAction, GameReply>;
    /// Asynchronous handle message type.
    ///
    /// Describes incoming/outgoing types for the `main()` function in case of
    /// asynchronous interaction.
    ///
    /// The unit tuple is used as we don't use asynchronous interaction in this
    /// contract.
    type Others = ();
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
    /// We use a [`GameState`] struct.
    type State = InOut<StateQuery, StateReply>;
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum StateQuery {
    Admins,
    Game { player_id: ActorId },
    AllGames,
    Config,
    MessagesAllowed,
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum StateReply {
    Admins(Vec<ActorId>),
    Game(Option<GameInstance>),
    AllGames(Vec<(ActorId, GameInstance)>),
    Config(Config),
    MessagesAllowed(bool),
}

/// Smart-contract input data structure, for example can contain configuration.
#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct GameInit {
    pub config: Config,
}

/// The main type used as an input message.
#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum GameAction {
    AddAdmin(ActorId),
    RemoveAdmin(ActorId),
    StartGame,
    Turn {
        step: u8,
    },
    Skip,
    RemoveGameInstance {
        account_id: ActorId,
    },
    RemoveGameInstances {
        accounts: Option<Vec<ActorId>>,
    },
    UpdateConfig {
        ms_per_block: Option<u64>,
        add_attribute_gas: Option<u64>,
        tokens_for_owner_gas: Option<u64>,
        gas_to_remove_game: Option<u64>,
        time_interval: Option<u32>,
        turn_deadline_ms: Option<u64>,
        reply_deposit: Option<u64>,
        max_number_of_blocks_for_reply: Option<u32>,
    },
    AllowMessages(bool),
}

/// The main type used as an output message.
#[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum GameReply {
    GameStarted { game: GameInstance },
    MoveMade { game: GameInstance },
}

/// Represent game instance status.
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum GameResult {
    Player,
    Bot,
    Draw,
}

/// Represent concrete game instance.
#[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
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
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum Mark {
    X,
    O,
}

#[derive(Debug, Default, Encode, Clone, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct Config {
    pub add_attribute_gas: u64,
    pub ms_per_block: u64,
    pub tokens_for_owner_gas: u64,
    pub gas_to_remove_game: u64,
    pub time_interval: u32,
    pub turn_deadline_ms: u64,
    pub number_of_blocks_for_reply: u32,
    pub reply_deposit: u64,
}
