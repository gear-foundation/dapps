use crate::non_fungible_token::token::*;
use gstd::{exec, msg, prelude::*, ActorId};
use sp_core::{
    sr25519::{Pair as Sr25519Pair, Public, Signature},
    Pair,
};

#[derive(Debug, Encode, Decode, TypeInfo, Clone)]
pub struct DelegatedApproveMessage {
    pub token_owner_id: ActorId,
    pub approved_actor_id: ActorId,
    pub nft_program_id: ActorId,
    pub token_id: TokenId,
    pub expiration_timestamp: u64,
}

impl DelegatedApproveMessage {
    pub(crate) fn validate(&self, signed_approve: &Signature, true_token_owner: &ActorId) {
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

        let owner = Public(self.token_owner_id.into());
        if !Sr25519Pair::verify(signed_approve, self.encode(), &owner) {
            panic!("Failed sign verification");
        }
    }
}
