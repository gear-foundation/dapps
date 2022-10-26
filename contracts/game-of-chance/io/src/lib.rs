#![no_std]

use gstd::{prelude::*, ActorId};

#[derive(Debug, Default, Encode, Decode, TypeInfo, Clone, PartialEq)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct Player {
    pub player_id: ActorId,
    pub balance: u128,
}

#[derive(Debug, Decode, Encode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum LtAction {
    Enter(u128),
    StartLottery {
        duration: u64,
        token_address: Option<ActorId>,
        participation_cost: u128,
        prize_fund: u128,
    },
    LotteryState,
    PickWinner,
}

#[derive(Debug, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum LtEvent {
    LotteryState {
        lottery_owner: ActorId,
        lottery_started: bool,
        lottery_start_time: u64,
        lottery_duration: u64,
        participation_cost: u128,
        prize_fund: u128,
        token_address: Option<ActorId>,
        players: BTreeMap<u32, Player>,
        lottery_id: u32,
    },
    Winner(u32),
    PlayerAdded(u32),
}

#[derive(Debug, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum LtState {
    GetWinners,
    GetPlayers,
    BalanceOf(u32),
    LotteryState,
}

#[derive(Debug, Encode, Decode, TypeInfo, PartialEq)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum LtStateReply {
    Winners(BTreeMap<u32, ActorId>),
    Players(BTreeMap<u32, Player>),
    Balance(u128),
    LotteryState {
        lottery_owner: ActorId,
        lottery_started: bool,
        lottery_start_time: u64,
        lottery_duration: u64,
        participation_cost: u128,
        prize_fund: u128,
        token_address: Option<ActorId>,
        players: BTreeMap<u32, Player>,
        lottery_id: u32,
        winner: ActorId,
    },
}
