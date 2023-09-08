#![no_std]

use gstd::{prelude::*, ActorId};
use rock_paper_scissors_io::*;

#[gmeta::metawasm]
pub mod metafns {
    pub type State = ContractState;

    pub fn config(state: State) -> GameConfig {
        state.game_config
    }

    pub fn lobby_list(state: State) -> Vec<ActorId> {
        state.lobby
    }

    pub fn game_stage(state: State) -> GameStage {
        state.stage
    }

    pub fn current_stage_start_timestamp(state: State) -> u64 {
        state.current_stage_start_timestamp
    }
}
