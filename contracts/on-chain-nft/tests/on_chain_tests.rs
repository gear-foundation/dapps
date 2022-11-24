mod utils;

use gear_lib::non_fungible_token::io::*;
use gear_lib::non_fungible_token::token::*;
use gstd::prelude::*;
use gtest::System;
use on_chain_nft::io::OnChainNFTEvent;

const USERS: &[u64] = &[3, 4, 5];
const ZERO_ID: u64 = 0;

#[test]
fn mint_success() {
    let sys = System::new();
    utils::init_nft(&sys);
    let nft = sys.get_program(1);
    let res = utils::mint(&nft, USERS[0], vec![0, 1]);
    let message = OnChainNFTEvent::Transfer(NFTTransfer {
        from: ZERO_ID.into(),
        to: USERS[0].into(),
        token_id: 0.into(),
    })
    .encode();
    assert!(res.contains(&(USERS[0], message)));
    // Check that we minted a token properly
    utils::check_token_from_state(&nft, USERS[0], 0);
}

#[test]
fn mint_failures() {
    let sys = System::new();
    utils::init_nft(&sys);
    let nft = sys.get_program(1);
    assert!(utils::mint(&nft, USERS[0], vec![3, 3]).main_failed());

    // mint token
    assert!(!utils::mint(&nft, USERS[0], vec![0, 1]).main_failed());
    // mint it again
    assert!(utils::mint(&nft, USERS[0], vec![0, 1]).main_failed());
}

#[test]
fn burn_success() {
    let sys = System::new();
    utils::init_nft(&sys);
    let nft = sys.get_program(1);
    assert!(!utils::mint(&nft, USERS[0], vec![0, 1]).main_failed());
    // Check that we minted a token properly
    utils::check_token_from_state(&nft, USERS[0], 0);

    let res = utils::burn(&nft, USERS[0], 0);
    let message = OnChainNFTEvent::Transfer(NFTTransfer {
        from: USERS[0].into(),
        to: ZERO_ID.into(),
        token_id: 0.into(),
    })
    .encode();
    assert!(res.contains(&(USERS[0], message)));
    // We should check against owner_id = 0 since the token is burned
    utils::check_token_from_state(&nft, 0, 0);
}

#[test]
fn burn_failures() {
    let sys = System::new();
    utils::init_nft(&sys);
    let nft = sys.get_program(1);
    assert!(!utils::mint(&nft, USERS[0], vec![0, 1]).main_failed());
    // must fail since the token doesn't exist
    assert!(utils::burn(&nft, USERS[0], 1).main_failed());
    // must fail since the caller is not the token owner
    assert!(utils::burn(&nft, USERS[1], 0).main_failed());
}

#[test]
fn transfer_success() {
    let sys = System::new();
    utils::init_nft(&sys);
    let nft = sys.get_program(1);
    assert!(!utils::mint(&nft, USERS[0], vec![0, 1]).main_failed());
    // Check that we minted a token properly
    utils::check_token_from_state(&nft, USERS[0], 0);

    let res = utils::transfer(&nft, USERS[0], USERS[1], 0);
    let message = OnChainNFTEvent::Transfer(NFTTransfer {
        from: USERS[0].into(),
        to: USERS[1].into(),
        token_id: 0.into(),
    })
    .encode();
    assert!(res.contains(&(USERS[0], message)));

    // Check the token now belongs to another user
    utils::check_token_from_state(&nft, USERS[1], 0);
}

#[test]
fn transfer_failures() {
    let sys = System::new();
    utils::init_nft(&sys);
    let nft = sys.get_program(1);
    assert!(!utils::mint(&nft, USERS[0], vec![0, 1]).main_failed());

    // must fail since the token doesn't exist
    assert!(utils::transfer(&nft, USERS[0], USERS[1], 1).main_failed());
    // must fail since the caller is not the token owner
    assert!(utils::transfer(&nft, USERS[1], USERS[0], 0).main_failed());
    // must fail since transfer to the zero address
    assert!(utils::transfer(&nft, USERS[1], ZERO_ID, 0).main_failed());
}

#[test]
fn approve_success() {
    let sys = System::new();
    utils::init_nft(&sys);
    let nft = sys.get_program(1);
    assert!(!utils::mint(&nft, USERS[0], vec![0, 1]).main_failed());
    // Check that we minted a token properly
    utils::check_token_from_state(&nft, USERS[0], 0);

    let res = utils::approve(&nft, USERS[0], USERS[1], 0);
    let message = OnChainNFTEvent::Approval(NFTApproval {
        owner: USERS[0].into(),
        approved_account: USERS[1].into(),
        token_id: 0.into(),
    })
    .encode();
    assert!(res.contains(&(USERS[0], message)));
    assert!(!utils::transfer(&nft, USERS[1], USERS[2], 0).main_failed());
}

#[test]
fn approve_failures() {
    let sys = System::new();
    utils::init_nft(&sys);
    let nft = sys.get_program(1);
    assert!(!utils::mint(&nft, USERS[0], vec![0, 1]).main_failed());
    // must fail since the token doesn't exist
    assert!(utils::approve(&nft, USERS[0], USERS[1], 1).main_failed());
    // must fail since the caller is not the token owner
    assert!(utils::approve(&nft, USERS[1], USERS[0], 0).main_failed());
    // must fail since approval to the zero address
    assert!(utils::approve(&nft, USERS[1], ZERO_ID, 0).main_failed());

    //approve
    assert!(!utils::approve(&nft, USERS[0], USERS[1], 0).main_failed());
    //transfer
    assert!(!utils::transfer(&nft, USERS[1], USERS[2], 0).main_failed());
    //must fail since approval was removed after transferring
    assert!(utils::transfer(&nft, USERS[1], USERS[0], 0).main_failed());
}

#[test]
fn test_token_uri_state() {
    let sys = System::new();
    utils::init_nft(&sys);
    let nft = sys.get_program(1);
    let res = utils::mint(&nft, USERS[0], vec![0, 1]);
    let message = OnChainNFTEvent::Transfer(NFTTransfer {
        from: ZERO_ID.into(),
        to: USERS[0].into(),
        token_id: 0.into(),
    })
    .encode();
    assert!(res.contains(&(USERS[0], message)));
    // Check that we minted a token properly
    utils::check_token_from_state(&nft, USERS[0], 0);

    let token_metadata = TokenMetadata {
        name: "CryptoKitty".to_string(),
        description: "Description".to_string(),
        media: "http://".to_string(),
        reference: "http://".to_string(),
    };
    let content = vec![String::from("PHN2ZyBoZWlnaHQ9JzIxMCcgd2lkdGg9JzUwMCc+PHBvbHlnb24gcG9pbnRzPScxMDAsMTAgNDAsMTk4IDE5MCw3OCAxMCw3OCAxNjAsMTk4JyBzdHlsZT0nZmlsbDpsaW1lO3N0cm9rZTpwdXJwbGU7c3Ryb2tlLXdpZHRoOjU7ZmlsbC1ydWxlOm5vbnplcm87Jy8+PC9zdmc+"), String::from("PHN2ZyBoZWlnaHQ9JzMwJyB3aWR0aD0nMjAwJz48dGV4dCB4PScwJyB5PScxNScgZmlsbD0nZ3JlZW4nPk9uIENoYWluIE5GVDwvdGV4dD48L3N2Zz4=")];
    utils::check_token_uri(&nft, 0, token_metadata, content);
}
