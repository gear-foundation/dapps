use crate::non_fungible_token::token::*;
use gstd::{exec, msg, prelude::*, ActorId};

#[derive(
    Default, Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, TypeInfo, Hash,
)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct DelegatedApproveMessage {
    pub token_owner_id: ActorId,
    pub approved_actor_id: ActorId,
    pub nft_program_id: ActorId,
    pub token_id: TokenId,
    pub expiration_timestamp: u64,
}

impl DelegatedApproveMessage {
    pub(crate) fn validate(&self, signed_approve: &[u8], true_token_owner: &ActorId) {
        if msg::source() != self.approved_actor_id {
            panic!("Source is wrong, msg::source must be equal to approved_actor_id")
        }

        if exec::program_id() != self.nft_program_id {
            panic!("You have tried to use delegated_approve with wrong program")
        }

        if self.approved_actor_id == ActorId::zero() {
            panic!("Zero address, just use burn if you want to remove token");
        }

        if true_token_owner != &self.token_owner_id {
            panic!("This user doesn't own the token")
        }

        if exec::block_timestamp() >= self.expiration_timestamp {
            panic!("Delegated approve has expired")
        }

        let owner: [u8; 32] = self.token_owner_id.into();
        if crate::sr25519::verify(signed_approve, self.encode(), owner).is_err() {
            panic!("Failed sign verification");
        }
    }
}
