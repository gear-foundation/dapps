#![no_std]

use gmeta::{metawasm, Metadata};
use gstd::{prelude::*, ActorId};
use vara_man_io::*;

#[metawasm]
pub mod metafns {
    pub type State = <VaraManMetadata as Metadata>::State;

    pub fn get_games(state: State) -> Vec<GameInstance> {
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
