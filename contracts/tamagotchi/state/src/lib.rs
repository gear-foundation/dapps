#![no_std]

use gstd::{exec, prelude::*};
use tamagotchi_io::*;

pub const HUNGER_PER_BLOCK: u64 = 1;
pub const BOREDOM_PER_BLOCK: u64 = 2;
pub const ENERGY_PER_BLOCK: u64 = 2;

#[gmeta::metawasm]
pub mod metafns {
    pub type State = Tamagotchi;

    pub fn current_state(state: State) -> TmgCurrentState {
        let fed = state.fed.saturating_sub(
            HUNGER_PER_BLOCK * ((exec::block_timestamp() - state.fed_block) / 1_000),
        );
        let entertained = state.entertained.saturating_sub(
            BOREDOM_PER_BLOCK * ((exec::block_timestamp() - state.entertained_block) / 1_000),
        );
        let rested = state.rested.saturating_sub(
            ENERGY_PER_BLOCK * ((exec::block_timestamp() - state.rested_block) / 1_000),
        );
        TmgCurrentState {
            fed,
            entertained,
            rested,
        }
    }
}
