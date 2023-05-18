use dex_pair_io::{
    hidden::{calculate_in_amount, calculate_out_amount, quote},
    *,
};
use gmeta::{metawasm, Metadata};
use gstd::{prelude::*, ActorId};
use primitive_types::U256;

#[metawasm]
pub mod metafns {
    pub type State = <ContractMetadata as Metadata>::State;

    pub fn token(state: State) -> (ActorId, ActorId) {
        state.token
    }

    pub fn reserve(state: State) -> (u128, u128) {
        state.reserve
    }

    pub fn price(state: State) -> (U256, U256) {
        state.cumulative_price
    }

    pub fn ft_state(state: State) -> FTState {
        state.ft_state
    }

    pub fn balance_of(state: State, actor: ActorId) -> U256 {
        state.ft_state.balance_of(actor)
    }

    pub fn factory(state: State) -> ActorId {
        state.factory
    }

    pub fn is_action_cached(state: State, actor: ActorId, action: CachedAction) -> bool {
        state.cached_actions.contains(&(actor, action))
    }

    pub fn quote(state: State, swap_kind: SwapKind, amount: u128) -> Result<u128, Error> {
        match swap_kind {
            SwapKind::AForB => super::quote(amount, state.reserve),
            SwapKind::BForA => super::quote(amount, (state.reserve.1, state.reserve.0)),
        }
    }

    pub fn calculate_out_amount(
        state: State,
        swap_kind: SwapKind,
        in_amount: u128,
    ) -> Result<u128, Error> {
        match swap_kind {
            SwapKind::AForB => super::calculate_out_amount(in_amount, state.reserve),
            SwapKind::BForA => {
                super::calculate_out_amount(in_amount, (state.reserve.1, state.reserve.0))
            }
        }
    }

    pub fn calculate_in_amount(
        state: State,
        swap_kind: SwapKind,
        out_amount: u128,
    ) -> Result<u128, Error> {
        match swap_kind {
            SwapKind::AForB => super::calculate_in_amount(out_amount, state.reserve),
            SwapKind::BForA => {
                super::calculate_in_amount(out_amount, (state.reserve.1, state.reserve.0))
            }
        }
    }
}
