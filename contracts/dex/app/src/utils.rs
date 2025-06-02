use sails_rs::prelude::*;

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum DexError {
    DeadlineExceeded,
}

#[derive(Debug, Encode, Decode, Clone, Copy, TypeInfo, Hash, PartialEq, Eq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum SwapKind {
    AForB,
    BForA,
}

#[derive(Debug, Encode, Decode, Clone, Copy, TypeInfo, Hash, PartialEq, Eq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum SwapStatus {
    Ready,
    Paused,
    TokenTransferError {
        out_token: ActorId,
        to: ActorId,
        in_amount: U256,
        out_amount: U256,
        out_is_a: bool,
    },
    TokenTransferOk {
        to: ActorId,
        in_amount: U256,
        out_amount: U256,
        out_is_a: bool,
    },
}
