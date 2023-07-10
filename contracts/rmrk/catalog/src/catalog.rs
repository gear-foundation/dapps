use crate::*;

#[derive(Debug, Default)]
pub struct Catalog {
    /// Specifies how an NFT should be rendered, ie "svg".
    pub base_type: String,

    /// Provided by user during Base creation.
    pub symbol: String,

    /// Mapping from `PartId` to fixed or slot `Part`.
    pub parts: HashMap<PartId, Part>,

    /// mapping of uint64 `partId` to boolean flag,
    /// indicating that a given `Part` can be equippable by any address
    pub is_equippable_to_all: Vec<PartId>,
}

impl Catalog {
    pub fn add_parts(
        &mut self,
        parts: BTreeMap<PartId, Part>,
    ) -> Result<CatalogReply, CatalogError> {
        if parts.is_empty() {
            return Err(CatalogError::ZeroLengthPassed);
        }
        for (part_id, part) in parts.clone() {
            if part_id == 0 {
                return Err(CatalogError::PartIdCantBeZero);
            }
            if self.parts.insert(part_id, part).is_some() {
                return Err(CatalogError::PartAlreadyExists);
            }
        }
        Ok(CatalogReply::PartsAdded(parts))
    }

    pub fn remove_parts(&mut self, parts: Vec<PartId>) -> Result<CatalogReply, CatalogError> {
        if parts.is_empty() {
            return Err(CatalogError::ZeroLengthPassed);
        }
        for part_id in parts.clone() {
            if self.parts.remove(&part_id).is_none() {
                return Err(CatalogError::PartDoesNotExist);
            }
        }
        Ok(CatalogReply::PartsRemoved(parts))
    }

    pub fn add_equippable_addresses(
        &mut self,
        part_id: PartId,
        collection_ids: Vec<CollectionId>,
    ) -> Result<CatalogReply, CatalogError> {
        if collection_ids.is_empty() {
            return Err(CatalogError::ZeroLengthPassed);
        }
        let part = get_mut_part(&mut self.parts, part_id)?;
        let equippable = if let Part::Slot(SlotPart { equippable, .. }) = part {
            equippable
        } else {
            return Err(CatalogError::WrongPartFormat);
        };

        for collection_id in collection_ids.iter() {
            equippable.push(*collection_id);
        }

        Ok(CatalogReply::EquippablesAdded {
            part_id,
            collection_ids,
        })
    }

    pub fn reset_equippable_addresses(
        &mut self,
        part_id: PartId,
    ) -> Result<CatalogReply, CatalogError> {
        self.is_equippable_to_all.retain(|&x| x != part_id);
        let part = get_mut_part(&mut self.parts, part_id)?;
        if let Part::Slot(SlotPart { equippable, .. }) = part {
            *equippable = vec![];
        } else {
            return Err(CatalogError::WrongPartFormat);
        }

        Ok(CatalogReply::EqippableAddressesReset)
    }

    pub fn set_equippable_to_all(&mut self, part_id: PartId) -> Result<CatalogReply, CatalogError> {
        let part = get_part(&self.parts, part_id)?;
        if let Part::Fixed { .. } = part {
            return Err(CatalogError::WrongPartFormat);
        }

        self.is_equippable_to_all.push(part_id);
        Ok(CatalogReply::EquippableToAllSet)
    }

    pub fn remove_equippable(
        &mut self,
        part_id: PartId,
        collection_id: CollectionId,
    ) -> Result<CatalogReply, CatalogError> {
        let part = get_mut_part(&mut self.parts, part_id)?;
        if let Part::Slot(SlotPart { equippable, .. }) = part {
            equippable.retain(|&x| x != collection_id);
        } else {
            return Err(CatalogError::WrongPartFormat);
        }

        Ok(CatalogReply::EquippableRemoved {
            part_id,
            collection_id,
        })
    }

    pub fn check_part(&self, part_id: PartId) -> Result<CatalogReply, CatalogError> {
        let part = get_part(&self.parts, part_id)?;
        Ok(CatalogReply::Part(part.clone()))
    }

    pub fn check_equippable(
        &self,
        part_id: PartId,
        collection_id: CollectionId,
    ) -> Result<CatalogReply, CatalogError> {
        for equippable in self.is_equippable_to_all.iter() {
            if *equippable == part_id {
                return Ok(CatalogReply::InEquippableList);
            }
        }
        let part = get_part(&self.parts, part_id)?;
        if let Part::Slot(SlotPart { equippable, .. }) = part {
            if equippable.iter().any(|&x| x == collection_id) {
                Ok(CatalogReply::InEquippableList)
            } else {
                Ok(CatalogReply::NotInEquippableList)
            }
        } else {
            Err(CatalogError::WrongPartFormat)
        }
    }
}

fn get_part(parts: &HashMap<PartId, Part>, part_id: PartId) -> Result<&Part, CatalogError> {
    if let Some(part) = parts.get(&part_id) {
        return Ok(part);
    }
    Err(CatalogError::PartDoesNotExist)
}

fn get_mut_part(
    parts: &mut HashMap<PartId, Part>,
    part_id: PartId,
) -> Result<&mut Part, CatalogError> {
    if let Some(part) = parts.get_mut(&part_id) {
        return Ok(part);
    }
    Err(CatalogError::PartDoesNotExist)
}
