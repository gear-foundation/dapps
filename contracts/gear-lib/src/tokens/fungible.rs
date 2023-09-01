//! The fungible token.

use super::types::{Amount, Operator, Owner};
use gstd::{collections::HashMap, prelude::*, ActorId};

#[cfg(test)]
use super::test_helper::msg;
#[cfg(not(test))]
use gstd::msg;

pub mod encodable;
pub mod extensions;

#[derive(Default, Debug, PartialEq, Eq, Clone)]
struct OwnerData {
    balance: Amount,
    allowances: HashMap<Operator, Amount>,
}

/// The fungible token implementation.
#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub struct FTState {
    total_supply: Amount,
    owners: HashMap<Owner, OwnerData>,
}

impl FTState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Gets the current total token supply.
    pub const fn total_supply(&self) -> Amount {
        self.total_supply
    }

    /// Allows `operator` to spend `amount` of [`msg::source()`]'s tokens.
    ///
    /// # Errors
    /// - [`FTError::ZeroRecipientAddress`] if `operator` is
    /// [`ActorId::zero()`].
    pub fn approve(&mut self, operator: Operator, amount: Amount) -> Result<FTApproval, FTError> {
        self.internal_approve(msg::source(), operator, amount)
    }

    fn unchecked_internal_approve(&mut self, owner: Owner, operator: Operator, amount: Amount) {
        self.owners
            .entry(owner)
            .or_default()
            .allowances
            .insert(operator, amount);
    }

    fn internal_approve(
        &mut self,
        owner: Owner,
        operator: Operator,
        amount: Amount,
    ) -> Result<FTApproval, FTError> {
        if operator.is_zero() {
            return Err(FTError::ZeroRecipientAddress);
        }

        self.unchecked_internal_approve(owner, operator, amount);

        Ok(FTApproval {
            owner,
            operator,
            amount,
        })
    }

    /// Returns a balance of `owner`'s tokens.
    pub fn balance_of(&self, owner: Owner) -> Amount {
        self.owners
            .get(&owner)
            .map(|owner_data| owner_data.balance)
            .unwrap_or_default()
    }

    /// Returns an allowance of `owner`'s tokens for `operator`.
    pub fn allowance(&self, owner: Owner, operator: Operator) -> Amount {
        self.internal_allowance(owner, operator)
            .copied()
            .unwrap_or_default()
    }

    fn internal_allowance(&self, owner: Owner, operator: Operator) -> Option<&Amount> {
        self.owners
            .get(&owner)
            .and_then(|owner_data| owner_data.allowances.get(&operator))
    }

    /// Transfers `amount` of tokens from [`msg::source()`] to `to`.
    ///
    /// # Errors
    /// - [`FTError::ZeroRecipientAddress`] if `to` is [`ActorId::zero()`].
    /// - [`FTError::InsufficientAmount`] if [`msg::source()`] doesn't have
    /// given `amount` of tokens.
    pub fn transfer(&mut self, to: ActorId, amount: Amount) -> Result<FTTransfer, FTError> {
        self.internal_transfer(msg::source(), to, amount)
    }

    fn internal_transfer(
        &mut self,
        from: Owner,
        to: ActorId,
        amount: Amount,
    ) -> Result<FTTransfer, FTError> {
        if to.is_zero() {
            return Err(FTError::ZeroRecipientAddress);
        }

        self.burn_balance(from, amount)?;
        self.owners
            .entry(to)
            .and_modify(|owner| owner.balance += amount)
            .or_insert(OwnerData {
                balance: amount,
                ..Default::default()
            });

        Ok(FTTransfer { from, to, amount })
    }

    fn burn_balance(&mut self, from: Owner, amount: Amount) -> Result<(), FTError> {
        self.owners
            .get_mut(&from)
            .and_then(|owner| {
                owner
                    .balance
                    .checked_sub(amount)
                    .map(|total_amount| owner.balance = total_amount)
            })
            .ok_or(FTError::InsufficientAmount)
    }

    /// Transfers `amount` of tokens from `from` to `to`.
    ///
    /// Requires an allowance for [`msg::source()`] to transfer `amount` or a
    /// larger amount of `from`'s tokens. Note that this function will **not**
    /// work as an equivalent of [`FTState::transfer()`] if [`msg::source()`]
    /// equals `from`.
    ///
    /// # Errors
    /// - [`FTError::ZeroSenderAddress`] if `from` is [`ActorId::zero()`].
    /// - [`FTError::ZeroRecipientAddress`] if `to` is [`ActorId::zero()`].
    /// - [`FTError::InsufficientAllowance`] if [`msg::source()`] doesn't have
    /// an allowance for given `amount` of tokens.
    /// - [`FTError::InsufficientAmount`] if `from` doesn't have given `amount`
    /// of tokens.
    pub fn transfer_from(
        &mut self,
        from: Owner,
        to: ActorId,
        amount: Amount,
    ) -> Result<FTTransfer, FTError> {
        if from.is_zero() {
            return Err(FTError::ZeroSenderAddress);
        }

        let msg_source = msg::source();
        let approved_amount = self
            .internal_allowance(from, msg_source)
            .and_then(|allowance| allowance.checked_sub(amount))
            .ok_or(FTError::InsufficientAllowance)?;

        self.unchecked_internal_approve(from, msg_source, approved_amount);
        self.internal_transfer(from, to, amount)
    }

    fn change_allowance(
        &mut self,
        msg_source: ActorId,
        operator: Operator,
        delta_amount: Amount,
        checked_op: impl FnOnce(Amount, Amount) -> Option<Amount>,
    ) -> Result<Amount, FTError> {
        self.internal_allowance(msg_source, operator)
            .and_then(|allowance| checked_op(*allowance, delta_amount))
            .ok_or(FTError::InsufficientAllowance)
    }

    /// Increases by `delta_amount` an allowance for `operator` to spend
    /// [`msg::source()`]'s tokens.
    ///
    /// # Errors
    /// - [`FTError::InsufficientAllowance`] if given `delta_amount` with the
    /// current allowance overflows the [`Amount`] type.
    pub fn increase_allowance(
        &mut self,
        operator: Operator,
        delta_amount: Amount,
    ) -> Result<FTApproval, FTError> {
        let msg_source = msg::source();
        let amount =
            self.change_allowance(msg_source, operator, delta_amount, Amount::checked_add)?;

        self.internal_approve(msg_source, operator, amount)
    }

    /// Decreases by `delta_amount` an allowance for `operator` to spend
    /// [`msg::source()`]'s tokens.
    ///
    /// # Errors
    /// - [`FTError::InsufficientAllowance`] if `operator` doesn't have given
    /// `delta_amount` of an allowance.
    pub fn decrease_allowance(
        &mut self,
        operator: Operator,
        delta_amount: Amount,
    ) -> Result<FTApproval, FTError> {
        let msg_source = msg::source();
        let amount =
            self.change_allowance(msg_source, operator, delta_amount, Amount::checked_sub)?;

        self.internal_approve(msg_source, operator, amount)
    }

    /// Mints to `to` `amount` of tokens.
    ///
    /// # Errors
    /// - [`FTError::ZeroRecipientAddress`] if `to` is [`ActorId::zero()`].
    /// - [`FTError::InsufficientAmount`] if given `amount` with the current
    /// total token supply overflows the [`Amount`] type.
    pub fn mint(&mut self, to: ActorId, amount: Amount) -> Result<FTTransfer, FTError> {
        if to.is_zero() {
            return Err(FTError::ZeroRecipientAddress);
        }

        if let Some(total_supply) = self.total_supply.checked_add(amount) {
            self.total_supply = total_supply;
        } else {
            return Err(FTError::InsufficientAmount);
        }

        self.owners.entry(to).or_default().balance += amount;

        Ok(FTTransfer {
            from: ActorId::zero(),
            to,
            amount,
        })
    }

    /// Burns from `from` `amount` of tokens.
    ///
    /// # Errors
    /// - [`FTError::ZeroSenderAddress`] if `from` is [`ActorId::zero()`].
    /// - [`FTError::InsufficientAmount`] if `from` doesn't have given `amount`
    /// of tokens.
    pub fn burn(&mut self, from: Owner, amount: Amount) -> Result<FTTransfer, FTError> {
        if from.is_zero() {
            return Err(FTError::ZeroSenderAddress);
        }

        self.burn_balance(from, amount)?;
        self.total_supply -= amount;

        Ok(FTTransfer {
            from,
            to: ActorId::zero(),
            amount,
        })
    }
}

/// Fungible token error variants.
#[derive(Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, TypeInfo, Hash)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum FTError {
    /// Token owner doesn't have a sufficient amount of tokens. Or there was the
    /// [`Amount`] overflow during token minting.
    InsufficientAmount,
    /// [`msg::source()`] or operator doesn't have a sufficient allowance of
    /// tokens. Or there was the [`Amount`] overflow during allowance
    /// increasing.
    InsufficientAllowance,
    /// A recipient/operator address is [`ActorId::zero()`].
    ZeroRecipientAddress,
    /// A sender address is [`ActorId::zero()`].
    ZeroSenderAddress,
}

/// The fungible token transfer event.
#[derive(
    Default, Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, TypeInfo, Hash,
)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct FTTransfer {
    /// A sender address.
    ///
    /// It equals [`ActorId::zero()`], if it's retrieved after token minting.
    pub from: ActorId,
    /// A recipient address.
    ///
    /// It equals [`ActorId::zero()`], if it's retrieved after token burning.
    pub to: ActorId,
    pub amount: Amount,
}

/// The fungible token approval event.
#[derive(
    Default, Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, TypeInfo, Hash,
)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct FTApproval {
    pub owner: Owner,
    pub operator: Operator,
    /// An amount of a total token allowance.
    pub amount: Amount,
}

#[cfg(test)]
mod tests {
    use super::*;

    const AMOUNT: u64 = 12345;
    const REMAINDER: u64 = AMOUNT / 2;

    #[test]
    fn mint() {
        let mut state = FTState::new();

        assert_eq!(
            state.mint(1.into(), AMOUNT.into()),
            Ok(FTTransfer {
                from: ActorId::zero(),
                to: 1.into(),
                amount: AMOUNT.into()
            })
        );
        assert_eq!(state.balance_of(1.into()), AMOUNT.into());
        assert_eq!(state.total_supply(), AMOUNT.into());
    }

    #[test]
    fn mint_failures() {
        let mut state = FTState::new();

        assert_eq!(
            state.mint(ActorId::zero(), Amount::default()),
            Err(FTError::ZeroRecipientAddress)
        );

        state.mint(1.into(), 1u64.into()).unwrap();

        assert_eq!(
            state.mint(1.into(), Amount::MAX),
            Err(FTError::InsufficientAmount)
        );
    }

    #[test]
    fn burn() {
        let mut state = FTState::new();

        state.mint(1.into(), AMOUNT.into()).unwrap();

        assert_eq!(
            state.burn(1.into(), (AMOUNT - REMAINDER).into()),
            Ok(FTTransfer {
                from: 1.into(),
                to: ActorId::zero(),
                amount: (AMOUNT - REMAINDER).into()
            })
        );
        assert_eq!(state.balance_of(1.into()), REMAINDER.into());
        assert_eq!(state.total_supply(), REMAINDER.into());
    }

    #[test]
    fn burn_failures() {
        let mut state = FTState::new();

        assert_eq!(
            state.burn(ActorId::zero(), Amount::default()),
            Err(FTError::ZeroSenderAddress)
        );

        state.mint(1.into(), AMOUNT.into()).unwrap();

        assert_eq!(
            state.burn(1.into(), (AMOUNT + 1).into()),
            Err(FTError::InsufficientAmount)
        );
    }

    #[test]
    fn transfer() {
        let mut state = FTState::new();

        state.mint(1.into(), AMOUNT.into()).unwrap();
        msg::set_source(1.into());

        assert_eq!(
            state.transfer(2.into(), REMAINDER.into()),
            Ok(FTTransfer {
                from: 1.into(),
                to: 2.into(),
                amount: REMAINDER.into()
            })
        );
        assert_eq!(state.balance_of(1.into()), (AMOUNT - REMAINDER).into());
        assert_eq!(state.balance_of(2.into()), REMAINDER.into());
    }

    #[test]
    fn transfer_failures() {
        let mut state = FTState::new();

        msg::set_source(1.into());

        assert_eq!(
            state.transfer(ActorId::zero(), Amount::default()),
            Err(FTError::ZeroRecipientAddress)
        );

        state.mint(1.into(), AMOUNT.into()).unwrap();

        assert_eq!(
            state.transfer(2.into(), (AMOUNT + 1).into()),
            Err(FTError::InsufficientAmount)
        );
    }

    #[test]
    fn approve() {
        let mut state = FTState::new();

        msg::set_source(1.into());

        assert_eq!(
            state.approve(2.into(), AMOUNT.into()),
            Ok(FTApproval {
                owner: 1.into(),
                operator: 2.into(),
                amount: AMOUNT.into()
            })
        );
        assert_eq!(state.allowance(1.into(), 2.into()), AMOUNT.into());

        assert_eq!(
            state.increase_allowance(2.into(), AMOUNT.into()),
            Ok(FTApproval {
                owner: 1.into(),
                operator: 2.into(),
                amount: (AMOUNT * 2).into()
            })
        );
        assert_eq!(state.allowance(1.into(), 2.into()), (AMOUNT * 2).into());

        assert_eq!(
            state.decrease_allowance(2.into(), REMAINDER.into()),
            Ok(FTApproval {
                owner: 1.into(),
                operator: 2.into(),
                amount: ((AMOUNT * 2) - REMAINDER).into()
            })
        );
        assert_eq!(
            state.allowance(1.into(), 2.into()),
            ((AMOUNT * 2) - REMAINDER).into()
        );
    }

    #[test]
    fn approve_failures() {
        let mut state = FTState::new();

        assert_eq!(
            state.approve(ActorId::zero(), Amount::default()),
            Err(FTError::ZeroRecipientAddress)
        );

        msg::set_source(1.into());
        state.approve(2.into(), 1u64.into()).unwrap();

        assert_eq!(
            state.increase_allowance(2.into(), Amount::MAX),
            Err(FTError::InsufficientAllowance)
        );
        assert_eq!(
            state.decrease_allowance(2.into(), 2u64.into()),
            Err(FTError::InsufficientAllowance)
        );
    }

    #[test]
    fn transfer_from() {
        let mut state = FTState::new();

        state.mint(1.into(), AMOUNT.into()).unwrap();
        msg::set_source(1.into());
        state.approve(3.into(), AMOUNT.into()).unwrap();
        msg::set_source(3.into());

        assert_eq!(
            state.transfer_from(1.into(), 2.into(), REMAINDER.into()),
            Ok(FTTransfer {
                from: 1.into(),
                to: 2.into(),
                amount: REMAINDER.into()
            })
        );
        assert_eq!(
            state.allowance(1.into(), 3.into()),
            (AMOUNT - REMAINDER).into()
        );
    }

    #[test]
    fn transfer_from_failures() {
        let mut state = FTState::new();

        assert_eq!(
            state.transfer_from(ActorId::zero(), 2.into(), Amount::default()),
            Err(FTError::ZeroSenderAddress)
        );

        msg::set_source(1.into());
        state.approve(2.into(), AMOUNT.into()).unwrap();
        msg::set_source(2.into());

        assert_eq!(
            state.transfer_from(1.into(), 2.into(), (AMOUNT + 1).into()),
            Err(FTError::InsufficientAllowance)
        );
    }
}
