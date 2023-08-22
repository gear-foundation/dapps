#![no_std]

use gmeta::metawasm;
use gstd::{prelude::*, ActorId};
use primitive_types::U256;
use rmrk_io::*;
use types::primitives::{CollectionAndToken, PartId, TokenId};

#[cfg(feature = "binary-vendor")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

#[metawasm]
pub mod metafns {
    pub type State = RMRKState;

    pub fn rmrk_owner(state: State, token_id: TokenId) -> RMRKOwner {
        if let Some((_, rmrk_owner)) = state.rmrk_owners.iter().find(|(id, _)| id == &token_id) {
            rmrk_owner.clone()
        } else {
            RMRKOwner::default()
        }
    }

    pub fn balance(state: State, account: ActorId) -> U256 {
        if let Some((_, balance)) = state.balances.iter().find(|(id, _)| id == &account) {
            *balance
        } else {
            U256::zero()
        }
    }

    pub fn pending_children(state: State, token_id: TokenId) -> Vec<CollectionAndToken> {
        if let Some((_, pending_children)) = state
            .pending_children
            .iter()
            .find(|(id, _)| id == &token_id)
        {
            pending_children.clone()
        } else {
            vec![]
        }
    }

    pub fn accepted_children(state: State, token_id: TokenId) -> Vec<CollectionAndToken> {
        if let Some((_, accepted_children)) = state
            .accepted_children
            .iter()
            .find(|(id, _)| id == &token_id)
        {
            accepted_children.clone()
        } else {
            vec![]
        }
    }
    pub fn get_assets_and_equippable_data(
        state: State,
        token_id: TokenId,
        asset_id: u64,
    ) -> (String, u64, ActorId, Vec<PartId>) {
        if let Some((_, active_assets)) = state
            .assets
            .active_assets
            .iter()
            .find(|(id, _)| id == &token_id)
        {
            if !active_assets.iter().any(|id| id == &asset_id) {
                return Default::default();
            }
        } else {
            return Default::default();
        }

        let metadata = if let Some((_, metadata)) =
            state.assets.assets.iter().find(|(id, _)| id == &asset_id)
        {
            metadata.clone()
        } else {
            String::new()
        };
        let equippable_group_id = if let Some((_, equippable_group_id)) = state
            .assets
            .equippable_group_ids
            .iter()
            .find(|(id, _)| id == &asset_id)
        {
            *equippable_group_id
        } else {
            0
        };
        let catalog_address = if let Some((_, catalog_address)) = state
            .assets
            .catalog_addresses
            .iter()
            .find(|(id, _)| id == &asset_id)
        {
            *catalog_address
        } else {
            ActorId::zero()
        };

        let part_ids = if let Some((_, part_ids)) =
            state.assets.part_ids.iter().find(|(id, _)| id == &asset_id)
        {
            part_ids.clone()
        } else {
            vec![]
        };
        (metadata, equippable_group_id, catalog_address, part_ids)
    }
}
