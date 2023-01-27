#![no_std]

use gmeta::{metawasm, Metadata};
use gstd::{prelude::*, ActorId};
use staking_io::{Staker, StakingMetadata};

#[metawasm]
pub trait Metawasm {
    type State = <StakingMetadata as Metadata>::State;

    fn get_stakers(state: Self::State) -> Vec<(ActorId, Staker)> {
        state.stakers
    }

    fn get_staker(address: ActorId, state: Self::State) -> Staker {
        match state.stakers.iter().find(|(id, _staker)| address.eq(id)) {
            Some((_id, staker)) => staker.clone(),
            None => panic!("Staker with the ID = {address:?} doesn't exists"),
        }
    }
}
