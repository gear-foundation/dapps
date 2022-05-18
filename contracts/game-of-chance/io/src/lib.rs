#![no_std]

use codec::{Decode, Encode};
use gstd::{prelude::*, ActorId};
use scale_info::TypeInfo;

#[derive(Debug, Default, Encode, Decode, TypeInfo, Clone)]
pub struct LotteryState {
    pub lottery_started: bool,
    pub lottery_start_time: u64,
    pub lottery_duration: u64,
}

#[derive(Debug, Default, Encode, Decode, TypeInfo, Clone, Copy)]
pub struct Player {
    pub player_id: ActorId,
    pub balance: u128,
}

#[derive(Debug, Decode, Encode, TypeInfo)]
pub enum LtAction {
    Enter(u128),
    StartLottery {
        duration: u64,
        token_address: Option<ActorId>,
    },
    LotteryState,
    PickWinner,
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum LtEvent {
    LotteryState(LotteryState),
    Winner(u32),
    PlayerAdded(u32),
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum LtState {
    GetWinners,
    GetPlayers,
    BalanceOf(u32),
    LotteryState,
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum LtStateReply {
    Winners(BTreeMap<u32, ActorId>),
    Players(BTreeMap<u32, Player>),
    Balance(u128),
    LotteryState(LotteryState),
}
