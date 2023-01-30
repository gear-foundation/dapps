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
    pub async fn add_child(&mut self, parent_token_id: TokenId, child_token_id: TokenId) {
        self.assert_token_does_not_exist(parent_token_id);

        let child_token = (msg::source(), child_token_id);

        // check if the child already exists in pending array
        if let Some(children) = self.pending_children.get(&parent_token_id) {
            // if child already exists
            if children.contains(&child_token) {
                panic!("RMRKCore: child already exists in pending array");
            }
        }

        // add child to pending children array
        self.internal_add_child(parent_token_id, child_token, ChildStatus::Pending);

        msg::reply(
            RMRKEvent::PendingChild {
                child_token_address: msg::source(),
                child_token_id,
                parent_token_id,
            },
            0,
        )
        .expect("Error in reply [RMRKEvent::PendingChild]");
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
    pub async fn accept_child(
        &mut self,
        parent_token_id: TokenId,
        child_contract_id: ActorId,
        child_token_id: TokenId,
    ) {
        let root_owner = self.find_root_owner(parent_token_id).await;
        self.assert_approved_or_owner(parent_token_id, &root_owner);

        let child_token = (child_contract_id, child_token_id);

        // remove child from pending array
        self.internal_remove_child(parent_token_id, child_token, ChildStatus::Pending);

        // add child to accepted children array
        self.internal_add_child(parent_token_id, child_token, ChildStatus::Accepted);

        msg::reply(
            RMRKEvent::AcceptedChild {
                child_contract_id,
                child_token_id,
                parent_token_id,
            },
            0,
        )
        .expect("Error in reply [RMRKEvent::AcceptedChild]");
    }

    /// Rejects an RMRK child being in the `Pending` status.
    /// It sends message to the child NFT contract to burn NFT token from it.
    ///
    /// # Requirements:
    /// * The `msg::source()` must be an RMRK owner or an approved account.
    /// * The indicated NFT with tokenId `child_token_id` must exist in the pending array of `parent_token_id`.
    ///
    /// Arguments:
    /// * `parent_token_id`: is the tokenId of the parent NFT.
    /// * `child_contract_id`: is the address of the child RMRK contract.
    /// * `child_token_id`: is the tokenId of the child instance.
    ///
    /// On success replies [`RMRKEvent::RejectedChild`].
    pub async fn reject_child(
        &mut self,
        parent_token_id: TokenId,
        child_contract_id: ActorId,
        child_token_id: TokenId,
    ) {
        let root_owner = self.find_root_owner(parent_token_id).await;
        self.assert_approved_or_owner(parent_token_id, &root_owner);

        let child_token = (child_contract_id, child_token_id);

        // remove child from pending array
        self.internal_remove_child(parent_token_id, child_token, ChildStatus::Pending);

        // send message to child contract to burn RMRK token from it
        burn_from_parent(&child_contract_id, child_token_id, &root_owner).await;

        msg::reply(
            RMRKEvent::RejectedChild {
                child_contract_id,
                child_token_id,
                parent_token_id,
            },
            0,
        )
        .expect("Error in reply [RMRKEvent::RejectedChild]");
    }

    /// Removes an RMRK child being in the `Accepted` status.
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
    /// On success replies [`RMRKEvent::RemovedChild`].
    pub async fn remove_child(
        &mut self,
        parent_token_id: TokenId,
        child_contract_id: ActorId,
        child_token_id: TokenId,
    ) {
        let root_owner = self.find_root_owner(parent_token_id).await;
        self.assert_approved_or_owner(parent_token_id, &root_owner);

        let child_token = (child_contract_id, child_token_id);

        // remove child from accepted children array
        self.internal_remove_child(parent_token_id, child_token, ChildStatus::Accepted);

        // send message to child contract to burn RMRK token from it
        burn_from_parent(&child_contract_id, child_token_id, &root_owner).await;

        msg::reply(
            RMRKEvent::RemovedChild {
                child_contract_id,
                child_token_id,
                parent_token_id,
            },
            0,
        )
        .expect("Error in reply `[RMRKEvent::RejectedChild]`");
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
    /// * The `root_owner` of `to` and `from` must be the same.
    ///
    /// # Arguments:
    /// * `from`: RMRK token from which the child token will be transferred.
    /// * `to`: RMRK token to which the child token will be transferred.
    /// * `child_token_id`: is the tokenId of the child in the RMRK child contract.
    ///
    /// On success replies [`RMRKEvent::ChildTransferred`].
    pub async fn transfer_child(&mut self, from: TokenId, to: TokenId, child_token_id: TokenId) {
        self.assert_token_does_not_exist(to);

        let child_token = (msg::source(), child_token_id);

        // check the status of the child
        let child_status = self
            .children_status
            .get(&child_token)
            .expect("RMRK: The child does not exist");

        let from_root_owner = self.find_root_owner(from).await;
        let to_root_owner = self.find_root_owner(to).await;
        self.assert_exec_origin(&from_root_owner);
        match child_status {
            ChildStatus::Pending => {
                self.internal_remove_child(from, child_token, ChildStatus::Pending);
                self.internal_add_child(to, child_token, ChildStatus::Pending);
            }
            ChildStatus::Accepted => {
                self.internal_remove_child(from, child_token, ChildStatus::Accepted);
                if from_root_owner == to_root_owner {
                    self.internal_add_child(to, child_token, ChildStatus::Accepted);
                } else {
                    self.internal_add_child(to, child_token, ChildStatus::Pending);
                }
            }
        }
        msg::reply(
            RMRKEvent::ChildTransferred {
                from,
                to,
                child_contract_id: msg::source(),
                child_token_id,
            },
            0,
        )
        .expect("Error in reply `[RMRKEvent::ChildTransferred]`");
    }

    /// That function is designed to be called from another RMRK contracts
    /// when root owner transfers his child NFT to another his NFT in another contract.
    /// It adds a child to the RMRK token with tokenId `parent_token_id` with status `Accepted`.
    ///
    /// # Requirements:
    /// * The `msg::source()` must be a child RMRK contract.
    /// * The `parent_token_id` must be an existing RMRK token that must have `child_token_id` in its `accepted_children`.
    ///
    /// # Arguments:
    /// * `parent_token_id`: RMRK token to which the child token will be transferred.
    /// * `child_token_id`: is the tokenId of the child of the RMRK child contract.
    ///
    /// On success replies [`RMRKEvent::AcceptedChild`].
    pub async fn add_accepted_child(&mut self, parent_token_id: TokenId, child_token_id: TokenId) {
        let root_owner = self.find_root_owner(parent_token_id).await;
        self.assert_exec_origin(&root_owner);

        let child_token = (msg::source(), child_token_id);

        self.internal_add_child(parent_token_id, child_token, ChildStatus::Accepted);

        msg::reply(
            RMRKEvent::AcceptedChild {
                child_contract_id: msg::source(),
                child_token_id,
                parent_token_id,
            },
            0,
        )
        .expect("Error in reply `[RMRKEvent::AcceptedChild]`");
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
    pub fn burn_child(&mut self, parent_token_id: TokenId, child_token_id: TokenId) {
        let child_token = (msg::source(), child_token_id);
        let child_status = self
            .children_status
            .remove(&child_token)
            .expect("Child does not exist");

        self.internal_remove_child(parent_token_id, child_token, child_status);

        msg::reply(
            RMRKEvent::ChildBurnt {
                parent_token_id,
                child_token_id,
            },
            0,
        )
        .expect("Error in reply [`RMRKEvent::ChildBurnt`]");
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

    fn internal_remove_child(
        &mut self,
        parent_token_id: TokenId,
        child_token: CollectionAndToken,
        child_status: ChildStatus,
    ) {
        self.children_status.remove(&child_token);
        match child_status {
            ChildStatus::Pending => {
                if let Some(children) = self.pending_children.get_mut(&parent_token_id) {
                    if !children.remove(&child_token) {
                        panic!("RMRK: child does not exist in pending array or has already been accepted");
                    }
                } else {
                    panic!("RMRK: there are no pending children at all");
                }
            }
            ChildStatus::Accepted => {
                if let Some(children) = self.accepted_children.get_mut(&parent_token_id) {
                    if children.contains(&child_token) {
                        children.remove(&child_token);
                    } else {
                        panic!("RMRK: child does not exist");
                    }
                }
            }
        }
    }
}
