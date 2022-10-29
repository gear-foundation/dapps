use super::{NFTError, NonFungibleToken};
use crate::{Id, Owner};
use gstd::ActorId;

pub trait NFTMint {
    /// # Returns
    /// - [`NFTError::ZeroRecipientAddress`], if `to` is [`ActorId::zero()`].
    /// - [`NFTError::TokenNotExists`], if a resulted total supply of all tokens
    /// is greater than [`Amount::MAX`](crate::Amount::MAX).
    /// - [`NFTError::TokenExists`], if the token with specified `id` already
    /// exists.
    fn mint(&mut self, to: ActorId, id: Id) -> Result<(), NFTError>;
}

impl<T: NonFungibleToken> NFTMint for T {
    fn mint(&mut self, to: ActorId, id: Id) -> Result<(), NFTError> {
        T::mint(self, to, id)
    }
}

pub trait NFTBurn {
    /// # Returns
    /// - [`NFTError::ZeroSenderAddress`], if `to` is [`ActorId::zero()`].
    /// - [`NFTError::TokenNotExists`], if the token with specified `id` not
    /// exists.
    fn burn(&mut self, from: Owner, id: Id) -> Result<(), NFTError>;
}

impl<T: NonFungibleToken> NFTBurn for T {
    fn burn(&mut self, from: Owner, id: Id) -> Result<(), NFTError> {
        T::burn(self, from, id)
    }
}
