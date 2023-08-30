use gstd::{prelude::*, ActorId};

#[derive(Debug, Encode, Decode, TypeInfo, PartialEq, Eq)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct FTTransfer {
    pub from: ActorId,
    pub to: ActorId,
    pub amount: u128,
}

#[derive(Debug, Encode, Decode, TypeInfo, PartialEq, Eq)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct FTApproval {
    pub from: ActorId,
    pub to: ActorId,
    pub amount: u128,
}
