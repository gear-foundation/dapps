#![no_std]

use gmeta::{In, Out, InOut, Metadata};
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
    type Others = Out<SignatureData>;
    type Reply = ();
    type Signal = ();
    type State = InOut<BattleQuery, BattleQueryReply>;
}

#[derive(Default, Encode, Decode, TypeInfo)]
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

#[derive(Encode, Decode, TypeInfo)]
pub struct SignatureData {
    pub key: ActorId,
    pub duration: u64,
    pub allowed_actions: Vec<ActionsForSession>,
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
    Register, 
    MakeMove,
}

#[derive(Default, Debug, Clone, Encode, Decode, TypeInfo)]
pub struct Config {
    pub max_power: u16,
    pub min_power: u16,
    pub health: u16,
    pub max_participants: u8,
    pub max_steps_in_round: u8,
    pub time_for_move: u32,
    pub min_gas_amount: u64,
    pub block_duration_ms: u64,
    pub gas_to_delete_session: u64,
}

#[derive(Encode, Decode, TypeInfo, PartialEq, Eq, Debug, Clone)]
pub enum Move {
    Attack,
    Defence,
}

#[derive(Default, Debug, Encode, Decode, TypeInfo, Clone, PartialEq, Eq)]
pub struct Pair {
    pub owner_ids: Vec<ActorId>,
    pub tmg_ids: Vec<ActorId>,
    pub moves: Vec<Option<Move>>,
    pub rounds: u8,
    pub game_is_over: bool,
    pub winner: ActorId,
    pub last_updated: u64,
    pub msg_id_in_waitlist: MessageId,
    pub amount_of_skipped_moves: u8,
    pub move_deadline: u64,
}

#[derive(Debug, PartialEq, Eq, Encode, Decode, TypeInfo, Default, Clone)]
pub enum BattleState {
    #[default]
    Registration,
    GameIsOn,
    WaitNextRound,
    GameIsOver,
}

#[derive(Encode, Decode, TypeInfo, Debug)]
pub enum BattleAction {
    StartRegistration,
    Register { tmg_id: TamagotchiId,  session_for_account: Option<ActorId> },
    MakeMove { pair_id: PairId, tmg_move: Move,  session_for_account: Option<ActorId> },
    StartBattle,
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

#[derive(Encode, Decode, TypeInfo, PartialEq, Eq, Debug)]
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
    },
    WaitlistMsgCancelled,
    SessionCreated,
    SessionDeleted,
}

#[derive(Encode, Decode, TypeInfo, PartialEq, Eq, Debug)]
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
    SessionForTheAccount(ActorId),
}

#[derive(Encode, Decode, TypeInfo, PartialEq, Eq, Debug)]
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
    SessionForTheAccount(Option<Session>),
}
