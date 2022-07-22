use crate::*;
use gstd::{exec, msg, ActorId};
use types::primitives::ResourceId;

impl RMRKToken {
    pub fn assert_zero_address(&self, account: &ActorId) {
        assert!(account != &ActorId::zero(), "RMRK: Zero address");
    }

    /// Checks that NFT with indicated ID already exists
    pub fn assert_token_exists(&self, token_id: TokenId) {
        if self.rmrk_owners.contains_key(&token_id) {
            panic!("RMRK: Token already exists");
        }
    }

    /// Checks that NFT with indicated ID already does not exist
    pub fn assert_token_does_not_exist(&self, token_id: TokenId) {
        if !self.rmrk_owners.contains_key(&token_id) {
            panic!("RMRK: Token does not exist");
        }
    }

    /// Checks that `msg::source()` is the owner of the token with indicated `token_id`
    pub fn assert_owner(&self, root_owner: &ActorId) {
        debug!("OWNER {:?}", root_owner);
        if msg::source() != *root_owner {
            panic!("RMRK: Wrong owner");
        }
    }

    /// Checks that `exec::origin()` is the owner of the token with indicated `token_id`
    pub fn assert_exec_origin(&self, root_owner: &ActorId) {
        debug!("EXEC OWNER {:?}", root_owner);
        if exec::origin() != *root_owner {
            panic!("Wrong owner");
        }
    }
    pub fn assert_approved_or_owner(&self, token_id: TokenId, root_owner: &ActorId) {
        if !matches!(
            self.token_approvals.get(&token_id),
            Some(approved_accounts) if approved_accounts.contains(&msg::source())
        ) {
            self.assert_owner(root_owner);
        }
    }

    pub fn assert_resource_exists_on_token(&self, token_id: TokenId, resource_id: ResourceId) {
        if let Some(active_resources) = self.multiresource.active_resources.get(&token_id) {
            assert!(
                active_resources.contains(&resource_id),
                "The resource does not exist or not accepted"
            );
        } else {
            panic!("Token has no active resources");
        }
    }
}
