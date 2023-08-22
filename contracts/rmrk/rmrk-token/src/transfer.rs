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
    pub fn transfer(
        &mut self,
        tx_manager: &mut TxManager,
        to: &ActorId,
        token_id: TokenId,
    ) -> Result<RMRKReply, RMRKError> {
        let state = tx_manager.get_state(msg::id());
        let rmrk_owner = self.get_rmrk_owner(token_id)?;
        match state {
            TxState::MsgSourceAccountChecked => {
                self.assert_zero_address(to)?;
                match rmrk_owner.token_id {
                    Some(parent_token_id) => {
                        let msg_id =
                            burn_child_msg(&rmrk_owner.owner_id, parent_token_id, token_id);
                        tx_manager.set_tx_state(TxState::MsgBurnChildSent, msg_id);
                        exec::wait_for(5);
                    }
                    None => {
                        let root_owner = rmrk_owner.owner_id;
                        self.decrease_balance(&root_owner);
                        self.increase_balance(to);
                        self.rmrk_owners.entry(token_id).and_modify(|rmrk| {
                            rmrk.owner_id = *to;
                        });
                        Ok(RMRKReply::Transferred)
                    }
                }
            }
            TxState::ReplyOnBurnChildReceived => {
                self.increase_balance(to);
                self.rmrk_owners.entry(token_id).and_modify(|rmrk| {
                    rmrk.owner_id = *to;
                    rmrk.token_id = None;
                });
                Ok(RMRKReply::Transferred)
            }
            _ => {
                unreachable!()
            }
        }
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
    pub fn transfer_to_nft(
        &mut self,
        tx_manager: &mut TxManager,
        to: &ActorId,
        destination_id: TokenId,
        token_id: TokenId,
    ) -> Result<RMRKReply, RMRKError> {
        self.assert_zero_address(to)?;
        let state = tx_manager.get_state(msg::id());
        let rmrk_owner = self.get_rmrk_owner(token_id)?;

        match state {
            TxState::MsgSourceAccountChecked => {
                if rmrk_owner.owner_id == *to {
                    let msg_id = transfer_child_msg(
                        to,
                        rmrk_owner.token_id.expect("Cant be None"),
                        destination_id,
                        token_id,
                    );
                    tx_manager.set_tx_state(TxState::MsgTransferChildSent, msg_id);
                    exec::wait_for(5);
                }

                if let Some(parent_token_id) = rmrk_owner.token_id {
                    let msg_id = burn_child_msg(&rmrk_owner.owner_id, parent_token_id, token_id);
                    tx_manager.set_tx_state(TxState::MsgBurnChildSent, msg_id);
                    exec::wait_for(5);
                }

                let msg_id = add_child_msg(to, destination_id, token_id);
                tx_manager.set_tx_state(TxState::MsgAddChildSent, msg_id);
                exec::wait_for(5);
            }
            TxState::ReplyOnBurnChildReceived => {
                let msg_id = add_child_msg(to, destination_id, token_id);
                tx_manager.set_tx_state(TxState::MsgAddChildSent, msg_id);
                exec::wait_for(5);
            }
            TxState::ReplyAddChildReceived | TxState::ReplyOnTransferChildReceived => {
                let root_owner = tx_manager.get_decoded_data::<ActorId>()?;
                let new_rmrk_owner = RMRKOwner {
                    owner_id: *to,
                    token_id: Some(destination_id),
                };
                if rmrk_owner.token_id.is_none() {
                    self.decrease_balance(&root_owner);
                }
                self.rmrk_owners.insert(token_id, new_rmrk_owner);
                Ok(RMRKReply::TransferredToNft)
            }
            _ => {
                unreachable!()
            }
        }
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
    pub fn approve(&mut self, to: &ActorId, token_id: TokenId) -> Result<RMRKReply, RMRKError> {
        self.assert_zero_address(to)?;

        self.if_token_exists(token_id)?;

        self.token_approvals
            .entry(token_id)
            .and_modify(|approvals| {
                approvals.insert(*to);
            })
            .or_insert_with(|| HashSet::from([*to]));
        Ok(RMRKReply::Approved)
    }
}
