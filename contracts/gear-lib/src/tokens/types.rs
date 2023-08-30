//! Token primitives.

use gstd::{prelude::*, ActorId};
use primitive_types::U256;

/// An owner of some tokens.
pub type Owner = ActorId;
/// An operator of some tokens.
pub type Operator = ActorId;
/// An amount of some tokens.
pub type Amount = U256;

/// The simple & flexible identifier type for different types of tokens.
#[derive(Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, TypeInfo, Hash)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum Id {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    U256(U256),
    Bytes(Vec<u8>),
}

macro_rules! impl_from_for_id {
    { $( $from:ty => $to:ident ),*, } => {
        $(
            impl From<$from> for Id {
                fn from(id: $from) -> Self {
                    Self::$to(id)
                }
            }
        )*
    };
}

impl_from_for_id! {
    u8 => U8,
    u16 => U16,
    u32 => U32,
    u64 => U64,
    u128 => U128,
    U256 => U256,
    Vec<u8> => Bytes,
}
