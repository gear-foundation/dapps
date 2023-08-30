use crate::non_fungible_token::{delegated::*, io::*, royalties::*, state::*, token::*};
use gstd::{collections::HashSet, msg, prelude::*, ActorId};

const ZERO_ID: ActorId = ActorId::zero();

pub trait NFTCore: NFTStateKeeper {
    /// Mints a new token
    ///
    /// Requirements:
    /// * `token_id` must be unique
    /// * `to` must be a non-zero account
    ///
    /// Arguments:
    /// * `to`: An account to which minted NFT will be assigned
    /// * `token_id`: the ID of minted NFT
    /// * `token_metadata`: optional additional metadata about NFT
    fn mint(
        &mut self,
        to: &ActorId,
        token_id: TokenId,
        token_metadata: Option<TokenMetadata>,
    ) -> NFTTransfer {
        self.assert_token_exists(token_id);
        self.assert_zero_address(to);
        self.get_mut().owner_by_id.insert(token_id, *to);
        self.get_mut()
            .tokens_for_owner
            .entry(*to)
            .and_modify(|tokens| tokens.push(token_id))
            .or_insert_with(|| vec![token_id]);
        self.get_mut()
            .token_metadata_by_id
            .insert(token_id, token_metadata);
        NFTTransfer {
            from: ZERO_ID,
            to: *to,
            token_id,
        }
    }

    /// Burns a token
    ///
    /// Requirements:
    /// * Only NFT owner can call that action
    /// * `token_id` must be the ID of the existing NFT
    ///
    /// Arguments:
    /// * `token_id`: the ID of  NFT that will be burnt
    fn burn(&mut self, token_id: TokenId) -> NFTTransfer {
        let owner = *self
            .get()
            .owner_by_id
            .get(&token_id)
            .expect("NonFungibleToken: token does not exist");
        self.assert_owner(&owner);
        self.get_mut().owner_by_id.remove(&token_id);
        self.get_mut().token_metadata_by_id.remove(&token_id);
        self.get_mut()
            .tokens_for_owner
            .entry(owner)
            .and_modify(|tokens| tokens.retain(|&token| token != token_id));
        NFTTransfer {
            from: owner,
            to: ZERO_ID,
            token_id,
        }
    }

    /// Transfers a token to the new owner
    ///
    /// Requirements:
    /// * Only the token owner or approved account can call that action
    /// * `to` must be a non-zero account
    /// * `token_id` must be the ID of the existing NFT
    ///
    /// Arguments:
    /// * `to`: An account to which NFT will be transferred
    /// * `token_id`: the ID of transferred NFT
    fn transfer(&mut self, to: &ActorId, token_id: TokenId) -> NFTTransfer {
        let owner = self.internal_transfer(to, token_id);
        NFTTransfer {
            from: owner,
            to: *to,
            token_id,
        }
    }

    /// Transfers a token to the new owner
    ///
    /// Requirements:
    /// * Only the token owner or approved account can call that action
    /// * `to` must be a non-zero account
    /// * `token_id` must be the ID of the existing NFT
    ///
    /// Arguments:
    /// * `to`: An account to which NFT will be transferred
    /// * `token_id`: the ID of transferred NFT
    fn transfer_payout(
        &mut self,
        to: &ActorId,
        token_id: TokenId,
        amount: u128,
    ) -> NFTTransferPayout {
        let owner = self.internal_transfer(to, token_id);
        let payouts = self.nft_payout(&owner, amount);
        NFTTransferPayout {
            from: owner,
            to: *to,
            token_id,
            payouts,
        }
    }

    fn internal_transfer(&mut self, to: &ActorId, token_id: TokenId) -> ActorId {
        let owner = *self
            .get()
            .owner_by_id
            .get(&token_id)
            .expect("NonFungibleToken: token does not exist");
        self.assert_can_transfer(token_id, &owner);
        self.assert_zero_address(to);
        // assign new owner
        self.get_mut()
            .owner_by_id
            .entry(token_id)
            .and_modify(|owner| *owner = *to);
        // push token to new owner
        self.get_mut()
            .tokens_for_owner
            .entry(*to)
            .and_modify(|tokens| tokens.push(token_id))
            .or_insert_with(|| vec![token_id]);
        // remove token from old owner
        self.get_mut()
            .tokens_for_owner
            .entry(owner)
            .and_modify(|tokens| tokens.retain(|&token| token != token_id));
        // remove approvals if any
        self.get_mut().token_approvals.remove(&token_id);
        owner
    }

    /// Gives a right to another account to manage the token with indicated ID
    ///
    /// Requirements:
    /// * Only the token owner can call that action
    /// * `to` must be a non-zero account
    /// * `token_id` must be the ID of the existing NFT
    ///
    /// Arguments:
    /// * `to`: An account that will be approved to manage the indicated NFT
    /// * `token_id`: the ID of the NFT
    fn approve(&mut self, to: &ActorId, token_id: TokenId) -> NFTApproval {
        let owner = *self
            .get()
            .owner_by_id
            .get(&token_id)
            .expect("NonFungibleToken: token does not exist");
        self.assert_owner(&owner);
        self.assert_zero_address(to);
        self.get_mut()
            .token_approvals
            .entry(token_id)
            .and_modify(|approvals| {
                approvals.insert(*to);
            })
            .or_insert_with(|| HashSet::from([*to]));
        NFTApproval {
            owner,
            approved_account: *to,
            token_id,
        }
    }

    fn revoke_approval(&mut self, approved_account: &ActorId, token_id: TokenId) -> NFTApproval {
        let owner = *self
            .get()
            .owner_by_id
            .get(&token_id)
            .expect("NonFungibleToken: token does not exist");
        self.assert_owner(&owner);
        self.get_mut()
            .token_approvals
            .entry(token_id)
            .and_modify(|approvals| {
                approvals.remove(approved_account);
            });
        NFTApproval {
            owner,
            approved_account: ZERO_ID,
            token_id,
        }
    }

    fn owner_of(&self, token_id: TokenId) -> ActorId {
        *self
            .get()
            .owner_by_id
            .get(&token_id)
            .expect("NonFungibleToken: token does not exist")
    }

    fn is_approved_to(&self, to: &ActorId, token_id: TokenId) -> bool {
        self.get()
            .token_approvals
            .get(&token_id)
            .expect("NonFungibleToken: token does not exist")
            .contains(to)
    }

    fn delegated_approve(
        &mut self,
        message: DelegatedApproveMessage,
        signed_approve: [u8; 64],
    ) -> NFTApproval {
        let to = &message.approved_actor_id;
        let token_id = message.token_id;
        let owner = *self
            .get()
            .owner_by_id
            .get(&token_id)
            .expect("NonFungibleToken: token does not exist");

        message.validate(&signed_approve, &owner);

        self.get_mut()
            .token_approvals
            .entry(token_id)
            .and_modify(|approvals| {
                approvals.insert(*to);
            })
            .or_insert_with(|| HashSet::from([*to]));
        NFTApproval {
            owner,
            approved_account: *to,
            token_id,
        }
    }

    /// Returns a `Payout` struct for a given token
    /// If NFT contract has no royalties it just returns BtreeMap {“owner”: "amount"}
    fn nft_payout(&self, owner: &ActorId, amount: u128) -> Payout {
        if let Some(ref royalties) = self.get().royalties {
            royalties.payouts(owner, amount)
        } else {
            [(*owner, amount)].into()
        }
    }

    /// Checks that NFT with indicated ID already exists
    fn assert_token_exists(&self, token_id: TokenId) {
        if self.get().owner_by_id.contains_key(&token_id) {
            panic!("NonFungibleToken: Token already exists");
        }
    }

    /// Checks account for a zero address
    fn assert_zero_address(&self, account: &ActorId) {
        if account == &ZERO_ID {
            panic!("NonFungibleToken: Zero address");
        }
    }

    /// Checks that `msg::source()` is allowed to manage the token with indicated `token_id`
    fn assert_can_transfer(&self, token_id: TokenId, owner: &ActorId) {
        if let Some(approved_accounts) = self.get().token_approvals.get(&token_id) {
            if approved_accounts.contains(&msg::source()) {
                return;
            }
        }
        self.assert_owner(owner);
    }

    /// Checks that `msg::source()` is the owner of the token with indicated `token_id`
    fn assert_owner(&self, owner: &ActorId) {
        if owner != &msg::source() {
            panic!("Not allowed to transfer");
        }
    }
}
