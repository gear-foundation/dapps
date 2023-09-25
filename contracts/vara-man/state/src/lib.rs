#![no_std]

use gstd::{prelude::*, ActorId};
use vara_man_io::*;

#[gmeta::metawasm]
pub mod metafns {
    pub type State = VaraMan;

    pub fn get_games(state: State) -> Vec<(ActorId, GameInstance)> {
        state.games
    }

    pub fn get_players(state: State) -> Vec<(ActorId, Player)> {
        state.players
    }

    pub fn get_player(state: State, address: ActorId) -> Option<Player> {
        state
            .players
            .iter()
            .find_map(|(a, p)| if a == &address { Some(p.clone()) } else { None })
    }
}
