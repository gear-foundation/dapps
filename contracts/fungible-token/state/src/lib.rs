#![no_std]

use fungible_token_io::*;
use gstd::{prelude::*, ActorId};

#[gmeta::metawasm]
pub mod metafns {
    pub type State = IoFungibleToken;

    pub fn name(state: State) -> String {
        state.name
    }

    pub fn symbol(state: State) -> String {
        state.symbol
    }

    pub fn decimals(state: State) -> u8 {
        state.decimals
    }

    pub fn total_supply(state: State) -> u128 {
        state.total_supply
    }

    pub fn balances_of(state: State, account: ActorId) -> u128 {
        match state.balances.iter().find(|(id, _balance)| account.eq(id)) {
            Some((_id, balance)) => *balance,
            None => panic!("Balance for account ID {account:?} not found",),
        }
    }
}
