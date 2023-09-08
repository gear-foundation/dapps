use crate::*;
use gstd::msg;

impl RMRKToken {
    /// That message is designed to be send from another RMRK contracts
    /// when minting an NFT(child_token_id) to another NFT(parent_token_id).
    /// It adds a child to the NFT with tokenId `parent_token_id`
    /// The status of added child is `Pending`.
    ///
    /// # Requirements:
    /// * Token with TokenId `parent_token_id` must exist.
    /// * There cannot be two identical children.
    ///
    /// # Arguments:
    /// * `parent_token_id`: is the tokenId of the parent NFT.
    /// * `child_token_id`: is the tokenId of the child instance.
    ///
    /// On success replies [`RMRKEvent::PendingChild`].
    pub fn add_child(
        &mut self,
        //    tx_manager: &mut TxManager,
        parent_token_id: TokenId,
        child_token_id: TokenId,
    ) -> Result<RMRKReply, RMRKError> {
        self.if_token_exists(parent_token_id)?;
        //   let (state, tx_data) = tx_manager.get_state_and_data(msg::id());
        let child_token = (msg::source(), child_token_id);

        // check if the child already exists in pending array
        if let Some(children) = self.pending_children.get(&parent_token_id) {
            // if child already exists
            if children.contains(&child_token) {
                return Err(RMRKError::ChildInPendingArray);
            }
        }

        // check if the child already exists in pending array
        if let Some(children) = self.accepted_children.get(&parent_token_id) {
            // if child already exists
            if children.contains(&child_token) {
                return Err(RMRKError::ChildInAcceptedArray);
            }
        }

        // add child to pending children array
        self.internal_add_child(parent_token_id, child_token, ChildStatus::Pending);

        Ok(RMRKReply::PendingChildAdded)
    }

    /// Accepts an RMRK child being in the `Pending` status.
    /// Removes RMRK child from `pending_children` and adds to `accepted_children`.
    ///
    /// # Requirements:
    /// * The `msg::source()` must be an RMRK owner of NFT with tokenId `parent_token_id` or an approved account.
    /// * The indicated NFT with tokenId `child_token_id` must exist in the pending array of `parent_token_id`.
    ///
    /// # Arguments:
    /// * `parent_token_id`: is the tokenId of the parent NFT
    /// * `child_token_id`: is the tokenId of the child instance
    ///
    /// On success replies [`RMRKEvent::AcceptedChild`].
    pub fn accept_child(
        &mut self,
        parent_token_id: TokenId,
        child_contract_id: ActorId,
        child_token_id: TokenId,
    ) -> Result<RMRKReply, RMRKError> {
        let child_token = (child_contract_id, child_token_id);

        self.check_child_status(child_token, ChildStatus::Pending)?;

        // remove child from pending array
        self.internal_remove_child(parent_token_id, child_token)?;

        // add child to accepted children array
        self.internal_add_child(parent_token_id, child_token, ChildStatus::Accepted);

        Ok(RMRKReply::ChildAccepted)
    }

    /// Rejects an RMRK child being in the `Pending` status or
    /// removes an RMRK child being in the `Accepted` status.
    /// It sends message to the child NFT contract to burn NFT token from it.
    ///
    /// # Requirements:
    /// * The `msg::source()` must be an RMRK owner or an approved account.
    ///
    /// # Arguments:
    /// * `parent_token_id`: is the tokenId of the parent NFT.
    /// * `child_contract_id`: is the address of the child RMRK contract.
    /// * `child_token_id`: is the tokenId of the child instance.
    ///
    /// On success replies [`RMRKEvent::RejectedChild`] or [`RMRKEvent::RemovedChild`].
    pub fn remove_or_reject_child(
        &mut self,
        tx_manager: &mut TxManager,
        parent_token_id: TokenId,
        child_contract_id: ActorId,
        child_token_id: TokenId,
        child_status: ChildStatus,
    ) -> Result<RMRKReply, RMRKError> {
        self.check_child_status((child_contract_id, child_token_id), child_status)?;
        let state = tx_manager.get_state(msg::id());

        match state {
            TxState::MsgSourceAccountChecked => {
                let msg_id = burn_from_parent_msg(&child_contract_id, child_token_id);
                tx_manager.set_tx_state(TxState::MsgBurnFromParentSent, msg_id);
                exec::wait_for(5);
            }
            TxState::ReplyOnBurnFromParentReceived => {
                // remove child from pending array
                let child_token = (child_contract_id, child_token_id);
                self.internal_remove_child(parent_token_id, child_token)?;
                match child_status {
                    ChildStatus::Pending => Ok(RMRKReply::ChildRejected),
                    ChildStatus::Accepted => Ok(RMRKReply::ChildRemoved),
                }
            }
            _ => {
                unreachable!()
            }
        }
    }

    /// That message is designed to be sent from another RMRK contracts
    /// when root owner transfers his child to another parent token within one contract.
    /// If root owner transfers child token from NFT to another his NFT
    /// it adds a child to the NFT  with a status that child had before.
    /// If root owner transfers child token from NFT to another NFT that he does not own
    /// it adds a child to the NFT  with a status `Pending`.
    ///
    /// # Requirements:
    /// * The `msg::source()` must be a child RMRK contract.
    /// * The `to` must be an existing RMRK token
    ///
    /// # Arguments:
    /// * `from`: RMRK token from which the child token will be transferred.
    /// * `to`: RMRK token to which the child token will be transferred.
    /// * `child_token_id`: is the tokenId of the child in the RMRK child contract.
    ///
    /// On success replies [`RMRKEvent::ChildTransferred`].
    pub fn transfer_child(
        &mut self,
        from: TokenId,
        to: TokenId,
        child_token_id: TokenId,
    ) -> Result<RMRKReply, RMRKError> {
        self.if_token_exists(from)?;
        self.if_token_exists(to)?;

        let child_token = (msg::source(), child_token_id);

        self.internal_remove_child(from, child_token)?;
        self.internal_add_child(to, child_token, ChildStatus::Pending);

        Ok(RMRKReply::ChildTransferred)
    }

    /// Burns a child of NFT.
    /// That function must be called from the child RMRK contract during `transfer`, `transfer_to_nft` and `burn` functions.
    ///
    /// # Requirements:
    /// * The `msg::source()` must be a child RMRK contract.
    /// * The indicated child must exist the children list of `parent_token_id`.
    ///
    /// # Arguments:
    /// * `parent_token_id`: is the tokenId of the parent NFT.
    /// * `child_token_id`: is the tokenId of the child instance.
    ///
    /// On success replies [`RMRKEvent::ChildBurnt`].
    pub fn burn_child(
        &mut self,
        parent_token_id: TokenId,
        child_token_id: TokenId,
    ) -> Result<RMRKReply, RMRKError> {
        let child_token = (msg::source(), child_token_id);

        self.internal_remove_child(parent_token_id, child_token)?;

        Ok(RMRKReply::ChildBurnt)
    }

    fn internal_add_child(
        &mut self,
        parent_token_id: TokenId,
        child_token: CollectionAndToken,
        child_status: ChildStatus,
    ) {
        match child_status {
            ChildStatus::Pending => {
                self.pending_children
                    .entry(parent_token_id)
                    .and_modify(|children| {
                        children.insert(child_token);
                    })
                    .or_insert_with(|| HashSet::from([child_token]));

                self.children_status
                    .insert(child_token, ChildStatus::Pending);
            }
            ChildStatus::Accepted => {
                self.accepted_children
                    .entry(parent_token_id)
                    .and_modify(|children| {
                        children.insert(child_token);
                    })
                    .or_insert_with(|| HashSet::from([child_token]));

                self.children_status
                    .insert(child_token, ChildStatus::Accepted);
            }
        }
    }

    pub fn internal_remove_child(
        &mut self,
        parent_token_id: TokenId,
        child_token: CollectionAndToken,
    ) -> Result<(), RMRKError> {
        let child_status = self.get_child_status(child_token)?;

        match child_status {
            ChildStatus::Pending => {
                if let Some(children) = self.pending_children.get_mut(&parent_token_id) {
                    if children.remove(&child_token) {
                        self.children_status.remove(&child_token);
                        return Ok(());
                    }
                }
            }
            ChildStatus::Accepted => {
                if let Some(children) = self.accepted_children.get_mut(&parent_token_id) {
                    if children.remove(&child_token) {
                        self.children_status.remove(&child_token);
                        return Ok(());
                    }
                }
            }
        }
        Err(RMRKError::TokenDoesNotExist)
    }
}
