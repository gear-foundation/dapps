use crate::{Amount, Id, Operator, Owner, StorageProvider};
use gstd::{prelude::*, ActorId};

#[cfg(feature = "testing")]
use crate::testing::msg;
#[cfg(not(feature = "testing"))]
use gstd::msg;

pub use base::*;
pub use extensions::*;

/// Core traits & items.
pub mod base;
/// Core extensions.
pub mod extensions;

/// The default implementation's state.
#[derive(Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, TypeInfo, Default)]
pub struct NFTState {
    pub owners: BTreeMap<Id, Owner>,
    pub approvals: BTreeMap<Id, BTreeSet<Operator>>,
    pub operators: BTreeMap<Owner, BTreeSet<Operator>>,
    pub balances: BTreeMap<Owner, Amount>,
    pub total_supply: Amount,
    pub attributes: BTreeMap<Id, BTreeMap<Vec<u8>, Vec<u8>>>,
}

impl NFTState {
    pub fn owner_of(&self, id: &Id) -> Result<&Owner, NFTError> {
        self.owners.get(id).ok_or(NFTError::TokenNotExists)
    }

    pub fn allowance(&self, owner: Owner, operator: Operator, id: Option<Id>) -> bool {
        Self::_allowance(&self.operators, &self.approvals, owner, operator, id)
    }

    pub fn _allowance(
        operators: &BTreeMap<Owner, BTreeSet<Operator>>,
        approvals: &BTreeMap<Id, BTreeSet<Operator>>,
        owner: Owner,
        operator: Operator,
        id: Option<Id>,
    ) -> bool {
        matches!(operators.get(&owner).or_else(|| id.and_then(|id| approvals.get(&id))), Some(operators) if operators.contains(&operator))
    }

    pub fn balance_of_mut(&mut self, owner: Owner) -> &mut Amount {
        self.balances.entry(owner).or_default()
    }

    pub fn increment_balance(&mut self, owner: Owner) {
        *self.balance_of_mut(owner) += Amount::one();
    }

    pub fn decrement_balance(&mut self, owner: Owner) {
        *self.balance_of_mut(owner) -= Amount::one();
    }
}

/// The default implementation of [`NFTCore`] & its extensions.
pub trait NonFungibleToken: StorageProvider<NFTState> {
    fn balance_of(&self, owner: Owner) -> Amount {
        *self
            .storage()
            .balances
            .get(&owner)
            .unwrap_or(&Amount::default())
    }

    fn owner_of(&self, id: Id) -> Owner {
        *self.storage().owners.get(&id).unwrap_or(&ActorId::zero())
    }

    fn allowance(&self, owner: Owner, operator: Operator, id: Option<Id>) -> bool {
        self.storage().allowance(owner, operator, id)
    }

    fn total_supply(&self) -> Amount {
        self.storage().total_supply
    }

    fn transfer(&mut self, to: ActorId, id: Id) -> Result<(), NFTError> {
        if to.is_zero() {
            return Err(NFTError::ZeroRecipientAddress);
        }

        let state = self.storage_mut();
        let owner = if let Some(owner) = state.owners.get_mut(&id) {
            owner
        } else {
            return Err(NFTError::TokenNotExists);
        };

        let msg_source = msg::source();

        if *owner != msg_source
            && !match state.approvals.get_mut(&id) {
                Some(operators) => operators.remove(&msg_source),
                None => NFTState::_allowance(
                    &state.operators,
                    &state.approvals,
                    *owner,
                    msg_source,
                    None,
                ),
            }
        {
            return Err(NFTError::NotApproved);
        }

        let from = *owner;
        *owner = to;

        state.decrement_balance(from);
        state.increment_balance(to);

        self.reply_transfer(NFTTransfer { from, to, id })?;

        Ok(())
    }

    fn reply_transfer(&self, transfer: NFTTransfer) -> Result<(), NFTError>;

    fn reply_approval(&self, approval: NFTApproval) -> Result<(), NFTError>;

    fn approve(
        &mut self,
        operator: Operator,
        id: Option<Id>,
        approve: bool,
    ) -> Result<(), NFTError> {
        if operator.is_zero() {
            return Err(NFTError::ZeroRecipientAddress);
        }

        let state = self.storage_mut();
        let msg_source = msg::source();
        let mut owner = msg_source;

        let operators = if let Some(id) = &id {
            let current_owner = *state.owner_of(id)?;

            if current_owner != msg_source {
                if !state.allowance(current_owner, msg_source, None) {
                    return Err(NFTError::NotApproved);
                }

                owner = current_owner
            }

            state.approvals.entry(id.clone()).or_default()
        } else {
            state.operators.entry(msg_source).or_default()
        };

        if approve {
            operators.insert(operator)
        } else {
            operators.remove(&operator)
        };

        self.reply_approval(NFTApproval {
            owner,
            operator,
            id,
            approved: approve,
        })?;

        Ok(())
    }

    fn mint(&mut self, to: ActorId, id: Id) -> Result<(), NFTError> {
        if to.is_zero() {
            return Err(NFTError::ZeroRecipientAddress);
        }

        let state = self.storage_mut();

        if state.total_supply == Amount::MAX {
            return Err(NFTError::TokenNotExists);
        }

        if state.owners.contains_key(&id) {
            return Err(NFTError::TokenExists);
        }

        state.increment_balance(to);
        state.owners.insert(id.clone(), to);

        state.total_supply += Amount::one();

        self.reply_transfer(NFTTransfer {
            from: ActorId::zero(),
            to,
            id,
        })?;

        Ok(())
    }

    fn burn(&mut self, from: Owner, id: Id) -> Result<(), NFTError> {
        if from.is_zero() {
            return Err(NFTError::ZeroSenderAddress);
        }

        let state = self.storage_mut();

        state.owner_of(&id)?;
        state.owners.remove(&id);
        state.decrement_balance(from);

        state.total_supply -= Amount::one();

        self.reply_transfer(NFTTransfer {
            from,
            to: ActorId::zero(),
            id,
        })?;

        Ok(())
    }

    fn get_attribute(&self, id: Id, key: Vec<u8>) -> Option<Vec<u8>> {
        self.storage()
            .attributes
            .get(&id)
            .and_then(|attributes| attributes.get(&key))
            .cloned()
    }

    fn set_attribute(&mut self, id: Id, key: Vec<u8>, data: Vec<u8>) -> Result<(), NFTError> {
        self.storage_mut()
            .attributes
            .entry(id.clone())
            .or_default()
            .insert(key.clone(), data.clone());

        self.reply_set_attribute(NFTAttribute { id, key, data })?;

        Ok(())
    }

    fn reply_set_attribute(&self, attribute: NFTAttribute) -> Result<(), NFTError>;
}

impl<T: NonFungibleToken> NFTCore for T {
    fn balance_of(&self, owner: Owner) -> Amount {
        T::balance_of(self, owner)
    }

    fn owner_of(&self, id: Id) -> Owner {
        T::owner_of(self, id)
    }

    fn allowance(&self, owner: Owner, operator: Operator, id: Option<Id>) -> bool {
        T::allowance(self, owner, operator, id)
    }

    fn approve(
        &mut self,
        operator: Operator,
        id: Option<Id>,
        approve: bool,
    ) -> Result<(), NFTError> {
        T::approve(self, operator, id, approve)
    }

    fn transfer(&mut self, to: ActorId, id: Id) -> Result<(), NFTError> {
        T::transfer(self, to, id)
    }

    fn total_supply(&self) -> Amount {
        T::total_supply(self)
    }
}

impl<T: NonFungibleToken> NFTMeta for T {
    fn get_attribute(&self, id: Id, key: Vec<u8>) -> Option<Vec<u8>> {
        T::get_attribute(self, id, key)
    }
}
