#![no_std]

pub use common::*;

#[cfg(feature = "testing")]
pub mod testing;

/// Common items between modules.
pub mod common;
/// The gFT core trait, items, extensions, and default implementation.
pub mod fungible_token;
/// The gMT core trait, items, extensions, and default implementation.
pub mod multi_token;
/// The gNFT core traits, items, extensions, and default implementation.
pub mod non_fungible_token;
