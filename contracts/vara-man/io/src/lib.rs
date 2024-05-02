#![no_std]

mod rand;

use gmeta::{In, InOut, Out, Metadata};
use gstd::{prelude::*, ActorId};
pub use rand::*;

pub const MAX_PARTICIPANTS: u16 = 10;

pub struct VaraManMetadata;

impl Metadata for VaraManMetadata {
    type Init = In<VaraManInit>;
    type Handle = InOut<VaraManAction, Result<VaraManEvent, VaraManError>>;
    type Others = Out<SignatureData>;
    type Reply = ();
    type Signal = ();
    type State = InOut<StateQuery, StateReply>;
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct VaraManInit {
    pub config: Config,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct TournamentState {
    pub tournament_name: String,
    pub admin: ActorId,
    pub level: Level,
    pub participants: Vec<(ActorId, Player)>,
    pub bid: u128,
    pub stage: Stage,
    pub duration_ms: u32,
}
#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct SingleGame {
    pub level: Level,
    pub points: u128,
    pub start_time: u64,
    pub game_over: bool,
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum VaraManEvent {
    GameFinished {
        winners: Vec<ActorId>,
        participants: Vec<ActorId>,
        prize: u128,
    },
    SingleGameFinished {
        prize: u128,
    },
    NewTournamentCreated {
        tournament_name: String,
        name: String,
        level: Level,
        bid: u128,
    },
    PlayerRegistered {
        admin_id: ActorId,
        name: String,
        bid: u128,
    },
    RegisterCanceled,
    TournamentCanceled {
        admin_id: ActorId,
    },
    PlayerDeleted {
        player_id: ActorId,
    },
    ResultTournamentRecorded {
        time: u128,
        points: u128,
    },
    GameStarted,
    AdminAdded(ActorId),
    StatusChanged(Status),
    ConfigChanged(Config),
    LeftGame,
    SessionCreated,
    SessionDeleted,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum VaraManAction {
    CreateNewTournament {
        tournament_name: String,
        name: String,
        level: Level,
        duration_ms: u32,
        session_for_account: Option<ActorId>,
    },
    StartTournament {
        session_for_account: Option<ActorId>,
    },
    RegisterForTournament {
        admin_id: ActorId,
        name: String,
        session_for_account: Option<ActorId>,
    },
    CancelRegister {
        session_for_account: Option<ActorId>,
    },
    CancelTournament {
        session_for_account: Option<ActorId>,
    },
    DeletePlayer {
        player_id: ActorId,
        session_for_account: Option<ActorId>,
    },
    RecordTournamentResult {
        time: u128,
        gold_coins: u128,
        silver_coins: u128,
        session_for_account: Option<ActorId>,
    },
    FinishTournament {
        admin_id: ActorId,
        time_start: u64,
    },
    FinishSingleGame {
        gold_coins: u128,
        silver_coins: u128,
        level: Level,
        session_for_account: Option<ActorId>,
    },
    LeaveGame {
        session_for_account: Option<ActorId>,
    },
    ChangeStatus(Status),
    ChangeConfig(Config),
    AddAdmin(ActorId),
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
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
pub enum VaraManError {
    GameIsPaused,
    EmptyName,
    AlreadyHaveTournament,
    NoSuchGame,
    NoSuchPlayer,
    WrongBid,
    SeveralRegistrations,
    SeveralGames,
    NotRegistered,
    GameDoesNotExist,
    AmountGreaterThanAllowed,
    TransferNativeTokenFailed,
    TransferFungibleTokenFailed,
    ThereIsNoSuchGame,
    NotAdmin,
    ConfigIsInvalid,
    SessionFull,
    WrongStage,
    WrongTypeOfGame,
    AccessDenied,
    MultipleError,
    GameOver,
}

#[derive(Encode, Decode, TypeInfo)]
pub enum StateQuery {
    All,
    GetTournament { player_id: ActorId },
    Config,
    Admins,
    Status,
    SessionForTheAccount(ActorId),
}

#[derive(Encode, Decode, TypeInfo)]
pub enum StateReply {
    All(VaraManState),
    Tournament(Option<(TournamentState, Option<u64>)>),
    Config(Config),
    Admins(Vec<ActorId>),
    Status(Status),
    SessionForTheAccount(Option<Session>),
}

#[derive(Debug, Default, Clone, Encode, Decode, TypeInfo)]
pub struct VaraManState {
    pub tournaments: Vec<(ActorId, TournamentState)>,
    pub players_to_game_id: Vec<(ActorId, ActorId)>,
    pub status: Status,
    pub config: Config,
    pub admins: Vec<ActorId>,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub enum Status {
    #[default]
    Paused,
    StartedUnrewarded,
    StartedWithFungibleToken {
        ft_address: ActorId,
    },
    StartedWithNativeToken,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub enum Stage {
    #[default]
    Registration,
    Started(u64),
    Finished(Vec<ActorId>),
}

#[derive(Debug, Default, Clone, Copy, Encode, Decode, TypeInfo)]
pub struct Config {
    pub one_point_in_value: u128,
    pub points_per_gold_coin_easy: u128,
    pub points_per_silver_coin_easy: u128,
    pub points_per_gold_coin_medium: u128,
    pub points_per_silver_coin_medium: u128,
    pub points_per_gold_coin_hard: u128,
    pub points_per_silver_coin_hard: u128,
    pub gas_for_finish_tournament: u64,
    pub time_for_single_round: u32,
    pub gas_to_delete_session: u64,
    pub block_duration_ms: u64,
}

impl Config {
    pub fn get_points_per_gold_coin_for_level(&self, level: Level) -> (u128, u128) {
        match level {
            Level::Easy => (
                self.points_per_gold_coin_easy,
                self.points_per_silver_coin_easy,
            ),
            Level::Medium => (
                self.points_per_gold_coin_medium,
                self.points_per_silver_coin_medium,
            ),
            Level::Hard => (
                self.points_per_gold_coin_hard,
                self.points_per_silver_coin_hard,
            ),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub struct Player {
    pub name: String,
    pub time: u128,
    pub points: u128,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub enum Level {
    #[default]
    Easy,
    Medium,
    Hard,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub enum Effect {
    Speed,
    Slow,
    Blind,
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

#[derive(Encode, Decode, TypeInfo)]
pub struct SignatureData {
    pub key: ActorId,
    pub duration: u64,
    pub allowed_actions: Vec<ActionsForSession>,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
pub enum ActionsForSession {
    CreateNewTournament,
    RegisterForTournament, 
    CancelRegister,
    CancelTournament,
    DeletePlayer,
    FinishSingleGame,
    StartTournament,
    RecordTournamentResult,
    LeaveGame
}

