use codec::{Decode, Encode};
use gstd::{prelude::*, ActorId};
use scale_info::TypeInfo;

#[derive(Debug, Encode, Decode, TypeInfo, PartialEq, Eq)]
pub struct FTTransfer {
    pub from: ActorId,
    pub to: ActorId,
    pub amount: u128,
}

#[derive(Debug, Encode, Decode, TypeInfo, PartialEq, Eq)]
pub struct FTApproval {
    pub from: ActorId,
    pub to: ActorId,
    pub amount: u128,
}
