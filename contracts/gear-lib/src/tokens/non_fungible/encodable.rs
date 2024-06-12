//! The encodable non-fungible token state.
//!
//! Due to limitations of the SCALE codec, it's impossible to encode the
//! [`HashMap`](gstd::collections::HashMap) & [`HashSet`](gstd::collections::HashSet) types, and therefore
//! [`super::NFTState`] too, so as a workaround there's the encodable
//! [`NFTState`] type that use [`Vec`] instead of unencodable types and can be
//! constructed from [`super::NFTState`].

use super::{Amount, Id, NFTError, NFTState as SuperNFTState, Operator, Owner};
use gstd::prelude::*;

/// The encodable non-fungible token state.
#[derive(Default, Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, TypeInfo, Hash)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct NFTState {
    pub total_supply: Amount,
    pub tokens: Vec<(Id, Token)>,
    pub owners: Vec<(Owner, TokenOwner)>,
}

impl NFTState {
    /// Returns a balance of `owner`'s tokens.
    pub fn balance_of(&self, owner: Owner) -> Amount {
        self.owners
            .iter()
            .find_map(|(stored_owner, token_owner)| {
                (*stored_owner == owner).then_some(token_owner.balance)
            })
            .unwrap_or_default()
    }

    /// Returns [`Owner`] of the token with given `id`.
    pub fn owner_of(&self, id: &Id) -> Owner {
        self.tokens
            .iter()
            .find_map(|(stored_id, token)| (stored_id == id).then_some(token.owner))
            .unwrap_or_default()
    }

    /// Returns [`true`] if `operator` is allowed to transfer all `owner`'s
    /// tokens or the token with given `id`.
    ///
    /// - If `id` is [`Some`], firstly checks if `operator` is allowed for all
    ///   `owner`'s tokens, and if not, whether `operator` is allowed for the
    ///   token with given `id`.
    /// - If `id` is [`None`], only checks if `operator` is allowed for all
    ///   `owner`'s tokens.
    pub fn allowance(&self, owner: Owner, operator: Operator, id: Option<&Id>) -> bool {
        self.owners
            .iter()
            .find_map(|(stored_owner, token_owner)| {
                (*stored_owner == owner).then_some(&token_owner.operators)
            })
            .map(|operators| {
                operators.contains(&operator)
                    || id
                        .map(|unwrapped_id| {
                            self.tokens
                                .iter()
                                .find_map(|(stored_id, token)| {
                                    (stored_id == unwrapped_id).then_some(&token.approvals)
                                })
                                .unwrap()
                                .contains(&operator)
                        })
                        .unwrap_or_default()
            })
            .unwrap_or_default()
    }

    /// Gets an attribute with given `key` for the token with given `id`.
    ///
    /// Returns [`None`] if an attribute with given `key` doesn't exist.
    ///
    /// # Errors
    /// - [`NFTError::TokenNotExists`] if the token doesn't exist.
    pub fn get_attribute(&self, id: &Id, key: &Vec<u8>) -> Result<Option<&Vec<u8>>, NFTError> {
        self.tokens
            .iter()
            .find_map(|(stored_id, token)| (stored_id == id).then_some(&token.attributes))
            .ok_or(NFTError::TokenNotExists)
            .map(|attributes| {
                attributes
                    .iter()
                    .find_map(|(stored_key, value)| (stored_key == key).then_some(value))
            })
    }
}

impl From<SuperNFTState> for NFTState {
    fn from(state: SuperNFTState) -> Self {
        let tokens = state
            .tokens
            .into_iter()
            .map(|(id, token)| {
                (
                    id,
                    Token {
                        owner: token.owner,
                        approvals: token.approvals.into_iter().collect(),
                        attributes: token.attributes.into_iter().collect(),
                    },
                )
            })
            .collect();
        let owners = state
            .owners
            .into_iter()
            .map(|(owner, token_owner)| {
                (
                    owner,
                    TokenOwner {
                        operators: token_owner.operators.into_iter().collect(),
                        balance: token_owner.balance,
                    },
                )
            })
            .collect();

        Self {
            total_supply: state.total_supply,
            tokens,
            owners,
        }
    }
}

#[derive(Default, Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, TypeInfo, Hash)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct Token {
    pub owner: Owner,
    pub approvals: Vec<Operator>,
    pub attributes: Vec<(Vec<u8>, Vec<u8>)>,
}

#[derive(Default, Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, TypeInfo, Hash)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct TokenOwner {
    pub operators: Vec<Operator>,
    pub balance: Amount,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokens::test_helper::msg;
    use gstd::ActorId;

    #[test]
    fn balance_of() {
        let mut state = SuperNFTState::new();

        state.mint(1.into(), 0u8.into()).unwrap();

        let encoded_state = NFTState::from(state);

        assert_eq!(encoded_state.balance_of(ActorId::zero()), 0u64.into());
        assert_eq!(encoded_state.balance_of(2.into()), 0u64.into());
        assert_eq!(encoded_state.balance_of(1.into()), 1u64.into());
    }

    #[test]
    fn owner_of() {
        let mut state = SuperNFTState::new();

        state.mint(1.into(), 0u8.into()).unwrap();

        let encoded_state = NFTState::from(state);

        assert_eq!(encoded_state.owner_of(&123u16.into()), ActorId::zero());
        assert_eq!(encoded_state.owner_of(&1u8.into()), ActorId::zero());
        assert_eq!(encoded_state.owner_of(&0u8.into()), 1.into());
    }

    #[test]
    fn allowance() {
        let mut state = SuperNFTState::new();

        state.mint(1.into(), 0u8.into()).unwrap();
        msg::set_source(1.into());
        state.approve(2.into(), Some(0u8.into()), true).unwrap();

        let mut encoded_state = NFTState::from(state.clone());

        assert!(!encoded_state.allowance(ActorId::zero(), 2.into(), Some(&0u8.into())));
        assert!(!encoded_state.allowance(1.into(), 3.into(), Some(&0u8.into())));
        assert!(encoded_state.allowance(1.into(), 2.into(), Some(&0u8.into())));
        assert!(!encoded_state.allowance(1.into(), 2.into(), None));

        state.approve(2.into(), None, true).unwrap();

        encoded_state = NFTState::from(state);

        assert!(!encoded_state.allowance(1.into(), 3.into(), None));
        assert!(encoded_state.allowance(1.into(), 2.into(), None));
        assert!(encoded_state.allowance(1.into(), 2.into(), Some(&123u8.into())));
    }

    #[test]
    fn get_attribute() {
        let key: Vec<_> = "A".into();
        let value: Vec<_> = "B".into();
        let mut state = SuperNFTState::new();

        state.mint(1.into(), 0u8.into()).unwrap();
        state
            .set_attribute(&0u8.into(), key.clone(), value.clone())
            .unwrap();

        let encoded_state = NFTState::from(state);

        assert_eq!(
            encoded_state.get_attribute(&123u8.into(), &key),
            Err(NFTError::TokenNotExists)
        );
        assert_eq!(
            encoded_state.get_attribute(&0u8.into(), &"ABCD".into()),
            Ok(None)
        );
        assert_eq!(
            encoded_state.get_attribute(&0u8.into(), &key),
            Ok(Some(&value))
        );
    }
}
