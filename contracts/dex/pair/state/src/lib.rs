#![no_std]

use dex_pair_io::*;
use gmeta::{metawasm, Metadata};
use gstd::{prelude::*, ActorId};

#[metawasm]
pub trait Metawasm {
    type State = <ContractMetadata as Metadata>::State;

    fn token_addresses(state: Self::State) -> (FungibleId, FungibleId) {
        state.token_addresses()
    }

    fn reserves(state: Self::State) -> (u128, u128) {
        state.reserves()
    }

    fn prices(state: Self::State) -> (u128, u128) {
        state.prices()
    }

    fn balance_of(address: ActorId, state: Self::State) -> u128 {
        state.balance_of(address)
    }
}
