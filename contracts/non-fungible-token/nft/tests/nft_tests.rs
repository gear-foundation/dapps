use codec::Encode;
use gear_lib::non_fungible_token::io::*;
use gtest::System;
mod utils;
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
