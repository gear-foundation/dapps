#![no_std]

use gmeta::{In, InOut, Metadata};
use gstd::{prelude::*, ActorId};
use types::primitives::*;

pub struct CatalogMetadata;

impl Metadata for CatalogMetadata {
    type Init = In<InitCatalog>;
    type Handle = InOut<CatalogAction, Result<CatalogReply, CatalogError>>;
    type Others = ();
    type Reply = ();
    type Signal = ();
    type State = CatalogState;
}

#[derive(Debug, Default, Encode, Decode, TypeInfo)]
pub struct CatalogState {
    pub admin: ActorId,
    pub base_type: String,
    pub symbol: String,
    pub parts: Vec<(PartId, Part)>,
    pub is_equippable_to_all: Vec<PartId>,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
pub enum EquippableList {
    All,
    Custom(BTreeSet<CollectionAndToken>),
}

#[derive(Debug, Clone, Default, Encode, Decode, TypeInfo, Eq, PartialEq)]
pub struct FixedPart {
    /// An optional zIndex of base part layer.
    /// specifies the stack order of an element.
    /// An element with greater stack order is always in front of an element with a lower stack order.
    pub z: Option<ZIndex>,

    /// The metadata URI of the part.
    pub metadata_uri: String,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo, Eq, PartialEq)]
pub struct SlotPart {
    /// Array of whitelisted collections that can be equipped in the given slot. Used with slot parts only.
    pub equippable: Vec<CollectionId>,

    /// An optional zIndex of base part layer.
    /// specifies the stack order of an element.
    /// An element with greater stack order is always in front of an element with a lower stack order.
    pub z: Option<ZIndex>,

    /// The metadata URI of the part.
    pub metadata_uri: String,
}

#[derive(Debug, Clone, Decode, Encode, TypeInfo, Eq, PartialEq)]
pub enum Part {
    Fixed(FixedPart),
    Slot(SlotPart),
}

#[derive(Debug, Decode, Encode, TypeInfo)]
pub struct InitCatalog {
    /// Catalog metadata URI of the Catalog
    pub catalog_type: String,
    /// Type of Catalog
    pub symbol: String,
}

#[derive(Debug, Decode, Encode, TypeInfo)]
pub enum CatalogAction {
    /// Adds parts to base contract.
    ///
    /// # Requirements:
    /// * The `msg::source()` must be the contract issuer.
    /// * `PartId` must be unique.
    ///
    /// # Arguments:
    /// * `BTreeMap<PartId, Part>`: a mapping from `PartId` to fixed or slot `Part`.
    ///
    /// On success replies `[BaseEvent::PartsAdded]`.
    AddParts(BTreeMap<PartId, Part>),

    /// Adds equippable to slot part.
    ///
    /// # Requirements:
    /// * The `msg::source()` must be the contract admin.
    /// * The indicated collection contract must be RMRK contract.
    /// * The token from indicated collections must have composable resource that refers to that base.
    ///
    /// # Arguments:
    /// * `collection_ids`: an addresses of RMRK contract.
    ///
    /// On success replies `[BaseEvent::EquippableAdded]`.
    AddEquippableAddresses {
        part_id: PartId,
        collection_ids: Vec<CollectionId>,
    },

    /// Removes parts from the base.
    ///
    /// # Requirements:
    /// * The `msg::source()` must be the contract issuer.
    /// * The parts with indicated PartIds must exist.
    ///
    /// # Arguments:
    /// * `Vec<PartId>`: Part IDs to be removed.
    ///
    /// On success replies `[BaseEvent::PartsRemoved]`.
    RemoveParts(Vec<PartId>),

    /// Removes equippable from the slot part.
    ///
    /// # Requirements:
    /// * The `msg::source()` must be the contract issuer.
    /// * Indicated equippable must exist.
    ///
    /// # Arguments:
    /// * `collection_id`: an address of RMRK contract.
    ///
    /// On success replies `[BaseEvent::EquippableRemoved]`.
    RemoveEquippable {
        part_id: PartId,
        collection_id: CollectionId,
    },

    /// Checks whether the part exists in the Base.
    ///
    /// # Arguments:
    /// * `PartId`: the Part Id.
    ///
    /// On success replies `[BaseEvent::Part]`.
    CheckPart(PartId),

    /// Checks whether the token from specified collection is in equippable list.
    ///
    /// # Arguments:
    /// * `part_id`: the Part Id.
    /// * `collection_id`: an address of RMRK contract.
    /// * `token_id`: the id of the token in RMRK contract.
    ///
    /// On success replies `[BaseEvent::Part]`.
    CheckEquippable {
        part_id: PartId,
        collection_id: CollectionId,
    },
    SetEquippableToAll {
        part_id: PartId,
    },
    ResetEquippableAddress {
        part_id: PartId,
    },
}

#[derive(Debug, Decode, Encode, TypeInfo, Clone, PartialEq)]
pub enum CatalogReply {
    PartsAdded(BTreeMap<PartId, Part>),
    EquippablesAdded {
        part_id: PartId,
        collection_ids: Vec<CollectionId>,
    },
    EqippableAddressesReset,
    PartsRemoved(Vec<PartId>),
    EquippableRemoved {
        part_id: PartId,
        collection_id: CollectionId,
    },
    Part(Part),
    InEquippableList,
    NotInEquippableList,
    EquippableToAllSet,
}

#[derive(Debug, Decode, Encode, TypeInfo)]
pub enum CatalogError {
    PartIdCantBeZero,
    BadConfig,
    PartAlreadyExists,
    ZeroLengthPassed,
    PartDoesNotExist,
    WrongPartFormat,
    NotAllowedToCall,
}
