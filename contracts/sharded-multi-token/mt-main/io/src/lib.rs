#![no_std]

use gmeta::{In, InOut, Metadata};
use gstd::{prelude::*, ActorId};
use mt_logic_io::TokenId;
use primitive_types::H256;

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
        /// Encoded high-level [`Action`](mt_logic_io::Action) operation.
        payload: Vec<u8>,
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
