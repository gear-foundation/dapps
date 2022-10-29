use crate::{Amount, Id, Operator, Owner};
use gstd::{prelude::*, ActorId};

pub trait NFTCore {
    fn balance_of(&self, owner: Owner) -> Amount;

    fn owner_of(&self, id: Id) -> Owner;

    /// # Returns
    /// - If `id` is [`None`] and `operator` is an operator of all `owner`'s
    /// tokens, [`true`], else [`false`].
    /// - If `id` is [`Some`] and `operator` is an operator of all `owner`'s
    /// tokens or of the single token with specified [`Id`], [`true`], else
    /// [`false`].
    fn allowance(&self, owner: Owner, operator: Operator, id: Option<Id>) -> bool;

    /// If `approve` is [`true`] and:
    /// - `id` is [`None`], makes `operator` an operator of all tokens of
    /// [`msg::source()`](gstd::msg::source).
    /// - `id` is [`Some`], makes `operator` an operator of the single token
    /// with specified [`Id`].
    ///
    /// If `approve` is [`false`], revokes an approval.
    ///
    /// An operator of the single token can transfer its approval or self-revoke
    /// it.
    ///
    /// # Returns
    /// - [`NFTError::ZeroRecipientAddress`], if `operator` is
    /// [`ActorId::zero()`].
    /// - [`NFTError::NotApproved`], if `id` is [`Some`] and
    /// [`msg::source()`](gstd::msg::source) isn't an operator of the token with
    /// specified [`Id`] or of all this token owner's tokens.
    fn approve(
        &mut self,
        operator: Operator,
        id: Option<Id>,
        approve: bool,
    ) -> Result<(), NFTError>;

    /// # Returns
    /// - [`NFTError::ZeroRecipientAddress`], if `to` is [`ActorId::zero()`].
    /// - [`NFTError::TokenNotExists`], if the token with specified `id` doesn't
    /// exist.
    /// - [`NFTError::NotApproved`], if [`msg::source()`](gstd::msg::source)
    /// isn't an operator of the token with specified `id` or of all this token
    /// owner's tokens.
    fn transfer(&mut self, to: ActorId, id: Id) -> Result<(), NFTError>;

    fn total_supply(&self) -> Amount;
}

/// The **recommended** gNFT's metadata extension.
///
/// Its default implemetation is included in
/// [`NonFungibleToken`](super::NonFungibleToken).
pub trait NFTMeta {
    fn get_attribute(&self, id: Id, key: Vec<u8>) -> Option<Vec<u8>>;
}

#[derive(Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, TypeInfo)]
pub struct NFTTransfer {
    pub from: ActorId,
    pub to: ActorId,
    pub id: Id,
}

#[derive(Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, TypeInfo)]
pub struct NFTApproval {
    pub owner: Owner,
    pub operator: Operator,
    pub id: Option<Id>,
    pub approved: bool,
}

#[derive(Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, TypeInfo)]
pub struct NFTAttribute {
    pub id: Id,
    pub key: Vec<u8>,
    pub data: Vec<u8>,
}

#[derive(Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, TypeInfo)]
pub enum NFTError {
    Custom(Vec<u8>),
    NotApproved,
    TokenExists,
    TokenNotExists,
    ZeroRecipientAddress,
    ZeroSenderAddress,
}
