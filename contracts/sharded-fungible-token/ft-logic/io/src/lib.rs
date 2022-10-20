#![no_std]
use gstd::{prelude::*, ActorId};
use primitive_types::H256;

#[derive(Debug, Encode, Decode, TypeInfo, Clone)]
pub enum FTLogicAction {
    Message {
        transaction_hash: H256,
        account: ActorId,
        payload: Vec<u8>,
    },
    GetBalance(ActorId),
    Clear(H256),
    UpdateStorageCodeHash(H256),
    MigrateStorages,
}

#[derive(Encode, Decode, TypeInfo)]
pub enum FTLogicEvent {
    Ok,
    Err,
    Balance(u128),
}

#[derive(Encode, Debug, Decode, TypeInfo, Copy, Clone)]
pub enum Action {
    Mint {
        recipient: ActorId,
        amount: u128,
    },
    Burn {
        sender: ActorId,
        amount: u128,
    },
    Transfer {
        sender: ActorId,
        recipient: ActorId,
        amount: u128,
    },
    Approve {
        approved_account: ActorId,
        amount: u128,
    },
}

#[derive(Encode, Decode, TypeInfo)]
pub struct InitFTLogic {
    pub admin: ActorId,
    pub storage_code_hash: H256,
}

#[derive(Encode, Debug, Decode, TypeInfo)]
pub enum FTLogicState {
    Storages,
}

#[derive(Encode, Debug, Decode, TypeInfo)]
pub enum FTLogicStateReply {
    Storages(BTreeMap<String, ActorId>),
}
