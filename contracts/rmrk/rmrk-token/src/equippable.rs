use crate::*;
use gstd::msg;
use rmrk_io::Equipment;

#[derive(Default)]
pub struct Assets {
    /// Mapping of uint64 Ids to asset metadata
    pub assets: HashMap<u64, String>,
    /// Mapping of uint64 asset ID to corresponding catalog address.
    pub catalog_addresses: HashMap<u64, ActorId>,
    /// Mapping of asset_id to equippable_group_ids.
    pub equippable_group_ids: HashMap<u64, u64>,
    /// Mapping of asset_id to catalog parts applicable to this asset, both fixed and slot
    pub part_ids: HashMap<u64, Vec<PartId>>,
    /// Mapping of tokenId to an array of pending assets
    pub pending_assets: HashMap<TokenId, Vec<u64>>,
    /// Mapping of tokenId to an array of active assets
    pub active_assets: HashMap<TokenId, Vec<u64>>,
    /// Mapping of tokenId to an array of priorities for active assets
    pub active_assets_priorities: HashMap<TokenId, Vec<u64>>,
    /// Mapping of tokenId to new asset, to asset to be replaced
    pub asset_replacement: HashMap<TokenId, HashMap<u64, u64>>,
    /// Mapping of `equippable_group_id` to parent contract address and valid `slot_id`.
    pub valid_parent_slots: HashMap<u64, HashMap<ActorId, PartId>>,
    /// Mapping of token ID and catalog address to slot part ID to equipment information.
    /// Used to compose an NFT.
    pub equipments: HashMap<(TokenId, ActorId), HashMap<PartId, Equipment>>,
}

impl Assets {
    pub fn add_equippable_asset_entry(
        &mut self,
        equippable_group_id: u64,
        catalog_address: Option<ActorId>,
        metadata_uri: String,
        part_ids: Vec<PartId>,
    ) -> Result<RMRKReply, RMRKError> {
        let id = (self.assets.len() as u64) + 1;
        if catalog_address.is_none() && !part_ids.is_empty() {
            return Err(RMRKError::CatalogRequiredForParts);
        }
        if let Some(address) = catalog_address {
            self.catalog_addresses.insert(id, address);
            self.equippable_group_ids.insert(id, equippable_group_id);
            self.part_ids.insert(id, part_ids);
        }
        self._add_asset_entry(id, metadata_uri)?;
        Ok(RMRKReply::EquippableAssetEntryAdded)
    }

    fn _add_asset_entry(&mut self, id: u64, metadata_uri: String) -> Result<RMRKReply, RMRKError> {
        if id == 0 {
            return Err(RMRKError::ZeroIdForbidden);
        }
        if self.assets.insert(id, metadata_uri).is_some() {
            return Err(RMRKError::AssetAlreadyExists);
        }
        Ok(RMRKReply::AssetSet)
    }

    pub fn add_asset_to_token(
        &mut self,
        token_id: TokenId,
        asset_id: u64,
        replaces_asset_with_id: u64,
    ) -> Result<RMRKReply, RMRKError> {
        if !self.assets.contains_key(&asset_id) {
            return Err(RMRKError::NoAssetMatchingId);
        }
        if let Some(active_assets) = self.active_assets.get(&token_id) {
            if active_assets.iter().any(|&id| id == asset_id) {
                return Err(RMRKError::AssetAlreadyExists);
            }
        }
        if let Some(pending_assets) = self.pending_assets.get_mut(&token_id) {
            if pending_assets.len() >= 128 {
                return Err(RMRKError::MaxPendingAssetsReached);
            }
            if pending_assets.iter().any(|&id| id == asset_id) {
                return Err(RMRKError::AssetAlreadyExists);
            }
            pending_assets.push(asset_id);
        }
        if replaces_asset_with_id != 0 {
            self.asset_replacement
                .entry(token_id)
                .and_modify(|ids| {
                    ids.insert(asset_id, replaces_asset_with_id);
                })
                .or_insert_with(|| HashMap::from([(asset_id, replaces_asset_with_id)]));
        }
        Ok(RMRKReply::AssetAddedToToken)
    }

    pub fn accept_asset(
        &mut self,
        token_id: TokenId,
        asset_id: u64,
    ) -> Result<RMRKReply, RMRKError> {
        if let Some(pending_assets) = self.pending_assets.get_mut(&token_id) {
            if let Some(index) = pending_assets.iter().position(|&id| id == asset_id) {
                pending_assets.remove(index);
            } else {
                return Err(RMRKError::AssetDoesNotExistInPendingArray);
            };
        }

        let replace_id = if let Some(replacements) = self.asset_replacement.get(&token_id) {
            if let Some(replace_id) = replacements.get(&asset_id) {
                *replace_id
            } else {
                0
            }
        } else {
            0
        };

        let replace_position = if replace_id != 0 {
            if let Some(active_assets) = self.active_assets.get(&token_id) {
                active_assets.iter().position(|&id| id == replace_id)
            } else {
                None
            }
        } else {
            None
        };

        if let Some(replace_index) = replace_position {
            self.active_assets.entry(token_id).and_modify(|assets| {
                assets[replace_index] = asset_id;
            });
        } else {
            let active_assets = self
                .active_assets
                .entry(token_id)
                .and_modify(|assets| assets.push(asset_id))
                .or_insert_with(|| vec![asset_id]);
            let len = (active_assets.len() as u64) - 1;
            self.active_assets_priorities
                .entry(token_id)
                .and_modify(|priorities| priorities.push(len))
                .or_insert_with(|| vec![0]);
        }

        Ok(RMRKReply::AssetAccepted)
    }

    pub fn set_valid_parent_for_equippable_group(
        &mut self,
        equippable_group_id: u64,
        slot_part_id: PartId,
        parent_id: ActorId,
    ) -> Result<RMRKReply, RMRKError> {
        if equippable_group_id == 0 || slot_part_id == 0 {
            return Err(RMRKError::ZeroIdForbidden);
        }
        self.valid_parent_slots
            .entry(equippable_group_id)
            .and_modify(|part_ids| {
                part_ids.insert(parent_id, slot_part_id);
            })
            .or_insert_with(|| HashMap::from([(parent_id, slot_part_id)]));
        Ok(RMRKReply::ValidParentEquippableGroupIdSet)
    }

    pub fn can_token_be_equipped_with_asset_into_slot(
        &self,
        parent_id: ActorId,
        token_id: TokenId,
        asset_id: u64,
        slot_part_id: PartId,
    ) -> Result<RMRKReply, RMRKError> {
        self.in_active_assets(token_id, asset_id)?;

        let equippable_slot_id = self.get_equippable_slot_id(asset_id, parent_id)?;
        if equippable_slot_id != slot_part_id {
            return Err(RMRKError::WrongSlotId);
        }
        Ok(RMRKReply::TokenBeEquippedWithAssetIntoSlot)
    }

    fn get_equippable_slot_id(
        &self,
        asset_id: u64,
        parent_id: ActorId,
    ) -> Result<PartId, RMRKError> {
        if let Some(equippable_group_id) = self.equippable_group_ids.get(&asset_id) {
            if let Some(equippable) = self.valid_parent_slots.get(equippable_group_id) {
                if let Some(equippable_slot) = equippable.get(&parent_id) {
                    return Ok(*equippable_slot);
                }
            }
        }
        Err(RMRKError::EquippableNotFound)
    }

    fn in_active_assets(&self, token_id: TokenId, asset_id: u64) -> Result<(), RMRKError> {
        if let Some(active_assets) = self.active_assets.get(&token_id) {
            if active_assets.iter().any(|&id| id == asset_id) {
                return Ok(());
            }
        }
        Err(RMRKError::ActiveAssetNotFound)
    }

    /// * `token_id`: ID of the token that had an asset equipped
    /// * `child_token_id`: ID of the child token we are equipping into the slot
    /// * `child_id`: Address of the child token's collection
    /// * `asset_id`: ID of the asset that we are equipping into
    /// * `slot_part_id`:  ID of the slot part that we are using to equip
    /// * `child_asset_id`: ID of the asset that we are equipping
    #[cfg_attr(feature = "cargo-clippy", allow(clippy::too_many_arguments))]
    pub fn equip(
        &mut self,
        tx_manager: &mut TxManager,
        token_id: TokenId,
        child_token_id: TokenId,
        child_id: &ActorId,
        asset_id: u64,
        slot_part_id: PartId,
        child_asset_id: u64,
    ) -> Result<RMRKReply, RMRKError> {
        let catalog_address = if let Some(id) = self.catalog_addresses.get(&asset_id) {
            id
        } else {
            return Err(RMRKError::CatalogDoesNotExist);
        };
        let state = tx_manager.get_state(msg::id());
        match state {
            TxState::MsgSourceAccountChecked => {
                // check whether the slot is used
                if let Some(parts) = self.equipments.get(&(token_id, *catalog_address)) {
                    if parts.get(&slot_part_id).is_some() {
                        return Err(RMRKError::SlotAlreadyUsed);
                    }
                }

                // check if a given asset accepts a given slot or not.
                if let Some(part_ids) = self.part_ids.get(&asset_id) {
                    if part_ids.iter().any(|&part_id| part_id == slot_part_id) {
                        // Check from catalog perspective:
                        // - the indicated part has the Slot type;
                        // - this NFT contract is in equiappable list
                        let msg_id = check_equippable_msg(catalog_address, slot_part_id, child_id);
                        tx_manager.set_tx_state(TxState::MsgCheckEquippableSent, msg_id);
                        exec::wait_for(5);
                    }
                }
                Err(RMRKError::TargetAssetCannotReceiveSlot)
            }
            TxState::ReplyCheckEquippableReceived => {
                // Check from child perspective intention to be used in part
                let msg_id = can_token_be_equipped_msg(
                    child_id,
                    &exec::program_id(),
                    child_token_id,
                    child_asset_id,
                    slot_part_id,
                );
                tx_manager.set_tx_state(TxState::MsgCanTokenBeEquippedSent, msg_id);
                exec::wait_for(5);
            }
            TxState::ReplyCanTokenBeEquippedReceived => {
                let equipment = Equipment {
                    asset_id,
                    child_asset_id,
                    child_token_id,
                    child_id: *child_id,
                };
                self.equipments
                    .entry((token_id, *catalog_address))
                    .and_modify(|parts| {
                        parts.entry(slot_part_id).insert(equipment.clone());
                    })
                    .or_insert_with(|| HashMap::from([(slot_part_id, equipment)]));
                Ok(RMRKReply::ChildAssetEquipped)
            }
            TxState::Error(error) => Err(error),
            _ => {
                unreachable!()
            }
        }
    }
}
