#![no_std]
use gmeta::{In, InOut, Metadata};
use gstd::{prelude::*, ActorId};
use primitive_types::{H256, H512};
pub struct FMainTokenMetadata;

impl Metadata for FMainTokenMetadata {
    type Init = In<InitFToken>;
    type Handle = InOut<FTokenAction, FTokenEvent>;
    type Others = ();
    type Reply = ();
    type Signal = ();
    type State = FTokenState;
}

#[derive(Default, Encode, Decode, TypeInfo, Debug)]
pub struct FTokenState {
    pub admin: ActorId,
    pub ft_logic_id: ActorId,
    pub transactions: Vec<(H256, TransactionStatus)>,
}

#[derive(Encode, Decode, TypeInfo, Debug)]
pub enum FTokenAction {
    Message {
        transaction_id: u64,
        payload: LogicAction,
    },
    UpdateLogicContract {
        ft_logic_code_hash: H256,
        storage_code_hash: H256,
    },
    GetBalance(ActorId),
    GetPermitId(ActorId),
    Clear(H256),
    MigrateStorageAddresses,
}

#[derive(Encode, Decode, TypeInfo, Debug)]
pub enum FTokenInnerAction {
    Message(Vec<u8>),
    UpdateLogicContract {
        ft_logic_code_hash: H256,
        storage_code_hash: H256,
    },
    GetBalance(ActorId),
    GetPermitId(ActorId),
    Clear(H256),
    MigrateStorageAddresses,
}

#[derive(Encode, Debug, Decode, TypeInfo, Copy, Clone)]
pub enum LogicAction {
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
    Permit {
        owner_account: ActorId,
        approved_account: ActorId,
        amount: u128,
        permit_id: u128,
        sign: H512,
    },
}

#[derive(Debug, Encode, Decode, TypeInfo, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum FTokenEvent {
    Ok,
    Err,
    Balance(u128),
    PermitId(u128),
}

#[derive(Encode, Decode, TypeInfo)]
pub struct InitFToken {
    pub storage_code_hash: H256,
    pub ft_logic_code_hash: H256,
}

#[derive(Encode, Decode, TypeInfo, Copy, Clone, Debug)]
pub enum TransactionStatus {
    InProgress,
    Success,
    Failure,
}
