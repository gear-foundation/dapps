use crate::services::dynamic_nft::TokenMetadata;
use gstd::{exec, msg};
use sails_rs::{
    collections::{HashMap, HashSet},
    prelude::*,
};
use vnft_service::utils::{Error, Result, *};

pub fn mint(
    owner_by_id: &mut HashMap<TokenId, ActorId>,
    tokens_for_owner: &mut HashMap<ActorId, HashSet<TokenId>>,
    token_metadata_by_id: &mut HashMap<TokenId, TokenMetadata>,
    token_id: &mut TokenId,
    to: ActorId,
    token_metadata: TokenMetadata,
) -> Result<()> {
    owner_by_id.insert(*token_id, to);
    tokens_for_owner
        .entry(to)
        .and_modify(|tokens| {
            tokens.insert(*token_id);
        })
        .or_insert_with(|| HashSet::from([*token_id]));
    token_metadata_by_id.insert(*token_id, token_metadata);
    *token_id += 1.into();
    Ok(())
}

pub fn burn(
    owner_by_id: &mut HashMap<TokenId, ActorId>,
    tokens_for_owner: &mut HashMap<ActorId, HashSet<TokenId>>,
    token_approvals: &mut HashMap<TokenId, ActorId>,
    token_metadata_by_id: &mut HashMap<TokenId, TokenMetadata>,
    token_id: TokenId,
) -> Result<()> {
    let owner = owner_by_id
        .remove(&token_id)
        .ok_or(Error::TokenDoesNotExist)?;
    if let Some(tokens) = tokens_for_owner.get_mut(&owner) {
        tokens.remove(&token_id);
        if tokens.is_empty() {
            tokens_for_owner.remove(&owner);
        }
    }
    token_approvals.remove(&token_id);
    token_metadata_by_id.remove(&token_id);
    Ok(())
}

pub fn start_metadata_update(
    gas_for_one_time_updating: u64,
    owner_by_id: &mut HashMap<TokenId, ActorId>,
    token_metadata_by_id: &mut HashMap<TokenId, TokenMetadata>,
    token_id: TokenId,
    msg_src: ActorId,
    updates_count: u32,
    update_period: u32,
) -> Result<()> {
    let owner = owner_by_id.get(&token_id).ok_or(Error::TokenDoesNotExist)?;

    if *owner != msg_src {
        return Err(Error::DeniedAccess);
    }
    let metadata = token_metadata_by_id
        .get_mut(&token_id)
        .ok_or(Error::TokenDoesNotExist)?;
    metadata.current_media_index =
        metadata.current_media_index.saturating_add(1) % metadata.media.len() as u64;
    if updates_count.saturating_sub(1) != 0 {
        let request = [
            "DynamicNft".encode(),
            "UpdateMetadata".to_string().encode(),
            (token_id, msg_src, update_period, updates_count - 1).encode(),
        ]
        .concat();
        msg::send_bytes_with_gas_delayed(
            exec::program_id(),
            request,
            gas_for_one_time_updating.saturating_mul(updates_count.into()),
            0,
            update_period,
        )
        .expect("Error in sending message");
    }

    Ok(())
}

pub fn update_metadata(
    owner_by_id: &mut HashMap<TokenId, ActorId>,
    token_metadata_by_id: &mut HashMap<TokenId, TokenMetadata>,
    token_id: TokenId,
    owner: ActorId,
    update_period: u32,
    updates_count: u32,
) -> Result<u64> {
    let current_owner = owner_by_id.get(&token_id).ok_or(Error::TokenDoesNotExist)?;

    if owner != *current_owner {
        return Err(Error::DeniedAccess);
    }

    let metadata = token_metadata_by_id
        .get_mut(&token_id)
        .ok_or(Error::TokenDoesNotExist)?;
    metadata.current_media_index =
        metadata.current_media_index.saturating_add(1) % metadata.media.len() as u64;

    if updates_count.saturating_sub(1) != 0 {
        let request = [
            "DynamicNft".encode(),
            "UpdateMetadata".to_string().encode(),
            (token_id, owner, update_period, updates_count - 1).encode(),
        ]
        .concat();

        msg::send_bytes_with_gas_delayed(
            exec::program_id(),
            request,
            exec::gas_available().saturating_sub(1_000_000_000),
            0,
            update_period,
        )
        .expect("Error in sending message");
    }

    Ok(metadata.current_media_index)
}
