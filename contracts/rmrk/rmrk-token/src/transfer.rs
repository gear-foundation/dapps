use crate::*;
use gstd::{msg, ActorId};

impl RMRKToken {
    /// Transfers NFT to another account.
    /// If the previous owner is another RMRK contract, it sends the message [`RMRKAction::BurnChild`] to the parent conract.
    ///
    /// # Requirements:
    /// * The `token_id` must exist.
    /// * The `msg::source()` must be approved or owner of the token.
    /// * The `to` address should be a non-zero address.
    ///
    /// # Arguments:
    /// * `to`: is the receiving address.
    /// * `token_id`: is the tokenId of the transfered token.
    ///
    /// On success replies [`RMRKEvent::ChildBurnt`].
    pub async fn transfer(&mut self, to: &ActorId, token_id: TokenId) {
        let root_owner = self.find_root_owner(token_id).await;
        self.assert_approved_or_owner(token_id, &root_owner);
        let rmrk_owner = self
            .rmrk_owners
            .get(&token_id)
            .expect("Token does not exist");

        self.assert_zero_address(to);

        // if the NFT owner is another NFT
        if let Some(parent_token_id) = rmrk_owner.token_id {
            burn_child(&rmrk_owner.owner_id, parent_token_id, token_id).await;
        }

        self.decrease_balance(&root_owner);

        // mint NFT to new root owner
        self.internal_mint(to, token_id, to, None);

        msg::reply(RMRKEvent::Transfer { to: *to, token_id }, 0)
            .expect("Error in reply [`RMRKEvent::ChildBurnt`]");
    }

    /// Transfers NFT to another NFT.
    ///
    /// # Requirements:
    /// * The `token_id` must exist.
    /// * The `msg::source()` must be approved or root owner of the token.
    /// * The `to` address should be a non-zero address
    ///
    /// # Arguments:
    /// * `to`: is the address of new parent RMRK contract.
    /// * `destination_id: is the tokenId of the parent RMRK token.
    /// * `token_id`: is the tokenId of the transfered token.
    ///
    /// On success replies [`RMRKEvent::TransferToNft`].
    pub async fn transfer_to_nft(
        &mut self,
        to: &ActorId,
        destination_id: TokenId,
        token_id: TokenId,
    ) {
        let root_owner = self.find_root_owner(token_id).await;
        self.assert_approved_or_owner(token_id, &root_owner);
        self.assert_zero_address(to);
        let rmrk_owner = self
            .rmrk_owners
            .get(&token_id)
            .expect("Token does not exist");
        let mut new_rmrk_owner: RMRKOwner = Default::default();
        let new_root_owner = get_root_owner(to, destination_id).await;

        // if root owner transfers child RMRK token between RMRK tokens inside the same RMRK contract
        if rmrk_owner.owner_id == *to {
            transfer_child(
                to,
                rmrk_owner.token_id.expect("Cant be None"),
                destination_id,
                token_id,
            )
            .await;
        } else {
            if rmrk_owner.token_id.is_some() {
                burn_child(
                    &rmrk_owner.owner_id,
                    rmrk_owner.token_id.expect("Cant be None"),
                    token_id,
                )
                .await;
            }
            if root_owner == new_root_owner {
                add_accepted_child(to, destination_id, token_id).await;
            } else {
                add_child(to, destination_id, token_id).await;
                self.increase_balance(&new_root_owner);
                self.decrease_balance(&root_owner);
            }
        }
        new_rmrk_owner.owner_id = *to;
        new_rmrk_owner.token_id = Some(destination_id);
        self.rmrk_owners.insert(token_id, new_rmrk_owner);
        msg::reply(
            RMRKEvent::TransferToNft {
                to: *to,
                token_id,
                destination_id,
            },
            0,
        )
        .expect("Error in reply [`RMRKEvent::TransferToNft`]");
    }

    /// Approves an account to transfer NFT.
    ///
    /// # Requirements:
    /// * The `token_id` must exist.
    /// * The `msg::source()` must be approved or root owner of the token.
    /// * The `to` address must be a non-zero address
    ///
    /// # Arguments:
    /// * `to`: is the address of approved account.
    /// * `token_id`: is the tokenId of the token.
    ///
    /// On success replies [`RMRKEvent::Approval`].
    pub async fn approve(&mut self, to: &ActorId, token_id: TokenId) {
        let root_owner = self.find_root_owner(token_id).await;
        self.assert_owner(&root_owner);
        self.assert_zero_address(to);
        self.token_approvals
            .entry(token_id)
            .and_modify(|approvals| {
                approvals.insert(*to);
            })
            .or_insert_with(|| BTreeSet::from([*to]));
        msg::reply(
            RMRKEvent::Approval {
                root_owner,
                approved_account: *to,
                token_id,
            },
            0,
        )
        .expect("Error in reply `[RMRKEvent::Approval]`");
    }
}
