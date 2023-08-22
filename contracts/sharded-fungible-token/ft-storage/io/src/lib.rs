#![no_std]
use gmeta::{InOut, Metadata};
use gstd::{prelude::*, ActorId};
use primitive_types::H256;

pub struct FTStorageMetadata;

impl Metadata for FTStorageMetadata {
    type Init = ();
    type Handle = InOut<FTStorageAction, FTStorageEvent>;
    type Others = ();
    type Reply = ();
    type Signal = ();
    type State = FTStorageState;
}

#[derive(Default, Encode, Decode, TypeInfo, Debug)]
pub struct FTStorageState {
    pub ft_logic_id: ActorId,
    pub transaction_status: Vec<(H256, bool)>,
    pub balances: Vec<(ActorId, u128)>,
    pub approvals: Vec<(ActorId, Vec<(ActorId, u128)>)>,
    pub permits: Vec<(ActorId, u128)>,
}

#[derive(Encode, Decode, Debug, Copy, Clone, TypeInfo)]
pub enum FTStorageAction {
    GetBalance(ActorId),
    GetPermitId(ActorId),
    IncrementPermitId {
        transaction_hash: H256,
        account: ActorId,
        expected_permit_id: u128,
    },
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
}

#[derive(Encode, Decode, Clone, Debug, TypeInfo)]
pub enum FTStorageEvent {
    Ok,
    Err,
    Balance(u128),
    PermitId(u128),
}
