#![no_std]

use gmeta::{In, InOut, Metadata, Out};
use gstd::{collections::BTreeMap, prelude::*};

/// Typings for u8 arrays.
pub type PublicKey = [u8; 32];
pub type Signature = [u8; 64];
pub type PieceId = u128;

pub struct ContractMetadata;

impl Metadata for ContractMetadata {
    type Init = In<InitIdentity>;
    type Handle = InOut<IdentityAction, IdentityEvent>;
    type Reply = ();
    type Others = ();
    type Signal = ();
    type State = Out<State>;
}

#[derive(TypeInfo, Encode, Decode)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct State {
    pub user_claims: BTreeMap<PublicKey, BTreeMap<PieceId, Claim>>,
    pub piece_counter: u128,
}

/// ClaimData represents an internal data stored inside a claim.
#[derive(Decode, Encode, TypeInfo, Debug, Clone, PartialEq)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct ClaimData {
    /// Set of hashed data (e.g. Vec::from(`[city]`, `[street]`)).
    pub hashed_info: Vec<[u8; 32]>,
    /// Date of issuance of this claim.
    pub issuance_date: u64,
    /// Validation status of the claim.
    pub valid: bool,
}

/// Claim is a main object stored inside the identity storage.
/// Consists of the claim data and all the public keys and signatures.
///
/// # Requirements:
/// * all public keys and signatures MUST be non-zero arrays
#[derive(Decode, Encode, TypeInfo, Debug, Clone, PartialEq)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct Claim {
    /// Issuer's  public key (e.g. who issued the claim). Can be equal to subject keys
    /// if the subject issues any claim about himself.
    pub issuer: PublicKey,
    /// Issuer's signature with the issuer keypair.
    pub issuer_signature: Signature,
    /// Subject's public key.
    pub subject: PublicKey,
    /// Map of verifiers PublicKey -> Signature
    pub verifiers: Vec<(PublicKey, Signature)>,
    /// Internal data of the claim
    pub data: ClaimData,
}

#[derive(Debug, Decode, Encode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum IdentityAction {
    /// Issues a new claim either by a subject himself
    /// or by an issuer on behalf of the subject
    ///
    /// # Requirements:
    /// * all public keys and signatures MUST be non-zero arrays
    IssueClaim {
        /// Issuer's public key.
        issuer: PublicKey,
        /// Issuer's signature with his keypair.
        issuer_signature: Signature,
        /// Subject's public key.
        subject: PublicKey,
        /// Claim's data.
        data: ClaimData,
    },
    /// Changes a validation status of the claim.
    /// Can only be performed by a subject or an issuer of the claim.
    ///
    /// # Requirements:
    /// * all public keys and signatures MUST be non-zero arrays
    ChangeClaimValidationStatus {
        /// Validator's public key. Can be either a subject's or an issuer's one.
        validator: PublicKey,
        /// Subject's public key.
        subject: PublicKey,
        /// Claim's id.
        piece_id: PieceId,
        /// New status of the claim.
        status: bool,
    },
    /// Verify a specific claim with a public key and a signature.
    /// Can not be performed by an issuer or a subject.
    ///
    /// # Requirements:
    /// * all public keys and signatures MUST be non-zero arrays
    VerifyClaim {
        /// Verifier's public key.
        verifier: PublicKey,
        /// Verifier's signature.
        verifier_signature: Signature,
        /// Subject's public key.
        subject: PublicKey,
        /// Claim's id.
        piece_id: PieceId,
    },
}

#[derive(Debug, Decode, Encode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum IdentityEvent {
    ClaimIssued {
        /// Issuer's public key.
        issuer: PublicKey,
        /// Subject's public key.
        subject: PublicKey,
        /// Claim's id generated automatically.
        piece_id: PieceId,
    },
    ClaimValidationChanged {
        /// Validator's public key.
        validator: PublicKey,
        /// Subjects's public key.
        subject: PublicKey,
        /// Claims' id.
        piece_id: PieceId,
        /// Claim's new validation status.
        status: bool,
    },
    VerifiedClaim {
        /// Verifier's public key.
        verifier: PublicKey,
        /// Subject's public key.
        subject: PublicKey,
        /// Claim's id.
        piece_id: PieceId,
    },
}

#[derive(Debug, Decode, Encode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum IdentityStateQuery {
    /// Get all the claims for a specified public key.
    ///
    /// Arguments:
    /// `PublicKey` - is the public key of a user whose claims are queried
    UserClaims(PublicKey),
    /// Get a specific claim with the provided public key and a claim id.
    ///
    /// Arguments:
    /// `PublicKey` - is the public key of a user whose claim is queried
    /// `PieceId` - is the claim id
    Claim(PublicKey, PieceId),
    /// Get all the verifiers' public keys for a corresponding claim.
    ///
    /// Arguments:
    /// `PublicKey` - is the public key of a user whose claim is queried
    /// `PieceId` - is the claim id
    Verifiers(PublicKey, PieceId),
    /// Get claim's validation status.
    ///
    /// Arguments:
    /// `PublicKey` - is the public key of a user whose claim is queried
    /// `PieceId` - is the claim id
    ValidationStatus(PublicKey, PieceId),
    /// Get claim's issuance date.
    ///
    /// Arguments:
    /// `PublicKey` - is the public key of a user whose claim is queried
    /// `PieceId` - is the claim id
    Date(PublicKey, PieceId),
    /// Check the claim with a hash from it's data set.
    ///
    /// Arguments:
    /// `PublicKey` - is the public key of a user whose claim is queried
    /// `PieceId` - is the claim id
    /// `[u8; 32]` - is the hash being queried.
    /// If it is in the claim hashed_info set then true is returned. Otherwise - false.
    CheckClaim(PublicKey, PieceId, [u8; 32]),
}

#[derive(Debug, Decode, Encode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum IdentityStateReply {
    UserClaims(Vec<(PieceId, Claim)>),
    Claim(Option<Claim>),
    Verifiers(Vec<PublicKey>),
    ValidationStatus(bool),
    Date(u64),
    CheckedClaim(PublicKey, PieceId, bool),
}

/// Initializes an identity storage.
#[derive(Decode, Encode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct InitIdentity;
