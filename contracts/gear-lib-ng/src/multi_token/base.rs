use crate::{Amount, Id, Operator, Owner};
use gstd::{prelude::*, ActorId};

pub trait MTCore {
    /// # Returns
    /// - If `id` is [`None`], balance of all owner's tokens.
    /// - If `id` is [`Some`], balance of all owner's tokens with specified
    /// [`Id`].
    fn balance_of(&self, owner: Owner, id: Option<Id>) -> Amount;

    /// # Returns
    /// - If `id` is [`None`], total supply of all tokens.
    /// - If `id` is [`Some`], total supply of all tokens with specified [`Id`].
    fn total_supply(&self, id: Option<Id>) -> Amount;

    /// # Returns
    /// - If `id` is [`None`] and `operator` is an operator of all `owner`'s
    /// tokens, [`true`], else [`false`].
    /// - If `id` is [`Some`] and `operator` is an operator of all `owner`'s
    /// tokens or only of all `owner`'s tokens with specified [`Id`], [`true`],
    /// else [`false`].
    fn allowance(&self, owner: Owner, operator: Operator, id: Option<Id>) -> bool;

    /// If `approve` is [`true`] and:
    /// - `id` is [`None`], makes `operator` an operator of all `owner`'s
    /// tokens.
    /// - `id` is [`Some`], makes `operator` an operator only of all `owner`'s
    /// tokens with specified [`Id`].
    ///
    /// If `approve` is [`false`], revokes an approval.
    ///
    /// # Returns
    /// - [`MTError::ZeroRecipientAddress`], if `operator` is
    /// [`ActorId::zero()`].
    fn approve(&mut self, operator: Operator, id: Option<Id>, approve: bool)
        -> Result<(), MTError>;

    /// # Returns
    /// - [`MTError::ZeroRecipientAddress`], if `to` is [`ActorId::zero()`].
    /// - [`MTError::InsufficientAmount`], if
    /// [`msg::source()`](gstd::msg::source) doesn't have enough tokens for a
    /// requested transfer.
    fn transfer(&mut self, to: ActorId, id: Id, amount: Amount) -> Result<(), MTError>;

    /// # Returns
    /// - [`MTError::ZeroSenderAddress`], if `from` is [`ActorId::zero()`].
    /// - [`MTError::ZeroRecipientAddress`], if `to` is [`ActorId::zero()`].
    /// - [`MTError::InsufficientAmount`], if `from` doesn't have enough tokens
    /// for a requested transfer.
    /// - [`MTError::NotApproved`], if `to` isn't an operator of all `from`'s
    /// tokens or all `from`'s tokens with the `id` ID.
    fn transfer_from(
        &mut self,
        from: Owner,
        to: ActorId,
        id: Id,
        amount: Amount,
    ) -> Result<(), MTError>;
}

#[derive(Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, TypeInfo)]
pub struct MTTransfer {
    pub from: ActorId,
    pub to: ActorId,
    pub id: Id,
    pub amount: Amount,
}

#[derive(Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, TypeInfo, Default)]
pub struct MTApproval {
    pub owner: ActorId,
    pub operator: Operator,
    pub id: Option<Id>,
    pub approved: bool,
}

#[derive(Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, TypeInfo)]
pub enum MTError {
    Custom(Vec<u8>),
    NotApproved,
    ZeroRecipientAddress,
    ZeroSenderAddress,
    InsufficientAmount,
}
