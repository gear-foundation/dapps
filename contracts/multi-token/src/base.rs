use gstd::{prelude::*, ActorId};

use crate::common::BalanceOfBatchReply;

pub trait ERC1155TokenBase {
    fn balance_of(&self, account: &ActorId, id: &u128) -> u128;
    fn balance_of_batch(&self, accounts: &[ActorId], ids: &[u128]) -> Vec<BalanceOfBatchReply>;
    fn set_approval_for_all(&mut self, operator: &ActorId, approved: bool);
    fn is_approved_for_all(&self, account: &ActorId, operator: &ActorId) -> bool;
    fn safe_transfer_from(&mut self, from: &ActorId, to: &ActorId, id: &u128, amount: u128);
    fn safe_batch_transfer_from(
        &mut self,
        from: &ActorId,
        to: &ActorId,
        ids: &[u128],
        amounts: &[u128],
    );
}

pub trait ExtendERC1155TokenBase {
    fn mint(&mut self, account: &ActorId, id: &u128, amount: u128);
    fn mint_batch(&mut self, account: &ActorId, ids: &[u128], amounts: &[u128]);
    fn burn_batch(&mut self, ids: &[u128], amounts: &[u128]);
    fn owner_of(&self, id: &u128) -> bool;
    fn owner_of_batch(&self, ids: &[u128]) -> bool;
}
