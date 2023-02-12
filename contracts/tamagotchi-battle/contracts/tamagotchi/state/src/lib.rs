#![no_std]
use gmeta::metawasm;
use gstd::{prelude::*, exec};
use tmg_io::Tamagotchi;

pub const HUNGER_PER_BLOCK: u64 = 1;
pub const BOREDOM_PER_BLOCK: u64 = 2;
pub const ENERGY_PER_BLOCK: u64 = 2;

#[metawasm]
pub trait Metawasm {
    type State = Tamagotchi;

    fn current_state(state: Self::State) -> TmgCurrentState {
        let fed = state.fed.saturating_sub(HUNGER_PER_BLOCK * ((exec::block_timestamp() - state.fed_block)/1_000));
        let entertained = state.entertained.saturating_sub(BOREDOM_PER_BLOCK * ((exec::block_timestamp() - state.entertained_block)/1_000));
        let rested = state.rested.saturating_sub( ENERGY_PER_BLOCK * ((exec::block_timestamp() - state.rested_block)/1_000));
        let current_state = TmgCurrentState { fed, entertained, rested};
        current_state
    }
}

#[derive(Encode, Decode, TypeInfo)]
struct TmgCurrentState {
    fed: u64,
    entertained: u64,
    rested: u64,
}
