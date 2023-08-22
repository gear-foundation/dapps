#![no_std]

pub mod primitives {
    use gstd::{prelude::*, ActorId};
    use primitive_types::U256;

    // The address of RMRK contract.
    pub type CollectionId = ActorId;

    // The identifier of resource for RMRK token.
    pub type ResourceId = u8;

    // The identifier of RMRK token.
    pub type TokenId = U256;

    // The address of Base storage contract.
    pub type BaseId = ActorId;

    // The identifier of part in Base storage contract.
    pub type PartId = u32;

    // The property specifies the stack order of an element.
    pub type ZIndex = u32;

    // Definition of the RMRK contract and token.
    pub type CollectionAndToken = (CollectionId, TokenId);

    // An array of parts.
    pub type Parts = Vec<PartId>;
}
