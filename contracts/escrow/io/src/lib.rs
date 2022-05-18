#![no_std]

use gstd::{prelude::*, ActorId};

#[derive(Decode, Encode)]
pub struct InitEscrow {
    pub ft_program_id: ActorId,
}

#[derive(Decode, Encode)]
pub enum EscrowAction {
    Create {
        buyer: ActorId,
        seller: ActorId,
        amount: u128,
    },
    Deposit {
        contract_id: u128,
    },
    Confirm {
        contract_id: u128,
    },
    Refund {
        contract_id: u128,
    },
    Cancel {
        contract_id: u128,
    },
}

#[derive(Decode, Encode)]
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
    Created {
        contract_id: u128,
    },
}
