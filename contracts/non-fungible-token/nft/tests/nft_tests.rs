use codec::Encode;
use gear_lib::non_fungible_token::delegated::DelegatedApproveMessage;
use gear_lib::non_fungible_token::io::*;
use gstd::ActorId;
use gtest::System;
mod utils;
use hex_literal::hex;
use sp_core::{sr25519::Pair as Sr25519Pair, Pair};
use utils::*;

const USERS: &[u64] = &[3, 4, 5];
const ZERO_ID: u64 = 0;

#[test]
fn mint_success() {
    let sys = System::new();
    init_nft(&sys);
    let nft = sys.get_program(1);
    let res = mint(&nft, USERS[0]);
    let message = NFTTransfer {
        from: ZERO_ID.into(),
        to: USERS[0].into(),
        token_id: 0.into(),
    }
    .encode();
    assert!(res.contains(&(USERS[0], message.encode())));
}

#[test]
fn burn_success() {
    let sys = System::new();
    init_nft(&sys);
    let nft = sys.get_program(1);
    assert!(!mint(&nft, USERS[0]).main_failed());
    let res = burn(&nft, USERS[0], 0);
    let message = NFTTransfer {
        from: USERS[0].into(),
        to: ZERO_ID.into(),
        token_id: 0.into(),
    }
    .encode();
    assert!(res.contains(&(USERS[0], message.encode())));
}

#[test]
fn burn_failures() {
    let sys = System::new();
    init_nft(&sys);
    let nft = sys.get_program(1);
    assert!(!mint(&nft, USERS[0]).main_failed());
    // must fail since the token doesn't exist
    assert!(burn(&nft, USERS[0], 1).main_failed());
    // must fail since the caller is not the token owner
    assert!(burn(&nft, USERS[1], 0).main_failed());
}

#[test]
fn transfer_success() {
    let sys = System::new();
    init_nft(&sys);
    let nft = sys.get_program(1);
    assert!(!mint(&nft, USERS[0]).main_failed());
    let res = transfer(&nft, USERS[0], USERS[1], 0);
    let message = NFTTransfer {
        from: USERS[0].into(),
        to: USERS[1].into(),
        token_id: 0.into(),
    }
    .encode();
    assert!(res.contains(&(USERS[0], message.encode())));
}

#[test]
fn transfer_failures() {
    let sys = System::new();
    init_nft(&sys);
    let nft = sys.get_program(1);
    assert!(!mint(&nft, USERS[0]).main_failed());

    // must fail since the token doesn't exist
    assert!(transfer(&nft, USERS[0], USERS[1], 1).main_failed());
    // must fail since the caller is not the token owner
    assert!(transfer(&nft, USERS[1], USERS[0], 0).main_failed());
    // must fail since transfer to the zero address
    assert!(transfer(&nft, USERS[1], ZERO_ID, 0).main_failed());
}

#[test]
fn owner_success() {
    let sys = System::new();
    init_nft(&sys);
    let nft = sys.get_program(1);
    assert!(!mint(&nft, USERS[0]).main_failed());
    assert!(!approve(&nft, USERS[0], USERS[1], 0).main_failed());
    let res = owner_of(&nft, USERS[1], 0);
    println!("{:?}", res.decoded_log::<ActorId>());
    let message = ActorId::from(USERS[0]).encode();
    assert!(res.contains(&(USERS[1], message.encode())));
}

#[test]
fn is_approved_to_success() {
    let sys = System::new();
    init_nft(&sys);
    let nft = sys.get_program(1);
    assert!(!mint(&nft, USERS[0]).main_failed());
    assert!(!approve(&nft, USERS[0], USERS[1], 0).main_failed());

    let res = is_approved_to(&nft, USERS[1], 0, USERS[1]);
    println!("{:?}", res.decoded_log::<bool>());
    let message = true.encode();
    assert!(res.contains(&(USERS[1], message.encode())));

    let res = is_approved_to(&nft, USERS[1], 0, USERS[0]);
    println!("{:?}", res.decoded_log::<bool>());
    let message = false.encode();
    assert!(res.contains(&(USERS[1], message.encode())));
}

#[test]
fn is_approved_to_failure() {
    let sys = System::new();
    init_nft(&sys);
    let nft = sys.get_program(1);
    assert!(!mint(&nft, USERS[0]).main_failed());
    assert!(!approve(&nft, USERS[0], USERS[1], 0).main_failed());
    let res = is_approved_to(&nft, USERS[1], 1, USERS[1]);
    println!("{:?}", res.decoded_log::<bool>());
    assert!(res.main_failed());
}

#[test]
fn approve_success() {
    let sys = System::new();
    init_nft(&sys);
    let nft = sys.get_program(1);
    assert!(!mint(&nft, USERS[0]).main_failed());
    let res = approve(&nft, USERS[0], USERS[1], 0);
    let message = NFTApproval {
        owner: USERS[0].into(),
        approved_account: USERS[1].into(),
        token_id: 0.into(),
    }
    .encode();
    assert!(res.contains(&(USERS[0], message.encode())));
    assert!(!transfer(&nft, USERS[1], USERS[2], 0).main_failed());
}

#[test]
fn approve_failures() {
    let sys = System::new();
    init_nft(&sys);
    let nft = sys.get_program(1);
    assert!(!mint(&nft, USERS[0]).main_failed());
    // must fail since the token doesn't exist
    assert!(approve(&nft, USERS[0], USERS[1], 1).main_failed());
    // must fail since the caller is not the token owner
    assert!(approve(&nft, USERS[1], USERS[0], 0).main_failed());
    // must fail since approval to the zero address
    assert!(approve(&nft, USERS[1], ZERO_ID, 0).main_failed());

    //approve
    assert!(!approve(&nft, USERS[0], USERS[1], 0).main_failed());
    //transfer
    assert!(!transfer(&nft, USERS[1], USERS[2], 0).main_failed());
    //must fail since approval was removed after transferring
    assert!(transfer(&nft, USERS[1], USERS[0], 0).main_failed());
}

#[test]
fn delegated_approve_success() {
    let sys = System::new();
    init_nft(&sys);
    let nft = sys.get_program(1);
    let pair = Sr25519Pair::from_seed(&hex!(
        "9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60"
    ));
    let owner_id = pair.public().0;

    assert!(!mint_to_actor(&nft, owner_id).main_failed());

    let message = DelegatedApproveMessage {
        token_owner_id: owner_id.into(),
        approved_actor_id: USERS[1].into(),
        nft_program_id: 1.into(),
        token_id: 0.into(),
        expiration_timestamp: sys.block_timestamp() + 10,
    };
    let signature = pair.sign(message.encode().as_slice());

    let res = delegated_approve(&nft, USERS[1], message, signature.0);
    let message = NFTApproval {
        owner: owner_id.into(),
        approved_account: USERS[1].into(),
        token_id: 0.into(),
    }
    .encode();
    assert!(res.contains(&(USERS[1], message.encode())));
    assert!(!transfer(&nft, USERS[1], USERS[2], 0).main_failed());
}

#[test]
fn delegated_approve_failures() {
    let sys = System::new();
    init_nft(&sys);
    let nft = sys.get_program(1);
    let pair = Sr25519Pair::from_seed(&hex!(
        "9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60"
    ));
    let owner_id = pair.public().0;

    assert!(!mint_to_actor(&nft, owner_id).main_failed());
    assert!(!mint(&nft, USERS[0]).main_failed());
    assert!(!mint_to_actor(&nft, owner_id).main_failed());

    // Not owner can't approve nft
    let message = DelegatedApproveMessage {
        token_owner_id: owner_id.into(),
        approved_actor_id: USERS[1].into(),
        nft_program_id: 1.into(),
        token_id: 1.into(),
        expiration_timestamp: sys.block_timestamp() + 10,
    };
    let signature = pair.sign(message.encode().as_slice());
    assert!(delegated_approve(&nft, USERS[1], message, signature.0).main_failed());

    // Only approved actor in delegated approve can send delegated approve action
    let message = DelegatedApproveMessage {
        token_owner_id: owner_id.into(),
        approved_actor_id: USERS[1].into(),
        nft_program_id: 1.into(),
        token_id: 0.into(),
        expiration_timestamp: sys.block_timestamp() + 10,
    };
    let signature = pair.sign(message.encode().as_slice());

    assert!(delegated_approve(&nft, USERS[0], message, signature.0).main_failed());
    // Must fail if user tries to approve token in wrong contract
    init_nft(&sys);
    let second_nft = sys.get_program(2);
    assert!(!mint_to_actor(&second_nft, owner_id).main_failed());

    let message = DelegatedApproveMessage {
        token_owner_id: owner_id.into(),
        approved_actor_id: USERS[1].into(),
        nft_program_id: 1.into(),
        token_id: 0.into(),
        expiration_timestamp: sys.block_timestamp() + 10,
    };
    let signature = pair.sign(message.encode().as_slice());

    assert!(delegated_approve(&second_nft, USERS[1], message, signature.0).main_failed());

    // Must fail if user tries to approve token to zero_id
    let message = DelegatedApproveMessage {
        token_owner_id: owner_id.into(),
        approved_actor_id: 0.into(),
        nft_program_id: 1.into(),
        token_id: 0.into(),
        expiration_timestamp: sys.block_timestamp() + 10,
    };
    let signature = pair.sign(message.encode().as_slice());
    assert!(delegated_approve(&nft, 0, message, signature.0).main_failed());

    // Signature not corresponds to the message content
    let message = DelegatedApproveMessage {
        token_owner_id: owner_id.into(),
        approved_actor_id: USERS[1].into(),
        nft_program_id: 1.into(),
        token_id: 0.into(),
        expiration_timestamp: sys.block_timestamp() + 10,
    };
    let signature = pair.sign(message.encode().as_slice());
    let wrong_message = DelegatedApproveMessage {
        token_owner_id: owner_id.into(),
        approved_actor_id: USERS[1].into(),
        nft_program_id: 1.into(),
        token_id: 2.into(),
        expiration_timestamp: sys.block_timestamp() + 10,
    };
    assert!(delegated_approve(&nft, USERS[1], wrong_message, signature.0).main_failed());

    // Approve expired
    let message = DelegatedApproveMessage {
        token_owner_id: owner_id.into(),
        approved_actor_id: USERS[1].into(),
        nft_program_id: 1.into(),
        token_id: 0.into(),
        expiration_timestamp: sys.block_timestamp() + 10,
    };
    let signature = pair.sign(message.encode().as_slice());

    sys.spend_blocks(1);
    assert!(delegated_approve(&nft, USERS[1], message, signature.0).main_failed());
}
