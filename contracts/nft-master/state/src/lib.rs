#![no_std]

use gmeta::{metawasm, Metadata};
use gstd::{prelude::*, ActorId};
use nft_master_io::*;

#[cfg(feature = "binary-vendor")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

#[metawasm]
pub mod metafns {
    pub type State = <NFTMasterMetadata as Metadata>::State;

    pub fn get_nfts(state: State) -> Vec<(ActorId, String)> {
        state.nfts
    }

    pub fn get_operators(state: State) -> Vec<ActorId> {
        state.operators
    }
}
