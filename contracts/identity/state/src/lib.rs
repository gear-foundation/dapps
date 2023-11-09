#![no_std]

use gstd::prelude::*;
use identity_io::*;

#[gmeta::metawasm]
pub mod metafns {
    pub type State = identity_io::State;

    pub fn query(mut state: State, query: IdentityStateQuery) -> IdentityStateReply {
        match query {
            IdentityStateQuery::UserClaims(pkey) => {
                IdentityStateReply::UserClaims(match state.user_claims.get(&pkey) {
                    None => vec![],
                    Some(claims) => Vec::from_iter(claims.clone()),
                })
            }
            IdentityStateQuery::Claim(pkey, piece_id) => IdentityStateReply::Claim(
                state
                    .user_claims
                    .entry(pkey)
                    .or_default()
                    .get(&piece_id)
                    .cloned(),
            ),
            IdentityStateQuery::ValidationStatus(pkey, piece_id) => {
                let mut status = false;
                if let Some(user_claim) = state.user_claims.get(&pkey) {
                    if let Some(claim) = user_claim.get(&piece_id) {
                        status = claim.data.valid
                    }
                }
                IdentityStateReply::ValidationStatus(status)
            }
            IdentityStateQuery::Date(pkey, piece_id) => {
                let mut date: u64 = 0;
                if let Some(user_claim) = state.user_claims.get(&pkey) {
                    if let Some(claim) = user_claim.get(&piece_id) {
                        date = claim.data.issuance_date
                    }
                }
                IdentityStateReply::Date(date)
            }
            IdentityStateQuery::Verifiers(pkey, piece_id) => {
                let mut verifiers: Vec<PublicKey> = vec![];
                if let Some(user_claim) = state.user_claims.get(&pkey) {
                    if let Some(claim) = user_claim.get(&piece_id) {
                        let (public_keys, _signatures): (Vec<PublicKey>, Vec<Signature>) =
                            claim.verifiers.clone().into_iter().unzip();
                        verifiers = public_keys;
                    }
                }
                IdentityStateReply::Verifiers(verifiers)
            }
            IdentityStateQuery::CheckClaim(pkey, piece_id, hash) => {
                let mut status = false;
                if let Some(user_claim) = state.user_claims.get(&pkey) {
                    if let Some(claim) = user_claim.get(&piece_id) {
                        status = claim.data.hashed_info.contains(&hash)
                    }
                }
                IdentityStateReply::CheckedClaim(pkey, piece_id, status)
            }
        }
    }
}
