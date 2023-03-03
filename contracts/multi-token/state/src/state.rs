use gear_lib::multitoken::io::TokenId;
use gmeta::{metawasm, Metadata};
use gstd::{prelude::*, ActorId};
use multitoken_io::MultitokenMetadata;

#[metawasm]
pub mod metafns {
    pub type State = <MultitokenMetadata as Metadata>::State;

    pub fn tokens_ids_for_owner(state: State, owner: ActorId) -> Vec<TokenId> {
        state.tokens_ids_for_owner(&owner)
    }

    pub fn get_balance(state: State, account: ActorId, id: TokenId) -> u128 {
        state.get_balance(&account, &id)
    }
}
