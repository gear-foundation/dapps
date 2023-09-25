#![no_std]

use gstd::{prelude::*, ActorId};
use oracle_randomness_io::*;

#[gmeta::metawasm]
pub mod metafns {
    pub type State = RandomnessOracle;

    pub fn get_owner(state: State) -> ActorId {
        state.owner
    }

    pub fn get_manager(state: State) -> ActorId {
        state.manager
    }

    pub fn get_values(state: State) -> Vec<(u128, state::Random)> {
        state
            .values
            .iter()
            .map(|(round, value)| (*round, value.clone()))
            .collect()
    }

    pub fn get_value(state: State, round: u128) -> state::Random {
        state
            .values
            .get(&round)
            .expect("Unable to find round!")
            .clone()
    }

    pub fn get_last_round(state: State) -> u128 {
        state.last_round
    }

    pub fn get_last_random_value(state: State) -> state::RandomSeed {
        state
            .values
            .get(&state.last_round)
            .expect("Unable to find round!")
            .clone()
            .randomness
    }

    pub fn get_random_value_from_round(state: State, round: u128) -> state::RandomSeed {
        state
            .values
            .get(&round)
            .expect("Unable to find round!")
            .clone()
            .randomness
    }
}
