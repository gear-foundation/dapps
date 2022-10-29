use gstd::{prelude::*, ActorId};
use primitive_types::U256;

#[doc(hidden)]
#[cfg(feature = "derive")]
pub use gear_lib_derive::StorageProvider;

/// An owner of some tokens.
pub type Owner = ActorId;
/// An operator of some tokens.
pub type Operator = ActorId;

/// Used for getting different data/storage/state from the same type by 2
/// functions.
///
/// This library use this trait for getting the default implementation's
/// contract state structs from user's ones.
///
/// **Usually this trait doesn't need to be implemented and should be derived by
/// the derive macro of the same name.**
///
/// For example:
/// ```
/// use gear_lib::StorageProvider;
///
/// struct Storage;
///
/// #[derive(StorageProvider)]
/// struct Contract {
///     #[storage_field]
///     storage: Storage,
/// }
/// ```
///
/// Now the `Storage` struct can be retrieved from the `Contract` struct by the
/// [`storage()`](Self::storage) or [`storage_mut()`](Self::storage_mut)
/// functions instead of specifying the field with the `Storage` struct.
///
/// The derive macro use the `#[storage_field]` attribute to identify fields for
/// generating implementations. Fields can have any name, not only "storage".
///
/// It's also possible to get 2 (and more) different types from 1 struct. Just
/// add one more field with the `storage_field` attribute, for example:
/// ```
/// use gear_lib::StorageProvider;
///
/// struct Storage1;
/// struct Storage2;
///
/// #[derive(StorageProvider)]
/// struct Contract {
///     #[storage_field]
///     storage1: Storage1,
///     #[storage_field]
///     storage2: Storage2,
/// }
/// ```
/// Now `Storage1` and `Storage2` can be retrieved by the same 2 functions.
/// The compiler will use type inference to know what `Storage*` should be
/// retrieved.
///
/// Pay attention that types in fields with the `storage_field` attribute
/// **MUST** be different because of the type inference logic, otherwise the
/// derive macro will generate "conflicting implentations" compilation errors.
pub trait StorageProvider<S> {
    fn storage(&self) -> &S;

    fn storage_mut(&mut self) -> &mut S;
}

/// An amount of some tokens.
pub type Amount = U256;

/// The simple & flexible identifier for different types of tokens.
#[derive(Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Clone, TypeInfo)]
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
