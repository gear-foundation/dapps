#![no_std]

use gstd::{prelude::*, ActorId};
use student_nft_io::*;

#[gmeta::metawasm]
pub mod metafns {
    pub type State = StudentNftState;

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
