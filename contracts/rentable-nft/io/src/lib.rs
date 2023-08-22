#![no_std]

pub use gear_lib::non_fungible_token::delegated::DelegatedApproveMessage;

use gear_lib::non_fungible_token::{
    io::{NFTApproval, NFTTransfer, NFTTransferPayout},
    royalties::*,
    state::NFTState,
    token::*,
};
use gmeta::{In, InOut, Metadata};
use gstd::{prelude::*, ActorId};
use primitive_types::H256;

pub struct NFTMetadata;

impl Metadata for NFTMetadata {
    type Init = In<InitNft>;
    type Handle = InOut<NFTAction, NFTEvent>;
    type Reply = ();
    type Others = ();
    type Signal = ();
    type State = IoNft;
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum NFTAction {
    Mint {
        transaction_id: u64,
        token_metadata: TokenMetadata,
    },
    Burn {
        transaction_id: u64,
        token_id: TokenId,
    },
    Transfer {
        transaction_id: u64,
        to: ActorId,
        token_id: TokenId,
    },
    TransferPayout {
        transaction_id: u64,
        to: ActorId,
        token_id: TokenId,
        amount: u128,
    },
    NFTPayout {
        owner: ActorId,
        amount: u128,
    },
    Approve {
        transaction_id: u64,
        to: ActorId,
        token_id: TokenId,
    },
    DelegatedApprove {
        transaction_id: u64,
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
    Clear {
        transaction_hash: H256,
    },
    SetUser {
        token_id: TokenId,
        address: ActorId,
        expires: u64, // unix timestamp
        transaction_id: u64,
    },
    UserOf {
        token_id: TokenId,
    },
    UserExpires {
        token_id: TokenId,
    },
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub struct InitNft {
    pub name: String,
    pub symbol: String,
    pub base_uri: String,
    pub royalties: Option<Royalties>,
}

#[derive(Clone, Debug, Encode, Decode, TypeInfo)]
pub enum NFTEvent {
    Transfer(NFTTransfer),
    TransferPayout(NFTTransferPayout),
    NFTPayout(Payout),
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
    TransactionMade,
    UpdateUser {
        token_id: TokenId,
        address: ActorId,
        expires: u64,
    },
    UserOf {
        address: ActorId,
    },
    UserExpires {
        expires: u64,
    },
}

#[derive(Debug, Clone, Copy, Default, Encode, Decode, TypeInfo)]
pub struct UserInfo {
    pub address: ActorId, // address of user role
    pub expires: u64,     // unix timestamp
}

#[derive(Debug, Clone, Default, Encode, Decode, TypeInfo)]
pub struct IoNft {
    pub token: IoNftState,
    pub token_id: TokenId,
    pub owner: ActorId,
    pub transactions: Vec<(H256, NFTEvent)>,
    pub users_info: Vec<(TokenId, UserInfo)>,
}

#[derive(Debug, Clone, Default, Encode, Decode, TypeInfo)]
pub struct IoNftState {
    pub name: String,
    pub symbol: String,
    pub base_uri: String,
    pub owner_by_id: Vec<(TokenId, ActorId)>,
    pub token_approvals: Vec<(TokenId, Vec<ActorId>)>,
    pub token_metadata_by_id: Vec<(TokenId, Option<TokenMetadata>)>,
    pub tokens_for_owner: Vec<(ActorId, Vec<TokenId>)>,
    pub royalties: Option<Royalties>,
}

impl From<&NFTState> for IoNftState {
    fn from(value: &NFTState) -> Self {
        let NFTState {
            name,
            symbol,
            base_uri,
            owner_by_id,
            token_approvals,
            token_metadata_by_id,
            tokens_for_owner,
            royalties,
        } = value;

        let owner_by_id = owner_by_id
            .iter()
            .map(|(hash, actor_id)| (*hash, *actor_id))
            .collect();

        let token_approvals = token_approvals
            .iter()
            .map(|(key, approvals)| (*key, approvals.iter().copied().collect()))
            .collect();

        let token_metadata_by_id = token_metadata_by_id
            .iter()
            .map(|(id, metadata)| (*id, metadata.clone()))
            .collect();

        let tokens_for_owner = tokens_for_owner
            .iter()
            .map(|(id, tokens)| (*id, tokens.clone()))
            .collect();

        Self {
            name: name.clone(),
            symbol: symbol.clone(),
            base_uri: base_uri.clone(),
            owner_by_id,
            token_approvals,
            token_metadata_by_id,
            tokens_for_owner,
            royalties: royalties.clone(),
        }
    }
}
