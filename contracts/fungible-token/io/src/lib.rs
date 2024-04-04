#![no_std]

use gmeta::{InOut, Metadata};
use gstd::{prelude::*, ActorId};

pub type TxId = u64;
pub type ValidUntil = u64;
pub struct FungibleTokenMetadata;

impl Metadata for FungibleTokenMetadata {
    type Init = InOut<InitConfig, FTReply>;
    type Handle = InOut<FTAction, Result<FTReply, FTError>>;
    type Others = ();
    type Reply = ();
    type Signal = ();
    type State = InOut<Query, QueryReply>;
}

#[derive(Debug, Decode, Encode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct InitConfig {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub initial_capacity: Option<u32>,
    pub config: Config,
}

#[derive(Debug, Decode, Encode, TypeInfo, Default, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct Config {
    pub tx_storage_period: u64,
    pub tx_payment: u128,
}

#[derive(Debug, Decode, Encode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum FTAction {
    Transfer {
        tx_id: Option<TxId>,
        from: ActorId,
        to: ActorId,
        amount: u128,
    },
    Approve {
        tx_id: Option<TxId>,
        to: ActorId,
        amount: u128,
    },
    BalanceOf(ActorId),
    AddAdmin {
        admin_id: ActorId,
    },
    DeleteAdmin {
        admin_id: ActorId,
    },
}

#[derive(Debug, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum FTReply {
    Initialized,
    Transferred {
        from: ActorId,
        to: ActorId,
        amount: u128,
    },
    Approved {
        from: ActorId,
        to: ActorId,
        amount: u128,
    },
    AdminAdded {
        admin_id: ActorId,
    },
    AdminRemoved {
        admin_id: ActorId,
    },
    Balance(u128),
}

#[derive(Debug, Encode, Decode, TypeInfo, MaxEncodedLen, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum FTError {
    NotAdmin,
    NotEnoughBalance,
    ZeroAddress,
    NotAllowedToTransfer,
    AdminAlreadyExists,
    CantDeleteYourself,
    TxAlreadyExists,
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum Query {
    Name,
    Symbol,
    Decimals,
    BalanceOf(ActorId),
    AllowanceOfAccount {
        account: ActorId,
        approved_account: ActorId,
    },
    Admins,
    GetTxValidityTime {
        account: ActorId,
        tx_id: TxId,
    },
    GetTxIdsForAccount {
        account: ActorId,
    },
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum QueryReply {
    Name(String),
    Symbol(String),
    Decimals(u8),
    TotalSupply(u128),
    Balance(u128),
    AllowanceOfAccount(u128),
    Admins(Vec<ActorId>),
    TxValidityTime(ValidUntil),
    TxIdsForAccount { tx_ids: Vec<TxId> },
}
