#![no_std]

mod instruction;

use gmeta::{In, InOut, Metadata};
use gstd::{prelude::*, ActorId, Decode, Encode, TypeInfo};
pub use instruction::*;
use mt_main_io::LogicAction;
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
    type Others = InOut<LogicAction, ()>;
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
