#![no_std]

use gstd::{collections::HashMap, msg, prelude::*};
use identity_io::*;

#[derive(Debug, Default)]
struct IdentityStorage {
    user_claims: HashMap<PublicKey, HashMap<PieceId, Claim>>,
    piece_counter: u128,
}

static mut IDENTITY: Option<IdentityStorage> = None;

impl IdentityStorage {
    /// Creates a new claim.
    ///
    /// # Requirements:
    /// * all the public keys and signatures MUST be non-zero.
    ///
    /// # Arguments:
    /// * `issuer` - the claim issuer's public key.
    /// * `issuer_signature` - the corresponding signature with the `issuer` public key.
    /// * `subject`- the subject's public key.
    /// * `data` - claim's data.
    fn issue_claim(
        &mut self,
        issuer: PublicKey,
        issuer_signature: Signature,
        subject: PublicKey,
        data: ClaimData,
    ) {
        self.user_claims.entry(subject).or_default().insert(
            self.piece_counter,
            Claim {
                issuer,
                issuer_signature,
                subject,
                verifiers: vec![],
                data,
            },
        );

        msg::reply(
            IdentityEvent::ClaimIssued {
                issuer,
                subject,
                piece_id: self.piece_counter,
            },
            0,
        )
        .expect("IDENTITY: Error during replying with IdentityEvent::ClaimIssued");

        self.piece_counter += 1;
    }

    /// Changes claim's validation status.
    ///
    /// # Requirements:
    /// * all the public keys and signatures MUST be non-zero.
    ///
    /// # Arguments:
    /// * `validator` - the claim issuer's or subject's public key.
    /// * `subject`- the subject's public key.
    /// * `piece_id` - claim's id.
    /// * `status` - new claim's status.
    fn change_validation_status(
        &mut self,
        validator: PublicKey,
        subject: PublicKey,
        piece_id: PieceId,
        status: bool,
    ) {
        let data_piece = self
            .user_claims
            .get(&subject)
            .expect("The user has no claims")
            .get(&piece_id)
            .expect("The user has not such claim with the provided piece_id");
        if data_piece.subject != validator && data_piece.issuer != validator {
            panic!("IDENTITY: You can not change this claim");
        }
        self.user_claims
            .entry(subject)
            .or_default()
            .entry(piece_id)
            .and_modify(|claim| claim.data.valid = status);

        msg::reply(
            IdentityEvent::ClaimValidationChanged {
                validator,
                subject,
                piece_id,
                status,
            },
            0,
        )
        .expect("IDENTITY: Error during replying with IdentityEvent::ClaimValidationChanged");
    }

    /// Verifies the claim.
    ///
    /// # Requirements:
    /// * all the public keys and signatures MUST be non-zero.
    /// * `verifier` - MUST differ from the claim's subject or issuer.
    ///
    /// # Arguments:
    /// * `verifier` - the claim verifier's public key.
    /// * `verifier_signature` - the corresponding signature with the `verifier` public key.
    /// * `piece_id` - claim's id.
    /// * `subject` - subject's public key.
    fn verify_claim(
        &mut self,
        verifier: PublicKey,
        verifier_signature: Signature,
        subject: PublicKey,
        piece_id: PieceId,
    ) {
        let piece = self
            .user_claims
            .get(&subject)
            .expect("The user has no claims")
            .get(&piece_id)
            .expect("The user has not such claim with the provided piece_id");
        if piece.issuer == verifier || piece.subject == verifier {
            panic!("IDENTITY: You can not verify this claim");
        }
        self.user_claims
            .entry(subject)
            .or_default()
            .entry(piece_id)
            .and_modify(|claim| {
                claim.verifiers.push((verifier, verifier_signature));
            });
        msg::reply(
            IdentityEvent::VerifiedClaim {
                verifier,
                subject,
                piece_id,
            },
            0,
        )
        .expect("IDENTITY: Error during replying with IdentityEvent::VerifiedClaim");
    }
}

#[no_mangle]
extern fn init() {
    let id_storage = IdentityStorage {
        piece_counter: 0,
        ..Default::default()
    };
    unsafe {
        IDENTITY = Some(id_storage);
    }
}

#[gstd::async_main]
async fn main() {
    let action: IdentityAction = msg::load().expect("Unable to decode IdentityAction");
    let identity = unsafe { IDENTITY.get_or_insert(Default::default()) };
    match action {
        IdentityAction::IssueClaim {
            issuer,
            issuer_signature,
            subject,
            data,
        } => identity.issue_claim(issuer, issuer_signature, subject, data),
        IdentityAction::ChangeClaimValidationStatus {
            validator,
            subject,
            piece_id,
            status,
        } => identity.change_validation_status(validator, subject, piece_id, status),
        IdentityAction::VerifyClaim {
            verifier,
            verifier_signature,
            subject,
            piece_id,
        } => identity.verify_claim(verifier, verifier_signature, subject, piece_id),
    }
}

#[no_mangle]
extern fn state() {
    let identity = unsafe { IDENTITY.get_or_insert(Default::default()) };

    msg::reply(
        State {
            user_claims: identity
                .user_claims
                .clone()
                .into_iter()
                .map(|(k, v)| (k, v.into_iter().collect()))
                .collect(),
            piece_counter: identity.piece_counter,
        },
        0,
    )
    .expect("Error during replying from `state()`");
}
