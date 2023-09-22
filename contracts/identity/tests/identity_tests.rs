use gstd::prelude::*;
use gtest::System;
use hex_literal::hex;
use identity_io::*;
use sha2::{Digest, Sha256};
use sp_core::{sr25519::Pair as Sr25519Pair, Pair};
mod utils;
use utils::*;

const USER: u64 = 10;
const PIECE_ID: PieceId = 0;
const DATE: u64 = 12288282;

#[test]
fn issue_claim_by_subject() {
    let sys = System::new();
    let id_program = init_identity(&sys, USER);
    let subject_pair = Sr25519Pair::from_seed(&hex!(
        "9D61B19DEFFD5A60BA844AF492EC2CC44449C5697B326919703BAC031CAE7F60"
    ));

    let mut hasher = Sha256::new();
    hasher.update(b"Amsterdam");

    // read hash digest and consume hasher
    let result = hasher.finalize().as_slice().try_into().expect("Wrong size");
    let claim_data = ClaimData {
        hashed_info: Vec::from([result]),
        issuance_date: DATE,
        valid: true,
    };

    let claim = Claim {
        issuer: subject_pair.public().0,
        issuer_signature: subject_pair.sign(claim_data.encode().as_slice()).0,
        subject: subject_pair.public().0,
        verifiers: vec![],
        data: claim_data,
    };

    issue_claim_utils(&id_program, USER, claim.clone(), PIECE_ID, false);
    let claims = vec![(PIECE_ID, claim.clone())];
    check_user_claims_state_utils(&id_program, subject_pair.public().0, claims);
    check_claim_state_utils(&id_program, subject_pair.public().0, PIECE_ID, claim);
    check_date_state_utils(&id_program, subject_pair.public().0, PIECE_ID, DATE);
}

#[test]
fn issue_claim_by_issuer() {
    let sys = System::new();
    let id_program = init_identity(&sys, USER);
    let subject_pair = Sr25519Pair::from_seed(&hex!(
        "9D61B19DEFFD5A60BA844AF492EC2CC44449C5697B326919703BAC031CAE7F60"
    ));
    let issuer_pair = Sr25519Pair::from_seed(&hex!(
        "9D61B2EBC4197073EF857A385EB42990CB647497353884B9703BAC031CAE7F60"
    ));

    let mut hasher = Sha256::new();
    hasher.update(b"Amsterdam");

    // read hash digest and consume hasher
    let result = hasher.finalize().as_slice().try_into().expect("Wrong size");
    let claim_data = ClaimData {
        hashed_info: Vec::from([result]),
        issuance_date: DATE,
        valid: true,
    };

    let claim = Claim {
        issuer: issuer_pair.public().0,
        issuer_signature: issuer_pair.sign(claim_data.encode().as_slice()).0,
        subject: subject_pair.public().0,
        verifiers: vec![],
        data: claim_data,
    };

    issue_claim_utils(&id_program, USER, claim.clone(), PIECE_ID, false);
    let claims = vec![(PIECE_ID, claim.clone())];
    check_user_claims_state_utils(&id_program, subject_pair.public().0, claims);
    check_claim_state_utils(&id_program, subject_pair.public().0, PIECE_ID, claim);
    check_date_state_utils(&id_program, subject_pair.public().0, PIECE_ID, DATE);
}

#[test]
fn issue_multiple_claim() {
    let sys = System::new();
    let id_program = init_identity(&sys, USER);
    let subject_pair = Sr25519Pair::from_seed(&hex!(
        "9D61B19DEFFD5A60BA844AF492EC2CC44449C5697B326919703BAC031CAE7F60"
    ));

    let mut hasher = Sha256::new();
    hasher.update(b"Amsterdam");

    // read hash digest and consume hasher
    let city = hasher.finalize().as_slice().try_into().expect("Wrong size");
    let mut hasher = Sha256::new();
    hasher.update(b"Nikolskaya");
    let street = hasher.finalize().as_slice().try_into().expect("Wrong size");

    let claim_data = ClaimData {
        hashed_info: Vec::from([city, street]),
        issuance_date: DATE,
        valid: true,
    };

    let claim = Claim {
        issuer: subject_pair.public().0,
        issuer_signature: subject_pair.sign(claim_data.encode().as_slice()).0,
        subject: subject_pair.public().0,
        verifiers: vec![],
        data: claim_data,
    };

    issue_claim_utils(&id_program, USER, claim.clone(), PIECE_ID, false);
    let claims = vec![(PIECE_ID, claim.clone())];
    check_user_claims_state_utils(&id_program, subject_pair.public().0, claims);
    check_claim_state_utils(&id_program, subject_pair.public().0, PIECE_ID, claim);
    check_date_state_utils(&id_program, subject_pair.public().0, PIECE_ID, DATE);
}

#[test]
fn validation_status_from_subject() {
    let sys = System::new();
    let id_program = init_identity(&sys, USER);
    let subject_pair = Sr25519Pair::from_seed(&hex!(
        "9D61B19DEFFD5A60BA844AF492EC2CC44449C5697B326919703BAC031CAE7F60"
    ));

    let mut hasher = Sha256::new();
    hasher.update(b"Amsterdam");

    // read hash digest and consume hasher
    let result = hasher.finalize().as_slice().try_into().expect("Wrong size");
    let claim_data = ClaimData {
        hashed_info: Vec::from([result]),
        issuance_date: DATE,
        valid: true,
    };

    let claim = Claim {
        issuer: subject_pair.public().0,
        issuer_signature: subject_pair.sign(claim_data.encode().as_slice()).0,
        subject: subject_pair.public().0,
        verifiers: vec![],
        data: claim_data,
    };

    issue_claim_utils(&id_program, USER, claim.clone(), PIECE_ID, false);
    let claims = vec![(PIECE_ID, claim.clone())];
    check_user_claims_state_utils(&id_program, subject_pair.public().0, claims);
    check_claim_state_utils(&id_program, subject_pair.public().0, PIECE_ID, claim);
    check_date_state_utils(&id_program, subject_pair.public().0, PIECE_ID, DATE);
    check_valid_state_utils(&id_program, subject_pair.public().0, PIECE_ID, true);

    validation_claim_utils(
        &id_program,
        USER,
        subject_pair.public().0,
        subject_pair.public().0,
        PIECE_ID,
        false,
        false,
    );
    check_valid_state_utils(&id_program, subject_pair.public().0, PIECE_ID, false);
}

#[test]
fn validation_status_from_issuer() {
    let sys = System::new();
    let id_program = init_identity(&sys, USER);
    let subject_pair = Sr25519Pair::from_seed(&hex!(
        "9D61B19DEFFD5A60BA844AF492EC2CC44449C5697B326919703BAC031CAE7F60"
    ));
    let issuer_pair = Sr25519Pair::from_seed(&hex!(
        "9D61B2EBC4197073EF857A385EB42990CB647497353884B9703BAC031CAE7F60"
    ));

    let mut hasher = Sha256::new();
    hasher.update(b"Amsterdam");

    // read hash digest and consume hasher
    let result = hasher.finalize().as_slice().try_into().expect("Wrong size");
    let claim_data = ClaimData {
        hashed_info: Vec::from([result]),
        issuance_date: DATE,
        valid: true,
    };

    let claim = Claim {
        issuer: issuer_pair.public().0,
        issuer_signature: issuer_pair.sign(claim_data.encode().as_slice()).0,
        subject: subject_pair.public().0,
        verifiers: vec![],
        data: claim_data,
    };

    issue_claim_utils(&id_program, USER, claim.clone(), PIECE_ID, false);
    let claims = vec![(PIECE_ID, claim.clone())];
    check_user_claims_state_utils(&id_program, subject_pair.public().0, claims);
    check_claim_state_utils(&id_program, subject_pair.public().0, PIECE_ID, claim);
    check_date_state_utils(&id_program, subject_pair.public().0, PIECE_ID, DATE);

    validation_claim_utils(
        &id_program,
        USER,
        issuer_pair.public().0,
        subject_pair.public().0,
        PIECE_ID,
        false,
        false,
    );
    check_valid_state_utils(&id_program, subject_pair.public().0, PIECE_ID, false);
}

#[test]
fn validation_status_failures() {
    let sys = System::new();
    let id_program = init_identity(&sys, USER);
    let subject_pair = Sr25519Pair::from_seed(&hex!(
        "9D61B19DEFFD5A60BA844AF492EC2CC44449C5697B326919703BAC031CAE7F60"
    ));
    let issuer_pair = Sr25519Pair::from_seed(&hex!(
        "9D61B2EBC4197073EF857A385EB42990CB647497353884B9703BAC031CAE7F60"
    ));

    let mut hasher = Sha256::new();
    hasher.update(b"Amsterdam");

    // read hash digest and consume hasher
    let result = hasher.finalize().as_slice().try_into().expect("Wrong size");
    let claim_data = ClaimData {
        hashed_info: Vec::from([result]),
        issuance_date: DATE,
        valid: true,
    };

    let claim = Claim {
        issuer: issuer_pair.public().0,
        issuer_signature: issuer_pair.sign(claim_data.encode().as_slice()).0,
        subject: subject_pair.public().0,
        verifiers: vec![],
        data: claim_data,
    };

    issue_claim_utils(&id_program, USER, claim.clone(), PIECE_ID, false);
    let claims = vec![(PIECE_ID, claim.clone())];
    check_user_claims_state_utils(&id_program, subject_pair.public().0, claims);
    check_claim_state_utils(&id_program, subject_pair.public().0, PIECE_ID, claim);
    check_date_state_utils(&id_program, subject_pair.public().0, PIECE_ID, DATE);

    // try to validate with the third key
    let third_key = Sr25519Pair::from_seed(&hex!(
        "9A61B2EBC4197073EF857A385EB42990CB647497353884B9703BAC031CAE7F60"
    ));
    validation_claim_utils(
        &id_program,
        USER,
        third_key.public().0,
        subject_pair.public().0,
        PIECE_ID,
        false,
        true,
    );
    // validate wrong PIECE_ID
    validation_claim_utils(
        &id_program,
        USER,
        subject_pair.public().0,
        subject_pair.public().0,
        PIECE_ID + 1,
        false,
        true,
    );
    // validate the user with no claims
    validation_claim_utils(
        &id_program,
        USER,
        subject_pair.public().0,
        third_key.public().0,
        PIECE_ID,
        false,
        true,
    );
}

#[test]
fn verify_claim() {
    let sys = System::new();
    let id_program = init_identity(&sys, USER);
    let subject_pair = Sr25519Pair::from_seed(&hex!(
        "9D61B19DEFFD5A60BA844AF492EC2CC44449C5697B326919703BAC031CAE7F60"
    ));

    let mut hasher = Sha256::new();
    hasher.update(b"Amsterdam");

    // read hash digest and consume hasher
    let result = hasher.finalize().as_slice().try_into().expect("Wrong size");
    let claim_data = ClaimData {
        hashed_info: Vec::from([result]),
        issuance_date: DATE,
        valid: true,
    };

    let claim = Claim {
        issuer: subject_pair.public().0,
        issuer_signature: subject_pair.sign(claim_data.encode().as_slice()).0,
        subject: subject_pair.public().0,
        verifiers: vec![],
        data: claim_data.clone(),
    };

    issue_claim_utils(&id_program, USER, claim.clone(), PIECE_ID, false);
    let claims = vec![(PIECE_ID, claim.clone())];
    check_user_claims_state_utils(&id_program, subject_pair.public().0, claims);
    check_claim_state_utils(&id_program, subject_pair.public().0, PIECE_ID, claim);
    check_date_state_utils(&id_program, subject_pair.public().0, PIECE_ID, DATE);

    let verifier_pair = Sr25519Pair::from_seed(&hex!(
        "9D61B19DEFFD5D60BA844AF492EC2CC44449C5697B326919703BAC031CAE7F60"
    ));
    verify_claim_utils(
        &id_program,
        USER,
        verifier_pair.public().0,
        verifier_pair.sign(claim_data.encode().as_slice()).0,
        subject_pair.public().0,
        PIECE_ID,
        false,
    );
    check_verifiers_state_utils(
        &id_program,
        subject_pair.public().0,
        PIECE_ID,
        vec![verifier_pair.public().0],
    );
}

#[test]
fn verify_claim_failures() {
    let sys = System::new();
    let id_program = init_identity(&sys, USER);
    let subject_pair = Sr25519Pair::from_seed(&hex!(
        "9D61B19DEFFD5A60BA844AF492EC2CC44449C5697B326919703BAC031CAE7F60"
    ));

    let mut hasher = Sha256::new();
    hasher.update(b"Amsterdam");

    // read hash digest and consume hasher
    let result = hasher.finalize().as_slice().try_into().expect("Wrong size");
    let claim_data = ClaimData {
        hashed_info: Vec::from([result]),
        issuance_date: DATE,
        valid: true,
    };

    let claim = Claim {
        issuer: subject_pair.public().0,
        issuer_signature: subject_pair.sign(claim_data.encode().as_slice()).0,
        subject: subject_pair.public().0,
        verifiers: vec![],
        data: claim_data.clone(),
    };

    issue_claim_utils(&id_program, USER, claim.clone(), PIECE_ID, false);
    let claims = vec![(PIECE_ID, claim.clone())];
    check_user_claims_state_utils(&id_program, subject_pair.public().0, claims);
    check_claim_state_utils(&id_program, subject_pair.public().0, PIECE_ID, claim);
    check_date_state_utils(&id_program, subject_pair.public().0, PIECE_ID, DATE);

    let verifier_pair = Sr25519Pair::from_seed(&hex!(
        "9D61B19DEFFD5D60BA844AF492EC2CC44449C5697B326919703BAC031CAE7F60"
    ));
    // verify user with no claims
    verify_claim_utils(
        &id_program,
        USER,
        verifier_pair.public().0,
        verifier_pair.sign(claim_data.encode().as_slice()).0,
        verifier_pair.public().0,
        PIECE_ID,
        true,
    );
    // verify wrong piece_id

    verify_claim_utils(
        &id_program,
        USER,
        verifier_pair.public().0,
        verifier_pair.sign(claim_data.encode().as_slice()).0,
        subject_pair.public().0,
        PIECE_ID + 1,
        true,
    );
}

#[test]
fn check_claim() {
    let sys = System::new();
    let id_program = init_identity(&sys, USER);
    let subject_pair = Sr25519Pair::from_seed(&hex!(
        "9D61B19DEFFD5A60BA844AF492EC2CC44449C5697B326919703BAC031CAE7F60"
    ));

    let mut hasher = Sha256::new();
    hasher.update(b"Amsterdam");

    // read hash digest and consume hasher
    let city = hasher.finalize().as_slice().try_into().expect("Wrong size");
    let mut hasher = Sha256::new();
    hasher.update(b"Nikolskaya");
    let street = hasher.finalize().as_slice().try_into().expect("Wrong size");

    let claim_data = ClaimData {
        hashed_info: Vec::from([city, street]),
        issuance_date: DATE,
        valid: true,
    };

    let claim = Claim {
        issuer: subject_pair.public().0,
        issuer_signature: subject_pair.sign(claim_data.encode().as_slice()).0,
        subject: subject_pair.public().0,
        verifiers: vec![],
        data: claim_data,
    };

    issue_claim_utils(&id_program, USER, claim.clone(), PIECE_ID, false);
    let claims = vec![(PIECE_ID, claim.clone())];
    check_user_claims_state_utils(&id_program, subject_pair.public().0, claims);
    check_claim_state_utils(&id_program, subject_pair.public().0, PIECE_ID, claim);
    check_date_state_utils(&id_program, subject_pair.public().0, PIECE_ID, DATE);

    check_claim_hash_state_utils(&id_program, subject_pair.public().0, PIECE_ID, city, true);
    check_claim_hash_state_utils(
        &id_program,
        subject_pair.public().0,
        PIECE_ID,
        [0; 32],
        false,
    );
}
