#![no_std]

use gmeta::{In, InOut, Metadata};
use gstd::{prelude::*, ActorId};
use primitive_types::U256;

pub struct ContractMetadata;

impl Metadata for ContractMetadata {
    type Init = In<MWInitConfig>;
    type Handle = InOut<MWAction, MWEvent>;
    type Reply = ();
    type Others = ();
    type Signal = ();
    type State = State;
}

#[derive(Debug, Default, Encode, Decode, TypeInfo, Hash, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct State {
    pub transactions: Vec<(TransactionId, Transaction)>,
    pub confirmations: Vec<(TransactionId, Vec<ActorId>)>,
    pub owners: Vec<ActorId>,
    pub required: u32,
    pub transaction_count: U256,
}

#[derive(Debug, Default, Encode, Decode, TypeInfo, Hash, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Transaction {
    pub destination: ActorId,
    pub payload: Vec<u8>,
    pub value: u128,
    pub description: Option<String>,
    pub executed: bool,
}

pub type TransactionId = U256;

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum MWAction {
    AddOwner(ActorId),
    RemoveOwner(ActorId),
    ReplaceOwner {
        old_owner: ActorId,
        new_owner: ActorId,
    },
    ChangeRequiredConfirmationsCount(u32),
    SubmitTransaction {
        destination: ActorId,
        data: Vec<u8>,
        value: u128,
        description: Option<String>,
    },
    ConfirmTransaction(U256),
    RevokeConfirmation(U256),
    ExecuteTransaction(U256),
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum MWEvent {
    Confirmation {
        sender: ActorId,
        transaction_id: U256,
    },
    Revocation {
        sender: ActorId,
        transaction_id: U256,
    },
    Submission {
        transaction_id: U256,
    },
    Execution {
        transaction_id: U256,
    },
    OwnerAddition {
        owner: ActorId,
    },
    OwnerRemoval {
        owner: ActorId,
    },
    OwnerReplace {
        old_owner: ActorId,
        new_owner: ActorId,
    },
    RequirementChange(U256),
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub struct MWInitConfig {
    pub owners: Vec<ActorId>,
    pub required: u32,
}
