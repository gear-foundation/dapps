#![no_std]

use gmeta::{metawasm, Metadata};
use gstd::{prelude::*, ActorId};
use staking_io::{Staker, StakingMetadata};

#[metawasm]
pub mod metafns {
    pub type State = <StakingMetadata as Metadata>::State;

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
