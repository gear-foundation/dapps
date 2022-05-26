#![no_std]

use codec::{Decode, Encode};
use gstd::{prelude::*, ActorId};
use scale_info::TypeInfo;

// These structs & enums are from contract, remove later to imports :)
pub type TokenId = u128;

#[derive(Debug, Decode, Encode, TypeInfo, Default, Clone)]
pub struct TokenMetadata {
    pub title: Option<String>,
    pub description: Option<String>,
    pub media: Option<String>,
    pub reference: Option<String>,
}

#[derive(Debug, Decode, Encode, TypeInfo)]
pub enum MTKAction {
    Mint {
        account: ActorId,
        id: TokenId,
        amount: u128,
        meta: Option<TokenMetadata>,
    },
    Burn {
        id: TokenId,
        amount: u128,
    },
    BalanceOfBatch {
        accounts: Vec<ActorId>,
        ids: Vec<TokenId>,
    },
    MintBatch {
        account: ActorId,
        ids: Vec<TokenId>,
        amounts: Vec<u128>,
        meta: Vec<Option<TokenMetadata>>,
    },
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub struct BalanceOfBatchReply {
    pub account: ActorId,
    pub id: TokenId,
    pub amount: u128,
}

#[derive(Debug, Decode, Encode, TypeInfo)]
pub struct TransferSingleReply {
    pub operator: ActorId,
    pub from: ActorId,
    pub to: ActorId,
    pub id: TokenId,
    pub amount: u128,
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum MTKEvent {
    TransferSingle(TransferSingleReply),
    BalanceOfBatch(Vec<BalanceOfBatchReply>),
    TransferBatch {
        operator: ActorId,
        from: ActorId,
        to: ActorId,
        ids: Vec<TokenId>,
        values: Vec<u128>,
    },
}

// Concert related stuff
#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum ConcertAction {
    Create {
        creator: ActorId,
        concert_id: u128,
        number_of_tickets: u128,
        date: u128,
    },
    Hold {
        concert_id: u128,
    },
    BuyTickets {
        concert_id: u128,
        amount: u128,
        metadata: Vec<Option<TokenMetadata>>,
    },
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum ConcertEvent {
    Creation {
        creator: ActorId,
        concert_id: u128,
        number_of_tickets: u128,
        date: u128,
    },
    Hold {
        concert_id: u128,
    },
    Purchase {
        concert_id: u128,
        amount: u128,
    },
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub struct InitConcert {
    pub owner_id: ActorId,
    pub mtk_contract: ActorId,
}
