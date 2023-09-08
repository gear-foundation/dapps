use gstd::{collections::BTreeMap, prelude::*, ActorId};

pub type Payout = BTreeMap<ActorId, u128>;

#[derive(Default, Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, TypeInfo, Hash)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct Royalties {
    accounts: Payout,
    percent: u16,
}

impl Royalties {
    pub fn validate(&self) {
        // percent must be less than or equal to 100% (100 * 100)
        if self.percent > 10_000u16 {
            panic!("royalty percent must be less than 100%");
        }
        let mut total_percents = 0;
        self.accounts.iter().for_each(|(_, percent)| {
            if *percent as u16 > 10_000u16 {
                panic!("account percent must be less than or equal to 100%");
            }
            total_percents += percent;
        });
        if total_percents as u16 > 10_000u16 {
            panic!("total percent of royalty be less than or equal to 100%");
        }
    }

    pub fn payouts(&self, owner: &ActorId, amount: u128) -> Payout {
        let royalty_payment = amount * self.percent as u128 / 10_000;
        let mut payouts: Payout = self
            .accounts
            .iter()
            .map(|(account, percent)| (*account, *percent * royalty_payment / 10_000))
            .collect();

        let rest = amount - royalty_payment;
        let owner_payout = payouts.get(owner).map_or(0, |p| *p) + rest;
        payouts.insert(*owner, owner_payout);
        payouts
    }
}
