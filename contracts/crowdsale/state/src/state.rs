use crowdsale_io::CrowdsaleMetadata;
use gmeta::{metawasm, Metadata};
use gstd::{prelude::*, ActorId};

#[metawasm]
pub mod metafns {
    pub type State = <CrowdsaleMetadata as Metadata>::State;

    pub fn current_price(state: State) -> u128 {
        state.get_current_price()
    }

    pub fn tokens_left(state: State) -> u128 {
        state.get_balance()
    }

    pub fn balance_of(state: State, address: ActorId) -> u128 {
        state.balance_of(&address)
    }
}
