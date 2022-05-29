use codec::{Decode, Encode};
use gstd::{prelude::*, ActorId};
use primitive_types::U256;
use scale_info::TypeInfo;

#[derive(Debug, Decode, Encode, TypeInfo)]
pub enum State {
    TokenPrice(),
    IsActive(),
    Info(),
}

#[derive(Debug, Decode, Encode, TypeInfo)]
pub enum StateReply {
    TokenPrice(u128),
    IsActive(bool),
    Info(AuctionInfo),
}

#[derive(Debug, Decode, Encode, TypeInfo)]
pub struct AuctionInfo {
    pub nft_contract_actor_id: ActorId,
    pub token_id: U256,
    pub token_owner: ActorId,
    pub starting_price: u128,
}
