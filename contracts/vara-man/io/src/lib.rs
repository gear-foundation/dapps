#![no_std]

mod rand;

use gmeta::{In, InOut, Metadata};
use gstd::{prelude::*, ActorId};
pub use rand::*;

pub const MAX_PARTICIPANTS: u16 = 10;

pub struct VaraManMetadata;

impl Metadata for VaraManMetadata {
    type Init = In<VaraManInit>;
    type Handle = InOut<VaraManAction, Result<VaraManEvent, VaraManError>>;
    type Others = ();
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
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum VaraManAction {
    CreateNewTournament {
        tournament_name: String,
        name: String,
        level: Level,
        duration_ms: u32,
    },
    StartTournament,
    RegisterForTournament {
        admin_id: ActorId,
        name: String,
    },
    CancelRegister,
    CancelTournament,
    DeletePlayer {
        player_id: ActorId,
    },
    RecordTournamentResult {
        time: u128,
        gold_coins: u128,
        silver_coins: u128,
    },
    FinishTournament {
        admin_id: ActorId,
        time_start: u64,
    },
    FinishSingleGame {
        gold_coins: u128,
        silver_coins: u128,
        level: Level,
    },
    LeaveGame,
    ChangeStatus(Status),
    ChangeConfig(Config),
    AddAdmin(ActorId),
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
}

#[derive(Encode, Decode, TypeInfo)]
pub enum StateReply {
    All(VaraManState),
    Tournament(Option<(TournamentState, Option<u64>)>),
    Config(Config),
    Admins(Vec<ActorId>),
    Status(Status),
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
