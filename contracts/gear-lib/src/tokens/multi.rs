//! The multi token.

use super::types::{Amount, Id, Operator, Owner};
use gstd::{
    cell::Cell,
    collections::{HashMap, HashSet},
    prelude::*,
    ActorId,
};

#[cfg(test)]
use super::test_helper::msg;
#[cfg(not(test))]
use gstd::msg;

#[derive(Default, Debug, PartialEq, Eq, Clone)]
struct TokenOwnerData {
    allowances: HashMap<Operator, Cell<Amount>>,
    balance: Cell<Amount>,
}

#[derive(Default, Debug, PartialEq, Eq, Clone)]
struct GeneralOwnerData {
    balance: Amount,
    operators: HashSet<Operator>,
}

#[derive(Default, Debug, PartialEq, Eq, Clone)]
struct Token {
    total_supply: Amount,
    owners: HashMap<Owner, TokenOwnerData>,
    attributes: HashMap<Vec<u8>, Vec<u8>>,
}

/// The multi token implementation.
#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub struct MTState {
    tokens: HashMap<Id, Token>,
    owners: HashMap<Owner, GeneralOwnerData>,
    total_supply: Amount,
}

impl MTState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Gets the current total token supply.
    ///
    /// - If `id` is [`Some`], returns the supply of the tokens with this `id`.
    /// - If `id` is [`None`], returns the supply of all tokens.
    pub fn total_supply(&self, id: Option<&Id>) -> Amount {
        id.map_or(self.total_supply, |unwrapped_id| {
            self.tokens
                .get(unwrapped_id)
                .map(|token| token.total_supply)
                .unwrap_or_default()
        })
    }

    /// Returns a balance of `owner`'s tokens.
    ///
    /// - If `id` is [`Some`], returns the balance of the tokens with this `id`.
    /// - If `id` is [`None`], returns the balance of all tokens.
    pub fn balance_of(&self, owner: Owner, id: Option<&Id>) -> Amount {
        id.map_or_else(
            || {
                self.owners
                    .get(&owner)
                    .map(|general_owner_data| general_owner_data.balance)
            },
            |unwrapped_id| {
                self.tokens.get(unwrapped_id).and_then(|token| {
                    token
                        .owners
                        .get(&owner)
                        .map(|token_owner_data| token_owner_data.balance.get())
                })
            },
        )
        .unwrap_or_default()
    }

    /// Mints to `to` `amount` of the tokens with given `id`.
    ///
    /// # Errors
    /// - [`MTError::ZeroRecipientAddress`] if `to` is [`ActorId::zero()`].
    /// - [`MTError::InsufficientAmount`] if given `amount` with the total
    ///   supply of all the tokens overflows the [`Amount`] type.
    pub fn mint(&mut self, to: ActorId, id: Id, amount: Amount) -> Result<MTTransfer, MTError> {
        self.mint_batch(to, vec![(id.clone(), amount)])?;

        Ok(MTTransfer {
            from: ActorId::zero(),
            to,
            id,
            amount,
        })
    }

    fn mint_batch(
        &mut self,
        to: ActorId,
        ids_for_amount: Vec<(Id, Amount)>,
    ) -> Result<MTTransferBatch, MTError> {
        if to.is_zero() {
            return Err(MTError::ZeroRecipientAddress);
        }

        let (total_supply, total_amount) = ids_for_amount
            .iter()
            .try_fold(Amount::default(), |total_amount, (_, amount)| {
                total_amount.checked_add(*amount)
            })
            .and_then(|total_amount| {
                total_amount
                    .checked_add(self.total_supply)
                    .map(|total_supply| (total_supply, total_amount))
            })
            .ok_or(MTError::InsufficientAmount)?;

        self.total_supply = total_supply;
        self.owners.entry(to).or_default().balance += total_amount;

        for (id, amount) in &ids_for_amount {
            let token = self.tokens.entry(id.clone()).or_default();

            token.total_supply += *amount;
            *token.owners.entry(to).or_default().balance.get_mut() += *amount;
        }

        Ok(MTTransferBatch {
            from: ActorId::zero(),
            to,
            ids_for_amount,
        })
    }

    /// Burns from `from` `amount` of the tokens with given `id`.
    ///
    /// # Errors
    /// - [`MTError::ZeroSenderAddress`] if `from` is [`ActorId::zero()`].
    /// - [`MTError::InsufficientAmount`] if `from` doesn't have given `amount`
    ///   of the tokens.
    pub fn burn(&mut self, from: Owner, id: Id, amount: Amount) -> Result<MTTransfer, MTError> {
        self.burn_batch(from, vec![(id.clone(), amount)])?;

        Ok(MTTransfer {
            from,
            to: ActorId::zero(),
            id,
            amount,
        })
    }

    fn burn_batch(
        &mut self,
        from: Owner,
        ids_for_amount: Vec<(Id, Amount)>,
    ) -> Result<MTTransferBatch, MTError> {
        if from.is_zero() {
            return Err(MTError::ZeroSenderAddress);
        }

        let mut balance_entries_and_amounts = Vec::with_capacity(ids_for_amount.len());

        for (id, amount) in &ids_for_amount {
            let balance = &self
                .tokens
                .get(id)
                .and_then(|token| token.owners.get(&from))
                .ok_or(MTError::InsufficientAmount)?
                .balance;

            balance_entries_and_amounts.push((balance, amount));
        }

        let total_amount = Self::burn_balances(
            &mut self.owners,
            from,
            balance_entries_and_amounts.into_iter(),
        )?;

        for (id, amount) in &ids_for_amount {
            self.tokens.get_mut(id).unwrap().total_supply -= *amount;
        }

        self.total_supply -= total_amount;

        Ok(MTTransferBatch {
            from,
            to: ActorId::zero(),
            ids_for_amount,
        })
    }

    fn burn_balances<'slf>(
        owners: &mut HashMap<Owner, GeneralOwnerData>,
        from: Owner,
        balance_entries_and_delta_amounts: impl Iterator<Item = (&'slf Cell<Amount>, &'slf Amount)>,
    ) -> Result<Amount, MTError> {
        let size_hint = balance_entries_and_delta_amounts.size_hint();
        let mut total_amount = Amount::default();
        let mut balance_entries_and_amounts =
            Vec::with_capacity(size_hint.1.unwrap_or(size_hint.0));

        for (balance_entry, amount) in balance_entries_and_delta_amounts {
            let new_amount = balance_entry
                .get()
                .checked_sub(*amount)
                .ok_or(MTError::InsufficientAmount)?;

            balance_entries_and_amounts.push((balance_entry, new_amount));
            total_amount += *amount;
        }

        for (balance_entry, new_amount) in balance_entries_and_amounts {
            balance_entry.set(new_amount);
        }

        owners.get_mut(&from).unwrap().balance -= total_amount;

        Ok(total_amount)
    }

    /// Allows or disallows `operator` to transfer all [`msg::source()`]'s
    /// tokens or only the ones with given `id`.
    ///
    /// # Errors
    /// - [`MTError::ZeroRecipientAddress`] if `operator` is
    ///   [`ActorId::zero()`].
    /// - [`MTError::InsufficientAmount`] if [`msg::source()`] doesn't have any
    ///   tokens or there are no tokens with given `id`.
    pub fn approve(
        &mut self,
        operator: Operator,
        approve: ApproveType,
    ) -> Result<MTApproval, MTError> {
        if operator.is_zero() {
            return Err(MTError::ZeroRecipientAddress);
        }

        let msg_source = msg::source();

        let does_entry_not_exist = match approve {
            ApproveType::Operator(approve_bool) => self
                .owners
                .get_mut(&msg_source)
                .map(|owner| {
                    if approve_bool {
                        owner.operators.insert(operator)
                    } else {
                        owner.operators.remove(&operator)
                    }
                })
                .is_none(),
            ApproveType::Allowance((ref id, amount)) => self
                .tokens
                .get_mut(id)
                .and_then(|token| token.owners.get_mut(&msg_source))
                .map(|owner| owner.allowances.insert(operator, amount.into()))
                .is_none(),
        };

        if does_entry_not_exist {
            Err(MTError::InsufficientAmount)
        } else {
            Ok(MTApproval {
                owner: msg_source,
                operator,
                approved: approve,
            })
        }
    }

    /// Returns an allowance of `owner`'s tokens for `operator`.
    ///
    /// - If `id` is [`Some`], returns an approved amount of the tokens with
    ///   this `id`.
    /// - If `id` is [`None`], returns [`Amount::MAX`] if `operator` is approved
    ///   for all `owner`s tokens, otherwise returns 0.
    pub fn allowance(&self, owner: Owner, operator: Operator, id: Option<&Id>) -> Amount {
        id.map_or_else(
            || {
                self.owners.get(&owner).and_then(|general_owner_data| {
                    general_owner_data
                        .operators
                        .contains(&operator)
                        .then_some(Amount::MAX)
                })
            },
            |unwrapped_id| {
                self.tokens
                    .get(unwrapped_id)
                    .and_then(|token| token.owners.get(&owner))
                    .and_then(|token_owner_data| {
                        token_owner_data.allowances.get(&operator).map(Cell::get)
                    })
            },
        )
        .unwrap_or_default()
    }

    /// Transfers `amount` of the tokens with given `id` from [`msg::source()`]
    /// to `to`.
    ///
    /// # Errors
    /// - [`MTError::ZeroRecipientAddress`] if `to` is [`ActorId::zero()`].
    /// - [`MTError::InsufficientAmount`] if [`msg::source()`] doesn't have
    ///   given `amount` of the tokens.
    pub fn transfer(&mut self, to: ActorId, id: Id, amount: Amount) -> Result<MTTransfer, MTError> {
        self.transfer_batch(to, vec![(id.clone(), amount)])?;

        Ok(MTTransfer {
            from: msg::source(),
            to,
            id,
            amount,
        })
    }

    /// Transfers multiple amounts of tokens with given IDs from
    /// [`msg::source()`] to `to`.
    ///
    /// # Errors
    /// - [`MTError::ZeroRecipientAddress`] if `to` is [`ActorId::zero()`].
    /// - [`MTError::InsufficientAmount`] if [`msg::source()`] doesn't have
    ///   given amount of one of given tokens.
    pub fn transfer_batch(
        &mut self,
        to: ActorId,
        ids_for_amount: Vec<(Id, Amount)>,
    ) -> Result<MTTransferBatch, MTError> {
        if to.is_zero() {
            return Err(MTError::ZeroRecipientAddress);
        }

        let msg_source = msg::source();

        Self::inner_transfer_batch(
            &mut self.owners,
            msg_source,
            to,
            Self::balance_entries_and_amounts(&mut self.tokens, msg_source, to, &ids_for_amount)?,
        )?;

        Ok(MTTransferBatch {
            from: msg_source,
            to,
            ids_for_amount,
        })
    }

    fn balance_entries_and_amounts<'slf>(
        tokens: &'slf mut HashMap<Id, Token>,
        from: Owner,
        to: ActorId,
        ids_for_amount: &'slf [(Id, Amount)],
    ) -> Result<Vec<BalanceEntryPairAndAmount<'slf>>, MTError> {
        for (id, _) in ids_for_amount {
            tokens
                .get_mut(id)
                .and_then(|token| {
                    token.owners.entry(to).or_default();
                    token.owners.get(&from)
                })
                .ok_or(MTError::InsufficientAmount)?;
        }

        let mut bepams = Vec::with_capacity(ids_for_amount.len());

        for (id, amount) in ids_for_amount {
            let owners = &tokens.get(id).unwrap().owners;

            bepams.push(
                (
                    &owners.get(&from).unwrap().balance,
                    &owners.get(&to).unwrap().balance,
                    amount,
                )
                    .into(),
            );
        }

        Ok(bepams)
    }

    /// Transfers `amount` of the tokens with given `id` from `from` to `to`.
    ///
    /// Requires [`msg::source()`] to have an allowance to transfer `amount` or
    /// a larger amount of the `from`'s tokens with given `id`, or to be an
    /// operator of all `from`'s tokens. Note that this function will **not**
    /// work as an equivalent of [`MTState::transfer()`] if [`msg::source()`]
    /// equals `from`.
    ///
    /// If [`msg::source`] is an operator of all `from`'s tokens, this function
    /// will **not** decrease [`msg::source()`]'s allowance for the tokens with
    /// given `id`.
    ///
    /// # Errors
    /// - [`MTError::ZeroSenderAddress`] if `from` is [`ActorId::zero()`].
    /// - [`MTError::ZeroRecipientAddress`] if `to` is [`ActorId::zero()`].
    /// - [`MTError::NotApproved`] if [`msg::source()`] doesn't have any
    ///   allowance for given `amount` of the tokens.
    /// - [`MTError::InsufficientAmount`] if `from` doesn't have given `amount`
    ///   of the tokens.
    pub fn transfer_from(
        &mut self,
        from: Owner,
        to: ActorId,
        id: Id,
        amount: Amount,
    ) -> Result<MTTransfer, MTError> {
        self.transfer_from_batch(from, to, vec![(id.clone(), amount)])?;

        Ok(MTTransfer {
            from,
            to,
            id,
            amount,
        })
    }

    /// Transfers multiple amounts of tokens with given IDs from `from` to `to`.
    ///
    /// Requires [`msg::source()`] to have an allowance to transfer all given
    /// amounts or a larger ones of `from`'s tokens with given IDs, or to be an
    /// operator of all `from`'s tokens. Note that this function will **not**
    /// work as an equivalent of [`MTState::transfer_batch()`] if
    /// [`msg::source()`] equals `from`.
    ///
    /// If [`msg::source`] is an operator of all `from`'s tokens, this function
    /// will **not** decrease [`msg::source()`]'s allowance for tokens with
    /// given IDs.
    ///
    /// # Errors
    /// - [`MTError::ZeroSenderAddress`] if `from` is [`ActorId::zero()`].
    /// - [`MTError::ZeroRecipientAddress`] if `to` is [`ActorId::zero()`].
    /// - [`MTError::NotApproved`] if [`msg::source()`] doesn't have any
    ///   allowance for given amounts of tokens.
    /// - [`MTError::InsufficientAmount`] if `from` doesn't have given amount of
    ///   one of given tokens.
    pub fn transfer_from_batch(
        &mut self,
        from: Owner,
        to: ActorId,
        ids_for_amount: Vec<(Id, Amount)>,
    ) -> Result<MTTransferBatch, MTError> {
        if from.is_zero() {
            return Err(MTError::ZeroSenderAddress);
        }

        if to.is_zero() {
            return Err(MTError::ZeroRecipientAddress);
        }

        let Some(owner) = self.owners.get(&from) else {
            return Err(MTError::InsufficientAmount);
        };

        let msg_source = msg::source();

        let (balance_entries_and_amounts, allowances_and_amounts) =
            if owner.operators.contains(&msg_source) {
                (
                    Self::balance_entries_and_amounts(&mut self.tokens, from, to, &ids_for_amount)?,
                    vec![],
                )
            } else {
                for (id, _) in &ids_for_amount {
                    self.tokens
                        .get_mut(id)
                        .and_then(|token| {
                            token.owners.entry(to).or_default();
                            token.owners.get(&from)
                        })
                        .ok_or(MTError::InsufficientAmount)?;
                }

                let mut bepams = Vec::with_capacity(ids_for_amount.len());
                let mut allowances_and_amounts = Vec::with_capacity(ids_for_amount.len());

                for (id, amount) in &ids_for_amount {
                    let owners = &self.tokens.get(id).unwrap().owners;
                    let from_owner = owners.get(&from).unwrap();
                    let allowance_and_amount = from_owner
                        .allowances
                        .get(&msg_source)
                        .and_then(|allowance_entry| {
                            allowance_entry
                                .get()
                                .checked_sub(*amount)
                                .map(|new_allowance| (allowance_entry, new_allowance))
                        })
                        .ok_or(MTError::NotApproved)?;
                    let to_balance_entry = &owners.get(&to).unwrap().balance;

                    allowances_and_amounts.push(allowance_and_amount);
                    bepams.push((&from_owner.balance, to_balance_entry, amount).into());
                }

                (bepams, allowances_and_amounts)
            };

        Self::inner_transfer_batch(&mut self.owners, from, to, balance_entries_and_amounts)?;

        for (allowance_entry, new_allowance) in allowances_and_amounts {
            allowance_entry.set(new_allowance);
        }

        Ok(MTTransferBatch {
            from,
            to,
            ids_for_amount,
        })
    }

    fn inner_transfer_batch(
        owners: &mut HashMap<Owner, GeneralOwnerData>,
        from: Owner,
        to: ActorId,
        bepams: Vec<BalanceEntryPairAndAmount<'_>>,
    ) -> Result<(), MTError> {
        let total_amount = Self::burn_balances(
            owners,
            from,
            bepams
                .iter()
                .map(|bepam| (bepam.from_balance_entry, bepam.amount)),
        )?;

        owners.entry(to).or_default().balance += total_amount;

        for bepam in bepams {
            bepam
                .to_balance_entry
                .set(bepam.to_balance_entry.get() + bepam.amount);
        }

        Ok(())
    }

    /// Gets an attribute with given `key` for the tokens with given `id`.
    ///
    /// Returns [`None`] if an attribute with given `key` doesn't exist.
    ///
    /// To set an attribute, use [`MTState::set_attribute()`].
    ///
    /// # Errors
    /// - [`MTError::InsufficientAmount`] if there are no tokens with given
    ///   `id`.
    pub fn get_attribute(&self, id: &Id, key: &Vec<u8>) -> Result<Option<&Vec<u8>>, MTError> {
        self.tokens
            .get(id)
            .ok_or(MTError::InsufficientAmount)
            .map(|token| token.attributes.get(key))
    }

    /// Sets an attribute with given `key` & `value` for the tokens with given
    /// `id`.
    ///
    /// Returns [`None`] if an attribute with given `key` doesn't exist,
    /// otherwise replaces the old value with given one and returns [`Some`]
    /// with the old one.
    ///
    /// Note that it's impossible to remove a set attribute, only overwrite it.
    ///
    /// To get an attribute, use [`MTState::get_attribute()`].
    ///
    /// # Errors
    /// - [`MTError::InsufficientAmount`] if there are no tokens with given
    ///   `id`.
    pub fn set_attribute(
        &mut self,
        id: &Id,
        key: Vec<u8>,
        value: Vec<u8>,
    ) -> Result<Option<Vec<u8>>, MTError> {
        self.tokens
            .get_mut(id)
            .ok_or(MTError::InsufficientAmount)
            .map(|token| token.attributes.insert(key, value))
    }
}

struct BalanceEntryPairAndAmount<'state> {
    from_balance_entry: &'state Cell<Amount>,
    to_balance_entry: &'state Cell<Amount>,
    amount: &'state Amount,
}

impl<'state> From<(&'state Cell<Amount>, &'state Cell<Amount>, &'state Amount)>
    for BalanceEntryPairAndAmount<'state>
{
    fn from(value: (&'state Cell<Amount>, &'state Cell<Amount>, &'state Amount)) -> Self {
        Self {
            from_balance_entry: value.0,
            to_balance_entry: value.1,
            amount: value.2,
        }
    }
}

/// The approval types.
#[derive(Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, TypeInfo, Hash)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum ApproveType {
    /// Approval for all tokens.
    Operator(bool),
    /// Approval for specific amount of the tokens with specific ID.
    Allowance((Id, Amount)),
}

/// The multi token transfer event.
#[derive(Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, TypeInfo, Hash)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct MTTransfer {
    /// A sender address.
    ///
    /// It equals [`ActorId::zero()`], if it's retrieved after token minting.
    pub from: ActorId,
    /// A recipient address.
    ///
    /// It equals [`ActorId::zero()`], if it's retrieved after token burning.
    pub to: ActorId,
    /// A tokens ID.
    pub id: Id,
    pub amount: Amount,
}

/// The multi token batch transfer event.
#[derive(Default, Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, TypeInfo, Hash)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct MTTransferBatch {
    /// A sender address.
    pub from: ActorId,
    /// A recipient address.
    pub to: ActorId,
    /// Pairs of a tokens ID & token amount.
    pub ids_for_amount: Vec<(Id, Amount)>,
}

/// The multi token approval event.
#[derive(Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, TypeInfo, Hash)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct MTApproval {
    pub owner: ActorId,
    pub operator: Operator,
    pub approved: ApproveType,
}

/// Multi token error variants.
#[derive(Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, TypeInfo, Hash)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum MTError {
    /// [`msg::source()`] doesn't have allowance to transfer tokens with given
    /// IDs.
    NotApproved,
    /// A recipient/operator address is [`ActorId::zero()`].
    ZeroRecipientAddress,
    /// A sender address is [`ActorId::zero()`].
    ZeroSenderAddress,
    /// Token owner doesn't have a sufficient amount of tokens. Or there was the
    /// [`Amount`] overflow during token minting/burning.
    InsufficientAmount,
}

#[cfg(test)]
mod tests {
    use super::*;

    const AMOUNT: u64 = 12345;
    const REMAINDER: u64 = AMOUNT / 2;

    #[test]
    fn meta() {
        let key: Vec<_> = "Name".into();
        let data: Vec<_> = "Nuclear Fish Tank".into();
        let mut state = MTState::new();

        assert_eq!(
            state.set_attribute(&0u8.into(), key.clone(), data.clone()),
            Err(MTError::InsufficientAmount)
        );

        state.mint(1.into(), 0u8.into(), 0u64.into()).unwrap();

        assert_eq!(
            state.set_attribute(&0u8.into(), key.clone(), "NFT".into()),
            Ok(None)
        );
        assert_eq!(
            state.set_attribute(&0u8.into(), key.clone(), data.clone()),
            Ok(Some("NFT".into()))
        );
        assert_eq!(state.get_attribute(&0u8.into(), &key), Ok(Some(&data)));
        assert_eq!(
            state.get_attribute(&0u8.into(), &"Nonexistent attribute".into()),
            Ok(None)
        );
        assert_eq!(
            state.get_attribute(&1u8.into(), &key),
            Err(MTError::InsufficientAmount)
        );
    }

    #[test]
    fn mint() {
        let mut state = MTState::new();

        assert_eq!(
            state.mint(1.into(), 0u8.into(), AMOUNT.into()),
            Ok(MTTransfer {
                from: ActorId::zero(),
                to: 1.into(),
                id: 0u8.into(),
                amount: AMOUNT.into()
            })
        );
        assert_eq!(state.balance_of(1.into(), Some(&0u8.into())), AMOUNT.into());
        assert_eq!(state.balance_of(1.into(), None), AMOUNT.into());
        assert_eq!(state.total_supply(Some(&0u8.into())), AMOUNT.into());
        assert_eq!(state.total_supply(None), AMOUNT.into());
    }

    #[test]
    fn mint_failures() {
        let mut state = MTState::new();

        assert_eq!(
            state.mint(ActorId::zero(), 0u8.into(), Amount::default()),
            Err(MTError::ZeroRecipientAddress)
        );

        state.mint(1.into(), 0u8.into(), 1u64.into()).unwrap();

        assert_eq!(
            state.mint(1.into(), 0u8.into(), Amount::MAX),
            Err(MTError::InsufficientAmount)
        );
    }

    #[test]
    fn burn() {
        let mut state = MTState::new();

        state.mint(1.into(), 0u8.into(), AMOUNT.into()).unwrap();

        assert_eq!(
            state.burn(1.into(), 0u8.into(), (AMOUNT - REMAINDER).into()),
            Ok(MTTransfer {
                from: 1.into(),
                to: ActorId::zero(),
                id: 0u8.into(),
                amount: (AMOUNT - REMAINDER).into()
            })
        );
        assert_eq!(
            state.balance_of(1.into(), Some(&0u8.into())),
            REMAINDER.into()
        );
        assert_eq!(state.balance_of(1.into(), None), REMAINDER.into());
        assert_eq!(state.total_supply(Some(&0u8.into())), REMAINDER.into());
        assert_eq!(state.total_supply(None), REMAINDER.into());
    }

    #[test]
    fn burn_failures() {
        let mut state = MTState::new();

        assert_eq!(
            state.burn(ActorId::zero(), 0u8.into(), Amount::default()),
            Err(MTError::ZeroSenderAddress)
        );

        state.mint(1.into(), 0u8.into(), AMOUNT.into()).unwrap();

        assert_eq!(
            state.burn(1.into(), 0u8.into(), (AMOUNT + 1).into()),
            Err(MTError::InsufficientAmount)
        );
    }

    #[test]
    fn transfer() {
        let mut state = MTState::new();

        state.mint(1.into(), 0u8.into(), AMOUNT.into()).unwrap();
        msg::set_source(1.into());

        assert_eq!(
            state.transfer(2.into(), 0u8.into(), REMAINDER.into()),
            Ok(MTTransfer {
                from: 1.into(),
                to: 2.into(),
                id: 0u8.into(),
                amount: REMAINDER.into()
            })
        );
        assert_eq!(
            state.balance_of(1.into(), Some(&0u8.into())),
            (AMOUNT - REMAINDER).into()
        );
        assert_eq!(
            state.balance_of(1.into(), None),
            (AMOUNT - REMAINDER).into()
        );
        assert_eq!(
            state.balance_of(2.into(), Some(&0u8.into())),
            REMAINDER.into()
        );
        assert_eq!(state.balance_of(2.into(), None), REMAINDER.into());
    }

    #[test]
    fn transfer_failures() {
        let mut state = MTState::new();

        msg::set_source(1.into());

        assert_eq!(
            state.transfer(ActorId::zero(), 0u8.into(), Amount::default()),
            Err(MTError::ZeroRecipientAddress)
        );
        assert_eq!(
            state.transfer(2.into(), 0u8.into(), Amount::default()),
            Err(MTError::InsufficientAmount)
        );

        state.mint(1.into(), 0u8.into(), AMOUNT.into()).unwrap();

        assert_eq!(
            state.transfer(2.into(), 0u8.into(), (AMOUNT + 1).into()),
            Err(MTError::InsufficientAmount)
        );
    }

    #[test]
    fn approve() {
        let mut state = MTState::new();

        state.mint(1.into(), 0u8.into(), 1u8.into()).unwrap();

        assert_eq!(
            state.allowance(1.into(), 2.into(), Some(&0u8.into())),
            0u8.into()
        );
        assert_eq!(state.allowance(1.into(), 2.into(), None), 0u8.into());

        let mut approved = ApproveType::Allowance((0u8.into(), AMOUNT.into()));

        msg::set_source(1.into());
        assert_eq!(
            state.approve(2.into(), approved.clone()),
            Ok(MTApproval {
                owner: 1.into(),
                operator: 2.into(),
                approved
            })
        );
        assert_eq!(
            state.allowance(1.into(), 2.into(), Some(&0u8.into())),
            AMOUNT.into()
        );
        assert_eq!(state.allowance(1.into(), 2.into(), None), 0u8.into());

        approved = ApproveType::Operator(true);

        assert_eq!(
            state.approve(2.into(), approved.clone()),
            Ok(MTApproval {
                owner: 1.into(),
                operator: 2.into(),
                approved
            })
        );
        assert_eq!(
            state.allowance(1.into(), 2.into(), Some(&0u8.into())),
            AMOUNT.into()
        );
        assert_eq!(state.allowance(1.into(), 2.into(), None), Amount::MAX);

        approved = ApproveType::Allowance((0u8.into(), REMAINDER.into()));

        assert_eq!(
            state.approve(2.into(), approved.clone()),
            Ok(MTApproval {
                owner: 1.into(),
                operator: 2.into(),
                approved
            })
        );
        assert_eq!(
            state.allowance(1.into(), 2.into(), Some(&0u8.into())),
            REMAINDER.into()
        );
        assert_eq!(state.allowance(1.into(), 2.into(), None), Amount::MAX);
    }

    #[test]
    fn approve_failures() {
        let mut state = MTState::new();

        assert_eq!(
            state.approve(ActorId::zero(), ApproveType::Operator(true)),
            Err(MTError::ZeroRecipientAddress)
        );
        assert_eq!(
            state.approve(1.into(), ApproveType::Operator(true)),
            Err(MTError::InsufficientAmount)
        );
    }

    #[test]
    fn transfer_from() {
        let mut state = MTState::new();

        state
            .mint(1.into(), 0u8.into(), (AMOUNT + REMAINDER).into())
            .unwrap();
        msg::set_source(1.into());
        state
            .approve(
                3.into(),
                ApproveType::Allowance((0u8.into(), AMOUNT.into())),
            )
            .unwrap();
        msg::set_source(3.into());

        assert_eq!(
            state.transfer_from(1.into(), 2.into(), 0u8.into(), AMOUNT.into()),
            Ok(MTTransfer {
                from: 1.into(),
                to: 2.into(),
                id: 0u8.into(),
                amount: AMOUNT.into()
            })
        );
        assert_eq!(
            state.allowance(1.into(), 3.into(), Some(&0u8.into())),
            0u8.into()
        );

        msg::set_source(1.into());
        state
            .approve(3.into(), ApproveType::Operator(true))
            .unwrap();
        msg::set_source(3.into());

        assert_eq!(
            state.transfer_from(1.into(), 2.into(), 0u8.into(), REMAINDER.into()),
            Ok(MTTransfer {
                from: 1.into(),
                to: 2.into(),
                id: 0u8.into(),
                amount: REMAINDER.into()
            })
        );
        assert_eq!(state.allowance(1.into(), 3.into(), None), Amount::MAX);
        assert_eq!(state.balance_of(1.into(), Some(&0u8.into())), 0u64.into());
        assert_eq!(state.balance_of(1.into(), None), 0u64.into());
        assert_eq!(
            state.balance_of(2.into(), Some(&0u8.into())),
            (AMOUNT + REMAINDER).into()
        );
        assert_eq!(
            state.balance_of(2.into(), None),
            (AMOUNT + REMAINDER).into()
        );
    }

    #[test]
    fn transfer_from_failures() {
        let mut state = MTState::new();

        assert_eq!(
            state.transfer_from(ActorId::zero(), 1.into(), 0u8.into(), Amount::default()),
            Err(MTError::ZeroSenderAddress)
        );
        assert_eq!(
            state.transfer_from(1.into(), ActorId::zero(), 0u8.into(), Amount::default()),
            Err(MTError::ZeroRecipientAddress)
        );
        assert_eq!(
            state.transfer_from(1.into(), 2.into(), 0u8.into(), Amount::default()),
            Err(MTError::InsufficientAmount)
        );

        state.mint(1.into(), 0u8.into(), 1u64.into()).unwrap();
        msg::set_source(3.into());

        assert_eq!(
            state.transfer_from(1.into(), 2.into(), 0u8.into(), Amount::default()),
            Err(MTError::NotApproved)
        );
        assert_eq!(
            state.transfer_from(1.into(), 2.into(), 2u8.into(), Amount::default()),
            Err(MTError::InsufficientAmount)
        );

        msg::set_source(1.into());
        state
            .approve(3.into(), ApproveType::Allowance((0u8.into(), 2u64.into())))
            .unwrap();
        msg::set_source(3.into());

        assert_eq!(
            state.transfer_from(1.into(), 2.into(), 0u8.into(), 2u64.into()),
            Err(MTError::InsufficientAmount)
        );

        msg::set_source(1.into());
        state
            .approve(3.into(), ApproveType::Operator(true))
            .unwrap();
        msg::set_source(3.into());

        assert_eq!(
            state.transfer_from(1.into(), 2.into(), 1u8.into(), Amount::default()),
            Err(MTError::InsufficientAmount)
        );
    }

    #[test]
    fn transfer_batch() {
        let mut state = MTState::new();

        state.mint(1.into(), 0u8.into(), AMOUNT.into()).unwrap();
        state
            .mint(1.into(), 1u8.into(), (AMOUNT + REMAINDER).into())
            .unwrap();
        msg::set_source(1.into());

        let ids_for_amount = vec![
            (0u8.into(), (AMOUNT - REMAINDER).into()),
            (1u8.into(), AMOUNT.into()),
        ];

        assert_eq!(
            state.transfer_batch(2.into(), ids_for_amount.clone()),
            Ok(MTTransferBatch {
                from: 1.into(),
                to: 2.into(),
                ids_for_amount
            })
        );

        assert_eq!(
            state.balance_of(1.into(), Some(&0u8.into())),
            REMAINDER.into()
        );
        assert_eq!(
            state.balance_of(1.into(), Some(&1u8.into())),
            REMAINDER.into()
        );
        assert_eq!(state.balance_of(1.into(), None), (REMAINDER * 2).into());

        assert_eq!(
            state.balance_of(2.into(), Some(&0u8.into())),
            (AMOUNT - REMAINDER).into()
        );
        assert_eq!(state.balance_of(2.into(), Some(&1u8.into())), AMOUNT.into());
        assert_eq!(
            state.balance_of(2.into(), None),
            ((AMOUNT - REMAINDER) + AMOUNT).into()
        );
    }

    #[test]
    fn transfer_batch_failures() {
        let mut state = MTState::new();

        state.mint(1.into(), 0u8.into(), 1u64.into()).unwrap();
        msg::set_source(1.into());

        assert_eq!(
            state.transfer_batch(
                2.into(),
                vec![
                    (0u8.into(), Amount::default()),
                    (1u8.into(), Amount::default())
                ]
            ),
            Err(MTError::InsufficientAmount)
        );

        state.mint(1.into(), 1u8.into(), 1u64.into()).unwrap();

        assert_eq!(
            state.transfer_batch(
                2.into(),
                vec![(0u8.into(), 1u64.into()), (1u8.into(), 2u64.into())]
            ),
            Err(MTError::InsufficientAmount)
        );
    }

    #[test]
    fn transfer_from_batch() {
        let mut state = MTState::new();

        state
            .mint(1.into(), 0u8.into(), (AMOUNT * 2 + REMAINDER).into())
            .unwrap();
        state
            .mint(1.into(), 1u8.into(), (AMOUNT + REMAINDER * 2).into())
            .unwrap();
        msg::set_source(1.into());
        state
            .approve(
                3.into(),
                ApproveType::Allowance((0u8.into(), AMOUNT.into())),
            )
            .unwrap();
        state
            .approve(
                3.into(),
                ApproveType::Allowance((1u8.into(), REMAINDER.into())),
            )
            .unwrap();
        msg::set_source(3.into());

        let mut ids_for_amount = vec![(0u8.into(), AMOUNT.into()), (1u8.into(), REMAINDER.into())];

        assert_eq!(
            state.transfer_from_batch(1.into(), 2.into(), ids_for_amount.clone()),
            Ok(MTTransferBatch {
                from: 1.into(),
                to: 2.into(),
                ids_for_amount,
            })
        );
        assert_eq!(
            state.allowance(1.into(), 3.into(), Some(&0u8.into())),
            0u8.into()
        );
        assert_eq!(
            state.allowance(1.into(), 3.into(), Some(&1u8.into())),
            0u8.into()
        );

        msg::set_source(1.into());
        state
            .approve(3.into(), ApproveType::Operator(true))
            .unwrap();
        msg::set_source(3.into());

        ids_for_amount = vec![(0u8.into(), REMAINDER.into()), (1u8.into(), AMOUNT.into())];

        assert_eq!(
            state.transfer_from_batch(1.into(), 2.into(), ids_for_amount.clone()),
            Ok(MTTransferBatch {
                from: 1.into(),
                to: 2.into(),
                ids_for_amount
            })
        );
        assert_eq!(state.allowance(1.into(), 3.into(), None), Amount::MAX);

        ids_for_amount = vec![];

        assert_eq!(
            state.transfer_from_batch(1.into(), 2.into(), ids_for_amount.clone()),
            Ok(MTTransferBatch {
                from: 1.into(),
                to: 2.into(),
                ids_for_amount
            })
        );

        assert_eq!(state.balance_of(1.into(), Some(&0u8.into())), AMOUNT.into());
        assert_eq!(
            state.balance_of(1.into(), Some(&1u8.into())),
            REMAINDER.into()
        );
        assert_eq!(
            state.balance_of(1.into(), None),
            (AMOUNT + REMAINDER).into()
        );

        assert_eq!(
            state.balance_of(2.into(), Some(&0u8.into())),
            (AMOUNT + REMAINDER).into()
        );
        assert_eq!(
            state.balance_of(2.into(), Some(&1u8.into())),
            (AMOUNT + REMAINDER).into()
        );
        assert_eq!(
            state.balance_of(2.into(), None),
            ((AMOUNT + REMAINDER) * 2).into()
        );
    }

    #[test]
    fn transfer_from_batch_failures() {
        let mut state = MTState::new();

        assert_eq!(
            state.transfer_from_batch(ActorId::zero(), 1.into(), vec![]),
            Err(MTError::ZeroSenderAddress)
        );
        assert_eq!(
            state.transfer_from_batch(1.into(), ActorId::zero(), vec![]),
            Err(MTError::ZeroRecipientAddress)
        );
        assert_eq!(
            state.transfer_from_batch(1.into(), 2.into(), vec![]),
            Err(MTError::InsufficientAmount)
        );

        state.mint(1.into(), 0u8.into(), 1u64.into()).unwrap();
        msg::set_source(3.into());

        assert_eq!(
            state.transfer_from_batch(
                1.into(),
                2.into(),
                vec![
                    (0u8.into(), Amount::default()),
                    (1u8.into(), Amount::default())
                ]
            ),
            Err(MTError::InsufficientAmount)
        );

        state.mint(1.into(), 1u8.into(), 1u64.into()).unwrap();
        msg::set_source(1.into());
        state
            .approve(3.into(), ApproveType::Allowance((0u8.into(), 1u64.into())))
            .unwrap();
        msg::set_source(3.into());

        assert_eq!(
            state.transfer_from_batch(
                1.into(),
                2.into(),
                vec![(0u8.into(), 1u64.into()), (1u8.into(), 1u64.into())]
            ),
            Err(MTError::NotApproved)
        );

        msg::set_source(1.into());
        state
            .approve(3.into(), ApproveType::Operator(true))
            .unwrap();
        msg::set_source(3.into());

        assert_eq!(
            state.transfer_from_batch(
                1.into(),
                2.into(),
                vec![(0u8.into(), 1u64.into()), (1u8.into(), 2u64.into())]
            ),
            Err(MTError::InsufficientAmount)
        );
    }
}
