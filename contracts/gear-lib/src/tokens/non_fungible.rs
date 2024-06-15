//! The non-fungible token.

use super::types::{Amount, Id, Operator, Owner};
use gstd::{
    collections::{HashMap, HashSet},
    prelude::*,
    ActorId,
};

pub mod encodable;

#[cfg(test)]
use super::test_helper::msg;
#[cfg(not(test))]
use gstd::msg;

#[derive(Default, Debug, PartialEq, Eq, Clone)]
struct Token {
    owner: Owner,
    approvals: HashSet<Operator>,
    attributes: HashMap<Vec<u8>, Vec<u8>>,
}

#[derive(Default, Debug, PartialEq, Eq, Clone)]
struct TokenOwner {
    operators: HashSet<Operator>,
    balance: Amount,
}

/// The non-fungible token implementation.
#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub struct NFTState {
    owners: HashMap<Owner, TokenOwner>,
    tokens: HashMap<Id, Token>,
    total_supply: Amount,
}

impl NFTState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Gets the current total token supply.
    pub const fn total_supply(&self) -> Amount {
        self.total_supply
    }

    /// Returns a balance of `owner`'s tokens.
    pub fn balance_of(&self, owner: Owner) -> Amount {
        self.owners
            .get(&owner)
            .map(|token_owner| token_owner.balance)
            .unwrap_or_default()
    }

    /// Returns [`Owner`] of the token with given `id`.
    pub fn owner_of(&self, id: &Id) -> Owner {
        self.internal_owner_of(id).unwrap_or_default()
    }

    fn internal_owner_of(&self, id: &Id) -> Result<Owner, NFTError> {
        self.tokens
            .get(id)
            .map(|token| token.owner)
            .ok_or(NFTError::TokenNotExists)
    }

    /// Returns [`true`] if `operator` is allowed to transfer all `owner`'s
    /// tokens or the token with given `id`.
    ///
    /// - If `id` is [`Some`], firstly checks if `operator` is allowed for all
    ///   `owner`'s tokens, and if not, whether `operator` is allowed for the
    ///   token with this `id`.
    /// - If `id` is [`None`], only checks if `operator` is allowed for all
    ///   `owner`'s tokens.
    pub fn allowance(&self, owner: Owner, operator: Operator, id: Option<&Id>) -> bool {
        Self::inner_allowance(
            &self.owners,
            id.map(|unwrapped_id| (&self.tokens, unwrapped_id)),
            owner,
            operator,
        )
    }

    fn inner_allowance(
        owners: &HashMap<Owner, TokenOwner>,
        tokens_and_id: Option<(&HashMap<Id, Token>, &Id)>,
        owner: Owner,
        operator: Operator,
    ) -> bool {
        owners
            .get(&owner)
            .map(|token_owner| {
                token_owner.operators.contains(&operator)
                    || tokens_and_id
                        .map(|(tokens, id)| tokens.get(id).unwrap().approvals.contains(&operator))
                        .unwrap_or_default()
            })
            .unwrap_or_default()
    }

    /// Transfers the token with given `id` to `to`.
    ///
    /// If [`msg::source()`] isn't the owner of the token, [`msg::source()`]
    /// must be an operator of all token's owner's tokens or the token with
    /// given `id`.
    ///
    /// # Errors
    /// - [`NFTError::ZeroRecipientAddress`] if `to` is [`ActorId::zero()`].
    /// - [`NFTError::TokenNotExists`] if the token doesn't exist.
    /// - [`NFTError::NotApproved`] if [`msg::source()`] isn't the owner of the
    ///   token and doesn't have any allowance for its transfer.
    pub fn transfer(&mut self, to: ActorId, id: Id) -> Result<NFTTransfer, NFTError> {
        if to.is_zero() {
            return Err(NFTError::ZeroRecipientAddress);
        }

        let Some(token) = self.tokens.get_mut(&id) else {
            return Err(NFTError::TokenNotExists);
        };
        let msg_source = msg::source();

        if !(token.owner == msg_source
            || token.approvals.remove(&msg_source)
            || Self::inner_allowance(&self.owners, None, token.owner, msg_source))
        {
            return Err(NFTError::NotApproved);
        }

        let from = token.owner;

        token.owner = to;

        self.decrement_balance(from);
        self.increment_balance(to);

        Ok(NFTTransfer { from, to, id })
    }

    /// Allows or disallows `operator` to transfer all [`msg::source()`]'s
    /// tokens or only the one with given `id`.
    ///
    /// - If `id` is [`Some`], sets an approval only for the token with this
    ///   `id`.
    /// - If `id` is [`None`], sets an approval for all [`msg::source()`]'s
    ///   tokens.
    ///
    /// If [`msg::source()`] is an operator of all tokens of the owner of the
    /// token with given `id`, [`msg::source()`] can also set approval for them.
    ///
    /// # Errors
    /// - [`NFTError::ZeroRecipientAddress`] if `operator` is
    ///   [`ActorId::zero()`].
    /// - [`NFTError::TokenNotExists`] if the token doesn't exist.
    /// - [`NFTError::NotApproved`] if [`msg::source()`] isn't the owner of the
    ///   token and operator of token's owner's tokens.
    pub fn approve(
        &mut self,
        operator: Operator,
        id: Option<Id>,
        approve: bool,
    ) -> Result<NFTApproval, NFTError> {
        if operator.is_zero() {
            return Err(NFTError::ZeroRecipientAddress);
        }

        let msg_source = msg::source();
        let mut owner = msg_source;

        let operators = if let Some(ref unwrapped_id) = id {
            let current_owner = self.internal_owner_of(unwrapped_id)?;

            if current_owner != owner {
                if !self.allowance(current_owner, owner, None) {
                    return Err(NFTError::NotApproved);
                }

                owner = current_owner;
            }

            &mut self.tokens.get_mut(unwrapped_id).unwrap().approvals
        } else {
            &mut self.owners.entry(owner).or_default().operators
        };

        if approve {
            operators.insert(operator);
        } else {
            operators.remove(&operator);
        }

        Ok(NFTApproval {
            owner,
            operator,
            id,
            approved: approve,
        })
    }

    fn balance_of_mut(&mut self, owner: Owner) -> &mut Amount {
        &mut self.owners.entry(owner).or_default().balance
    }

    fn increment_balance(&mut self, owner: Owner) {
        *self.balance_of_mut(owner) += Amount::one();
    }

    fn decrement_balance(&mut self, owner: Owner) {
        *self.balance_of_mut(owner) -= Amount::one();
    }

    /// Mints to `to` the token with given `id`.
    ///
    /// # Errors
    /// - [`NFTError::ZeroRecipientAddress`] if `to` is [`ActorId::zero()`].
    /// - [`NFTError::TokenNotExists`] if the total supply of tokens reached the
    ///   maximum limit of the [`Amount`] type.
    /// - [`NFTError::TokenExists`] if the token with given `id` already exists.
    pub fn mint(&mut self, to: ActorId, id: Id) -> Result<NFTTransfer, NFTError> {
        if to.is_zero() {
            return Err(NFTError::ZeroRecipientAddress);
        }

        if self.total_supply == Amount::MAX {
            return Err(NFTError::TokenNotExists);
        }

        if self.tokens.contains_key(&id) {
            return Err(NFTError::TokenExists);
        }

        self.increment_balance(to);
        self.tokens.insert(
            id.clone(),
            Token {
                owner: to,
                ..Default::default()
            },
        );

        self.total_supply += Amount::one();

        Ok(NFTTransfer {
            from: ActorId::zero(),
            to,
            id,
        })
    }

    /// Burns from `from` the token with given `id`.
    ///
    /// # Errors
    /// - [`NFTError::ZeroSenderAddress`] if `from` is [`ActorId::zero()`].
    /// - [`NFTError::TokenNotExists`] if the token doesn't exist.
    pub fn burn(&mut self, from: Owner, id: Id) -> Result<NFTTransfer, NFTError> {
        if from.is_zero() {
            return Err(NFTError::ZeroSenderAddress);
        }

        self.internal_owner_of(&id)?;
        self.tokens.remove(&id);
        self.decrement_balance(from);

        self.total_supply -= Amount::one();

        Ok(NFTTransfer {
            from,
            to: ActorId::zero(),
            id,
        })
    }

    /// Gets an attribute with given `key` for the token with given `id`.
    ///
    /// Returns [`None`] if an attribute with given `key` doesn't exist.
    ///
    /// To set an attribute, use [`NFTState::set_attribute()`].
    ///
    /// # Errors
    /// - [`NFTError::TokenNotExists`] if the token doesn't exist.
    pub fn get_attribute(&self, id: &Id, key: &Vec<u8>) -> Result<Option<&Vec<u8>>, NFTError> {
        self.tokens
            .get(id)
            .ok_or(NFTError::TokenNotExists)
            .map(|token| token.attributes.get(key))
    }

    /// Sets an attribute with given `key` & `value` for the token with given
    /// `id`.
    ///
    /// Returns [`None`] if an attribute with given `key` doesn't exist,
    /// otherwise replaces the old value with given one and returns [`Some`]
    /// with the old one.
    ///
    /// Note that it's impossible to remove a set attribute, only overwrite it.
    ///
    /// To get an attribute, use [`NFTState::get_attribute()`].
    ///
    /// # Errors
    /// - [`NFTError::TokenNotExists`] if the token doesn't exist.
    pub fn set_attribute(
        &mut self,
        id: &Id,
        key: Vec<u8>,
        value: Vec<u8>,
    ) -> Result<Option<Vec<u8>>, NFTError> {
        self.tokens
            .get_mut(id)
            .ok_or(NFTError::TokenNotExists)
            .map(|token| token.attributes.insert(key, value))
    }
}

/// The non-fungible token transfer event.
#[derive(Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, TypeInfo, Hash)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct NFTTransfer {
    /// A sender address.
    ///
    /// It equals [`ActorId::zero()`], if it's retrieved after token minting.
    pub from: ActorId,
    /// A recipient address.
    ///
    /// It equals [`ActorId::zero()`], if it's retrieved after token burning.
    pub to: ActorId,
    /// A token ID.
    pub id: Id,
}

/// The non-fungible token approval event.
#[derive(Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct NFTApproval {
    pub owner: Owner,
    pub operator: Operator,
    /// If it's [`Some`], it means this approval is only for the token with this
    /// `id`, if it's [`None`] - this approval is for all `owner`s tokens.
    pub id: Option<Id>,
    pub approved: bool,
}

/// Non-fungible token error variants.
#[derive(Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, TypeInfo, Hash)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum NFTError {
    /// [`msg::source()`] doesn't have any allowance to make a transfer/approval
    /// for the token.
    NotApproved,
    /// The token already exists.
    TokenExists,
    /// The token doesn't exist.
    TokenNotExists,
    /// A recipient/operator address is [`ActorId::zero()`].
    ZeroRecipientAddress,
    /// A sender address is [`ActorId::zero()`].
    ZeroSenderAddress,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn meta() {
        let key: Vec<_> = "Name".into();
        let data: Vec<_> = "Nuclear Fish Tank".into();
        let mut state = NFTState::new();

        assert_eq!(
            state.set_attribute(&0u8.into(), key.clone(), data.clone()),
            Err(NFTError::TokenNotExists)
        );

        state.mint(1.into(), 0u8.into()).unwrap();

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
            Err(NFTError::TokenNotExists)
        );
    }

    #[test]
    fn mint() {
        let mut state = NFTState::new();

        assert_eq!(
            state.mint(1.into(), 0u8.into()),
            Ok(NFTTransfer {
                from: ActorId::zero(),
                to: 1.into(),
                id: 0u8.into()
            })
        );

        assert_eq!(state.owner_of(&0u8.into()), 1.into());
        assert_eq!(state.balance_of(1.into()), 1u64.into());
        assert_eq!(state.total_supply(), 1u64.into());
    }

    #[test]
    fn mint_failures() {
        let mut state = NFTState::new();

        assert_eq!(
            state.mint(ActorId::zero(), 0u8.into()),
            Err(NFTError::ZeroRecipientAddress)
        );

        state.mint(1.into(), 0u8.into()).unwrap();

        assert_eq!(state.mint(1.into(), 0u8.into()), Err(NFTError::TokenExists));

        state.total_supply = Amount::MAX;

        assert_eq!(
            state.mint(1.into(), 1u8.into()),
            Err(NFTError::TokenNotExists)
        );
    }

    #[test]
    fn burn() {
        let mut state = NFTState::new();

        state.mint(1.into(), 0u8.into()).unwrap();

        assert_eq!(
            state.burn(1.into(), 0u8.into()),
            Ok(NFTTransfer {
                from: 1.into(),
                to: ActorId::zero(),
                id: 0u8.into()
            })
        );
        assert_eq!(state.owner_of(&0u8.into()), ActorId::zero());
        assert_eq!(state.balance_of(1.into()), 0u64.into());
        assert_eq!(state.total_supply(), 0u64.into());
    }

    #[test]
    fn burn_failures() {
        let mut state = NFTState::new();

        assert_eq!(
            state.burn(ActorId::zero(), 0u8.into()),
            Err(NFTError::ZeroSenderAddress)
        );
        assert_eq!(
            state.burn(1.into(), 0u8.into()),
            Err(NFTError::TokenNotExists)
        );
    }

    #[test]
    fn transfer() {
        let mut state = NFTState::new();

        state.mint(1.into(), 0u8.into()).unwrap();
        msg::set_source(1.into());

        assert_eq!(
            state.transfer(2.into(), 0u8.into()),
            Ok(NFTTransfer {
                from: 1.into(),
                to: 2.into(),
                id: 0u8.into()
            })
        );
        assert_eq!(state.balance_of(1.into()), 0u64.into());
        assert_eq!(state.balance_of(2.into()), 1u64.into());
        assert_eq!(state.owner_of(&0u8.into()), 2.into());
    }

    #[test]
    fn transfer_failures() {
        let mut state = NFTState::new();

        assert_eq!(
            state.transfer(ActorId::zero(), 0u8.into()),
            Err(NFTError::ZeroRecipientAddress)
        );
        assert_eq!(
            state.transfer(2.into(), 0u8.into()),
            Err(NFTError::TokenNotExists)
        );

        state.mint(1.into(), 0u8.into()).unwrap();
        msg::set_source(2.into());

        assert_eq!(
            state.transfer(2.into(), 0u8.into()),
            Err(NFTError::NotApproved)
        );
    }

    #[test]
    fn approve() {
        let mut state = NFTState::new();

        state.mint(1.into(), 0u8.into()).unwrap();
        msg::set_source(1.into());

        assert_eq!(
            state.approve(2.into(), Some(0u8.into()), true),
            Ok(NFTApproval {
                owner: 1.into(),
                operator: 2.into(),
                id: Some(0u8.into()),
                approved: true
            })
        );
        assert!(state.allowance(1.into(), 2.into(), Some(&0u8.into())));
        assert_eq!(
            state.approve(2.into(), Some(0u8.into()), false),
            Ok(NFTApproval {
                owner: 1.into(),
                operator: 2.into(),
                id: Some(0u8.into()),
                approved: false
            })
        );
        assert!(!state.allowance(1.into(), 2.into(), Some(&0u8.into())));

        assert_eq!(
            state.approve(2.into(), None, true),
            Ok(NFTApproval {
                owner: 1.into(),
                operator: 2.into(),
                id: None,
                approved: true
            })
        );
        assert!(state.allowance(1.into(), 2.into(), None));
        assert!(state.allowance(1.into(), 2.into(), Some(&0u8.into())));

        assert_eq!(
            state.approve(2.into(), None, false),
            Ok(NFTApproval {
                owner: 1.into(),
                operator: 2.into(),
                id: None,
                approved: false
            })
        );
        assert!(!state.allowance(1.into(), 2.into(), None));
        assert!(!state.allowance(1.into(), 2.into(), Some(&0u8.into())));
    }

    #[test]
    fn approved_transfer() {
        let mut state = NFTState::new();

        state.mint(1.into(), 0u8.into()).unwrap();
        msg::set_source(1.into());
        state.approve(2.into(), Some(0u8.into()), true).unwrap();
        msg::set_source(2.into());
        state.transfer(2.into(), 0u8.into()).unwrap();

        assert!(!state.allowance(1.into(), 2.into(), Some(&0u8.into())));

        state.approve(1.into(), None, true).unwrap();
        state.mint(2.into(), 1u8.into()).unwrap();
        msg::set_source(1.into());
        state.transfer(1.into(), 1u8.into()).unwrap();

        assert!(state.allowance(2.into(), 1.into(), None));
        assert!(state.allowance(2.into(), 1.into(), Some(&0u8.into())));
    }

    #[test]
    fn approve_failures() {
        let mut state = NFTState::new();

        assert_eq!(
            state.approve(ActorId::zero(), None, true),
            Err(NFTError::ZeroRecipientAddress)
        );
        assert_eq!(
            state.approve(2.into(), Some(0u8.into()), true),
            Err(NFTError::TokenNotExists)
        );

        state.mint(1.into(), 0u8.into()).unwrap();
        msg::set_source(2.into());

        assert_eq!(
            state.approve(2.into(), Some(0u8.into()), true),
            Err(NFTError::NotApproved)
        );
    }
}
