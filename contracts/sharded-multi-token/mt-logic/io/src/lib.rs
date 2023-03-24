#![no_std]

mod instruction;

use gmeta::{In, InOut, Metadata};
use gstd::{prelude::*, ActorId, Decode, Encode, TypeInfo};
pub use instruction::*;
pub use mt_storage_io::TokenId;
use primitive_types::H256;

/// Upper bit of `TokenId` is a flag, that indicates if this is NFT or not.
pub const NFT_BIT: TokenId = 1 << (mem::size_of::<TokenId>() * 8 - 1);

/// Lower bits specifies NFT index.
pub const NFT_INDEX_MASK: TokenId = (!0) as TokenId;

/// Determines nft subtype by upper 64 bits(half of `TokenId`).
pub const NFT_TYPE_MASK: TokenId = ((!0) as TokenId) << 64;

pub struct MTLogicMetadata;

impl Metadata for MTLogicMetadata {
    type Init = In<InitMTLogic>;
    type Handle = InOut<MTLogicAction, MTLogicEvent>;
    type Others = InOut<Action, ()>;
    type Reply = ();
    type Signal = ();
    type State = MTLogicState;
}

/// Internal transaction entities possible status.
#[derive(Debug, Encode, Decode, TypeInfo, Clone, Copy)]
pub enum TransactionStatus {
    /// Transaction is in progress.
    InProgress,
    /// Transaction completed successfully.
    Success,
    /// Transaction is failed.
    Failure,
}

/// The contract state.
#[derive(Debug, Encode, Decode, TypeInfo, Clone)]
pub struct MTLogicState {
    /// Multitoken logic admin address.
    pub admin: ActorId,
    /// Multitoken main contract address.
    pub mtoken_id: ActorId,
    /// Stores abstract transactions statuses.
    pub transaction_status: Vec<(H256, TransactionStatus)>,
    /// Stores instructions which may contain a few multitoken operations.
    pub instructions: Vec<(H256, (Instruction, Instruction))>,
    /// Unique hash-identifier of storage contract code.
    pub storage_code_hash: H256,
    /// Mapping with specific id to multitoken storage impl: `String` -> `ActorId`(dedicated storage contract).
    pub id_to_storage: Vec<(String, ActorId)>,
    /// Global token nonce(counter).
    pub token_nonce: TokenId,
    /// Mapping with token URIs: `token_id` -> `String`(URI).
    pub token_uris: Vec<(TokenId, String)>,
    /// Mapping with tokens total supply: `token_id` -> `u128`.
    pub token_total_supply: Vec<(TokenId, u128)>,
    /// Mapping with token creators: `token_id` -> `ActorId`.
    pub token_creators: Vec<(TokenId, ActorId)>,
}

/// Sends the contract info about what it should do.
#[derive(Debug, Encode, Decode, TypeInfo, Clone)]
pub enum MTLogicAction {
    /// Handles high-level token operations.
    Message {
        /// Unique operation transaction hash.
        transaction_hash: H256,
        /// The actual account that made the operation (initiator).
        account: ActorId,
        /// Encoded high-level [`Action`] operation.
        payload: Vec<u8>,
    },
    /// Returns `account` token balance.
    ///
    /// # Requirements
    /// - `token_id` must exists in [`MTStorageState`](mt_storage_io::MTStorageState) state, in `balances` field.
    ///
    /// On success, replies with [`MTLogicEvent::Balance`].
    GetBalance {
        /// Token ID to get the balance.
        token_id: TokenId,
        /// Specifies the account whose balance you want to find out.
        account: ActorId,
    },
    /// Returns status approval for `approval_target` from `account`.
    ///
    /// # Requirements
    /// - `account` must exists in [`MTStorageState`](mt_storage_io::MTStorageState) state, in `approvals` field.
    ///
    /// On success, replies with [`MTLogicEvent::Approval`].
    GetApproval {
        /// An account that provides approve.
        account: ActorId,
        /// An account that is being verified.
        approval_target: ActorId,
    },
    /// Deletes the stored transaction entity with its status by unique hash.
    Clear(H256),
    /// Updates unique hash-identifier of storage contract code.
    UpdateStorageCodeHash(H256),
    /// Unimplemented.
    MigrateStorages,
}

/// A result of processed [`MTLogicAction`].
#[derive(Encode, Decode, TypeInfo)]
pub enum MTLogicEvent {
    /// Should be returned from [`MTLogicAction::Message`], if the operation is completed without errors.
    Ok,
    /// Should be returned from [`MTLogicAction::Message`], if the operation is completed with errors.
    Err,
    /// Should be returned from [`MTLogicAction::GetBalance`].
    Balance(u128),
    /// Should be returned from [`MTLogicAction::GetApproval`].
    Approval(bool),
}

/// High-level token-related operations.
#[derive(Encode, Debug, Decode, TypeInfo, Clone)]
pub enum Action {
    /// Transfer `amount` of `token_id` tokens from `sender` to `recipient`.
    ///
    /// # Requirements
    /// - `sender` must be equal to `msg_source` or `msg_source` must be approved by `sender`.
    /// - `sender` must have enough `amount` of `token_id` tokens.
    ///
    /// On success, replies with [`MTLogicEvent::Ok`].
    Transfer {
        /// Identifier of the token with which transfer will be performed.
        token_id: u128,
        /// Account from which tokens will be transferred.
        sender: ActorId,
        /// Transfer recipient.
        recipient: ActorId,
        /// Tokens amount for transfer.
        amount: u128,
    },
    /// Gives `approve` to `account` for various token-related operations.
    ///
    /// On success, replies with [`MTLogicEvent::Ok`].
    Approve {
        /// Account to which access is granted.
        account: ActorId,
        /// Approve flag.
        is_approved: bool,
    },
    /// Creates new token.
    ///
    /// On success, replies with [`MTLogicEvent::Ok`].
    Create {
        /// Initial token amount which will be minted to [`msg::source()`](gstd::msg::source), if `is_nft` flag is set, then ignored.
        initial_amount: u128,
        /// Base URI with token metadata.
        uri: String,
        /// Indicates if this token is nft.
        is_nft: bool,
    },
    /// Mints new fungible `token_id` tokens for `to` with `amounts`.
    ///
    /// # Requirements
    /// - `token_id` must be fungible.
    /// - `amounts` must be equal to `to`.
    ///
    /// On success, replies with [`MTLogicEvent::Ok`].
    MintBatchFT {
        /// Identifier of the token with which mint will be performed.
        token_id: TokenId,
        /// Vector with recipients.
        to: Vec<ActorId>,
        /// Vector with amounts.
        amounts: Vec<u128>,
    },
    /// Mints new non-fungible `token_id` tokens for `to`.
    ///
    /// # Requirements
    /// - `token_id` must be non-fungible.
    ///
    /// On success, replies with [`MTLogicEvent::Ok`].
    MintBatchNFT {
        /// Identifier of the token with which mint will be performed.
        token_id: TokenId,
        /// Vector with recipients.
        to: Vec<ActorId>,
    },
    /// Burns new fungible `token_id` tokens from `burn_from` for `amounts`.
    ///
    /// # Requirements
    /// - `token_id` must be fungible.
    /// - `amounts` must be equal to `burn_from`.
    /// - `burn_from` must approve [`msg::source()`](gstd::msg::source) if not equal.
    ///
    /// On success, replies with [`MTLogicEvent::Ok`].
    BurnBatchFT {
        /// Identifier of the token with which burn will be performed.
        token_id: TokenId,
        /// Vector with targets.
        burn_from: Vec<ActorId>,
        /// Vector with burn amounts.
        amounts: Vec<u128>,
    },
    /// Burns new non-fungible `token_id` token from `from`.
    ///
    /// # Requirements
    /// - `token_id` must be non-fungible.
    /// - `from` must approve [`msg::source()`](gstd::msg::source) if not equal.
    /// - `from` must be owner of `token_id`.
    ///
    /// On success, replies with [`MTLogicEvent::Ok`].
    BurnNFT {
        /// Identifier of the token with which burn will be performed.
        token_id: TokenId,
        /// Burn target(account).
        from: ActorId,
    },
}

/// Initializes the contract.
///
/// # Requirements
/// - `admin` mustn't be [`ActorId::zero()`].
/// - `storage_code_hash` mustn't be zero.
#[derive(Encode, Decode, TypeInfo)]
pub struct InitMTLogic {
    /// Has ability to update storage code hash.
    pub admin: ActorId,
    /// Unique hash-identifier of storage contract code.
    pub storage_code_hash: H256,
}
