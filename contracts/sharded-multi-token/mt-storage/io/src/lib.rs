#![no_std]

use gmeta::{InOut, Metadata};
use gstd::{prelude::*, ActorId};
use primitive_types::H256;

pub type TokenId = u128;

pub struct MTStorageMetadata;

impl Metadata for MTStorageMetadata {
    type Init = ();
    type Handle = InOut<MTStorageAction, MTStorageEvent>;
    type Others = ();
    type Reply = ();
    type Signal = ();
    type State = MTStorageState;
}

/// The contract state.
#[derive(Encode, Decode, Clone, Debug, TypeInfo)]
pub struct MTStorageState {
    /// Address of multitoken logic contract.
    pub mt_logic_id: ActorId,
    /// Stores abstract transactions statuses.
    pub transaction_status: Vec<(H256, bool)>,
    /// Mapping with balances: `TokenId` -> `ActorId` -> `u128`.
    pub balances: Vec<(TokenId, Vec<(ActorId, u128)>)>,
    /// Mapping with approvals: `ActorId` -> `ActorId` -> `bool`.
    pub approvals: Vec<(ActorId, Vec<(ActorId, bool)>)>,
}

/// Sends the contract info about what it should do.
#[derive(Encode, Decode, Debug, Clone, TypeInfo)]
pub enum MTStorageAction {
    /// Returns `account` token balance.
    ///
    /// # Requirements
    /// - `token_id` must exists in [`MTStorageState`] state, in `balances` field.
    ///
    /// On success, replies with [`MTStorageEvent::Balance`].
    GetBalance {
        /// Token ID to get the balance.
        token_id: TokenId,
        /// Specifies the account whose balance you want to find out.
        account: ActorId,
    },
    /// Returns status approval for `approval_target` from `account`.
    ///
    /// # Requirements
    /// - `account` must exists in [`MTStorageState`] state, in `approvals` field.
    ///
    /// On success, replies with [`MTStorageEvent::Approval`].
    GetApproval {
        /// An account that provides approve.
        account: ActorId,
        /// An account that is being verified.
        approval_target: ActorId,
    },
    /// Transfer `amount` of `token_id` tokens from `sender` to `recipient`.
    ///
    /// # Requirements
    /// - [`msg::source()`](gstd::msg::source) must be multitoken logic contract.
    /// - `sender` must be equal to `msg_source` or `msg_source` must be approved by `sender`.
    /// - `sender` must have enough `amount` of `token_id` tokens.
    ///
    /// On success, replies with [`MTStorageEvent::Ok`].
    Transfer {
        /// Unique transfer transaction hash.
        transaction_hash: H256,
        /// Identifier of the token with which transfer will be performed.
        token_id: TokenId,
        /// The actual account that made the transfer (initiator).
        msg_source: ActorId,
        /// Account from which tokens will be transferred.
        sender: ActorId,
        /// Transfer recipient.
        recipient: ActorId,
        /// Tokens amount for transfer.
        amount: u128,
    },
    /// Gives `approve` to `account` for various token-related operations.
    ///
    /// # Requirements
    /// - [`msg::source()`](gstd::msg::source) must be multitoken logic contract.
    ///
    /// On success, replies with [`MTStorageEvent::Ok`].
    Approve {
        /// Unique approve transaction hash.
        transaction_hash: H256,
        /// The actual account that made the approve (initiator).
        msg_source: ActorId,
        /// Account to which access is granted.
        account: ActorId,
        /// Approve flag.
        approve: bool,
    },
    /// Deletes the stored transaction entity with its status by unique hash.
    ClearTransaction(H256),
    /// Increase `account` balance of `token_id` tokens.
    ///
    /// # Requirements
    /// - [`msg::source()`](gstd::msg::source) must be multitoken logic contract.
    ///
    /// On success, replies with [`MTStorageEvent::Ok`].
    IncreaseBalance {
        /// Unique operation transaction hash.
        transaction_hash: H256,
        /// Identifier of the token with which increase balance will be performed.
        token_id: TokenId,
        /// An account that needs to increase its balance.
        account: ActorId,
        /// Number of tokens by which the balance will be increased.
        amount: u128,
    },
    /// Decrease `account` balance of `token_id` tokens.
    ///
    /// # Requirements
    /// - [`msg::source()`](gstd::msg::source) must be multitoken logic contract.
    ///
    /// On success, replies with [`MTStorageEvent::Ok`].
    DecreaseBalance {
        /// Unique operation transaction hash.
        transaction_hash: H256,
        /// Identifier of the token with which decrease balance will be performed.
        token_id: TokenId,
        /// The actual account that made the decrease operation (initiator).
        msg_source: ActorId,
        /// An account that needs to decrease its balance.
        account: ActorId,
        /// Number of tokens by which the balance will be decreased.
        amount: u128,
    },
}

/// A result of processed [`MTStorageAction`].
#[derive(Encode, Decode, Clone, Debug, TypeInfo)]
pub enum MTStorageEvent {
    /// Should be returned from any mutable operation from [`MTStorageAction`], if the operation is completed without errors.
    Ok,
    /// Should be returned from any mutable operation from [`MTStorageAction`], if the operation is completed with errors.
    Err,
    /// Should be returned from [`MTStorageAction::GetBalance`].
    Balance(u128),
    /// Should be returned from [`MTStorageAction::GetApproval`].
    Approval(bool),
}
