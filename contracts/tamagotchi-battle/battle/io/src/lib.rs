#![no_std]

use gmeta::{InOut, Metadata};
use gstd::{prelude::*, ActorId, MessageId, ReservationId};
pub type TamagotchiId = ActorId;
pub type PairId = u8;
pub struct BattleMetadata;

impl Metadata for BattleMetadata {
    type Init = ();
    type Handle = InOut<BattleAction, BattleEvent>;
    type Others = ();
    type Reply = ();
    type Signal = ();
    type State = Battle;
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
    pub players_to_pairs: BTreeMap<ActorId, Vec<PairId>>,
    pub completed_games: u8,
    pub reservations: BTreeMap<ActorId, ReservationId>,
}
#[derive(Default, Debug, Clone, Encode, Decode, TypeInfo)]
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

#[derive(Encode, Decode, TypeInfo, PartialEq, Eq, Debug, Clone)]
pub enum Move {
    Attack,
    Defence,
}

#[derive(Default, Debug, Encode, Decode, TypeInfo, Clone)]
pub struct Pair {
    pub owner_ids: Vec<ActorId>,
    pub tmg_ids: Vec<ActorId>,
    pub moves: Vec<Option<Move>>,
    pub rounds: u8,
    pub game_is_over: bool,
    pub winner: ActorId,
    pub move_deadline: u64,
    pub msg_id: MessageId,
}

#[derive(Debug, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub enum BattleState {
    Registration,
    GameIsOn,
    WaitNextRound,
    GameIsOver,
}

impl Default for BattleState {
    fn default() -> Self {
        BattleState::Registration
    }
}

#[derive(Encode, Decode, TypeInfo, Debug)]
pub enum BattleAction {
    StartRegistration,
    Register {
        tmg_id: TamagotchiId,
    },
    MakeMove {
        pair_id: PairId,
        tmg_move: Move,
    },
    StartBattle,
    AddAdmin(ActorId),
    CheckIfMoveMade {
        pair_id: PairId,
        tmg_id: Option<TamagotchiId>,
    },
}

#[derive(Encode, Decode, TypeInfo, PartialEq, Eq)]
pub enum BattleEvent {
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
}
