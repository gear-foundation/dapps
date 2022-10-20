#![no_std]
use gstd::{prelude::*, ActorId};
use primitive_types::H256;

#[derive(Encode, Decode, TypeInfo, Debug)]
pub enum FTokenAction {
    Message {
        transaction_id: u64,
        payload: Vec<u8>,
    },
    UpdateLogicContract {
        ft_logic_code_hash: H256,
        storage_code_hash: H256,
    },
    GetBalance(ActorId),
    Clear(H256),
    MigrateStorageAddresses,
}

#[derive(Encode, Decode, TypeInfo)]
pub enum FTokenEvent {
    Ok,
    Err,
    Balance(u128),
}

#[derive(Encode, Decode, TypeInfo)]
pub struct InitFToken {
    pub storage_code_hash: H256,
    pub ft_logic_code_hash: H256,
}

#[derive(Encode, Decode, TypeInfo)]
pub enum FTokenState {
    TransactionStatus(ActorId, u64),
    FTLogicId,
}

#[derive(Encode, Decode, TypeInfo, Copy, Clone)]
pub enum TransactionStatus {
    InProgress,
    Success,
    Failure,
}

#[derive(Encode, Decode, TypeInfo)]
pub enum FTokenStateReply {
    TransactionStatus(Option<TransactionStatus>),
    FTLogicId(ActorId),
}
