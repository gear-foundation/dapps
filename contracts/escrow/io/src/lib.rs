#![no_std]

use gstd::{prelude::*, ActorId};
use primitive_types::U256;

/// Escrow wallet ID.
pub type WalletId = U256;

#[derive(Decode, Encode, TypeInfo)]
pub struct InitEscrow {
    /// Address of a fungible token program.
    pub ft_program_id: ActorId,
}

#[derive(Decode, Encode, TypeInfo)]
pub enum EscrowAction {
    Create {
        buyer: ActorId,
        seller: ActorId,
        amount: u128,
    },
    Deposit(WalletId),
    Confirm(WalletId),
    Refund(WalletId),
    Cancel(WalletId),
}

#[derive(Decode, Encode, TypeInfo)]
pub enum EscrowEvent {
    Cancelled {
        buyer: ActorId,
        seller: ActorId,
        amount: u128,
    },
    Refunded {
        buyer: ActorId,
        amount: u128,
    },
    Confirmed {
        seller: ActorId,
        amount: u128,
    },
    Deposited {
        buyer: ActorId,
        amount: u128,
    },
    Created(WalletId),
}

#[derive(Decode, Encode, TypeInfo)]
pub enum EscrowState {
    GetInfo(WalletId),
}

#[derive(Decode, Encode, TypeInfo)]
pub enum EscrowStateReply {
    Info(Wallet),
}

#[derive(Decode, Encode, TypeInfo, Clone, Copy)]
pub struct Wallet {
    pub buyer: ActorId,
    pub seller: ActorId,
    pub state: WalletState,
    pub amount: u128,
}

#[derive(Decode, Encode, TypeInfo, PartialEq, Eq, Clone, Copy)]
pub enum WalletState {
    AwaitingDeposit,
    AwaitingConfirmation,
    Closed,
}
