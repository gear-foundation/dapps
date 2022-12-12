use crate::non_fungible_token::{royalties::*, token::*};
use gstd::{prelude::*, ActorId};

#[derive(Debug, Encode, Decode, TypeInfo, PartialEq, Eq, Clone)]
pub struct NFTTransfer {
    pub from: ActorId,
    pub to: ActorId,
    pub token_id: TokenId,
}

#[derive(Debug, Encode, Decode, TypeInfo, PartialEq, Eq, Clone)]
pub struct NFTTransferPayout {
    pub from: ActorId,
    pub to: ActorId,
    pub token_id: TokenId,
    pub payouts: Payout,
}

#[derive(Debug, Encode, Decode, TypeInfo, PartialEq, Eq, Clone)]
pub struct NFTApproval {
    pub owner: ActorId,
    pub approved_account: ActorId,
    pub token_id: TokenId,
}
