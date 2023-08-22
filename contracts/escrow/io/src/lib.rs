#![no_std]

use gmeta::{In, InOut, Metadata};
use gstd::{prelude::*, ActorId};
use primitive_types::U256;

pub struct EscrowMetadata;

impl Metadata for EscrowMetadata {
    type Init = In<InitEscrow>;
    type Handle = InOut<EscrowAction, EscrowEvent>;
    type Others = ();
    type Reply = ();
    type Signal = ();
    type State = EscrowState;
}

#[derive(Default, Encode, Decode, Clone, TypeInfo)]
pub struct EscrowState {
    pub ft_program_id: ActorId,
    pub wallets: Vec<(WalletId, Wallet)>,
    pub id_nonce: WalletId,
    pub transaction_id: u64,
    pub transactions: Vec<(u64, Option<EscrowAction>)>,
}

/// An escrow wallet ID.
pub type WalletId = U256;

/// Initializes an escrow program.
#[derive(Decode, Encode, TypeInfo)]
pub struct InitEscrow {
    /// Address of a fungible token program.
    pub ft_program_id: ActorId,
}

/// An enum to send the program info about what it should do.
///
/// After a successful processing of this enum, the program replies with [`EscrowEvent`].
#[derive(Clone, Decode, Encode, TypeInfo)]
pub enum EscrowAction {
    /// Creates one escrow wallet and replies with its ID.
    ///
    /// # Requirements
    /// * [`msg::source()`](gstd::msg::source) must be `buyer` or `seller` for this wallet.
    /// * `buyer` or `seller` mustn't have the zero address.
    ///
    /// On success, returns [`EscrowEvent::Created`].
    Create {
        /// A buyer.
        buyer: ActorId,
        /// A seller.
        seller: ActorId,
        /// An amount of tokens.
        amount: u128,
    },

    /// Makes a deposit from a buyer to an escrow wallet
    /// and changes wallet's [`WalletState`] to [`AwaitingConfirmation`](WalletState::AwaitingConfirmation).
    ///
    /// Transfers tokens to an escrow wallet until a deal is confirmed (by [`EscrowAction::Confirm`]) or cancelled ([`EscrowAction::Cancel`]).
    ///
    /// # Requirements
    /// * [`msg::source()`](gstd::msg::source) must be a buyer for this wallet.
    /// * Wallet mustn't be paid or closed (that is, wallet's [`WalletState`] must be [`AwaitingDeposit`](WalletState::AwaitingDeposit)).
    ///
    /// On success, returns [`EscrowEvent::Deposited`].
    Deposit(
        /// A wallet ID.
        WalletId,
    ),

    /// Confirms a deal by transferring tokens from an escrow wallet
    /// to a seller and changing wallet's [`WalletState`] to [`Closed`](WalletState::Closed).
    ///
    /// Transfers tokens from an escrow wallet to a seller for this wallet.
    ///
    /// # Requirements
    /// * [`msg::source()`](gstd::msg::source) must be a buyer for this wallet.
    /// * Wallet must be paid and unclosed (that is, wallet's [`WalletState`] must be [`AwaitingDeposit`](WalletState::AwaitingConfirmation)).
    ///
    /// On success, returns [`EscrowEvent::Confirmed`].
    Confirm(
        /// A wallet ID.
        WalletId,
    ),

    /// Refunds tokens from an escrow wallet to a buyer
    /// and changes wallet's [`WalletState`] back to [`AwaitingDeposit`](WalletState::AwaitingDeposit)
    /// (that is, a wallet can be reused).
    ///
    /// Refunds tokens from an escrow wallet to a buyer for this wallet.
    ///
    /// # Requirements
    /// * [`msg::source()`](gstd::msg::source) must be a seller for this wallet.
    /// * Wallet must be paid and unclosed (that is, wallet's [`WalletState`] must be [`AwaitingDeposit`](WalletState::AwaitingConfirmation)).
    ///
    /// On success, returns [`EscrowEvent::Refunded`].
    Refund(
        /// A wallet ID.
        WalletId,
    ),

    /// Cancels a deal and closes an escrow wallet by changing its [`WalletState`] to [`Closed`](WalletState::Closed).
    ///
    /// # Requirements
    /// * [`msg::source()`](gstd::msg::source) must be a buyer or seller for this wallet.
    /// * Wallet mustn't be paid or closed (that is, wallet's [`WalletState`] must be [`AwaitingDeposit`](WalletState::AwaitingDeposit)).
    ///
    /// On success, returns [`EscrowEvent::Cancelled`].
    Cancel(
        /// A wallet ID.
        WalletId,
    ),

    /// Continues the transaction if it fails due to lack of gas
    /// or due to an error in the token contract.
    ///
    /// # Requirements:
    /// * `transaction_id` should exists in `transactions` table;
    ///
    /// When transaction already processed replies with [`EscrowEvent::TransactionProcessed`].
    Continue(
        /// Identifier of suspended transaction.
        u64,
    ),
}

/// An enum that contains a result of processed [`EscrowAction`].
#[derive(Decode, Encode, TypeInfo)]
pub enum EscrowEvent {
    Cancelled(
        /// An ID of a wallet with a cancelled deal.
        WalletId,
    ),
    Refunded(
        /// Transaction id.
        u64,
        /// An ID of a refunded wallet.
        WalletId,
    ),
    Confirmed(
        /// Transaction id.
        u64,
        /// An ID of a wallet with a confirmed deal.
        WalletId,
    ),
    Deposited(
        /// Transaction id.
        u64,
        /// An ID of a deposited wallet.
        WalletId,
    ),
    Created(
        /// An ID of a created wallet.
        WalletId,
    ),
    TransactionProcessed,
    TransactionFailed,
}

/// Escrow wallet information.
#[derive(Decode, Encode, TypeInfo, Clone, Copy, Debug, PartialEq, Eq)]
pub struct Wallet {
    /// A buyer.
    pub buyer: ActorId,
    /// A seller.
    pub seller: ActorId,
    /// A wallet state.
    pub state: WalletState,
    /// An amount of tokens that a wallet can have. **Not** a current amount on a wallet balance!
    pub amount: u128,
}

/// An escrow wallet state.
#[derive(Decode, Encode, TypeInfo, PartialEq, Eq, Clone, Copy, Debug)]
pub enum WalletState {
    AwaitingDeposit,
    AwaitingConfirmation,
    Closed,
}
