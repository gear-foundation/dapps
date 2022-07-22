use crate::*;
use resource_io::Resource;
use types::primitives::{BaseId, PartId, ResourceId, TokenId};
pub const MAX_RESOURCE_LEN: u8 = 128;

#[derive(Debug, Default)]
pub struct MultiResource {
    pub pending_resources: BTreeMap<TokenId, BTreeSet<ResourceId>>,
    pub active_resources: BTreeMap<TokenId, BTreeSet<ResourceId>>,
    pub resource_overwrites: BTreeMap<TokenId, BTreeMap<ResourceId, ResourceId>>,
    pub active_resources_priorities: BTreeMap<TokenId, Vec<u8>>,
}

impl RMRKToken {
    /// Adds resource entry on resource storage contract.
    /// It sends a message to resource storage contract with information about new resource.
    ///
    /// # Requirements:
    /// * The `msg::source()` must be the contract admin.
    ///
    /// Arguments:
    /// * `id`: is a resource identifier
    /// * `src`: a string pointing to the media associated with the resource.
    /// * `thumb`: a string pointing to thumbnail media associated with the resource.
    /// * `metadata_uri`:  a string pointing to a metadata file associated with the resource.
    ///
    /// On success reply `[RMRKEvent::ResourceEntryAdded]`.
    pub async fn add_resource_entry(&mut self, resource_id: ResourceId, resource: Resource) {
        assert!(
            msg::source() == self.admin,
            "Only admin can add resource to storage contract"
        );
        // sends message to resource storage contract
        add_resource_entry(&self.resource_id, resource_id, resource.clone()).await;
        msg::reply(RMRKEvent::ResourceEntryAdded(resource), 0)
            .expect("Error in reply `[RMRKEvent::ResourceEntryAdded]`");
    }

    /// Adds resource to an existing token.
    /// Checks that the resource woth indicated id exists in the resource storage contract.
    /// Proposed resource is placed in the "Pending" array.
    /// A pending resource can be also proposed to overwrite an existing resource.
    ///
    /// # Requirements
    /// Token with indicated `token_id` must exist.
    /// The proposed resource must not already exist for the token.
    /// The resource that is proposed to be overwritten must exist for the token.
    /// The length of resources in pending status must be less or equal to `MAX_RESOURCE_LEN`.
    ///
    /// # Arguments:
    /// * `token_id`: an id of the token.
    /// * `resource_id`: a proposed resource.
    /// * `overwrite_id`: a resource to be overwritten.
    ///
    /// On success reply `[RMRKEvent::ResourceAdded]`.
    pub async fn add_resource(&mut self, token_id: TokenId, resource_id: u8, overwrite_id: u8) {
        self.assert_token_does_not_exist(token_id);
        assert_resource_exists(&self.resource_id, resource_id).await;

        if let Some(token_resources) = self.multiresource.active_resources.get(&token_id) {
            assert!(
                !token_resources.contains(&resource_id),
                "Resource already exists on token"
            );
        }
        if let Some(pending_resources) = self.multiresource.pending_resources.get(&token_id) {
            assert!(
                pending_resources.len() < MAX_RESOURCE_LEN as usize,
                "Max pending resources reached"
            );
        }

        if overwrite_id != 0 {
            if let Some(token_resources) = self.multiresource.active_resources.get(&token_id) {
                assert!(
                    token_resources.contains(&overwrite_id),
                    "Proposed overwritten resource must exist on token"
                );
            } else {
                panic!("No resources to overwrite")
            }
            self.add_overwrite_resource(token_id, resource_id, overwrite_id);
        }

        self.add_pending_resource(token_id, resource_id);

        msg::reply(
            RMRKEvent::ResourceAdded {
                token_id,
                resource_id,
                overwrite_id,
            },
            0,
        )
        .expect("Error in reply `[RMRKEvent::ResourceAdded]`");
    }

    /// Accepts resource from pending list.
    /// Moves the resource from the pending array to the accepted array.
    ///
    /// # Requirements
    /// Only root owner or approved account can accept a resource.
    /// `resource_id` must exist for the token in the pending array.
    ///
    /// # Arguments:
    /// * `token_id`: an id of the token.
    /// * `resource_id`: a resource to be accepted.
    ///
    /// On success reply  `[RMRKEvent::ResourceAccepted]`.
    pub async fn accept_resource(&mut self, token_id: TokenId, resource_id: u8) {
        let root_owner = self.find_root_owner(token_id).await;
        self.assert_approved_or_owner(token_id, &root_owner);

        if let Some(pending_resources) = self.multiresource.pending_resources.get_mut(&token_id) {
            assert!(
                pending_resources.remove(&resource_id),
                "RMRK: Resource does not exist in token pending resources"
            );
        } else {
            panic!("RMRK: Token has no pending resources")
        }

        if let Some(resources) = self.multiresource.resource_overwrites.get_mut(&token_id) {
            if let Some(overwrite_resource) = resources.remove(&resource_id) {
                self.multiresource
                    .active_resources
                    .entry(token_id)
                    .and_modify(|r| {
                        r.remove(&overwrite_resource);
                    });
            }
        }
        self.add_active_resource(token_id, resource_id);
        self.multiresource
            .active_resources_priorities
            .remove(&token_id);
        msg::reply(
            RMRKEvent::ResourceAccepted {
                token_id,
                resource_id,
            },
            0,
        )
        .expect("Error in reply `[RMRKEvent::ResourceAccepted]`");
    }

    /// Rejects a resource, dropping it from the pending array.
    ///
    /// # Requirements
    /// Only root owner or approved account can reject a resource.
    /// `resource_id` must exist for the token in the pending array.
    ///
    /// # Arguments:
    /// * `token_id`: an id of the token.
    /// * `resource_id`: a resource to be rejected.
    ///
    /// On success reply  `[RMRKEvent::ResourceRejected]`.
    pub async fn reject_resource(&mut self, token_id: TokenId, resource_id: u8) {
        let root_owner = self.find_root_owner(token_id).await;
        self.assert_approved_or_owner(token_id, &root_owner);
        if let Some(pending_resources) = self.multiresource.pending_resources.get_mut(&token_id) {
            assert!(
                pending_resources.remove(&resource_id),
                "RMRK: Resource does not exist"
            );
        } else {
            panic!("RMRK: Token has no pending resources")
        }

        msg::reply(
            RMRKEvent::ResourceRejected {
                token_id,
                resource_id,
            },
            0,
        )
        .expect("Error in reply `[RMRKEvent::ResourceRejected]`");
    }

    /// Sets the priority of the active resources array
    /// Priorities have a 1:1 relationship with their corresponding index in
    /// the active resources array. E.G, a priority array of [1, 3, 2] indicates
    ///  that the the active resource at index 1 of the active resource array
    ///  has a priority of 1, index 2 has a priority of 3, and index 3 has a priority
    ///  of 2. There is no validation on priority value input; out of order indexes
    ///  must be handled by the frontend.
    ///
    /// # Requirements
    /// Only root owner or approved account can set priority
    /// The length of the priorities array must be equal to the present length of the active resources array
    ///
    /// # Arguments:
    /// * `token_id`: an id of the token.
    /// * `priorities`: An array of priorities to set.
    ///
    /// On success reply `[RMRKEvent::PrioritySet]`.
    pub async fn set_priority(&mut self, token_id: TokenId, priorities: Vec<u8>) {
        let root_owner = self.find_root_owner(token_id).await;
        self.assert_approved_or_owner(token_id, &root_owner);

        if let Some(active_resources) = self.multiresource.active_resources.get(&token_id) {
            assert!(
                active_resources.len() == priorities.len(),
                "Wrong priority list length"
            );
        }
        self.multiresource
            .active_resources_priorities
            .insert(token_id, priorities.clone());
        msg::reply(
            RMRKEvent::PrioritySet {
                token_id,
                priorities,
            },
            0,
        )
        .expect("Error in reply `[RMRKEvent::PrioritySet]`");
    }

    pub async fn check_slot_resource(
        &self,
        token_id: TokenId,
        resource_id: ResourceId,
        base_id: BaseId,
        slot_id: PartId,
    ) {
        assert!(
            self.multiresource
                .active_resources
                .get(&token_id)
                .expect("Token has no resources")
                .contains(&resource_id),
            "Token has no resource with that ID"
        );
        let resource = get_resource(&self.resource_id, resource_id).await;
        if let Resource::Slot(slot_resource) = resource {
            assert!(slot_resource.base == base_id, "Base contracts do not match");
            assert!(slot_resource.slot == slot_id, "Slots ids do not match");
        } else {
            panic!("Resource must be Slot");
        }
        msg::reply(RMRKEvent::SlotResourceIsOk, 0)
            .expect("Error in reply `[RMRKEvent::SlotResourceIsOk]`");
    }

    fn add_pending_resource(&mut self, token_id: TokenId, resource_id: ResourceId) {
        self.multiresource
            .pending_resources
            .entry(token_id)
            .and_modify(|r| {
                r.insert(resource_id);
            })
            .or_insert_with(|| BTreeSet::from([resource_id]));
    }

    fn add_active_resource(&mut self, token_id: TokenId, resource_id: ResourceId) {
        self.multiresource
            .active_resources
            .entry(token_id)
            .and_modify(|r| {
                r.insert(resource_id);
            })
            .or_insert_with(|| BTreeSet::from([resource_id]));
    }

    fn add_overwrite_resource(
        &mut self,
        token_id: TokenId,
        resource_id: ResourceId,
        overwrite_id: ResourceId,
    ) {
        self.multiresource
            .resource_overwrites
            .entry(token_id)
            .and_modify(|r| {
                r.insert(resource_id, overwrite_id);
            })
            .or_insert_with(|| BTreeMap::from([(resource_id, overwrite_id)]));
    }
}
