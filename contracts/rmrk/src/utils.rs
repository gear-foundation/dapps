use crate::*;

pub fn get_rmrk_owner(
    rmrk_owners: &HashMap<TokenId, RMRKOwner>,
    token_id: TokenId,
) -> Result<RMRKOwner, RMRKError> {
    if let Some(rmrk_owner) = rmrk_owners.get(&token_id) {
        Ok(rmrk_owner.clone())
    } else {
        Err(RMRKError::TokenDoesNotExist)
    }
}

impl From<&RMRKToken> for RMRKState {
    fn from(rmrk: &RMRKToken) -> RMRKState {
        RMRKState {
            name: rmrk.name.clone(),
            symbol: rmrk.symbol.clone(),
            admin: rmrk.admin,
            token_approvals: rmrk
                .token_approvals
                .iter()
                .map(|(key, value)| (*key, value.iter().copied().collect()))
                .collect(),
            rmrk_owners: rmrk
                .rmrk_owners
                .iter()
                .map(|(key, value)| (*key, value.clone()))
                .collect(),
            pending_children: rmrk
                .pending_children
                .iter()
                .map(|(key, value)| (*key, value.iter().copied().collect()))
                .collect(),
            accepted_children: rmrk
                .accepted_children
                .iter()
                .map(|(key, value)| (*key, value.iter().copied().collect()))
                .collect(),
            children_status: rmrk
                .children_status
                .iter()
                .map(|(key, value)| (*key, *value))
                .collect(),
            balances: rmrk
                .balances
                .clone()
                .iter()
                .map(|(key, value)| (*key, *value))
                .collect(),
            assets: AssetsState::default(),
        }
    }
}

impl From<&Assets> for AssetsState {
    fn from(assets: &Assets) -> AssetsState {
        AssetsState {
            assets: assets
                .assets
                .iter()
                .map(|(key, value)| (*key, value.clone()))
                .collect(),
            catalog_addresses: assets
                .catalog_addresses
                .iter()
                .map(|(key, value)| (*key, *value))
                .collect(),
            equippable_group_ids: assets
                .equippable_group_ids
                .iter()
                .map(|(key, value)| (*key, *value))
                .collect(),
            part_ids: assets
                .part_ids
                .iter()
                .map(|(key, value)| (*key, value.clone()))
                .collect(),
            pending_assets: assets
                .pending_assets
                .iter()
                .map(|(key, value)| (*key, value.clone()))
                .collect(),
            active_assets: assets
                .active_assets
                .iter()
                .map(|(key, value)| (*key, value.clone()))
                .collect(),
            active_assets_priorities: assets
                .active_assets_priorities
                .iter()
                .map(|(key, value)| (*key, value.clone()))
                .collect(),
            asset_replacement: assets
                .asset_replacement
                .iter()
                .map(|(key, value)| {
                    (
                        *key,
                        value.iter().map(|(key, value)| (*key, *value)).collect(),
                    )
                })
                .collect(),
            valid_parent_slots: assets
                .valid_parent_slots
                .iter()
                .map(|(key, value)| {
                    (
                        *key,
                        value.iter().map(|(key, value)| (*key, *value)).collect(),
                    )
                })
                .collect(),
            equipments: assets
                .equipments
                .iter()
                .map(|(key, value)| {
                    (
                        *key,
                        value
                            .iter()
                            .map(|(key, value)| (*key, value.clone()))
                            .collect(),
                    )
                })
                .collect(),
        }
    }
}
