use codec::{Decode, Encode};
use gstd::{prelude::*, ActorId};
use primitive_types::U256;
use scale_info::TypeInfo;

#[derive(Debug, Decode, Encode, TypeInfo)]
pub enum State {
    Status,
    Info,
}

#[derive(Debug, Decode, Encode, TypeInfo)]
pub enum StateReply {
    Status(Status),
    Info(AuctionInfo),
}

#[derive(Debug, Decode, Default, Encode, TypeInfo, Clone)]
pub enum Status {
    #[default]
    None,
    IsRunning,
    Purchased {
        price: u128,
    },
    Expired,
    Stopped,
}

#[derive(Debug, Decode, Encode, TypeInfo)]
pub struct AuctionInfo {
    pub nft_contract_actor_id: ActorId,
    pub token_id: U256,
    pub token_owner: ActorId,
    pub auction_owner: ActorId,
    pub starting_price: u128,
    pub current_price: u128,
    pub discount_rate: u128,
    pub time_left: u64,
}
