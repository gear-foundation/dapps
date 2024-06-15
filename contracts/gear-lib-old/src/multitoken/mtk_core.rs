use crate::multitoken::{io::*, state::*};
use gstd::{msg, prelude::*, ActorId};

const ZERO_ID: ActorId = ActorId::zero();

pub trait MTKCore: StateKeeper + MTKTokenState {
    fn assert_can_burn(&mut self, owner: &ActorId, id: &TokenId, amount: u128) {
        if self.get_balance(owner, id) < amount {
            panic!("MTK: Not enough balance");
        }
    }

    fn assert_can_transfer(&self, from: &ActorId, id: &u128, amount: u128) {
        if !(from == &msg::source() || self.get_balance(&msg::source(), id) >= amount) {
            panic!("MTK: Wrong owner or insufficient balance");
        }
    }

    fn assert_approved(&self, owner: &ActorId, operator: &ActorId) {
        if !self.get().approvals.contains_key(owner)
            && *self.get().approvals[owner].get(operator).unwrap_or(&false)
        {
            panic!("MTK: Caller is not approved");
        }
    }

    // The internal implementation of mint action with all the checks and panics
    fn mint_impl(
        &mut self,
        account: &ActorId,
        id: &TokenId,
        amount: u128,
        meta: Option<TokenMetadata>,
    ) {
        if let Some(metadata) = meta {
            if amount > 1 {
                panic!("MTK: Mint metadata to a fungible token")
            }
            self.get_mut().token_metadata.insert(*id, metadata);
            // since we have metadata = means we have an nft, so add it to the owners
            self.get_mut().owners.insert(*id, *account);
        }
        let prev_balance = self.get_balance(account, id);
        self.set_balance(account, id, prev_balance.saturating_add(amount));
    }

    /// Mints multiple new tokens (in case all input length is 1 - simple mint)
    /// Requirements:
    /// * `ids` element must a unique value
    /// * `account` must be a non-zero account
    ///   Arguments:
    /// * `account`: An account to which minted token will be assigned
    /// * `ids`: The vector of IDs of minted tokens
    /// * `amounts`: The vector of amounts of tokens to mint (1 in case of an NFT)
    /// * `meta`: The vector of optional additional metadata for NFTs
    fn mint(
        &mut self,
        account: &ActorId,
        ids: Vec<TokenId>,
        amounts: Vec<u128>,
        meta: Vec<Option<TokenMetadata>>,
    ) {
        if account == &ZERO_ID {
            panic!("MTK: Mint to zero address")
        }

        if ids.len() != amounts.len() {
            panic!("MTK: ids and amounts length mismatch")
        }

        meta.into_iter()
            .enumerate()
            .for_each(|(i, meta)| self.mint_impl(account, &ids[i], amounts[i], meta));

        msg::reply(
            MTKEvent::Transfer {
                operator: msg::source(),
                from: ZERO_ID,
                to: *account,
                ids: ids.to_vec(),
                amounts: amounts.to_vec(),
            },
            0,
        )
        .expect("Error during a reply with MTKEvent::Transfer");
    }

    // The internal implementation of burn action with all the checks and panics
    fn burn_impl(&mut self, id: &TokenId, amount: u128) {
        self.get_mut().owners.remove(id);
        self.set_balance(
            &msg::source(),
            id,
            self.get_balance(&msg::source(), id).saturating_sub(amount),
        );
    }

    /// Burns multiple tokens (in case all input length is 1 - simple burn)
    /// Requirements:
    /// * Only token owner can perform this action
    /// * `ids` element must be the ID of the existing token
    /// * `amounts` element must not exceed user's token balance
    ///   Arguments:
    /// * `ids`: The vector of ids of the token to be burnt
    /// * `amounts`: The vector of amounts of token to be burnt
    fn burn(&mut self, ids: Vec<TokenId>, amounts: Vec<u128>) {
        if ids.len() != amounts.len() {
            panic!("MTK: ids and amounts length mismatch")
        }

        for (id, amount) in ids.iter().zip(amounts.clone()) {
            self.assert_can_burn(&msg::source(), id, amount);
        }

        ids.iter()
            .enumerate()
            .for_each(|(i, id)| self.burn_impl(id, amounts[i]));

        msg::reply(
            MTKEvent::Transfer {
                operator: msg::source(),
                from: msg::source(),
                to: ZERO_ID,
                ids: ids.to_vec(),
                amounts: amounts.to_vec(),
            },
            0,
        )
        .expect("Error during a reply with MTKEvent::Transfer");
    }

    // The internal implementation of transfer action with all the checks and panics
    fn transfer_from_impl(&mut self, from: &ActorId, to: &ActorId, id: &TokenId, amount: u128) {
        let from_balance = self.get_balance(from, id);

        if from_balance < amount {
            panic!("MTK: insufficient balance for transfer")
        }
        self.set_balance(from, id, from_balance.saturating_sub(amount));
        let to_balance = self.get_balance(to, id);
        self.set_balance(to, id, to_balance.saturating_add(amount));
    }

    /// Transfers multiple tokens to a new user (in case all input length is 1 - simple transfer)
    /// Requirements:
    /// * Only the token owner or approved account can call that action
    /// * `to` must be a non-zero account
    /// * `ids` element must be the ID of the existing token
    /// * `amounts` element must not exceed from's balance
    ///   Arguments:
    /// * `from`: An account from which token will be transferred
    /// * `to`: An account to which token will be transferred
    /// * `ids`: The vector of IDs of transferred token
    /// * `amounts`: The vector of amounts of transferred token
    fn transfer_from(
        &mut self,
        from: &ActorId,
        to: &ActorId,
        ids: Vec<TokenId>,
        amounts: Vec<u128>,
    ) {
        if from == to {
            panic!("MTK: sender and recipient addresses are the same")
        }

        if from != &msg::source() {
            panic!("MTK: caller is not owner nor approved")
        }

        if to == &ZERO_ID {
            panic!("MTK: transfer to the zero address")
        }

        if ids.len() != amounts.len() {
            panic!("MTK: ids and amounts length mismatch")
        }

        for (id, amount) in ids.iter().zip(amounts.clone()) {
            self.assert_can_transfer(from, id, amount);
        }

        ids.iter()
            .enumerate()
            .for_each(|(i, id)| self.transfer_from_impl(from, to, id, amounts[i]));

        msg::reply(
            MTKEvent::Transfer {
                operator: msg::source(),
                from: *from,
                to: *to,
                ids: ids.to_vec(),
                amounts: amounts.to_vec(),
            },
            0,
        )
        .expect("Error during a reply with MTKEvent::Transfer");
    }

    /// Gives a right to another account to manage its tokens
    /// Requirements:
    /// * Only the token owner can call that action
    /// * `to` must be a non-zero account
    ///   Arguments:
    /// * `to`: An account that will be approved to manage the tokens
    fn approve(&mut self, to: &ActorId) {
        if to == &ZERO_ID {
            panic!("MTK: approving zero address")
        }
        self.get_mut()
            .approvals
            .get_mut(&msg::source())
            .expect("Caller has not approved any accounts")
            .insert(*to, true);
        msg::reply(
            MTKEvent::Approval {
                from: msg::source(),
                to: *to,
            },
            0,
        )
        .expect("Error during a reply with MTKEvent::Approval");
    }

    /// Removed a right to another account to manage its tokens
    /// Requirements:
    /// * Only the token owner can call that action
    /// * `to` must be a non-zero account
    ///   Arguments:
    /// * `to`: An account that won't be able to manage the tokens
    fn revoke_approval(&mut self, to: &ActorId) {
        self.get_mut()
            .approvals
            .get_mut(&msg::source())
            .expect("Caller has not approved any accounts")
            .remove_entry(to);

        msg::reply(
            MTKEvent::RevokeApproval {
                from: msg::source(),
                to: *to,
            },
            0,
        )
        .expect("Error during a reply with MTKEvent::RevokeApproval");
    }

    /// Returns the amount of multiple specific tokens multiple users have
    /// (in case all input length is 1 - simple balance_of)
    /// Arguments:
    /// * `accounts`: The vectors of IDs of the actor
    /// * `id`: The vector of token IDs which balance will be returned
    fn balance_of(&self, accounts: Vec<ActorId>, ids: Vec<TokenId>) {
        if accounts.len() != ids.len() {
            panic!("MTK: accounts and ids length mismatch")
        }

        let res = ids
            .iter()
            .zip(accounts)
            .map(|(id, account)| BalanceReply {
                account,
                id: *id,
                amount: self.get_balance(&account, id),
            })
            .collect();

        msg::reply(MTKEvent::BalanceOf(res), 0)
            .expect("Error during a reply with MTKEvent::BalanceOf");
    }
}
