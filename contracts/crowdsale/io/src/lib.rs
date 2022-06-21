#![no_std]

use codec::{Decode, Encode};

use gstd::ActorId;
use scale_info::TypeInfo;

#[derive(Debug, Default, Encode, Decode, TypeInfo, Clone)]
pub struct IcoState {
    pub ico_started: bool,
    pub start_time: u64,
    pub duration: u64,
    pub ico_ended: bool,
}

#[derive(Debug, Decode, Encode, Clone, TypeInfo)]
pub enum IcoAction {
    StartSale {
        duration: u64,
        start_price: u128,
        tokens_goal: u128,
        price_increase_step: u128,
        time_increase_step: u128,
    },
    Buy(u128),
    EndSale,
    BalanceOf(ActorId),
}

#[derive(Debug, Decode, Encode, Clone, TypeInfo)]
pub enum IcoEvent {
    SaleStarted {
        duration: u64,
        start_price: u128,
        tokens_goal: u128,
        price_increase_step: u128,
        time_increase_step: u128,
    },
    Bought {
        buyer: ActorId,
        amount: u128,
        change: u128,
    },
    SaleEnded,
    BalanceOf {
        address: ActorId,
        balance: u128,
    },
}

#[derive(Debug, Decode, Encode, Clone, TypeInfo)]
pub struct IcoInit {
    pub token_address: ActorId,
    pub owner: ActorId,
}

#[derive(Debug, Decode, Encode, TypeInfo)]
pub enum State {
    CurrentPrice,
    TokensLeft,
    Balance(ActorId),
}

#[derive(Debug, Decode, Encode, TypeInfo)]
pub enum StateReply {
    CurrentPrice(u128),
    TokensLeft(u128),
    Balance(u128),
}
