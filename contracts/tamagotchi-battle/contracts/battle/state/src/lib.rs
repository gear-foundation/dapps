#![no_std]
use battle_io::*;
use gmeta::metawasm;
use gstd::{prelude::*, ActorId};

#[metawasm]
pub mod metafns {
    pub type State = Battle;

    pub fn player(state: State, tmg_id: ActorId) -> Player {
        state
            .players
            .get(&tmg_id)
            .unwrap_or(&Default::default())
            .clone()
    }

    pub fn power_and_health(state: State, tmg_id: ActorId) -> (u16, u16) {
        let player = state
            .players
            .get(&tmg_id)
            .unwrap_or(&Default::default())
            .clone();
        (player.power, player.health)
    }

    pub fn round(state: State) -> Round {
        state.round
    }

    pub fn battle_state(state: State) -> BattleState {
        state.state
    }
}
