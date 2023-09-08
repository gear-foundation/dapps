use crate::non_fungible_token::{royalties::*, token::*};
use gstd::{prelude::*, ActorId};

#[derive(
    Default, Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, TypeInfo, Hash,
)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct NFTTransfer {
    pub from: ActorId,
    pub to: ActorId,
    pub token_id: TokenId,
}

#[derive(Default, Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, TypeInfo, Hash)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct NFTTransferPayout {
    pub from: ActorId,
    pub to: ActorId,
    pub token_id: TokenId,
    pub payouts: Payout,
}

#[derive(
    Default, Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, TypeInfo, Hash,
)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct NFTApproval {
    pub owner: ActorId,
    pub approved_account: ActorId,
    pub token_id: TokenId,
}
