use sails_rs::prelude::*;

pub const VICTORIES: [[usize; 3]; 8] = [
    [0, 1, 2],
    [3, 4, 5],
    [6, 7, 8],
    [0, 3, 6],
    [1, 4, 7],
    [2, 5, 8],
    [0, 4, 8],
    [2, 4, 6],
];

#[derive(Debug, Default, Encode, Clone, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct Config {
    pub s_per_block: u64,
    pub gas_to_remove_game: u64,
    pub gas_to_delete_session: u64,
    pub time_interval: u32,
    pub turn_deadline_ms: u64,
    pub minimum_session_duration_ms: u64,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
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
    NotAllowedToSendMessages,
}

/// Represent game instance status.
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum GameResult {
    Player,
    Bot,
    Draw,
}

/// Represent concrete game instance.
#[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
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
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum Mark {
    X,
    O,
}
