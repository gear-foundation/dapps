#![no_std]
use gmeta::metawasm;
use gstd::{exec, prelude::*};
use tmg_io::Tamagotchi;

pub const HUNGER_PER_BLOCK: u64 = 1;
pub const BOREDOM_PER_BLOCK: u64 = 2;
pub const ENERGY_PER_BLOCK: u64 = 2;

#[metawasm]
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

#[derive(Encode, Decode, TypeInfo)]
pub struct TmgCurrentState {
    pub fed: u64,
    pub entertained: u64,
    pub rested: u64,
}
