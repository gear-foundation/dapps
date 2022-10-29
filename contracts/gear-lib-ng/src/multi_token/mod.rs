use crate::{Amount, Id, Operator, Owner, StorageProvider};
use cell::Cell;
use gstd::{prelude::*, ActorId};

#[cfg(feature = "testing")]
use crate::testing::msg;
#[cfg(not(feature = "testing"))]
use gstd::msg;

pub use base::*;
pub use extensions::*;

/// The core trait & items.
pub mod base;
/// Core extensions.
pub mod extensions;

/// The default implementation's state.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Default)]
pub struct MTState {
    pub total_supply: BTreeMap<Option<Id>, Amount>,
    pub balances: BTreeMap<(Owner, Option<Id>), Cell<Amount>>,
    pub approvals: BTreeMap<(Owner, Id), BTreeSet<Operator>>,
    pub operators: BTreeMap<Owner, BTreeSet<Operator>>,
}

impl MTState {
    pub fn burn_balances(
        &mut self,
        from: Owner,
        ids_for_amount: &BTreeMap<Id, Amount>,
    ) -> Result<Amount, MTError> {
        let mut total_amount = Amount::default();
        let mut balances_and_amounts = Vec::with_capacity(ids_for_amount.len());

        for (id, amount) in ids_for_amount {
            if let Some((balance, amount, result_amount)) = self
                .balances
                .get(&(from, Some(id.clone())))
                .and_then(|balance| {
                    balance
                        .get()
                        .checked_sub(*amount)
                        .map(|result_amount| (balance, amount, result_amount))
                })
            {
                balances_and_amounts.push((balance, result_amount));
                total_amount += *amount;
            } else {
                return Err(MTError::InsufficientAmount);
            };
        }

        for (balance, amount) in balances_and_amounts {
            balance.set(amount);
        }

        *self.balances.entry((from, None)).or_default().get_mut() -= total_amount;

        Ok(total_amount)
    }
}

/// The default implementation of [`MTCore`] & its extensions.
pub trait MultiToken: StorageProvider<MTState> {
    fn balance_of(&self, owner: Owner, id: Option<Id>) -> Amount {
        self.storage()
            .balances
            .get(&(owner, id))
            .unwrap_or(&Cell::new(Amount::default()))
            .get()
    }

    fn balance_of_batch(
        &self,
        owners_for_ids: BTreeMap<Owner, BTreeSet<Option<Id>>>,
    ) -> BTreeMap<Owner, BTreeSet<(Option<Id>, Amount)>> {
        owners_for_ids
            .into_iter()
            .map(|(owner, ids)| {
                (
                    owner,
                    ids.into_iter()
                        .map(|id| (id.clone(), self.balance_of(owner, id)))
                        .collect(),
                )
            })
            .collect()
    }

    fn total_supply(&self, id: Option<Id>) -> Amount {
        *self
            .storage()
            .total_supply
            .get(&id)
            .unwrap_or(&Amount::default())
    }

    fn allowance(&self, owner: Owner, operator: Operator, id: Option<Id>) -> bool {
        let state = self.storage();

        matches!(state.operators.get(&owner).or_else(|| id.and_then(|id| state.approvals.get(&(owner, id)))), Some(operators) if operators.contains(&operator))
    }

    fn approve(
        &mut self,
        operator: Operator,
        id: Option<Id>,
        approve: bool,
    ) -> Result<(), MTError> {
        if operator.is_zero() {
            return Err(MTError::ZeroRecipientAddress);
        }

        let state = self.storage_mut();
        let msg_source = msg::source();

        let operators = if let Some(ref id) = id {
            state.approvals.entry((msg_source, id.clone())).or_default()
        } else {
            state.operators.entry(msg_source).or_default()
        };

        if approve {
            operators.insert(operator)
        } else {
            operators.remove(&operator)
        };

        self.reply_approval(MTApproval {
            owner: msg_source,
            operator,
            id,
            approved: approve,
        })?;

        Ok(())
    }

    fn transfer(&mut self, to: ActorId, id: Id, amount: Amount) -> Result<(), MTError> {
        self._transfer(msg::source(), to, id, amount)?;

        Ok(())
    }

    fn transfer_from(
        &mut self,
        from: Owner,
        to: ActorId,
        id: Id,
        amount: Amount,
    ) -> Result<(), MTError> {
        self.transfer_from_batch_checks(from, to, &BTreeMap::from([(id.clone(), amount)]))?;
        self._transfer(from, to, id, amount)?;

        Ok(())
    }

    fn _transfer(
        &mut self,
        from: Owner,
        to: ActorId,
        id: Id,
        amount: Amount,
    ) -> Result<(), MTError> {
        self._transfer_batch(from, to, &BTreeMap::from([(id.clone(), amount)]))?;
        self.reply_transfer(MTTransfer {
            from,
            to,
            id,
            amount,
        })?;

        Ok(())
    }

    fn transfer_from_batch_checks(
        &mut self,
        from: Owner,
        to: ActorId,
        ids_for_amount: &BTreeMap<Id, Amount>,
    ) -> Result<(), MTError> {
        if from.is_zero() {
            return Err(MTError::ZeroSenderAddress);
        }

        let state = self.storage_mut();

        if !matches!(state.operators.get(&from), Some(operators) if operators.contains(&to)) {
            for id in ids_for_amount.keys() {
                if !matches!(state.approvals.get(&(from, id.clone())), Some(operators) if operators.contains(&to))
                {
                    return Err(MTError::NotApproved);
                }
            }
        }

        Ok(())
    }

    fn _transfer_batch(
        &mut self,
        from: Owner,
        to: ActorId,
        ids_for_amount: &BTreeMap<Id, Amount>,
    ) -> Result<(), MTError> {
        if to.is_zero() {
            return Err(MTError::ZeroRecipientAddress);
        }

        let state = self.storage_mut();
        let total_amount = state.burn_balances(from, ids_for_amount)?;

        *state.balances.entry((to, None)).or_default().get_mut() += total_amount;

        for (id, amount) in ids_for_amount {
            *state
                .balances
                .entry((to, Some(id.clone())))
                .or_default()
                .get_mut() += *amount;
        }

        Ok(())
    }

    fn transfer_batch_with_reply(
        &mut self,
        from: Owner,
        to: ActorId,
        ids_for_amount: BTreeMap<Id, Amount>,
    ) -> Result<(), MTError> {
        self._transfer_batch(from, to, &ids_for_amount)?;
        self.reply_transfer_batch(MTTransferBatch {
            from,
            to,
            ids_for_amount,
        })?;

        Ok(())
    }

    fn transfer_batch(
        &mut self,
        to: ActorId,
        ids_for_amount: BTreeMap<Id, Amount>,
    ) -> Result<(), MTError> {
        self.transfer_batch_with_reply(msg::source(), to, ids_for_amount)?;

        Ok(())
    }

    fn transfer_from_batch(
        &mut self,
        from: Owner,
        to: ActorId,
        ids_for_amount: BTreeMap<Id, Amount>,
    ) -> Result<(), MTError> {
        self.transfer_from_batch_checks(from, to, &ids_for_amount)?;
        self.transfer_batch_with_reply(from, to, ids_for_amount)?;

        Ok(())
    }

    fn mint(&mut self, to: ActorId, id: Id, amount: Amount) -> Result<(), MTError> {
        self._mint_batch(to, &BTreeMap::from([(id.clone(), amount)]))?;
        self.reply_transfer(MTTransfer {
            from: ActorId::zero(),
            to,
            id,
            amount,
        })?;

        Ok(())
    }

    fn mint_batch(
        &mut self,
        to: ActorId,
        ids_for_amount: BTreeMap<Id, Amount>,
    ) -> Result<(), MTError> {
        self._mint_batch(to, &ids_for_amount)?;
        self.reply_transfer_batch(MTTransferBatch {
            from: ActorId::zero(),
            to,
            ids_for_amount,
        })?;

        Ok(())
    }

    fn _mint_batch(
        &mut self,
        to: ActorId,
        ids_for_amount: &BTreeMap<Id, Amount>,
    ) -> Result<(), MTError> {
        if to.is_zero() {
            return Err(MTError::ZeroRecipientAddress);
        }

        let state = self.storage_mut();
        let state_total_supply = state.total_supply.entry(None).or_default();

        let (total_supply, total_amount) = if let Some(total_supply_and_amount) = ids_for_amount
            .values()
            .try_fold(Amount::default(), |total_amount, amount| {
                total_amount.checked_add(*amount)
            })
            .and_then(|total_amount| {
                total_amount
                    .checked_add(*state_total_supply)
                    .map(|total_supply| (total_supply, total_amount))
            }) {
            total_supply_and_amount
        } else {
            return Err(MTError::InsufficientAmount);
        };

        *state_total_supply = total_supply;
        *state.balances.entry((to, None)).or_default().get_mut() += total_amount;

        for (id, amount) in ids_for_amount {
            *state.total_supply.entry(Some(id.clone())).or_default() += *amount;
            *state
                .balances
                .entry((to, Some(id.clone())))
                .or_default()
                .get_mut() += *amount;
        }

        Ok(())
    }

    fn burn(&mut self, from: Owner, id: Id, amount: Amount) -> Result<(), MTError> {
        self._burn_batch(from, &BTreeMap::from([(id.clone(), amount)]))?;
        self.reply_transfer(MTTransfer {
            from,
            to: ActorId::zero(),
            id,
            amount,
        })?;

        Ok(())
    }

    fn burn_batch(
        &mut self,
        from: Owner,
        ids_for_amount: BTreeMap<Id, Amount>,
    ) -> Result<(), MTError> {
        self._burn_batch(from, &ids_for_amount)?;
        self.reply_transfer_batch(MTTransferBatch {
            from,
            to: ActorId::zero(),
            ids_for_amount,
        })?;

        Ok(())
    }

    fn _burn_batch(
        &mut self,
        from: Owner,
        ids_for_amount: &BTreeMap<Id, Amount>,
    ) -> Result<(), MTError> {
        if from.is_zero() {
            return Err(MTError::ZeroSenderAddress);
        }

        let state = self.storage_mut();
        let total_amount = state.burn_balances(from, ids_for_amount)?;

        for (id, amount) in ids_for_amount {
            *state.total_supply.entry(Some(id.clone())).or_default() -= *amount;
        }

        *state.total_supply.entry(None).or_default() -= total_amount;

        Ok(())
    }

    fn reply_transfer(&self, transfer: MTTransfer) -> Result<(), MTError>;

    fn reply_approval(&self, approval: MTApproval) -> Result<(), MTError>;

    fn reply_transfer_batch(&self, transfer_batch: MTTransferBatch) -> Result<(), MTError>;
}

impl<T: MultiToken> MTCore for T {
    fn balance_of(&self, owner: Owner, id: Option<Id>) -> Amount {
        T::balance_of(self, owner, id)
    }

    fn total_supply(&self, id: Option<Id>) -> Amount {
        T::total_supply(self, id)
    }

    fn allowance(&self, owner: Owner, operator: Operator, id: Option<Id>) -> bool {
        T::allowance(self, owner, operator, id)
    }

    fn approve(
        &mut self,
        operator: Operator,
        id: Option<Id>,
        approve: bool,
    ) -> Result<(), MTError> {
        T::approve(self, operator, id, approve)
    }

    fn transfer(&mut self, to: ActorId, id: Id, amount: Amount) -> Result<(), MTError> {
        T::transfer(self, to, id, amount)
    }

    fn transfer_from(
        &mut self,
        from: Owner,
        to: ActorId,
        id: Id,
        amount: Amount,
    ) -> Result<(), MTError> {
        T::transfer_from(self, from, to, id, amount)
    }
}
