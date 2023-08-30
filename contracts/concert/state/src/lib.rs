#![no_std]

use concert_io::*;
use gear_lib_old::multitoken::io::TokenMetadata;
use gmeta::{metawasm, Metadata};
use gstd::{prelude::*, ActorId};

#[metawasm]
pub mod metafns {
    pub type State = <ContractMetadata as Metadata>::State;

    pub fn current_concert(state: State) -> CurrentConcert {
        state.current_concert()
    }

    pub fn buyers(state: State) -> Vec<ActorId> {
        state.buyers
    }

    pub fn user_tickets(state: State, user: ActorId) -> Vec<Option<TokenMetadata>> {
        state.user_tickets(user)
    }
}
