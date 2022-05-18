use codec::{Decode, Encode};
use gstd::{prelude::*, ActorId};
use scale_info::TypeInfo;

#[derive(Debug, Decode, Encode, TypeInfo)]
pub struct InitConfig {
    pub name: String,
    pub symbol: String,
    pub base_uri: String,
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum State {
    Name,
    Symbol,
    Uri,
    BalanceOf(ActorId, u128),
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum StateReply {
    Name(String),
    Symbol(String),
    Uri(String),
    Balance(u128),
}

#[derive(Debug, Decode, Encode, TypeInfo)]
pub enum Action {
    Mint(ActorId, u128, u128),
    BalanceOf(ActorId, u128),
    BalanceOfBatch(Vec<ActorId>, Vec<u128>),
    MintBatch(ActorId, Vec<u128>, Vec<u128>),
    SafeTransferFrom(ActorId, ActorId, u128, u128),
    SafeBatchTransferFrom(ActorId, ActorId, Vec<u128>, Vec<u128>),
    SetApprovalForAll(ActorId, bool),
    IsApprovedForAll(ActorId, ActorId),
    BurnBatch(Vec<u128>, Vec<u128>),
    OwnerOf(u128),
    OwnerOfBatch(Vec<u128>),
}

#[derive(Debug, Decode, Encode, TypeInfo)]
pub struct TransferSingleReply {
    pub operator: ActorId,
    pub from: ActorId,
    pub to: ActorId,
    pub id: u128,
    pub amount: u128,
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub struct BalanceOfBatchReply {
    pub account: ActorId,
    pub id: u128,
    pub amount: u128,
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum Event {
    TransferSingle(TransferSingleReply),
    Balance(u128),
    BalanceOfBatch(Vec<BalanceOfBatchReply>),
    MintOfBatch(Vec<BalanceOfBatchReply>),
    TransferBatch {
        operator: ActorId,
        from: ActorId,
        to: ActorId,
        ids: Vec<u128>,
        values: Vec<u128>,
    },
    ApprovalForAll {
        owner: ActorId,
        operator: ActorId,
        approved: bool,
    },
}
