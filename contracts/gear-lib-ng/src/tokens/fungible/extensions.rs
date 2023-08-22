//! Fungible token implementation extensions.

use gstd::prelude::*;

/// Fungible token metadata.
#[derive(Default, Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, TypeInfo, Hash)]
pub struct FTStateMeta {
    pub name: Option<String>,
    pub symbol: Option<String>,
    pub decimals: u8,
}
