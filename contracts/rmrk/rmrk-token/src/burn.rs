use crate::*;
use gstd::msg;

impl RMRKToken {
    /// Burns RMRK token.
    /// It recursively burn all the children NFTs.
    /// It checks whether the token is a child of another token.
    /// If so, it sends a message to the parent NFT  to remove the child.
    ///
    /// # Requirements:
    /// * The `msg::source()` must be the root owner of the token.
    ///
    /// # Arguments:
    /// * `token_id`: is the tokenId of the burnt token.
    ///
    /// On success replies [`RMRKEvent::Transfer`].
    pub async fn burn(&mut self, token_id: TokenId) {
        let root_owner = self.find_root_owner(token_id).await;
        self.assert_owner(&root_owner);

        let rmrk_owner = self
            .rmrk_owners
            .remove(&token_id)
            .expect("RMRK: Token does not exist");
        // If the burnt NFT is a child of another NFT.
        if let Some(parent_token_id) = rmrk_owner.token_id {
            burn_child(&rmrk_owner.owner_id, parent_token_id, token_id).await;
        }

        self.decrease_balance(&root_owner);

        self.token_approvals.remove(&token_id);

        // burn all children
        self.internal_burn_children(token_id, &root_owner).await;

        msg::reply(
            RMRKEvent::Transfer {
                to: ActorId::zero(),
                token_id,
            },
            0,
        )
        .expect("Error in reply [RMRKEvent::Transfer]");
    }

    /// Burns RMRK tokens. It must be called from the RMRK parent contract when the root owner removes or rejects child NFTs.
    /// The input argument is an `BTreeSet<TokenId>` since a parent contract can have multiple children that must be burnt.
    /// It also recursively send messages [`RMRKAction::BurnFromParent`] to children of burnt tokens if any.
    ///
    /// # Requirements:
    /// * The `msg::source()` must be RMRK parent contract.
    /// * All tokens in `BTreeSet<TokenId>` must exist.
    ///
    /// # Arguments:
    /// * `token_ids`: is the tokenIds of the burnt tokens.
    ///
    /// On success replies [`RMRKEvent::TokensBurnt`].
    pub async fn burn_from_parent(&mut self, token_id: TokenId, root_owner: &ActorId) {
        let rmrk_owner = self
            .rmrk_owners
            .get(&token_id)
            .expect("Token does not exist");
        if msg::source() != rmrk_owner.owner_id {
            panic!("Caller must be parent RMRK contract")
        }
        self.token_approvals.remove(&token_id);
        self.decrease_balance(root_owner);
        self.rmrk_owners.remove(&token_id);
        self.internal_burn_children(token_id, root_owner).await;

        msg::reply(RMRKEvent::TokenBurnt(token_id), 0)
            .expect("Error in reply [RMRKEvent::TokensBurnt]");
    }

    // burn all pending and accepted children
    async fn internal_burn_children(&mut self, token_id: TokenId, root_owner: &ActorId) {
        if let Some(children) = self.pending_children.get(&token_id) {
            for (child_contract_id, child_token_id) in children {
                burn_from_parent(child_contract_id, *child_token_id, root_owner).await;
            }
        }

        if let Some(children) = self.accepted_children.get(&token_id) {
            for (child_contract_id, child_token_id) in children {
                burn_from_parent(child_contract_id, *child_token_id, root_owner).await;
            }
        }
    }
}
