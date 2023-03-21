use super::{FTError, FungibleToken};
use crate::{Amount, Owner, StorageProvider};
use gstd::{prelude::*, ActorId};

pub trait FTMint {
    /// # Returns
    /// - [`FTError::InsufficientAmount`], if a resulted total supply is greater
    /// than [`Amount::MAX`].
    fn mint(&mut self, to: ActorId, amount: Amount) -> Result<(), FTError>;
}

impl<T: FungibleToken> FTMint for T {
    fn mint(&mut self, to: ActorId, amount: Amount) -> Result<(), FTError> {
        T::mint(self, to, amount)
    }
}

pub trait FTBurn {
    /// # Returns
    /// - [`FTError::ZeroSenderAddress`], if `from` is [`ActorId::zero()`].
    /// - [`FTError::InsufficientAmount`], if a resulted total supply is less
    /// than 0.
    fn burn(&mut self, from: Owner, amount: Amount) -> Result<(), FTError>;
}

impl<T: FungibleToken> FTBurn for T {
    fn burn(&mut self, from: Owner, amount: Amount) -> Result<(), FTError> {
        T::burn(self, from, amount)
    }
}

/// The gFT's metadata extension.
pub trait FTMeta {
    fn name(&self) -> Option<String>;

    fn symbol(&self) -> Option<String>;

    fn decimals(&self) -> u8;
}

/// The state for [`FTMeta`].
#[derive(Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, TypeInfo, Default)]
pub struct FTStateMeta {
    pub name: Option<String>,
    pub symbol: Option<String>,
    pub decimals: u8,
}

/// The default implementation of [`FTMeta`].
pub trait FungibleTokenMeta: StorageProvider<FTStateMeta> {
    fn name(&self) -> Option<String> {
        self.storage().name.clone()
    }

    fn symbol(&self) -> Option<String> {
        self.storage().symbol.clone()
    }

    fn decimals(&self) -> u8 {
        self.storage().decimals
    }
}

impl<T: FungibleTokenMeta> FTMeta for T {
    fn name(&self) -> Option<String> {
        T::name(self)
    }

    fn symbol(&self) -> Option<String> {
        T::symbol(self)
    }

    fn decimals(&self) -> u8 {
        T::decimals(self)
    }
}
