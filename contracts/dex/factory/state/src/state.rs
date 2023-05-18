use dex_factory_io::*;
use gmeta::{metawasm, Metadata};
use gstd::{prelude::*, ActorId};

#[metawasm]
pub mod metafns {
    pub type State = <ContractMetadata as Metadata>::State;

    pub fn fee_to(state: State) -> ActorId {
        state.fee_to
    }

    pub fn fee_to_setter(state: State) -> ActorId {
        state.fee_to_setter
    }

    pub fn pair(state: State, pair: (ActorId, ActorId)) -> ActorId {
        state.pair(pair)
    }

    pub fn all_pairs_length(state: State) -> u32 {
        state.pairs.len().try_into().unwrap()
    }

    pub fn all_pairs(state: State) -> Vec<((ActorId, ActorId), ActorId)> {
        state.pairs
    }
}
