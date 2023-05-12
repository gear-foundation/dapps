use super::{Amount, Operator, Owner};
use gstd::prelude::*;

#[derive(Default, Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, TypeInfo, Hash)]
pub struct FTState {
    pub total_supply: Amount,
    pub owners: Vec<(Owner, OwnerData)>,
}

impl FTState {
    pub fn balance_of(&self, owner: Owner) -> Amount {
        self.owners
            .iter()
            .find_map(|(stored_owner, owner_data)| {
                (*stored_owner == owner).then_some(owner_data.balance)
            })
            .unwrap_or_default()
    }

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

#[derive(Default, Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, TypeInfo, Hash)]
pub struct OwnerData {
    pub balance: Amount,
    pub allowances: Vec<(Operator, Amount)>,
}

impl From<super::FTState> for FTState {
    fn from(state: super::FTState) -> Self {
        Self {
            total_supply: state.total_supply,
            owners: state
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
                .collect(),
        }
    }
}
