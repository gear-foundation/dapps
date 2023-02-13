use app_io::*;
use gmeta::{metawasm, Metadata};
use gstd::{prelude::*, ActorId};

#[metawasm]
pub trait Metawasm {
    type State = <ContractMetadata as Metadata>::State;

    fn pingers(state: Self::State) -> Vec<ActorId> {
        state.pingers()
    }

    fn ping_count(actor: ActorId, state: Self::State) -> u128 {
        state.ping_count(actor)
    }
}
