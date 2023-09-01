#![no_std]

use gstd::{prelude::*, ActorId};
use supply_chain_io::*;

include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

#[gmeta::metawasm]
pub mod metafns {
    pub type State = supply_chain_io::State;

    pub fn item_info(state: State, item_id: ItemId) -> Option<ItemInfo> {
        state
            .items
            .into_iter()
            .find_map(|(some_item_id, item_info)| (some_item_id == item_id).then_some(item_info))
    }

    pub fn participants(state: State) -> Participants {
        Participants {
            producers: state.producers,
            distributors: state.distributors,
            retailers: state.retailers,
        }
    }

    pub fn roles(state: State, actor: ActorId) -> Vec<Role> {
        let mut roles = vec![Role::Consumer];

        if state.producers.contains(&actor) {
            roles.push(Role::Producer);
        }
        if state.distributors.contains(&actor) {
            roles.push(Role::Distributor);
        }
        if state.retailers.contains(&actor) {
            roles.push(Role::Retailer);
        }

        roles
    }

    pub fn existing_items(state: State) -> Vec<(ItemId, ItemInfo)> {
        state.items
    }

    pub fn fungible_token(state: State) -> ActorId {
        state.fungible_token
    }

    pub fn non_fungible_token(state: State) -> ActorId {
        state.non_fungible_token
    }

    pub fn is_action_cached(state: State, actor: ActorId, action: InnerAction) -> bool {
        if let Some(action) = action.into() {
            state.cached_actions.contains(&(actor, action))
        } else {
            false
        }
    }
}
