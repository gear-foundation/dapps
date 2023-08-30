#![no_std]

use gear_lib_old::non_fungible_token::{
    io::{NFTApproval, NFTTransfer, NFTTransferPayout},
    royalties::*,
    state::*,
    token::*,
};
use gmeta::{In, InOut, Metadata, Out};
use gstd::{prelude::*, ActorId};

pub type LayerId = u128;
pub type ItemId = u128;

pub struct ContractMetadata;

impl Metadata for ContractMetadata {
    type Init = In<InitOnChainNFT>;
    type Handle = InOut<OnChainNFTAction, OnChainNFTEvent>;
    type Reply = ();
    type Others = ();
    type Signal = ();
    type State = Out<State>;
}

#[derive(Debug, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum OnChainNFTQuery {
    /// Returns an NFT for a specified `token_id`.
    ///
    /// Requirements:
    /// * `token_id` MUST exist
    ///
    /// Arguments:
    /// * `token_id` - is the id of the NFT
    ///
    /// On success, returns TokenURI struct.
    TokenURI { token_id: TokenId },
    /// Base NFT query. Derived from gear-lib.
    Base(NFTQuery),
}

#[derive(Debug, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum OnChainNFTAction {
    /// Mints an NFT consisting of layers provided in the `description` parameter.
    ///
    /// Requirements:
    /// * `description` MUST contain layers and layers' items that EXIST
    ///
    /// Arguments:
    /// * `token_metadata` - is a default token metadata from gear-lib.
    /// * `description` - is the vector of layer's item id, where
    /// the index i is the layer id.
    ///
    /// On success, returns NFTEvent::Mint from gear-lib.
    Mint {
        /// Metadata
        token_metadata: TokenMetadata,
        /// Layers description of an NFT
        description: Vec<ItemId>,
    },
    /// Burns an NFT.
    ///
    /// Requirements:
    /// * `token_id` MUST exist
    /// Arguments:
    ///
    /// * `token_id` - is the id of the burnt token
    ///
    /// On success, returns NFTEvent::Burn from gear-lib.
    Burn {
        /// Token id to burn.
        token_id: TokenId,
    },
    /// Transfers an NFT.
    ///
    /// Requirements:
    /// * `token_id` MUST exist
    /// * `to` MUST be a non-zero addresss
    ///
    /// Arguments:
    /// * `token_id` - is the id of the transferred token
    ///
    /// On success, returns NFTEvent::Transfer from gear-lib.
    Transfer {
        /// A recipient address.
        to: ActorId,
        /// Token id to transfer.
        token_id: TokenId,
    },
    /// Approves an account to perform operation upon the specifiefd NFT.
    ///
    /// Requirements:
    /// * `token_id` MUST exist
    /// * `to` MUST be a non-zero addresss
    ///
    /// Arguments:
    /// * `token_id` - is the id of the transferred token
    ///
    /// On success, returns NFTEvent::Approval from gear-lib.
    Approve {
        /// An account being approved.
        to: ActorId,
        /// Token id approved for the account.
        token_id: TokenId,
    },
    /// Transfers payouts from an NFT to an account.
    ///
    /// Requirements:
    /// * `token_id` MUST exist
    /// * `to` MUST be a non-zero addresss
    /// * `amount` MUST be a non-zero number
    ///
    /// Arguments:
    /// * `token_id` - is the id of the transferred token
    ///
    /// On success, returns NFTEvent::Approval from gear-lib.
    TransferPayout {
        /// Payout recipient
        to: ActorId,
        /// Token id to get the payout from.
        token_id: TokenId,
        /// Payout amount.
        amount: u128,
    },
}

#[derive(Debug, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct TokenURI {
    /// Token metadata derived from gear-lib
    pub metadata: TokenMetadata,
    /// List of base64encoded svgs representing different layers of an NFT.
    pub content: Vec<String>,
}

/// Initializes on-chain NFT
/// Requirements:
/// * all fields except `royalties` should be specified
#[derive(Debug, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct InitOnChainNFT {
    /// NFT name
    pub name: String,
    /// NFT symbol
    pub symbol: String,
    /// NFT base_uri (not applicable in on-chain)
    pub base_uri: String,
    /// Base Image is base64encoded svg.
    /// Provides a base layer for all future nfts.
    pub base_image: String,
    /// Layers map - mapping of layerid the list of layer items.
    /// Each layer item is a base64encoded svg.
    pub layers: Vec<(LayerId, Vec<String>)>,
    /// Royalties for NFT
    pub royalties: Option<Royalties>,
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum OnChainNFTEvent {
    Transfer(NFTTransfer),
    TransferPayout(NFTTransferPayout),
    Approval(NFTApproval),
}

#[derive(Debug, Clone, Default, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct IoNFTState {
    pub name: String,
    pub symbol: String,
    pub base_uri: String,
    pub owner_by_id: Vec<(TokenId, ActorId)>,
    pub token_approvals: Vec<(TokenId, Vec<ActorId>)>,
    pub token_metadata_by_id: Vec<(TokenId, Option<TokenMetadata>)>,
    pub tokens_for_owner: Vec<(ActorId, Vec<TokenId>)>,
    pub royalties: Option<Royalties>,
}

#[derive(Debug, Clone, Default, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct State {
    pub token: IoNFTState,
    pub token_id: TokenId,
    pub owner: ActorId,
    pub base_image: String,
    pub layers: Vec<(LayerId, Vec<String>)>,
    pub nfts: Vec<(TokenId, Vec<ItemId>)>,
    pub nfts_existence: Vec<String>,
}

impl From<&NFTState> for IoNFTState {
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

impl IoNFTState {
    pub fn token(&self, token_id: TokenId) -> Token {
        let mut token = Token::default();
        if let Some((_, owner_id)) = self.owner_by_id.iter().find(|(id, _)| token_id.eq(id)) {
            token.id = token_id;
            token.owner_id = *owner_id;
        }
        if let Some((_, approved_account_ids)) =
            self.token_approvals.iter().find(|(id, _)| token_id.eq(id))
        {
            token.approved_account_ids = approved_account_ids.iter().copied().collect();
        }
        if let Some((_, Some(metadata))) = self
            .token_metadata_by_id
            .iter()
            .find(|(id, _)| token_id.eq(id))
        {
            token.name = metadata.name.clone();
            token.description = metadata.description.clone();
            token.media = metadata.media.clone();
            token.reference = metadata.reference.clone();
        }
        token
    }

    pub fn tokens_for_owner(&self, owner: &ActorId) -> Vec<Token> {
        let mut tokens = vec![];

        if let Some((_owner, token_ids)) = self.tokens_for_owner.iter().find(|(id, _)| owner.eq(id))
        {
            for token_id in token_ids {
                tokens.push(self.token(*token_id))
            }
        }
        tokens
    }

    pub fn total_supply(&self) -> u128 {
        self.owner_by_id.len() as u128
    }

    pub fn supply_for_owner(&self, owner: &ActorId) -> u128 {
        if let Some((_owner, tokens)) = self.tokens_for_owner.iter().find(|(id, _)| owner.eq(id)) {
            tokens.len() as u128
        } else {
            0
        }
    }

    pub fn all_tokens(&self) -> Vec<Token> {
        self.owner_by_id
            .iter()
            .map(|(token_id, _toks)| self.token(*token_id))
            .collect()
    }

    pub fn approved_tokens(&self, account: &ActorId) -> Vec<Token> {
        self.owner_by_id
            .iter()
            .filter_map(|(id, _)| {
                self.token_approvals
                    .iter()
                    .find(|(token_id, _)| id.eq(token_id))
                    .and_then(|(id, approvals)| {
                        approvals.contains(account).then_some(self.token(*id))
                    })
            })
            .collect()
    }
}
