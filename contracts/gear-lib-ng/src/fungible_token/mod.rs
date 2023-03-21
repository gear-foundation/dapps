use crate::{Amount, Operator, Owner, StorageProvider};
use gstd::{prelude::*, ActorId};

#[cfg(feature = "testing")]
use super::testing::msg;
#[cfg(not(feature = "testing"))]
use gstd::msg;

pub use base::*;
pub use extensions::*;

/// The core trait & items.
pub mod base;
/// Core extensions.
pub mod extensions;

/// The default implementation's state.
#[derive(Debug, Default, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, TypeInfo, Hash)]
pub struct FTState {
    pub total_supply: Amount,
    pub balances: BTreeMap<Owner, Amount>,
    pub allowances: BTreeMap<Owner, BTreeMap<Operator, Amount>>,
}

impl FTState {
    pub fn approve(
        &mut self,
        owner: Owner,
        operator: Operator,
        amount: Amount,
    ) -> Result<(), FTError> {
        if operator.is_zero() {
            return Err(FTError::ZeroRecipientAddress);
        }

        self.allowances
            .entry(owner)
            .or_default()
            .insert(operator, amount);

        Ok(())
    }

    pub fn burn_balance(&mut self, from: Owner, amount: Amount) -> Result<(), FTError> {
        self.balances
            .get_mut(&from)
            .and_then(|balance| balance.checked_sub(amount).map(|amount| *balance = amount))
            .ok_or(FTError::InsufficientAmount)
    }

    pub fn approved_amount(
        &self,
        owner: Owner,
        operator: Operator,
        amount: Amount,
        checked_operation: fn(Amount, Amount) -> Option<Amount>,
    ) -> Result<Amount, FTError> {
        self.allowance(owner, operator)
            .and_then(|allowance| checked_operation(*allowance, amount))
            .ok_or(FTError::InsufficientAllowance)
    }

    pub fn allowance(&self, owner: Owner, operator: Operator) -> Option<&Amount> {
        self.allowances
            .get(&owner)
            .and_then(|allowances| allowances.get(&operator))
    }
}

/// The default implementation of [`FTCore`] & its extensions.
pub trait FungibleToken: StorageProvider<FTState> {
    fn total_supply(&self) -> Amount {
        self.storage().total_supply
    }

    fn balance_of(&self, owner: Owner) -> Amount {
        *self
            .storage()
            .balances
            .get(&owner)
            .unwrap_or(&Amount::default())
    }

    fn transfer(&mut self, to: ActorId, amount: Amount) -> Result<(), FTError> {
        self._transfer(msg::source(), to, amount)?;

        Ok(())
    }

    fn allowance(&self, owner: Owner, operator: Operator) -> Amount {
        *self
            .storage()
            .allowance(owner, operator)
            .unwrap_or(&Amount::default())
    }

    fn transfer_from(&mut self, from: Owner, to: ActorId, amount: Amount) -> Result<(), FTError> {
        let state = self.storage_mut();
        let msg_source = msg::source();
        let approved_amount =
            state.approved_amount(from, msg_source, amount, Amount::checked_sub)?;

        state.approve(from, msg_source, approved_amount)?;
        self._transfer(from, to, amount)?;

        Ok(())
    }

    fn approve(&mut self, operator: Operator, amount: Amount) -> Result<(), FTError> {
        let msg_source = msg::source();

        self._approve(msg_source, operator, amount)?;

        Ok(())
    }

    fn _approve(
        &mut self,
        msg_source: ActorId,
        operator: Operator,
        amount: Amount,
    ) -> Result<(), FTError> {
        self.storage_mut().approve(msg_source, operator, amount)?;
        self.reply_approval(FTApproval {
            owner: msg_source,
            operator,
            amount,
        })?;

        Ok(())
    }

    fn increase_allowance(
        &mut self,
        operator: Operator,
        delta_amount: Amount,
    ) -> Result<(), FTError> {
        let msg_source = msg::source();
        let amount = self.storage().approved_amount(
            msg_source,
            operator,
            delta_amount,
            Amount::checked_add,
        )?;

        self._approve(msg_source, operator, amount)?;

        Ok(())
    }

    fn decrease_allowance(
        &mut self,
        operator: Operator,
        delta_amount: Amount,
    ) -> Result<(), FTError> {
        let msg_source = msg::source();
        let amount = self.storage().approved_amount(
            msg_source,
            operator,
            delta_amount,
            Amount::checked_sub,
        )?;

        self._approve(msg_source, operator, amount)?;

        Ok(())
    }

    fn reply_transfer(&self, transfer: FTTransfer) -> Result<(), FTError>;

    fn reply_approval(&self, approval: FTApproval) -> Result<(), FTError>;

    fn mint(&mut self, to: ActorId, amount: Amount) -> Result<(), FTError> {
        let state = self.storage_mut();

        if let Some(total_supply) = state.total_supply.checked_add(amount) {
            state.total_supply = total_supply
        } else {
            return Err(FTError::InsufficientAmount);
        }

        *state.balances.entry(to).or_default() += amount;

        self.reply_transfer(FTTransfer {
            from: ActorId::zero(),
            to,
            amount,
        })?;

        Ok(())
    }

    fn burn(&mut self, from: Owner, amount: Amount) -> Result<(), FTError> {
        if from.is_zero() {
            return Err(FTError::ZeroSenderAddress);
        }

        let state = self.storage_mut();

        state.burn_balance(from, amount)?;
        state.total_supply -= amount;

        self.reply_transfer(FTTransfer {
            from,
            to: ActorId::zero(),
            amount,
        })?;

        Ok(())
    }

    fn _transfer(&mut self, from: Owner, to: ActorId, amount: Amount) -> Result<(), FTError> {

        let state = self.storage_mut();

        state.burn_balance(from, amount)?;
        state
            .balances
            .entry(to)
            .and_modify(|balance| *balance += amount)
            .or_insert(amount);

        self.reply_transfer(FTTransfer { from, to, amount })?;

        Ok(())
    }
}

impl<T: FungibleToken> FTCore for T {
    fn total_supply(&self) -> Amount {
        T::total_supply(self)
    }

    fn balance_of(&self, owner: Owner) -> Amount {
        T::balance_of(self, owner)
    }

    fn allowance(&self, owner: Owner, operator: Operator) -> Amount {
        T::allowance(self, owner, operator)
    }

    fn transfer(&mut self, to: ActorId, amount: Amount) -> Result<(), FTError> {
        T::transfer(self, to, amount)
    }

    fn transfer_from(&mut self, from: Owner, to: ActorId, amount: Amount) -> Result<(), FTError> {
        T::transfer_from(self, from, to, amount)
    }

    fn approve(&mut self, operator: Operator, amount: Amount) -> Result<(), FTError> {
        T::approve(self, operator, amount)
    }

    fn increase_allowance(
        &mut self,
        operator: Operator,
        delta_amount: Amount,
    ) -> Result<(), FTError> {
        T::increase_allowance(self, operator, delta_amount)
    }

    fn decrease_allowance(
        &mut self,
        operator: Operator,
        delta_amount: Amount,
    ) -> Result<(), FTError> {
        T::decrease_allowance(self, operator, delta_amount)
    }
}
