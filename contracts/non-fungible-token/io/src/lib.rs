#![no_std]

use codec::{Decode, Encode};
use gear_lib::non_fungible_token::{
    io::{NFTApproval, NFTTransfer, NFTTransferPayout},
    royalties::*,
    token::*,
};
use gstd::{prelude::*, ActorId};
use scale_info::TypeInfo;

pub use gear_lib::non_fungible_token::delegated::DelegatedApproveMessage;

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum NFTAction {
    Mint {
        token_metadata: TokenMetadata,
    },
    Burn {
        token_id: TokenId,
    },
    Transfer {
        to: ActorId,
        token_id: TokenId,
    },
    TransferPayout {
        to: ActorId,
        token_id: TokenId,
        amount: u128,
    },
    Approve {
        to: ActorId,
        token_id: TokenId,
    },
    DelegatedApprove {
        message: DelegatedApproveMessage,
        signature: [u8; 64],
    },
    Owner {
        token_id: TokenId,
    },
    IsApproved {
        to: ActorId,
        token_id: TokenId,
    },
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub struct InitNFT {
    pub name: String,
    pub symbol: String,
    pub base_uri: String,
    pub royalties: Option<Royalties>,
}

#[derive(Encode, Decode, TypeInfo, Debug)]
pub enum NFTEvent {
    Transfer(NFTTransfer),
    TransferPayout(NFTTransferPayout),
    Approval(NFTApproval),
    Owner {
        owner: ActorId,
        token_id: TokenId,
    },
    IsApproved {
        to: ActorId,
        token_id: TokenId,
        approved: bool,
    },
}
