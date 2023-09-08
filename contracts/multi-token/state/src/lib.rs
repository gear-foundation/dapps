#![no_std]

use gear_lib_old::multitoken::io::TokenId;
use gstd::{prelude::*, ActorId};

#[gmeta::metawasm]
pub mod metafns {
    pub type State = multi_token_io::State;

    pub fn tokens_ids_for_owner(state: State, owner: ActorId) -> Vec<TokenId> {
        state.tokens_ids_for_owner(&owner)
    }

    pub fn get_balance(state: State, account: ActorId, id: TokenId) -> u128 {
        state.get_balance(&account, &id)
    }
}
