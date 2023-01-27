#![no_std]

use dex_factory_io::*;
use gmeta::{metawasm, Metadata};
use gstd::{prelude::*, ActorId};

#[metawasm]
pub trait Metawasm {
    type State = <ContractMetadata as Metadata>::State;

    fn fee_to(state: Self::State) -> ActorId {
        state.fee_to
    }

    fn fee_to_setter(state: Self::State) -> ActorId {
        state.fee_to_setter
    }

    fn pair_address(pair: Pair, state: Self::State) -> ActorId {
        state.pair_address(pair.0, pair.1)
    }

    fn all_pairs_length(state: Self::State) -> u32 {
        state.all_pairs_length()
    }

    fn owner(state: Self::State) -> ActorId {
        state.owner_id
    }
}

type Pair = (FungibleId, FungibleId);
