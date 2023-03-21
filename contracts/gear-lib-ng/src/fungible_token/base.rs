use crate::{Amount, Operator, Owner};
use gstd::{prelude::*, ActorId};

pub trait FTCore {
    fn total_supply(&self) -> Amount;

    fn balance_of(&self, owner: Owner) -> Amount;

    fn allowance(&self, owner: Owner, operator: Operator) -> Amount;

    /// # Returns
    /// - [`FTError::InsufficientAmount`], if
    /// [`msg::source()`](gstd::msg::source) doesn't have enough tokens for a
    /// requested transfer.
    fn transfer(&mut self, to: ActorId, amount: Amount) -> Result<(), FTError>;

    /// # Returns
    /// - [`FTError::ZeroSenderAddress`], if `from` is [`ActorId::zero()`].
    /// - [`FTError::InsufficientAmount`], if `from` doesn't have enough tokens
    /// for a requested transfer.
    /// - [`FTError::InsufficientAllowance`], if `from` doesn't approve enough
    /// tokens for `to` for a requested transfer.
    fn transfer_from(&mut self, from: Owner, to: ActorId, amount: Amount) -> Result<(), FTError>;

    /// # Returns
    /// - [`FTError::ZeroRecipientAddress`], if `operator` is
    /// [`ActorId::zero()`].
    fn approve(&mut self, operator: Operator, amount: Amount) -> Result<(), FTError>;

    /// # Returns
    /// - [`FTError::ZeroRecipientAddress`], if `operator` is
    /// [`ActorId::zero()`].
    /// - [`FTError::InsufficientAllowance`], if a resulted allowance is greater
    /// than [`Amount::MAX`].
    fn increase_allowance(
        &mut self,
        operator: Operator,
        delta_amount: Amount,
    ) -> Result<(), FTError>;

    /// # Returns
    /// - [`FTError::ZeroRecipientAddress`], if `operator` is
    /// [`ActorId::zero()`].
    /// - [`FTError::InsufficientAllowance`], if a resulted allowance is less
    /// than 0.
    fn decrease_allowance(
        &mut self,
        operator: Operator,
        delta_amount: Amount,
    ) -> Result<(), FTError>;
}

#[derive(Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, TypeInfo, Default)]
pub struct FTTransfer {
    pub from: ActorId,
    pub to: ActorId,
    pub amount: Amount,
}

#[derive(Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, TypeInfo, Default)]
pub struct FTApproval {
    pub owner: Owner,
    pub operator: Operator,
    pub amount: Amount,
}

#[derive(Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, TypeInfo)]
pub enum FTError {
    Custom(Vec<u8>),
    InsufficientAmount,
    InsufficientAllowance,
    ZeroRecipientAddress,
    ZeroSenderAddress,
}
