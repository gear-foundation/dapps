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
    pub async fn mint_to_nft(
        &mut self,
        parent_id: &ActorId,
        parent_token_id: TokenId,
        token_id: TokenId,
    ) {
        self.assert_token_exists(token_id);

        // message to destination contract about adding child
        add_child(parent_id, parent_token_id, token_id).await;

        // find the root owner
        let root_owner = get_root_owner(parent_id, parent_token_id).await;

        self.internal_mint(&root_owner, token_id, parent_id, Some(parent_token_id));

        msg::reply(
            RMRKEvent::MintToNft {
                parent_id: *parent_id,
                parent_token_id,
                token_id,
            },
            0,
        )
        .expect("Error in reply [RMRKEvent::MintToNft]");
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
    pub fn mint_to_root_owner(&mut self, root_owner: &ActorId, token_id: TokenId) {
        self.assert_zero_address(root_owner);
        // check that token does not exist
        self.assert_token_exists(token_id);

        self.internal_mint(root_owner, token_id, root_owner, None);

        msg::reply(
            RMRKEvent::MintToRootOwner {
                root_owner: *root_owner,
                token_id,
            },
            0,
        )
        .expect("Error in reply [RMRKEvent::MintToRootOwner]");
    }

    pub fn internal_mint(
        &mut self,
        root_owner: &ActorId,
        token_id: TokenId,
        parent_id: &ActorId,
        parent_token_id: Option<TokenId>,
    ) {
        self.increase_balance(root_owner);
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
