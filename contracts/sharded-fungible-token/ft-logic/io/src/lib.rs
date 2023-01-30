#![no_std]
use gstd::{prelude::*, ActorId};
use primitive_types::{H256, H512};

use gmeta::{In, InOut, Metadata};
pub struct FLogicMetadata;
pub mod instruction;
use instruction::Instruction;

impl Metadata for FLogicMetadata {
    type Init = In<InitFTLogic>;
    type Handle = InOut<FTLogicAction, FTLogicEvent>;
    type Others = InOut<Action, ()>;
    type Reply = ();
    type Signal = ();
    type State = FTLogicState;
}

#[derive(Encode, Decode, TypeInfo, Debug)]
pub struct FTLogicState {
    pub admin: ActorId,
    pub ftoken_id: ActorId,
    pub transaction_status: Vec<(H256, TransactionStatus)>,
    pub instructions: Vec<(H256, (Instruction, Instruction))>,
    pub storage_code_hash: H256,
    pub id_to_storage: Vec<(String, ActorId)>,
}

#[derive(Encode, Decode, TypeInfo, Clone, Debug)]
pub enum TransactionStatus {
    InProgress,
    Success,
    Failure,
}

#[derive(Debug, Encode, Decode, TypeInfo, Clone)]
pub enum FTLogicAction {
    Message {
        transaction_hash: H256,
        account: ActorId,
        payload: Vec<u8>,
    },
    GetBalance(ActorId),
    GetPermitId(ActorId),
    Clear(H256),
    UpdateStorageCodeHash(H256),
    MigrateStorages,
}

#[derive(Encode, Decode, TypeInfo)]
pub enum FTLogicEvent {
    Ok,
    Err,
    Balance(u128),
    PermitId(u128),
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
    Permit {
        owner_account: ActorId,
        approved_account: ActorId,
        amount: u128,
        permit_id: u128,
        sign: H512,
    },
}

#[derive(Encode, Debug, Decode, TypeInfo, Copy, Clone)]
pub struct PermitUnsigned {
    pub owner_account: ActorId,
    pub approved_account: ActorId,
    pub amount: u128,
    pub permit_id: u128,
}

#[derive(Encode, Decode, TypeInfo)]
pub struct InitFTLogic {
    pub admin: ActorId,
    pub storage_code_hash: H256,
}
