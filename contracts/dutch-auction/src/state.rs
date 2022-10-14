use gstd::{prelude::*, ActorId};
use primitive_types::U256;

#[derive(Debug, Decode, Encode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum State {
    Info,
}

#[derive(Debug, Decode, Encode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum StateReply {
    Info(AuctionInfo),
}

#[derive(Debug, Decode, Default, Encode, TypeInfo, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
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
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct AuctionInfo {
    pub nft_contract_actor_id: ActorId,
    pub token_id: U256,
    pub token_owner: ActorId,
    pub auction_owner: ActorId,
    pub starting_price: u128,
    pub current_price: u128,
    pub discount_rate: u128,
    pub time_left: u64,
    pub status: Status,
}
