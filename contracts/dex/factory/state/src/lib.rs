#![no_std]

use dex_factory_io::*;
use gmeta::{metawasm, Metadata};
use gstd::{prelude::*, ActorId};

#[cfg(feature = "binary-vendor")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

#[metawasm]
pub mod metafns {
    pub type State = <ContractMetadata as Metadata>::State;

    pub fn fee_to(state: State) -> ActorId {
        state.fee_to
    }

    pub fn fee_to_setter(state: State) -> ActorId {
        state.fee_to_setter
    }

    pub fn pair_address(state: State, pair: (FungibleId, FungibleId)) -> ActorId {
        state.pair_address(pair.0, pair.1)
    }

    pub fn all_pairs_length(state: State) -> u32 {
        state.all_pairs_length()
    }

    pub fn owner(state: State) -> ActorId {
        state.owner_id
    }
}
