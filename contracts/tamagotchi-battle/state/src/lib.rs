#![no_std]

use gstd::{prelude::*, ActorId};
use tamagotchi_battle_io::*;

#[gmeta::metawasm]
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

    pub fn battle_state(state: State) -> BattleState {
        state.state
    }

    pub fn pairs_for_player(state: State, player: ActorId) -> Vec<PairId> {
        state
            .players_to_pairs
            .get(&player)
            .unwrap_or(&Vec::new())
            .clone()
    }

    pub fn pair_ids(state: State) -> Vec<PairId> {
        state.pairs.keys().cloned().collect()
    }
    pub fn current_turn(state: State, pair_id: PairId) -> ActorId {
        if let Some(pair) = state.pairs.get(&pair_id) {
            let current_turn = pair.moves.len();
            return pair.owner_ids[current_turn];
        }
        ActorId::zero()
    }
    pub fn game_is_over(state: State, pair_id: PairId) -> bool {
        if let Some(pair) = state.pairs.get(&pair_id) {
            return pair.game_is_over;
        }
        true
    }

    pub fn tmg_ids(state: State) -> Vec<ActorId> {
        state.players_ids
    }

    pub fn pair(state: State, pair_id: PairId) -> Pair {
        state
            .pairs
            .get(&pair_id)
            .unwrap_or(&Default::default())
            .clone()
    }
}
