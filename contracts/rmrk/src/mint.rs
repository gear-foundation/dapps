use crate::*;
use gstd::{msg, ActorId};

impl RMRKToken {
    /// Mints token that will belong to another token in another RMRK contract.
    ///
    /// # Requirements:
    /// * The `parent_id` must be a deployed RMRK contract.
    /// * The token with id `parent_token_id` must exist in `parent_id` contract.
    /// * The `token_id` must not exist.
    ///
    /// # Arguments:
    /// * `parent_id`: is the address of RMRK parent contract.
    /// * `parent_token_id`: is the parent RMRK token.
    /// * `token_id`: is the tokenId of new RMRK token.
    ///
    /// On success replies [`RMRKEvent::MintToNft`].
    pub fn mint_to_nft(
        &mut self,
        tx_manager: &mut TxManager,
        args: (ActorId, TokenId, TokenId),
    ) -> Result<RMRKReply, RMRKError> {
        let state = tx_manager.get_state(msg::id());
        let (parent_id, parent_token_id, token_id) = args;
        match state {
            TxState::Initial => {
                self.token_already_exists(token_id)?;
                let msg_id = add_child_msg(&parent_id, parent_token_id, token_id);
                tx_manager.set_tx_state(TxState::MsgAddChildSent, msg_id);
                exec::wait_for(5);
            }
            TxState::ReplyAddChildReceived => {
                self.internal_mint(token_id, &parent_id, Some(parent_token_id));
                tx_manager.set_tx_state(TxState::Completed, MessageId::zero());
                Ok(RMRKReply::MintedToNft)
            }
            TxState::Error(error) => Err(error),
            _ => {
                unreachable!()
            }
        }
    }

    /// Mints token to the user.
    ///
    /// # Requirements:
    /// * The `token_id` must not exist.
    /// * The `to` address should be a non-zero address.
    ///
    /// # Arguments:
    /// * `root_owner`: is the address who will own the token.
    /// * `token_id`: is the tokenId of new RMRK token.
    ///
    /// On success replies [`RMRKEvent::MintToRootOwner`].
    pub fn mint_to_root_owner(
        &mut self,
        root_owner: &ActorId,
        token_id: TokenId,
    ) -> Result<RMRKReply, RMRKError> {
        self.assert_zero_address(root_owner)?;
        // check that token does not exist
        self.token_already_exists(token_id)?;
        self.increase_balance(root_owner);
        self.internal_mint(token_id, root_owner, None);

        Ok(RMRKReply::MintedToRootOwner)
    }

    pub fn internal_mint(
        &mut self,
        token_id: TokenId,
        parent_id: &ActorId,
        parent_token_id: Option<TokenId>,
    ) {
        self.rmrk_owners.insert(
            token_id,
            RMRKOwner {
                token_id: parent_token_id,
                owner_id: *parent_id,
            },
        );
    }

    pub fn increase_balance(&mut self, account: &ActorId) {
        self.balances
            .entry(*account)
            .and_modify(|balance| *balance += 1.into())
            .or_insert_with(|| 1.into());
    }

    pub fn decrease_balance(&mut self, account: &ActorId) {
        self.balances
            .entry(*account)
            .and_modify(|balance| *balance -= 1.into());
    }
}
