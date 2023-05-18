#![no_std]

use gmeta::{metawasm, Metadata};
use gstd::{prelude::*, ActorId};
use student_nft_io::*;

#[cfg(feature = "binary-vendor")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

#[metawasm]
pub mod metafns {
    pub type State = <ContractMetadata as Metadata>::State;

    pub fn get_nfts(state: State) -> Vec<(NftId, Nft)> {
        state.nfts
    }

    pub fn get_nft_owners(state: State) -> Vec<(ActorId, NftId)> {
        state.nft_owners
    }

    pub fn get_courses(state: State) -> Vec<(CourseId, Course)> {
        state.courses
    }

    pub fn get_emotes(state: State) -> Vec<(EmoteId, EmoteState)> {
        state.emotes
    }
}
