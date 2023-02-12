#![no_std]
use battle_io::*;
use gmeta::metawasm;
use gstd::{prelude::*, ActorId};

#[metawasm]
pub trait Metawasm {
    type State = Battle;

    fn player(tmg_id: ActorId, state: Self::State) -> Player {
        state
            .players
            .get(&tmg_id)
            .unwrap_or(&Default::default())
            .clone()
    }

    fn power_and_health(tmg_id: ActorId, state: Self::State) -> (u16, u16) {
        let player = state
            .players
            .get(&tmg_id)
            .unwrap_or(&Default::default())
            .clone();
        (player.power, player.health)
    }

    fn round(state: Self::State) -> Round {
        state.round
    }

    fn battle_state(state: Self::State) -> BattleState {
        state.state
    }
}
