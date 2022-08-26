use gear_lib::non_fungible_token::io::*;
use gear_lib::non_fungible_token::token::*;
use gstd::prelude::*;
use gtest::System;
mod utils;
use on_chain_nft_io::OnChainNFTEvent;
use utils::*;
const USERS: &[u64] = &[3, 4, 5];
const ZERO_ID: u64 = 0;

#[test]
fn mint_success() {
    let sys = System::new();
    init_nft(&sys);
    let nft = sys.get_program(1);
    let res = mint(&nft, USERS[0], vec![0, 1]);
    let message = OnChainNFTEvent::Transfer(NFTTransfer {
        from: ZERO_ID.into(),
        to: USERS[0].into(),
        token_id: 0.into(),
    })
    .encode();
    assert!(res.contains(&(USERS[0], message)));
    // Check that we minted a token properly
    check_token_from_state(&nft, USERS[0], 0);
}

#[test]
fn mint_failures() {
    let sys = System::new();
    init_nft(&sys);
    let nft = sys.get_program(1);
    assert!(mint(&nft, USERS[0], vec![3, 3]).main_failed());

    // mint token
    assert!(!mint(&nft, USERS[0], vec![0, 1]).main_failed());
    // mint it again
    assert!(mint(&nft, USERS[0], vec![0, 1]).main_failed());
}

#[test]
fn burn_success() {
    let sys = System::new();
    init_nft(&sys);
    let nft = sys.get_program(1);
    assert!(!mint(&nft, USERS[0], vec![0, 1]).main_failed());
    // Check that we minted a token properly
    check_token_from_state(&nft, USERS[0], 0);

    let res = burn(&nft, USERS[0], 0);
    let message = OnChainNFTEvent::Transfer(NFTTransfer {
        from: USERS[0].into(),
        to: ZERO_ID.into(),
        token_id: 0.into(),
    })
    .encode();
    assert!(res.contains(&(USERS[0], message)));
    // We should check against owner_id = 0 since the token is burned
    check_token_from_state(&nft, 0, 0);
}

#[test]
fn burn_failures() {
    let sys = System::new();
    init_nft(&sys);
    let nft = sys.get_program(1);
    assert!(!mint(&nft, USERS[0], vec![0, 1]).main_failed());
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
    assert!(!mint(&nft, USERS[0], vec![0, 1]).main_failed());
    // Check that we minted a token properly
    check_token_from_state(&nft, USERS[0], 0);

    let res = transfer(&nft, USERS[0], USERS[1], 0);
    let message = OnChainNFTEvent::Transfer(NFTTransfer {
        from: USERS[0].into(),
        to: USERS[1].into(),
        token_id: 0.into(),
    })
    .encode();
    assert!(res.contains(&(USERS[0], message)));

    // Check the token now belongs to another user
    check_token_from_state(&nft, USERS[1], 0);
}

#[test]
fn transfer_failures() {
    let sys = System::new();
    init_nft(&sys);
    let nft = sys.get_program(1);
    assert!(!mint(&nft, USERS[0], vec![0, 1]).main_failed());

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
    assert!(!mint(&nft, USERS[0], vec![0, 1]).main_failed());
    // Check that we minted a token properly
    check_token_from_state(&nft, USERS[0], 0);

    let res = approve(&nft, USERS[0], USERS[1], 0);
    let message = OnChainNFTEvent::Approval(NFTApproval {
        owner: USERS[0].into(),
        approved_account: USERS[1].into(),
        token_id: 0.into(),
    })
    .encode();
    assert!(res.contains(&(USERS[0], message)));
    assert!(!transfer(&nft, USERS[1], USERS[2], 0).main_failed());
}

#[test]
fn approve_failures() {
    let sys = System::new();
    init_nft(&sys);
    let nft = sys.get_program(1);
    assert!(!mint(&nft, USERS[0], vec![0, 1]).main_failed());
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
fn test_token_uri_state() {
    let sys = System::new();
    init_nft(&sys);
    let nft = sys.get_program(1);
    let res = mint(&nft, USERS[0], vec![0, 1]);
    let message = OnChainNFTEvent::Transfer(NFTTransfer {
        from: ZERO_ID.into(),
        to: USERS[0].into(),
        token_id: 0.into(),
    })
    .encode();
    assert!(res.contains(&(USERS[0], message)));
    // Check that we minted a token properly
    check_token_from_state(&nft, USERS[0], 0);

    let token_metadata = TokenMetadata {
        name: "CryptoKitty".to_string(),
        description: "Description".to_string(),
        media: "http://".to_string(),
        reference: "http://".to_string(),
    };
    let content = vec![String::from("PHN2ZyBoZWlnaHQ9JzIxMCcgd2lkdGg9JzUwMCc+PHBvbHlnb24gcG9pbnRzPScxMDAsMTAgNDAsMTk4IDE5MCw3OCAxMCw3OCAxNjAsMTk4JyBzdHlsZT0nZmlsbDpsaW1lO3N0cm9rZTpwdXJwbGU7c3Ryb2tlLXdpZHRoOjU7ZmlsbC1ydWxlOm5vbnplcm87Jy8+PC9zdmc+"), String::from("PHN2ZyBoZWlnaHQ9JzMwJyB3aWR0aD0nMjAwJz48dGV4dCB4PScwJyB5PScxNScgZmlsbD0nZ3JlZW4nPk9uIENoYWluIE5GVDwvdGV4dD48L3N2Zz4=")];
    check_token_uri(&nft, 0, token_metadata, content);
}
