use crate::services::game::Storage;
use sails_rs::prelude::*;

pub const MAX_PARTICIPANTS: u16 = 10;

#[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum GameError {
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
    ExceededLimit,
    MintFungibleToken,
    ReplyFungibleToken,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum Stage {
    #[default]
    Registration,
    Started(u64),
    Finished(Vec<ActorId>),
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum Level {
    #[default]
    Easy,
    Medium,
    Hard,
}

#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct Player {
    pub name: String,
    pub time: u128,
    pub points: u128,
}

#[derive(Debug, Default, Clone, Copy, Encode, Decode, TypeInfo, PartialEq, Eq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct Config {
    pub one_point_in_value: u128,
    pub max_number_gold_coins: u16,
    pub max_number_silver_coins: u16,
    pub points_per_gold_coin_easy: u128,
    pub points_per_silver_coin_easy: u128,
    pub points_per_gold_coin_medium: u128,
    pub points_per_silver_coin_medium: u128,
    pub points_per_gold_coin_hard: u128,
    pub points_per_silver_coin_hard: u128,
    pub gas_for_finish_tournament: u64,
    pub gas_for_mint_fungible_token: u64,
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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum Status {
    #[default]
    Paused,
    StartedUnrewarded,
    StartedWithFungibleToken {
        ft_address: ActorId,
    },
    StartedWithNativeToken,
}

#[derive(Debug, Default, Clone, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct VaraManState {
    pub tournaments: Vec<(ActorId, TournamentState)>,
    pub players_to_game_id: Vec<(ActorId, ActorId)>,
    pub status: Status,
    pub config: Config,
    pub admins: Vec<ActorId>,
    pub dns_info: Option<(ActorId, String)>,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct TournamentState {
    pub tournament_name: String,
    pub admin: ActorId,
    pub level: Level,
    pub participants: Vec<(ActorId, Player)>,
    pub bid: u128,
    pub stage: Stage,
    pub duration_ms: u32,
}

impl From<Storage> for VaraManState {
    fn from(value: Storage) -> Self {
        let Storage {
            tournaments,
            players_to_game_id,
            status,
            config,
            admins,
            dns_info,
        } = value;

        let tournaments = tournaments
            .into_iter()
            .map(|(id, tournament)| {
                let tournament_state = TournamentState {
                    tournament_name: tournament.tournament_name,
                    admin: tournament.admin,
                    level: tournament.level,
                    participants: tournament.participants.into_iter().collect(),
                    bid: tournament.bid,
                    stage: tournament.stage,
                    duration_ms: tournament.duration_ms,
                };
                (id, tournament_state)
            })
            .collect();

        let players_to_game_id = players_to_game_id.into_iter().collect();

        Self {
            tournaments,
            players_to_game_id,
            status,
            config,
            admins,
            dns_info,
        }
    }
}
