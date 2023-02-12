#![no_std]

use gmeta::{InOut, Metadata};
use gstd::{prelude::*, ActorId};
pub type TamagotchiId = ActorId;
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
    pub admin: ActorId,
    pub players: BTreeMap<ActorId, Player>,
    pub players_ids: Vec<ActorId>,
    pub state: BattleState,
    pub current_winner: ActorId,
    pub round: Round,
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
}

#[derive(Encode, Decode, TypeInfo, Debug)]
pub enum Move {
    Attack,
    Defence,
}

#[derive(Default, Debug, Encode, Decode, TypeInfo)]
pub struct Round {
    pub players: Vec<ActorId>,
    pub tmg_ids: Vec<ActorId>,
    pub moves: Vec<Move>,
    pub steps: u8,
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
    Register { tmg_id: TamagotchiId },
    MakeMove(Move),
    StartNewGame,
    StartNewGameForce,
    StartBattle,
    StartNewRound,
    UpdateAdmin(ActorId),
}

#[derive(Encode, Decode, TypeInfo, PartialEq, Eq)]
pub enum BattleEvent {
    Registered { tmg_id: TamagotchiId },
    MoveMade,
    GoToWaitingState,
    GameIsOver,
    InfoUpdated,
    NewGame,
    GameStarted,
    RoundResult((u16, u16)),
    NewRound,
    AdminUpdated,
}
