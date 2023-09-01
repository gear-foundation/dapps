#![no_std]

use gstd::{prelude::*, ActorId};
use nft_master_io::*;

#[gmeta::metawasm]
pub mod metafns {
    pub type State = NFTMasterState;

    pub fn get_nfts(state: State) -> Vec<(ActorId, String)> {
        state.nfts
    }

    pub fn get_operators(state: State) -> Vec<ActorId> {
        state.operators
    }
}
