#![no_std]

use gmeta::{In, InOut, Metadata};
use gstd::{prelude::*, ActorId};
use primitive_types::H256;

pub type TokenId = u128;

pub struct MTMainMetadata;

impl Metadata for MTMainMetadata {
    type Init = In<InitMToken>;
    type Handle = InOut<MTokenAction, MTokenEvent>;
    type Others = ();
    type Reply = ();
    type Signal = ();
    type State = MTokenState;
}

/// The contract state.
#[derive(Debug, Encode, Decode, TypeInfo, Clone)]
pub struct MTokenState {
    /// Multitoken main contract admin.
    pub admin: ActorId,
    /// Address of multitoken logic contract.
    pub mt_logic_id: ActorId,
    /// Stores abstract transactions statuses.
    pub transactions: Vec<(H256, TransactionStatus)>,
}

/// Internal transaction entities possible status.
#[derive(Encode, Decode, TypeInfo, Copy, Clone, Debug)]
pub enum TransactionStatus {
    /// Transaction is in progress.
    InProgress,
    /// Transaction completed successfully.
    Success,
    /// Transaction is failed.
    Failure,
}

/// Sends the contract info about what it should do.
#[derive(Encode, Decode, TypeInfo, Debug)]
pub enum MTokenAction {
    /// Handles high-level token operations.
    Message {
        /// Operation transaction id(each new abstract-transaction must increase).
        transaction_id: u64,
        /// Encoded high-level [`LogicAction`] operation.
        payload: LogicAction,
    },
    /// Updates unique hash-identifier or multitoken storage and logic contract code.
    ///
    /// On success, replies with [`MTokenEvent::Ok`].
    UpdateLogicContract {
        /// Unique hash-identifier of logic contract code.
        mt_logic_code_hash: H256,
        /// Unique hash-identifier of storage contract code.
        storage_code_hash: H256,
    },
    /// Returns `account` token balance.
    ///
    /// On success, replies with [`MTokenEvent::Balance`].
    GetBalance {
        /// Token ID to get the balance.
        token_id: TokenId,
        /// Specifies the account whose balance you want to find out.
        account: ActorId,
    },
    /// Returns status approval for `approval_target` from `account`.
    ///
    /// On success, replies with [`MTokenEvent::Approval`].
    GetApproval {
        /// An account that provides approve.
        account: ActorId,
        /// An account that is being verified.
        approval_target: ActorId,
    },
    /// Deletes the stored transaction entity with its status by unique hash.
    Clear(H256),
    /// Unimplemented.
    MigrateStorageAddresses,
}

/// A result of processed [`MTokenAction`].
#[derive(Encode, Decode, TypeInfo, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum MTokenEvent {
    /// Should be returned from [`MTokenAction::Message`], if the operation is completed without errors.
    Ok,
    /// Should be returned from [`MTokenAction::Message`], if the operation is completed with errors.
    Err,
    /// Should be returned from [`MTokenAction::GetBalance`].
    Balance(u128),
    /// Should be returned from [`MTokenAction::GetApproval`].
    Approval(bool),
}

/// High-level token-related operations.
#[derive(Encode, Debug, Decode, TypeInfo, Clone)]
pub enum LogicAction {
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
/// - `storage_code_hash` mustn't be zero.
/// - `mt_logic_code_hash` mustn't be zero.
#[derive(Encode, Decode, TypeInfo)]
pub struct InitMToken {
    /// Unique hash-identifier of storage contract code.
    pub storage_code_hash: H256,
    /// Unique hash-identifier of logic contract code.
    pub mt_logic_code_hash: H256,
}
