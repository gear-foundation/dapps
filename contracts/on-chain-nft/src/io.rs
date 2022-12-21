use gear_lib::non_fungible_token::{
    io::{NFTApproval, NFTTransfer, NFTTransferPayout},
    royalties::*,
    state::*,
    token::*,
};
use gstd::{prelude::*, ActorId};

pub type LayerId = u128;
pub type ItemId = u128;

#[derive(Debug, Encode, Decode, TypeInfo)]
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
pub enum OnChainNFTEvent {
    Transfer(NFTTransfer),
    TransferPayout(NFTTransferPayout),
    Approval(NFTApproval),
}
