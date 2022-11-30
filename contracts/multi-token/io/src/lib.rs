#![no_std]

use gear_lib::multitoken::io::*;
use gstd::{prelude::*, ActorId};

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
pub enum MyMTKAction {
    /// Mints a token.
    ///
    /// # Requirements:
    /// * if minting an NFT `amount` MUST equal to 1.
    /// * a sender MUST be an owner or an approved account.
    ///
    /// On success returns `MTKEvent::Transfer`.
    Mint {
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
    /// On success returns `MTKEvent::Transfer`.
    Burn {
        /// Token ID.
        id: TokenId,
        /// Amount of token to be burnt.
        amount: u128,
    },

    /// Gets an amount of tokens with `id` a user `account` has.
    ///
    /// On success returns `MTKEvent::BalanceOf`.
    BalanceOf {
        /// A user which balance is queried.
        account: ActorId,
        /// Token ID.
        id: TokenId,
    },

    /// Gets the amounts of multiple tokens for multiple users.
    ///
    /// On success returns `MTKEvent::BalanceOf`.
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
    /// `amounts[idx]` MUST be equal to 1,
    /// *`tokens_metadata` size MUST equal to the length of ids.
    /// * a sender MUST be an owner or an approved account.
    ///
    /// On success returns `MTKEvent::Transfer`
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
    /// On success returns `MTKEvent::Transfer`.
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
    /// On success returns `MTKEvent::Transfer`.
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
    /// On success returns `MTKEvent::Transfer
    BurnBatch {
        /// Tokens' IDs to burn.
        ids: Vec<TokenId>,
        /// Tokens' amounts to burn.
        amounts: Vec<u128>,
    },

    /// Allows a `account` to use tokens.
    ///
    /// On success returns `MTKEvent::Approval
    Approve {
        /// Approved account.
        account: ActorId,
    },

    /// Disallows a `account` to use tokens.
    ///
    /// On success returns `MTKEvent::RevokeApproval
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
    /// On success returns `MTKEvent::Transfer`.
    Transform {
        /// Token's ID to burn.
        id: TokenId,
        /// Amount of burnt token.
        amount: u128,
        /// NFT minting data.
        nfts: Vec<BurnToNFT>,
    },
}

/// Initializes a Multitoken.
///
#[derive(Debug, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct InitMTK {
    /// Multitoken name.
    pub name: String,
    /// Multitoken symbol.
    pub symbol: String,
    /// Multitoken base URI.
    pub base_uri: String,
}
