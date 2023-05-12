use super::types::{Amount, Id, Operator, Owner};
use gstd::{prelude::*, ActorId};
use hashbrown::{HashMap, HashSet};

#[cfg(test)]
use super::tests::msg;
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

    pub const fn total_supply(&self) -> Amount {
        self.total_supply
    }

    pub fn balance_of(&self, owner: Owner) -> Amount {
        self.owners
            .get(&owner)
            .map(|token_owner| token_owner.balance)
            .unwrap_or_default()
    }

    pub fn owner_of(&self, id: &Id) -> Owner {
        self.internal_owner_of(id).unwrap_or_default()
    }

    fn internal_owner_of(&self, id: &Id) -> Result<Owner, NFTError> {
        self.tokens
            .get(id)
            .map(|token| token.owner)
            .ok_or(NFTError::TokenNotExists)
    }

    pub fn allowance(&self, owner: Owner, operator: Operator, id: Option<&Id>) -> bool {
        Self::inner_allowance(
            &self.owners,
            id.map(|id| (&self.tokens, id)),
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
        owners.get(&owner).map_or(false, |token_owner| {
            match (token_owner.operators.contains(&operator), tokens_and_id) {
                (true, _) => true,
                (false, Some((tokens, id))) => {
                    tokens.get(id).unwrap().approvals.contains(&operator)
                }
                (false, _) => false,
            }
        })
    }

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

        let operators = if let Some(id) = &id {
            let current_owner = self.internal_owner_of(id)?;

            if current_owner != owner {
                if !self.allowance(current_owner, owner, None) {
                    return Err(NFTError::NotApproved);
                }

                owner = current_owner;
            }

            &mut self.tokens.get_mut(id).unwrap().approvals
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

    pub fn get_attribute(&self, id: &Id, key: &Vec<u8>) -> Result<Option<&Vec<u8>>, NFTError> {
        self.tokens
            .get(id)
            .map_or(Err(NFTError::TokenNotExists), |token| {
                Ok(token.attributes.get(key))
            })
    }

    pub fn set_attribute(
        &mut self,
        id: &Id,
        key: Vec<u8>,
        value: Vec<u8>,
    ) -> Result<Option<Vec<u8>>, NFTError> {
        self.tokens
            .get_mut(id)
            .map_or(Err(NFTError::TokenNotExists), |token| {
                Ok(token.attributes.insert(key, value))
            })
    }
}

#[derive(Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, TypeInfo, Hash)]
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

#[derive(Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, TypeInfo, Hash)]
pub enum NFTError {
    NotApproved,
    TokenExists,
    TokenNotExists,
    ZeroRecipientAddress,
    ZeroSenderAddress,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn meta() {
        let key: Vec<u8> = "Name".into();
        let data: Vec<u8> = "Nuclear Fish Tank".into();
        let mut state = NFTState::new();

        assert_eq!(
            state.set_attribute(&0u8.into(), key.clone(), data.clone()),
            Err(NFTError::TokenNotExists)
        );

        state.mint(1.into(), 0u8.into()).unwrap();

        assert_eq!(
            state.set_attribute(&0u8.into(), key.clone(), data.clone()),
            Ok(None)
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
        assert_eq!(state.balance_of(1.into()), 1.into());
        assert_eq!(state.total_supply(), 1.into());
    }

    #[test]
    fn mint_failures() {
        let mut state = NFTState::new();

        assert_eq!(
            state.mint(0.into(), 0u8.into()),
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
        assert_eq!(state.balance_of(1.into()), 0.into());
        assert_eq!(state.total_supply(), 0.into());
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
        assert_eq!(state.balance_of(1.into()), 0.into());
        assert_eq!(state.balance_of(2.into()), 1.into());
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
