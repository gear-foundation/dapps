#![no_std]

use codec::{Decode, Encode};
use gstd::{prelude::*, ActorId};
use scale_info::TypeInfo;

#[derive(Debug, Decode, Encode, TypeInfo)]
pub struct InitConfig {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
}

#[derive(Debug, Decode, Encode, TypeInfo)]
pub enum FTAction {
    Mint(u128),
    Burn(u128),
    Transfer {
        from: ActorId,
        to: ActorId,
        amount: u128,
    },
    Approve {
        to: ActorId,
        amount: u128,
    },
    TotalSupply,
    BalanceOf(ActorId),
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum FTEvent {
    Transfer {
        from: ActorId,
        to: ActorId,
        amount: u128,
    },
    Approve {
        from: ActorId,
        to: ActorId,
        amount: u128,
    },
    TotalSupply(u128),
    Balance(u128),
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum State {
    Name,
    Symbol,
    Decimals,
    TotalSupply,
    BalanceOf(ActorId),
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum StateReply {
    Name(String),
    Symbol(String),
    Decimals(u8),
    TotalSupply(u128),
    Balance(u128),
}
