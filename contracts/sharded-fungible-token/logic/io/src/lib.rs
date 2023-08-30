#![no_std]

use gmeta::{In, InOut, Metadata, Out};
use gstd::{prelude::*, ActorId};
use instruction::Instruction;
use primitive_types::H256;
use sharded_fungible_token_io::LogicAction;

pub struct FLogicMetadata;
pub mod instruction;

impl Metadata for FLogicMetadata {
    type Init = In<InitFTLogic>;
    type Handle = InOut<FTLogicAction, FTLogicEvent>;
    type Others = InOut<LogicAction, ()>;
    type Reply = ();
    type Signal = ();
    type State = Out<FTLogicState>;
}

#[derive(Encode, Decode, TypeInfo, Debug)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct FTLogicState {
    pub admin: ActorId,
    pub ftoken_id: ActorId,
    pub transaction_status: Vec<(H256, TransactionStatus)>,
    pub instructions: Vec<(H256, (Instruction, Instruction))>,
    pub storage_code_hash: H256,
    pub id_to_storage: Vec<(String, ActorId)>,
}

#[derive(Encode, Decode, TypeInfo, Clone, Debug)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum TransactionStatus {
    InProgress,
    Success,
    Failure,
}

#[derive(Debug, Encode, Decode, TypeInfo, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
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
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum FTLogicEvent {
    Ok,
    Err,
    Balance(u128),
    PermitId(u128),
}

#[derive(Encode, Debug, Decode, TypeInfo, Copy, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct PermitUnsigned {
    pub owner_account: ActorId,
    pub approved_account: ActorId,
    pub amount: u128,
    pub permit_id: u128,
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct InitFTLogic {
    pub admin: ActorId,
    pub storage_code_hash: H256,
}
