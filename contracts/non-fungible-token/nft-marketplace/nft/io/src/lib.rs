#![no_std]
use codec::{Decode, Encode};
use gstd::{prelude::*, ActorId, BTreeMap, String, Vec};
use primitive_types::U256;
pub use royalties::*;
use scale_info::TypeInfo;

#[derive(Debug, Encode, Decode, TypeInfo)]
pub struct InitNFT {
    pub name: String,
    pub symbol: String,
    pub base_uri: String,
    pub supply: U256,
    pub royalties: Option<Royalties>,
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum NFTAction {
    Mint { media: String, reference: String },
    Burn(U256),
    Transfer { to: ActorId, token_id: U256 },
    Approve { to: ActorId, token_id: U256 },
    OwnerOf(U256),
    BalanceOf(ActorId),
    TokensForOwner(ActorId),
    NFTPayout { owner: ActorId, amount: u128 },
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum NFTEvent {
    Transfer {
        from: ActorId,
        to: ActorId,
        token_id: U256,
    },
    Approval {
        owner: ActorId,
        spender: ActorId,
        token_id: U256,
    },
    ApprovalForAll {
        owner: ActorId,
        operator: ActorId,
        approved: bool,
    },
    OwnerOf(ActorId),
    BalanceOf(U256),
    TokensForOwner(Vec<U256>),
    NFTPayout(BTreeMap<ActorId, u128>),
}
