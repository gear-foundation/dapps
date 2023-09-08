#![no_std]

use gstd::{prelude::*, ActorId};
use staking_io::*;

#[gmeta::metawasm]
pub mod metafns {
    pub type State = IoStaking;

    pub fn get_stakers(state: State) -> Vec<(ActorId, Staker)> {
        state.stakers
    }

    pub fn get_staker(state: State, address: ActorId) -> Option<Staker> {
        state
            .stakers
            .iter()
            .find(|(id, _staker)| address.eq(id))
            .map(|(_, staker)| staker.clone())
    }
}
