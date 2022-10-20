#![no_std]
use gstd::{prelude::*, ActorId};
use primitive_types::H256;

#[derive(Encode, Decode, Debug, Copy, Clone, TypeInfo)]
pub enum FTStorageAction {
    GetBalance(ActorId),
    IncreaseBalance {
        transaction_hash: H256,
        account: ActorId,
        amount: u128,
    },
    DecreaseBalance {
        transaction_hash: H256,
        msg_source: ActorId,
        account: ActorId,
        amount: u128,
    },
    Approve {
        transaction_hash: H256,
        msg_source: ActorId,
        account: ActorId,
        amount: u128,
    },
    Transfer {
        transaction_hash: H256,
        msg_source: ActorId,
        sender: ActorId,
        recipient: ActorId,
        amount: u128,
    },
    Clear(H256),
}

#[derive(Encode, Decode, Clone, Debug, TypeInfo)]
pub enum FTStorageEvent {
    Ok,
    Err,
    Balance(u128),
}

#[derive(Encode, Decode, TypeInfo)]
pub enum FTStorageState {
    Balance(ActorId),
}

#[derive(Encode, Decode, Debug, TypeInfo)]
pub enum FTStorageStateReply {
    Balance(u128),
}
