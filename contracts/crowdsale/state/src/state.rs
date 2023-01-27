use crowdsale_io::CrowdsaleMetadata;
use gmeta::{metawasm, Metadata};
use gstd::{prelude::*, ActorId};

#[metawasm]
pub trait Metawasm {
    type State = <CrowdsaleMetadata as Metadata>::State;

    fn current_price(state: Self::State) -> u128 {
        state.get_current_price()
    }

    fn tokens_left(state: Self::State) -> u128 {
        state.get_balance()
    }

    fn balance_of(address: ActorId, state: Self::State) -> u128 {
        state.balance_of(&address)
    }
}
