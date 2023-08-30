use crate::utils::get_rmrk_owner;
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
    pub fn burn(
        &mut self,
        tx_manager: &mut TxManager,
        token_id: TokenId,
    ) -> Result<RMRKReply, RMRKError> {
        let rmrk_owner = get_rmrk_owner(&self.rmrk_owners, token_id)?;
        let state = tx_manager.get_state(msg::id());

        match state {
            TxState::MsgSourceAccountChecked => {
                if let Some(owner_token_id) = rmrk_owner.token_id {
                    let msg_id = burn_child_msg(&rmrk_owner.owner_id, owner_token_id, token_id);
                    tx_manager.set_tx_state(TxState::MsgBurnChildSent, msg_id);
                    exec::wait_for(5);
                } else {
                    let root_owner = rmrk_owner.owner_id;
                    self.assert_owner(&root_owner);

                    // burn children
                    let msg_id = self.internal_burn_children(tx_manager, token_id);
                    if msg_id != MessageId::zero() {
                        tx_manager.set_tx_state(TxState::MsgBurnFromParentSent, msg_id);
                        exec::wait_for(5);
                    } else {
                        // no children
                        self.rmrk_owners.remove(&token_id);
                        self.decrease_balance(&root_owner);
                        self.token_approvals.remove(&token_id);
                        Ok(RMRKReply::Burnt)
                    }
                }
            }
            TxState::ReplyOnBurnChildReceived | TxState::ReplyOnBurnFromParentReceived => {
                let root_owner = if rmrk_owner.token_id.is_some() {
                    tx_manager.get_decoded_data::<ActorId>()?
                } else {
                    rmrk_owner.owner_id
                };

                if state == TxState::ReplyOnBurnFromParentReceived {
                    let child_token = tx_manager.get_payload::<(ActorId, TokenId)>()?;
                    self.internal_remove_child(token_id, child_token)?;
                }
                // burn children
                let msg_id = self.internal_burn_children(tx_manager, token_id);
                if msg_id != MessageId::zero() {
                    tx_manager.set_tx_state(TxState::MsgBurnFromParentSent, msg_id);
                    exec::wait_for(5);
                } else {
                    // no children
                    if rmrk_owner.token_id.is_none() {
                        self.decrease_balance(&root_owner);
                    };
                    self.rmrk_owners.remove(&token_id);
                    self.token_approvals.remove(&token_id);
                    Ok(RMRKReply::Burnt)
                }
            }
            _ => {
                unreachable!()
            }
        }
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
    pub fn burn_from_parent(
        &mut self,
        tx_manager: &mut TxManager,
        token_id: TokenId,
    ) -> Result<RMRKReply, RMRKError> {
        let rmrk_owner = self.get_rmrk_owner(token_id)?;
        let state = tx_manager.get_state(msg::id());

        // Caller must be parent RMRK contract
        if msg::source() != rmrk_owner.owner_id {
            return Err(RMRKError::NotRMRKParentContract);
        }
        match state {
            TxState::Initial | TxState::ReplyOnBurnFromParentReceived => {
                if state == TxState::ReplyOnBurnFromParentReceived {
                    let child_token = tx_manager.get_payload::<(ActorId, TokenId)>()?;
                    self.internal_remove_child(token_id, child_token)?;
                }

                let msg_id = self.internal_burn_children(tx_manager, token_id);

                if msg_id != MessageId::zero() {
                    tx_manager.set_tx_state(TxState::MsgBurnFromParentSent, msg_id);
                    exec::wait_for(5);
                } else {
                    // no children
                    self.token_approvals.remove(&token_id);
                    self.rmrk_owners.remove(&token_id);
                    Ok(RMRKReply::TokenBurnt)
                }
            }
            TxState::Error(error) => Err(error),
            _ => {
                unreachable!()
            }
        }
    }

    // burn all pending and accepted children
    fn internal_burn_children(&self, tx_manager: &mut TxManager, token_id: TokenId) -> MessageId {
        if let Some(children) = self.pending_children.get(&token_id) {
            if let Some((child_contract_id, child_token_id)) = children.into_iter().next() {
                tx_manager.set_processing_msg((*child_contract_id, *child_token_id).encode());
                return burn_from_parent_msg(child_contract_id, *child_token_id);
            }
        }

        if let Some(children) = self.accepted_children.get(&token_id) {
            if let Some((child_contract_id, child_token_id)) = children.into_iter().next() {
                tx_manager.set_processing_msg((*child_contract_id, *child_token_id).encode());
                return burn_from_parent_msg(child_contract_id, *child_token_id);
            }
        }
        MessageId::zero()
    }
}
