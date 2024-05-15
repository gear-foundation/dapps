#![no_std]

use gmeta::{In, InOut, Metadata};
use gstd::{
    collections::{BTreeMap, BTreeSet},
    prelude::*,
    ActorId, MessageId, ReservationId,
};

pub type TamagotchiId = ActorId;
pub type PairId = u8;
pub struct BattleMetadata;

impl Metadata for BattleMetadata {
    type Init = In<Config>;
    type Handle = InOut<BattleAction, Result<BattleReply, BattleError>>;
    type Others = ();
    type Reply = ();
    type Signal = ();
    type State = InOut<BattleQuery, BattleQueryReply>;
}

#[derive(Default, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct Battle {
    pub admins: Vec<ActorId>,
    pub players: BTreeMap<ActorId, Player>,
    pub players_ids: Vec<ActorId>,
    pub current_players: Vec<ActorId>,
    pub state: BattleState,
    pub current_winner: ActorId,
    pub pairs: BTreeMap<PairId, Pair>,
    pub players_to_pairs: BTreeMap<ActorId, BTreeSet<PairId>>,
    pub completed_games: u8,
    pub reservations: BTreeMap<ActorId, ReservationId>,
    pub config: Config,
}
#[derive(Default, Debug, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct Player {
    pub owner: ActorId,
    pub name: String,
    pub date_of_birth: u64,
    pub tmg_id: TamagotchiId,
    pub defence: u16,
    pub power: u16,
    pub health: u16,
    pub color: String,
    pub victories: u32,
}

#[derive(Default, Debug, Clone, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct Config {
    pub max_power: u16,
    pub min_power: u16,
    pub health: u16,
    pub max_participants: u8,
    pub max_steps_in_round: u8,
    pub time_for_move: u32,
    pub min_gas_amount: u64,
    pub block_duration_ms: u64,
}

#[derive(Encode, Decode, TypeInfo, PartialEq, Eq, Debug, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum Move {
    Attack,
    Defence,
}

#[derive(Default, Debug, Encode, Decode, TypeInfo, Clone, PartialEq, Eq)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct Pair {
    pub owner_ids: Vec<ActorId>,
    pub tmg_ids: Vec<ActorId>,
    pub moves: Vec<Option<Move>>,
    pub rounds: u8,
    pub game_is_over: bool,
    pub winner: ActorId,
    pub last_updated: u64,
    pub msg_ids_in_waitlist: BTreeSet<MessageId>,
    pub amount_of_skipped_moves: u8,
    pub move_deadline: u64,
}

#[derive(Debug, PartialEq, Eq, Encode, Decode, TypeInfo, Default, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum BattleState {
    #[default]
    Registration,
    GameIsOn,
    WaitNextRound,
    GameIsOver,
}

#[derive(Encode, Decode, TypeInfo, Debug)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum BattleAction {
    StartRegistration,
    Register { tmg_id: TamagotchiId },
    MakeMove { pair_id: PairId, tmg_move: Move },
    StartBattle,
    AddAdmin(ActorId),
}

#[derive(Encode, Decode, TypeInfo, PartialEq, Eq, Debug)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum BattleReply {
    RegistrationStarted,
    Registered { tmg_id: TamagotchiId },
    MoveMade,
    GoToWaitingState,
    GameIsOver,
    InfoUpdated,
    NewGame,
    BattleStarted,
    RoundResult((PairId, u16, u16, Option<Move>, Option<Move>)),
    NewRound,
    AdminAdded,
    BattleWasCancelled,
    GameFinished {
        players: Vec<ActorId>
    }
}

#[derive(Encode, Decode, TypeInfo, PartialEq, Eq, Debug)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum BattleError {
    WrongState,
    NotEnoughPlayers,
    NotAdmin,
    GameIsOver,
    PairDoesNotExist,
    PlayerDoesNotExist,
    NoGamesForPlayer,
    NotPlayerGame,
    MaxNumberWasReached,
    TmgInGame,
    NotTmgOwner,
    TamagotchiHasDied,
}

#[derive(Encode, Decode, TypeInfo, Debug)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum BattleQuery {
    GetPlayer { tmg_id: ActorId },
    Players,
    PlayersIds,
    State,
    GetPairs,
    GetPair { pair_id: PairId },
    Admins,
    CurrentPlayers,
    CompletedGames,
    Winner,
}

#[derive(Encode, Decode, TypeInfo, PartialEq, Eq, Debug)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum BattleQueryReply {
    Player { player: Option<Player> },
    Players { players: BTreeMap<ActorId, Player>},
    PlayersIds { players_ids: Vec<ActorId> },
    State { state: BattleState },
    Pairs { pairs: BTreeMap<PairId, Pair> },
    Pair { pair: Option<Pair> },
    Admins { admins: Vec<ActorId>},
    CurrentPlayers { current_players: Vec<ActorId>},
    CompletedGames {completed_games: u8},
    Winner { winner: ActorId},
}
