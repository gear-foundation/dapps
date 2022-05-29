#![no_std]

use codec::{Decode, Encode};
use gstd::{prelude::*, ActorId};
use primitive_types::U256;
use scale_info::TypeInfo;

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum Action {
    Buy,
    Create(CreateConfig),
    ForceStop,
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum Event {
    AuctionStarted {
        token_owner: ActorId,
        price: u128,
        token_id: U256,
    },
    AuctionStoped {
        token_owner: ActorId,
        token_id: U256,
    },
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub struct InitConfig {}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub struct Duration {
    pub days: u64,
    pub hours: u64,
    pub minutes: u64,
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub struct CreateConfig {
    pub nft_contract_actor_id: ActorId,
    pub token_owner: ActorId,
    pub token_id: U256,
    pub starting_price: u128,
    pub discount_rate: u128,
    pub duration: Duration,
}
