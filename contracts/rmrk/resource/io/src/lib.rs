#![no_std]

use gstd::{prelude::*, ActorId};
use types::primitives::{BaseId, PartId, ResourceId};

#[derive(Debug, Default, Clone, Encode, Decode, TypeInfo)]
pub struct BasicResource {
    /// URI like IPFS hash
    pub src: String,

    /// If the resource has the thumb property, this will be a URI to a thumbnail of the given
    /// resource.
    pub thumb: Option<String>,

    /// Reference to IPFS location of metadata
    pub metadata_uri: String,
}

#[derive(Debug, Default, Clone, Encode, Decode, TypeInfo)]
pub struct ComposedResource {
    /// URI like ipfs hash
    pub src: String,

    /// If the resource has the thumb property, this will be a URI to a thumbnail of the given
    /// resource.
    pub thumb: String,

    /// Reference to IPFS location of metadata
    pub metadata_uri: String,

    // The address of base contract
    pub base: BaseId,

    //  If a resource is composed, it will have an array of parts that compose it
    pub parts: Vec<PartId>,
}

#[derive(Debug, Default, Clone, Encode, Decode, TypeInfo)]
pub struct SlotResource {
    /// URI like ipfs hash
    pub src: String,

    /// If the resource has the thumb property, this will be a URI to a thumbnail of the given
    /// resource.
    pub thumb: String,

    /// Reference to IPFS location of metadata
    pub metadata_uri: String,

    // The address of base contract
    pub base: BaseId,

    /// If the resource has the slot property, it was designed to fit into a specific Base's slot.
    pub slot: PartId,
}
#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum Resource {
    Basic(BasicResource),
    Slot(SlotResource),
    Composed(ComposedResource),
}

#[derive(Debug, Decode, Encode, TypeInfo)]
pub struct InitResource {
    pub resource_name: String,
}

#[derive(Debug, Decode, Encode, TypeInfo)]
pub enum ResourceAction {
    /// Adds resource entry on resource storage contract.
    ///
    /// # Requirements:
    /// * The `msg::source()` must be the contract admin (RMRK contract).
    /// * `id` can not be equal to zero.
    /// * Resource with indicated `id` must not exist.
    ///
    /// # Arguments:
    /// * `resource_id`: is a resource identifier.
    /// * `resource`: is a resource struct that can be `Basic`, `Slot` or `Composed`.
    ///
    /// On success replies [`ResourceEvent::ResourceEntryAdded`].
    AddResourceEntry {
        resource_id: ResourceId,
        resource: Resource,
    },

    /// Adds part ids to composed resource.
    ///
    /// # Requirements:
    /// * The `msg::source()` must be the contract admin (RMRK contract).
    /// * `part_id` must exist in the base contract.
    /// * Resource with indicated `id` must not exist.
    ///
    /// # Arguments:
    /// * `part_id`: the part id to be added to composed resource.
    /// * `resource_id`: the composed resource id.
    ///
    /// On success replies [`ResourceEvent::PartIdAddedToResource`].
    AddPartToResource {
        resource_id: ResourceId,
        part_id: PartId,
    },

    /// Used to check from the RMRK contract whether the resource with indicated id exists or not.
    ///
    /// # Arguments:
    /// * `id`: is a resource identifier.
    ///
    /// On success replies [`ResourceEvent::Resource`].
    GetResource { id: ResourceId },
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum ResourceEvent {
    ResourceEntryAdded {
        resource_id: ResourceId,
        resource: Resource,
    },
    PartIdAddedToResource(PartId),
    Resource(Resource),
}

#[derive(Debug, Decode, Encode, TypeInfo)]
pub enum ResourceState {
    ResourceStorageInfo,
    ResourceInfo(ResourceId),
}

#[derive(Debug, Decode, Encode, TypeInfo)]
pub enum ResourceStateReply {
    ResourceStorageInfo {
        name: String,
        admin: ActorId,
        resources: BTreeMap<ResourceId, Resource>,
    },
    ResourceInfo(Option<Resource>),
}
