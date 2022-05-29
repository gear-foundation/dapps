use gstd::{prelude::*, ActorId};
use scale_info::TypeInfo;
use crate::{Royalties, Payout};

impl Royalties {
    pub fn validate(&self) {
        // percent must be less then 100% (100 * 100)
        if self.percent > 10_000u16 {
            panic!("royalty percent must be less than 100%");
        }
        let mut total_percents = 0;
        self.accounts.iter().for_each(|(_, percent)| {
            if *percent > 10_000u16 {
                panic!("account percent must be less than 100%");
            }
            total_percents += percent;
        });
        if total_percents > 10_000u16 {
            panic!("total percent of royalty be less than 100%");
        }
    }

    pub fn payouts(&self, owner: &ActorId, amount: u128,) -> Payout {
        let royalty_payment = amount * self.percent as u128 / 10_000;
        let mut payouts: Payout = self
                    .accounts   
                    .iter()
                    .map(|(account, percent)| {
                        (
                            *account,
                            *percent as u128 * royalty_payment / 10_000
                        )
                    })
                    .collect();
        
        let rest = amount - royalty_payment;
        let owner_payout = payouts.get(owner).map_or(0, |p| *p) + rest;
        payouts.insert(*owner, owner_payout);
        payouts
    }
}