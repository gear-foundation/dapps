#![no_std]

use gstd::{prelude::*, ActorId};
use types::primitives::*;

#[derive(Debug, Default, Encode, Decode, TypeInfo)]
struct Base {
    /// Original creator of the Base.
    issuer: ActorId,

    /// Specifies how an NFT should be rendered, ie "svg".
    base_type: String,

    /// Provided by user during Base creation.
    symbol: String,

    /// Parts that the base has.
    /// Mapping from `PartId` to fixed or slot `Part`.
    parts: BTreeMap<PartId, Part>,
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

    /// An IPFS Uri pointing to main media file of this part.
    pub src: String,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo, Eq, PartialEq)]
pub struct SlotPart {
    /// Array of whitelisted collections with tokens that can be equipped in the given slot. Used with slot parts only.
    pub equippable: EquippableList,

    /// An optional zIndex of base part layer.
    /// specifies the stack order of an element.
    /// An element with greater stack order is always in front of an element with a lower stack order.
    pub z: Option<ZIndex>,

    /// An IPFS Uri pointing to main media file of this part.
    pub src: String,
}

#[derive(Debug, Clone, Decode, Encode, TypeInfo, Eq, PartialEq)]
pub enum Part {
    Fixed(FixedPart),
    Slot(SlotPart),
}

#[derive(Debug, Decode, Encode, TypeInfo)]
pub struct InitBase {
    pub base_type: String,
    pub symbol: String,
}

#[derive(Debug, Decode, Encode, TypeInfo)]
pub enum BaseAction {
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
    /// * The `msg::source()` must be the contract issuer.
    /// * The indicated collection contract must be RMRK contract.
    /// * The token from indicated collections must have composable resource that refers to that base.
    ///
    /// # Arguments:
    /// * `collection_id`: an address of RMRK contract.
    /// * `token_id`: the id of the token in RMRK contract.
    ///
    /// On success replies `[BaseEvent::EquippableAdded]`.
    AddEquippable {
        part_id: PartId,
        collection_id: CollectionId,
        token_id: TokenId,
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
    /// * `token_id`: the id of the token in RMRK contract.
    ///
    /// On success replies `[BaseEvent::EquippableRemoved]`.
    RemoveEquippable {
        part_id: PartId,
        collection_id: CollectionId,
        token_id: TokenId,
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
        token_id: TokenId,
    },
}

#[derive(Debug, Decode, Encode, TypeInfo)]
pub enum BaseEvent {
    PartsAdded(BTreeMap<PartId, Part>),
    EquippableAdded {
        part_id: PartId,
        collection_id: CollectionId,
        token_id: TokenId,
    },
    PartsRemoved(Vec<PartId>),
    EquippableRemoved {
        part_id: PartId,
        collection_id: CollectionId,
        token_id: TokenId,
    },
    Part(Part),
    InEquippableList,
}

#[derive(Debug, Decode, Encode, TypeInfo)]
pub enum BaseState {
    Parts,
    Part(PartId),
    IsEquippable {
        part_id: PartId,
        collection_id: CollectionId,
        token_id: TokenId,
    },
}

#[derive(Debug, Decode, Encode, TypeInfo, Eq, PartialEq)]
pub enum BaseStateReply {
    Parts(Vec<Part>),
    Part(Option<Part>),
    IsEquippable(bool),
}
