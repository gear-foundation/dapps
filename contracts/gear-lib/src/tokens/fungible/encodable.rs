//! The encodable fungible token state.
//!
//! Due to limitations of the SCALE codec, it's impossible to encode the
//! [`HashMap`](gstd::collections::HashMap) & [`HashSet`](gstd::collections::HashSet) types, and therefore
//! [`super::FTState`] too, so as a workaround there's the encodable [`FTState`]
//! type that use [`Vec`] instead of unencodable types and can be constructed
//! from [`super::FTState`].

use super::{Amount, FTState as SuperFTState, Operator, Owner};
use gstd::prelude::*;

/// The encodable fungible token state.
#[derive(Default, Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, TypeInfo, Hash)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct FTState {
    pub total_supply: Amount,
    pub owners: Vec<(Owner, OwnerData)>,
}

impl FTState {
    /// Returns a balance of `owner`'s tokens.
    pub fn balance_of(&self, owner: Owner) -> Amount {
        self.owners
            .iter()
            .find_map(|(stored_owner, owner_data)| {
                (*stored_owner == owner).then_some(owner_data.balance)
            })
            .unwrap_or_default()
    }

    /// Returns an allowance of `owner`'s tokens for `operator`.
    pub fn allowance(&self, owner: Owner, operator: Operator) -> Amount {
        self.owners
            .iter()
            .find_map(|(stored_owner, owner_data)| {
                (*stored_owner == owner).then(|| {
                    owner_data
                        .allowances
                        .iter()
                        .find_map(|(stored_operator, allowance)| {
                            (*stored_operator == operator).then_some(*allowance)
                        })
                        .unwrap_or_default()
                })
            })
            .unwrap_or_default()
    }
}

impl From<SuperFTState> for FTState {
    fn from(state: SuperFTState) -> Self {
        let owners = state
            .owners
            .into_iter()
            .map(|(owner, owner_data)| {
                (
                    owner,
                    OwnerData {
                        balance: owner_data.balance,
                        allowances: owner_data.allowances.into_iter().collect(),
                    },
                )
            })
            .collect();

        Self {
            total_supply: state.total_supply,
            owners,
        }
    }
}

#[derive(Default, Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, TypeInfo, Hash)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct OwnerData {
    pub balance: Amount,
    pub allowances: Vec<(Operator, Amount)>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokens::test_helper::msg;
    use gstd::ActorId;

    #[test]
    fn balance_of() {
        let mut state = SuperFTState::new();

        state.mint(1.into(), 1u64.into()).unwrap();

        let encoded_state = FTState::from(state);

        assert_eq!(encoded_state.balance_of(2.into()), 0u64.into());
        assert_eq!(encoded_state.balance_of(ActorId::zero()), 0u64.into());
        assert_eq!(encoded_state.balance_of(1.into()), 1u64.into());
    }

    #[test]
    fn allowance() {
        let mut state = SuperFTState::new();

        msg::set_source(1.into());
        state.approve(2.into(), 100u64.into()).unwrap();

        let encoded_state = FTState::from(state);

        assert_eq!(encoded_state.allowance(1.into(), 3.into()), 0u64.into());
        assert_eq!(encoded_state.allowance(2.into(), 1.into()), 0u64.into());
        assert_eq!(encoded_state.allowance(1.into(), 2.into()), 100u64.into());
    }
}
