#![no_std]

use dex_pair_io::*;
use gmeta::{metawasm, Metadata};
use gstd::{prelude::*, ActorId};

#[cfg(feature = "binary-vendor")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

#[metawasm]
pub mod metafns {
    pub type State = <ContractMetadata as Metadata>::State;

    pub fn token_addresses(state: State) -> (FungibleId, FungibleId) {
        state.token_addresses()
    }

    pub fn reserves(state: State) -> (u128, u128) {
        state.reserves()
    }

    pub fn prices(state: State) -> (u128, u128) {
        state.prices()
    }

    pub fn balance_of(state: State, address: ActorId) -> u128 {
        state.balance_of(address)
    }
}
