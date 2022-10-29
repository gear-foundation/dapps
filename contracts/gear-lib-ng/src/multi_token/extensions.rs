use super::{MTError, MultiToken};
use crate::{Amount, Id, Owner, StorageProvider};
use gstd::{prelude::*, ActorId};

pub trait MTMint {
    /// # Returns
    /// - [`MTError::ZeroRecipientAddress`], if `to` is [`ActorId::zero()`].
    /// - [`MTError::InsufficientAmount`], if a resulted total supply of all
    /// tokens is greater than [`Amount::MAX`].
    fn mint(&mut self, to: ActorId, id: Id, amount: Amount) -> Result<(), MTError>;
}

impl<T: MultiToken> MTMint for T {
    fn mint(&mut self, to: ActorId, id: Id, amount: Amount) -> Result<(), MTError> {
        T::mint(self, to, id, amount)
    }
}

pub trait MTBurn {
    /// # Returns
    /// - [`MTError::ZeroSenderAddress`], if `to` is [`ActorId::zero()`].
    /// - [`MTError::InsufficientAmount`], if a resulted total supply of tokens
    /// with specified `id` is less than 0.
    fn burn(&mut self, from: Owner, id: Id, amount: Amount) -> Result<(), MTError>;
}

impl<T: MultiToken> MTBurn for T {
    fn burn(&mut self, from: Owner, id: Id, amount: Amount) -> Result<(), MTError> {
        T::burn(self, from, id, amount)
    }
}

/// The gMT's metadata extension.
pub trait MTMeta {
    fn get_attribute(&self, id: Id, key: Vec<u8>) -> Option<Vec<u8>>;
}

#[derive(Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, TypeInfo)]
pub struct MTAttribute {
    pub id: Id,
    pub key: Vec<u8>,
    pub data: Vec<u8>,
}

/// The state for [`MTMeta`].
#[derive(Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, TypeInfo, Default)]
pub struct MTStateMeta {
    pub attributes: BTreeMap<Id, BTreeMap<Vec<u8>, Vec<u8>>>,
}

/// The default implementation of [`MTMeta`].
pub trait MultiTokenMeta: StorageProvider<MTStateMeta> {
    fn get_attribute(&self, id: Id, key: Vec<u8>) -> Option<Vec<u8>> {
        self.storage()
            .attributes
            .get(&id)
            .and_then(|attributes| attributes.get(&key))
            .cloned()
    }

    fn set_attribute(&mut self, id: Id, key: Vec<u8>, data: Vec<u8>) -> Result<(), MTError> {
        self.storage_mut()
            .attributes
            .entry(id.clone())
            .or_default()
            .insert(key.clone(), data.clone());

        self.reply_set_attribute(MTAttribute { id, key, data })?;

        Ok(())
    }

    fn reply_set_attribute(&self, attribute: MTAttribute) -> Result<(), MTError>;
}

impl<T: MultiTokenMeta> MTMeta for T {
    fn get_attribute(&self, id: Id, key: Vec<u8>) -> Option<Vec<u8>> {
        T::get_attribute(self, id, key)
    }
}

/// The gMT's batch extension.
pub trait MTBatch {
    /// # Returns
    /// - [`MTError::ZeroRecipientAddress`], if `to` is [`ActorId::zero()`].
    /// - [`MTError::InsufficientAmount`], if
    /// [`msg::source()`](gstd::msg::source) doesn't have enough tokens for a
    /// requested transfer.
    fn transfer_batch(
        &mut self,
        to: ActorId,
        ids_for_amount: BTreeMap<Id, Amount>,
    ) -> Result<(), MTError>;

    /// # Returns
    /// - [`MTError::ZeroSenderAddress`], if `from` is [`ActorId::zero()`].
    /// - [`MTError::ZeroRecipientAddress`], if `to` is [`ActorId::zero()`].
    /// - [`MTError::InsufficientAmount`], if `from` doesn't have enough tokens
    /// for a requested transfer.
    /// - [`MTError::NotApproved`], if `to` isn't an operator of all `from`'s
    /// tokens or all `from`'s tokens with IDs from specified `ids_for_amount`.
    fn transfer_from_batch(
        &mut self,
        from: Owner,
        to: ActorId,
        ids_for_amount: BTreeMap<Id, Amount>,
    ) -> Result<(), MTError>;

    /// # Returns
    /// - [`MTError::ZeroRecipientAddress`], if `to` is [`ActorId::zero()`].
    /// - [`MTError::InsufficientAmount`], if a resulted total supply of all
    /// tokens is greater than [`Amount::MAX`].
    fn mint_batch(
        &mut self,
        to: ActorId,
        ids_for_amount: BTreeMap<Id, Amount>,
    ) -> Result<(), MTError>;

    /// # Returns
    /// - [`MTError::ZeroSenderAddress`], if `to` is [`ActorId::zero()`].
    /// - [`MTError::InsufficientAmount`], if a resulted total supply of tokens
    /// with one or more IDs from specified `ids_for_amount` is less than 0.
    fn burn_batch(
        &mut self,
        from: Owner,
        ids_for_amount: BTreeMap<Id, Amount>,
    ) -> Result<(), MTError>;

    fn balance_of_batch(
        &self,
        owners_for_ids: BTreeMap<Owner, BTreeSet<Option<Id>>>,
    ) -> BTreeMap<Owner, BTreeSet<(Option<Id>, Amount)>>;
}

#[derive(Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, TypeInfo, Default)]
pub struct MTTransferBatch {
    pub from: Owner,
    pub to: ActorId,
    pub ids_for_amount: BTreeMap<Id, Amount>,
}

impl<T: MultiToken> MTBatch for T {
    fn transfer_batch(
        &mut self,
        to: ActorId,
        ids_for_amount: BTreeMap<Id, Amount>,
    ) -> Result<(), MTError> {
        T::transfer_batch(self, to, ids_for_amount)
    }

    fn transfer_from_batch(
        &mut self,
        from: Owner,
        to: ActorId,
        ids_for_amount: BTreeMap<Id, Amount>,
    ) -> Result<(), MTError> {
        T::transfer_from_batch(self, from, to, ids_for_amount)
    }

    fn mint_batch(
        &mut self,
        to: ActorId,
        ids_for_amount: BTreeMap<Id, Amount>,
    ) -> Result<(), MTError> {
        T::mint_batch(self, to, ids_for_amount)
    }

    fn burn_batch(
        &mut self,
        from: Owner,
        ids_for_amount: BTreeMap<Id, Amount>,
    ) -> Result<(), MTError> {
        T::burn_batch(self, from, ids_for_amount)
    }

    fn balance_of_batch(
        &self,
        owners_for_ids: BTreeMap<Owner, BTreeSet<Option<Id>>>,
    ) -> BTreeMap<Owner, BTreeSet<(Option<Id>, Amount)>> {
        T::balance_of_batch(self, owners_for_ids)
    }
}
