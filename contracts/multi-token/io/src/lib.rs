#![no_std]

use gmeta::{In, InOut, Metadata, Out};
use gstd::{
    collections::{HashMap, HashSet},
    prelude::*,
    ActorId,
};

pub type TokenId = u128;

pub struct MultitokenMetadata;

impl Metadata for MultitokenMetadata {
    type Init = In<InitMtk>;
    type Handle = InOut<MtkAction, Result<MtkEvent, MtkError>>;
    type Others = ();
    type Reply = ();
    type Signal = ();
    type State = Out<State>;
}

#[derive(Debug, Default)]
pub struct MtkData {
    pub name: String,
    pub symbol: String,
    pub base_uri: String,
    pub balances: HashMap<TokenId, HashMap<ActorId, u128>>,
    pub approvals: HashMap<ActorId, HashSet<ActorId>>,
    pub token_metadata: HashMap<TokenId, TokenMetadata>,
    pub owners: HashMap<TokenId, ActorId>,
}

#[derive(Debug, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct State {
    pub name: String,
    pub symbol: String,
    pub base_uri: String,
    pub balances: Vec<(TokenId, Vec<(ActorId, u128)>)>,
    pub approvals: Vec<(ActorId, Vec<ActorId>)>,
    pub token_metadata: Vec<(TokenId, TokenMetadata)>,
    // owner for nft
    pub owners: Vec<(TokenId, ActorId)>,
    pub creator: ActorId,
    pub supply: Vec<(TokenId, u128)>,
}

/// Transform to NFT piece of data.
#[derive(Debug, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct BurnToNFT {
    /// To which account mint NFTs.
    pub account: ActorId,
    /// NFTs' IDs.
    pub nfts_ids: Vec<TokenId>,
    /// NFTs' metadata.
    pub nfts_metadata: Vec<Option<TokenMetadata>>,
}

#[derive(Debug, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum MtkAction {
    /// Mints a token.
    ///
    /// # Requirements:
    /// * if minting an NFT `amount` MUST equal to 1.
    ///
    /// On success returns `MtkEvent::Transfer`.
    Mint {
        /// Token id
        id: TokenId,
        /// Token amount.
        amount: u128,
        /// Token metadata, applicable if minting an NFT.
        token_metadata: Option<TokenMetadata>,
    },

    /// Burns a token.
    ///
    /// # Requirements:
    /// * a sender MUST have sufficient amount of token to burn.
    /// * a sender MUST be the owner.
    ///
    /// On success returns `MtkEvent::Transfer`.
    Burn {
        /// Token ID.
        id: TokenId,
        /// Amount of token to be burnt.
        amount: u128,
    },

    /// Gets an amount of tokens with `id` a user `account` has.
    ///
    /// On success returns `MtkEvent::BalanceOf`.
    BalanceOf {
        /// A user which balance is queried.
        account: ActorId,
        /// Token ID.
        id: TokenId,
    },

    /// Gets the amounts of multiple tokens for multiple users.
    ///
    /// On success returns `MtkEvent::BalanceOf`.
    BalanceOfBatch {
        /// Users which balances are queried.
        accounts: Vec<ActorId>,
        /// Tokens' IDs.
        ids: Vec<TokenId>,
    },

    /// Mints multiple tokens.
    ///
    /// # Requirements:
    /// * if minting an NFT with a specific TokenId at index `idx`
    ///   `amounts[idx]` MUST be equal to 1,
    ///   *`tokens_metadata` size MUST equal to the length of ids.
    /// * a sender MUST be an owner or an approved account.
    ///
    /// On success returns `MtkEvent::Transfer`
    MintBatch {
        /// Tokens' IDs to mint.
        ids: Vec<TokenId>,
        /// Tokens' amounts.
        amounts: Vec<u128>,
        /// Tokens' metadata.
        tokens_metadata: Vec<Option<TokenMetadata>>,
    },

    /// Transfers token from `from` to `to`.
    ///
    /// Requirements:
    /// * `from` and `to` MUST be different accounts.
    /// * `from` MUST have sufficient amount of tokens.
    /// * `to` MUST be a non-zero account.
    ///
    /// On success returns `MtkEvent::Transfer`.
    TransferFrom {
        /// From which account to transfer.
        from: ActorId,
        /// To which account to transfer.
        to: ActorId,
        /// Token's ID.
        id: TokenId,
        /// Token's amount.
        amount: u128,
    },

    /// Transfers multiple tokens from `from` to `to`.
    ///
    /// Requirements:
    /// * `from` and `to` MUST be different accounts.
    /// * `from` MUST have sufficient amount of tokens.
    /// * `to` MUST be a non-zero account.
    /// * `ids` and `amounts` MUST be the same length.
    ///
    /// On success returns `MtkEvent::Transfer`.
    BatchTransferFrom {
        /// From which account to transfer.
        from: ActorId,
        /// To which account to transfer.
        to: ActorId,
        /// Tokens' IDs.
        ids: Vec<TokenId>,
        /// Tokens' amounts.
        amounts: Vec<u128>,
    },

    /// Burns multiple tokens.
    ///
    /// # Requirements:
    /// * a sender MUST have sufficient amount of tokens to burn,
    /// * a sender MUST be the owner.
    ///
    /// On success returns `MtkEvent::Transfer
    BurnBatch {
        /// Tokens' IDs to burn.
        ids: Vec<TokenId>,
        /// Tokens' amounts to burn.
        amounts: Vec<u128>,
    },

    /// Allows a `account` to use tokens.
    ///
    /// On success returns `MtkEvent::Approval
    Approve {
        /// Approved account.
        account: ActorId,
    },

    /// Disallows a `account` to use tokens.
    ///
    /// On success returns `MtkEvent::RevokeApproval
    RevokeApproval {
        /// Disapproved account.
        account: ActorId,
    },

    /// Transforms user's tokens to multiple NFTs.
    ///
    /// # Requirements:
    /// * a sender MUST have sufficient amount of tokens to burn,
    /// * a sender MUST be the owner.
    ///
    /// On success returns `MtkEvent::Transfer`.
    Transform {
        /// Token's ID to burn.
        id: TokenId,
        /// Amount of burnt token.
        amount: u128,
        /// NFT minting data.
        nfts: Vec<BurnToNFT>,
    },
}

#[derive(Debug, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum MtkEvent {
    Transfer {
        from: ActorId,
        to: ActorId,
        ids: Vec<TokenId>,
        amounts: Vec<u128>,
    },
    BalanceOf(Vec<BalanceReply>),
    Approval {
        from: ActorId,
        to: ActorId,
    },
    RevokeApproval {
        from: ActorId,
        to: ActorId,
    },
}

#[derive(Debug, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum MtkError {
    ZeroAddress,
    LengthMismatch,
    MintMetadataToFungibleToken,
    NotEnoughBalance,
    TokenAlreadyExists,
    IdIsNotUnique,
    AmountGreaterThanOneForNft,
    TokenIdDoesNotExist,
    SenderAndRecipientAddressesAreSame,
    CallerIsNotOwnerOrApproved,
    WrongOwnerOrInsufficientBalance,
    InsufficientBalanceForTransfer,
    IncorrectData,
    WrongId,
    NoApprovals,
    ThereIsNoThisApproval,
}

#[derive(Debug, Encode, Decode, TypeInfo, PartialEq, Eq)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct BalanceReply {
    pub account: ActorId,
    pub id: TokenId,
    pub amount: u128,
}

#[derive(Debug, Decode, Encode, TypeInfo, Default, Clone, PartialEq, Eq)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct TokenMetadata {
    pub title: Option<String>,
    pub description: Option<String>,
    pub media: Option<String>,
    pub reference: Option<String>,
}

/// Initializes a Multitoken.
///
#[derive(Debug, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct InitMtk {
    /// Multitoken name.
    pub name: String,
    /// Multitoken symbol.
    pub symbol: String,
    /// Multitoken base URI.
    pub base_uri: String,
}

impl State {
    pub fn tokens_ids_for_owner(&self, owner: &ActorId) -> Vec<TokenId> {
        let mut tokens: Vec<TokenId> = Vec::new();
        let balances = &self.balances;
        for (token, bals) in balances {
            if bals.iter().any(|(id, _b)| owner.eq(id)) {
                tokens.push(*token);
            }
        }
        tokens
    }
    pub fn get_balance(&self, account: &ActorId, id: &TokenId) -> u128 {
        if let Some((_token_id, balances)) = self
            .balances
            .iter()
            .find(|(token_id, _balances)| id.eq(token_id))
        {
            if let Some((_owner, balance)) =
                balances.iter().find(|(owner, _balance)| account.eq(owner))
            {
                return *balance;
            }
        }
        0
    }
}
