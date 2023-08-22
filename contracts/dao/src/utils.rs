use crate::contract::Dao;
use gstd::{msg, prelude::*, ActorId};

impl Dao {
    // calculates the funds that the member can redeem based on his shares
    pub fn redeemable_funds(&self, share: u128) -> u128 {
        if self.total_shares > 0 {
            (share.saturating_mul(self.balance)) / self.total_shares
        } else {
            panic!("Zero total shares in DAO!");
        }
    }

    // checks that account is DAO member
    pub fn is_member(&self, account: &ActorId) -> bool {
        matches!(self.members.get(account), Some(member) if member.shares != 0)
    }

    // check that `msg::source()` is either a DAO member or a delegate key
    pub fn check_for_membership(&self) {
        match self.member_by_delegate_key.get(&msg::source()) {
            Some(member) if !self.is_member(member) => panic!("account is not a DAO member"),
            None => panic!("account is not a delegate"),
            _ => {}
        }
    }

    // Determine either this is a new transaction
    // or the transaction which has to be completed
    pub fn get_transaction_id(&mut self, transaction_id: Option<u64>) -> u64 {
        match transaction_id {
            Some(transaction_id) => transaction_id,
            None => {
                let transaction_id = self.transaction_id;
                self.transaction_id = self.transaction_id.wrapping_add(1);
                transaction_id
            }
        }
    }

    pub fn assert_admin(&self) {
        assert_eq!(msg::source(), self.admin, "msg::source() must be DAO admin");
    }

    pub fn assert_not_zero_address(address: &ActorId) {
        assert!(!address.is_zero(), "Zero address");
    }
}
