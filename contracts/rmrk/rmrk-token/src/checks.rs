use crate::*;
use gstd::{msg, ActorId};
use types::primitives::CollectionId;

impl RMRKToken {
    pub fn assert_zero_address(&self, account: &ActorId) -> Result<(), RMRKError> {
        if account == &ActorId::zero() {
            return Err(RMRKError::ZeroIdForbidden);
        }
        Ok(())
    }
    pub fn check_child_status(
        &self,
        child_token: (CollectionId, TokenId),
        child_status: ChildStatus,
    ) -> Result<(), RMRKError> {
        if let Some(status) = self.children_status.get(&child_token) {
            if *status != child_status {
                return Err(RMRKError::WrongChildStatus);
            }
        } else {
            return Err(RMRKError::ChildDoesNotExist);
        }
        Ok(())
    }

    /// Checks that NFT with indicated ID already exists
    pub fn token_already_exists(&self, token_id: TokenId) -> Result<(), RMRKError> {
        if self.rmrk_owners.contains_key(&token_id) {
            return Err(RMRKError::TokenAlreadyExists);
        }
        Ok(())
    }

    /// Checks that NFT with indicated ID already does not exist
    pub fn if_token_exists(&self, token_id: TokenId) -> Result<(), RMRKError> {
        if !self.rmrk_owners.contains_key(&token_id) {
            return Err(RMRKError::TokenDoesNotExist);
        }
        Ok(())
    }

    /// Checks that `msg::source()` is the owner of the token with indicated `token_id`
    pub fn assert_owner(&self, root_owner: &ActorId) {
        if msg::source() != *root_owner {
            panic!("RMRK: Wrong owner");
        }
    }

    pub fn get_rmrk_owner(&self, token_id: TokenId) -> Result<&RMRKOwner, RMRKError> {
        if let Some(rmrk_owner) = self.rmrk_owners.get(&token_id) {
            Ok(rmrk_owner)
        } else {
            Err(RMRKError::TokenDoesNotExist)
        }
    }

    pub fn get_child_status(
        &self,
        child_token: (CollectionId, TokenId),
    ) -> Result<&ChildStatus, RMRKError> {
        if let Some(status) = self.children_status.get(&child_token) {
            Ok(status)
        } else {
            Err(RMRKError::ChildDoesNotExist)
        }
    }
}
