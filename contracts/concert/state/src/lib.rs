#![no_std]

use concert_io::*;
use gear_lib::multitoken::io::TokenMetadata;
use gmeta::{metawasm, Metadata};
use gstd::{prelude::*, ActorId};

#[metawasm]
pub trait Metawasm {
    type State = <ContractMetadata as Metadata>::State;

    fn current_concert(state: Self::State) -> CurrentConcert {
        state.current_concert()
    }

    fn buyers(state: Self::State) -> Vec<ActorId> {
        state.buyers
    }

    fn user_tickets(user: ActorId, state: Self::State) -> Vec<Option<TokenMetadata>> {
        state.user_tickets(user)
    }
}
