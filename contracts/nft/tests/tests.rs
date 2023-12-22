use gstd::{ActorId, Encode};
use gtest::System;
mod utils;
use nft_io::*;
use utils::*;

const USERS: &[u64] = &[3, 4, 5];
const ZERO_ID: u64 = 0;

#[test]
fn mint_success() {
    let sys = System::new();
    init_nft(&sys);
    let nft = sys.get_program(1);

    let res = mint(&nft, USERS[0], USERS[1].into());
    let token_metadata = TokenMetadata {
        name: "CryptoKitty".to_string(),
        description: "Description".to_string(),
        media: "http://".to_string(),
        reference: "http://".to_string(),
    };
    let message = NftEvent::Minted {
        to: USERS[1].into(),
        token_metadata,
    }
    .encode();
    assert!(res.contains(&(USERS[0], message)));

    let state = get_state(&nft).expect("Unexpected invalid state.");
    assert_eq!(state.owner_by_id, [(0_u128, USERS[1].into())]);
    assert_eq!(state.tokens_for_owner, [(USERS[1].into(), vec![0])]);
}

#[test]
fn mint_failures() {
    let sys = System::new();
    sys.init_logger();
    let nft = gtest::Program::current_opt(&sys);

    let collection = Collection {
        name: String::from("MyToken"),
        description: String::from("My token"),
    };

    let init_nft = InitNft {
        collection,
        config: Config {
            max_mint_count: Some(1),
        },
    };

    let res = nft.send(USERS[0], init_nft);
    assert!(!res.main_failed());

    // zero address
    let res = mint(&nft, USERS[0], 0.into());
    assert!(res.main_failed());

    // limit_exceed
    let nft = sys.get_program(1);
    let res = mint(&nft, USERS[0], USERS[1].into());
    assert!(!res.main_failed());
    let res = mint(&nft, USERS[0], USERS[1].into());
    assert!(res.main_failed())
}

#[test]
fn burn_success() {
    let sys = System::new();
    init_nft(&sys);
    let nft = sys.get_program(1);
    let res = mint(&nft, USERS[0], USERS[1].into());
    assert!(!res.main_failed());
    let res = burn(&nft, USERS[1], 0);
    let message = NftEvent::Burnt { token_id: 0 }.encode();
    assert!(res.contains(&(USERS[1], message)));

    let state = get_state(&nft).expect("Unexpected invalid state.");
    assert!(state.owner_by_id.is_empty());
    assert!(state.tokens_for_owner.is_empty());
}

#[test]
fn burn_failures() {
    let sys = System::new();
    init_nft(&sys);
    let nft = sys.get_program(1);
    mint(&nft, USERS[0], USERS[1].into());
    // must fail since the token doesn't exist
    assert!(burn(&nft, USERS[1], 1).main_failed());
    // must fail since the caller is not the token owner
    assert!(burn(&nft, USERS[0], 0).main_failed());
}

#[test]
fn transfer_success() {
    let sys = System::new();
    init_nft(&sys);
    let nft = sys.get_program(1);
    assert!(!mint(&nft, USERS[0], USERS[1].into()).main_failed());

    let res = transfer(&nft, USERS[1], USERS[2], 0);
    let message = NftEvent::Transferred {
        from: USERS[1].into(),
        to: USERS[2].into(),
        token_id: 0,
    }
    .encode();
    assert!(res.contains(&(USERS[1], message)));

    let state = get_state(&nft).expect("Unexpected invalid state.");
    assert_eq!(state.owner_by_id, [(0_u128, USERS[2].into())]);
    assert_eq!(state.tokens_for_owner, [(USERS[2].into(), vec![0])]);
}

#[test]
fn transfer_failures() {
    let sys = System::new();
    init_nft(&sys);
    let nft = sys.get_program(1);
    assert!(!mint(&nft, USERS[0], USERS[1].into()).main_failed());

    // must fail since the token doesn't exist
    assert!(transfer(&nft, USERS[1], USERS[2], 1).main_failed());
    // must fail since the caller is not the token owner
    assert!(transfer(&nft, USERS[0], USERS[2], 0).main_failed());
    // must fail since transfer to the zero address
    assert!(transfer(&nft, USERS[1], ZERO_ID, 0).main_failed());
}

#[test]
fn approve_success() {
    let sys = System::new();
    init_nft(&sys);
    let nft = sys.get_program(1);
    assert!(!mint(&nft, USERS[0], USERS[1].into()).main_failed());
    let res = approve(&nft, USERS[1], USERS[2], 0);
    let message = NftEvent::Approved {
        owner: USERS[1].into(),
        approved_account: USERS[2].into(),
        token_id: 0,
    }
    .encode();
    assert!(res.contains(&(USERS[1], message)));
    let state = get_state(&nft).expect("Unexpected invalid state.");
    assert_eq!(state.token_approvals, [(0_u128, USERS[2].into())]);

    assert!(!transfer(&nft, USERS[2], USERS[0], 0).main_failed());

    let state = get_state(&nft).expect("Unexpected invalid state.");
    assert!(state.token_approvals.is_empty());
}

#[test]
fn approve_failures() {
    let sys = System::new();
    init_nft(&sys);
    let nft = sys.get_program(1);
    assert!(!mint(&nft, USERS[0], USERS[1].into()).main_failed());
    // must fail since the token doesn't exist
    assert!(approve(&nft, USERS[1], USERS[2], 1).main_failed());
    // must fail since the caller is not the token owner
    assert!(approve(&nft, USERS[0], USERS[2], 0).main_failed());
    // must fail since approval to the zero address
    assert!(approve(&nft, USERS[1], ZERO_ID, 0).main_failed());

    //approve
    assert!(!approve(&nft, USERS[1], USERS[2], 0).main_failed());
    //transfer
    assert!(!transfer(&nft, USERS[1], USERS[0], 0).main_failed());
    //must fail since approval was removed after transferring
    assert!(transfer(&nft, USERS[2], USERS[0], 0).main_failed());
}

#[test]
fn owner_success() {
    let sys = System::new();
    init_nft(&sys);
    let nft = sys.get_program(1);
    assert!(!mint(&nft, USERS[0], USERS[1].into()).main_failed());
    let res = owner_of(&nft, USERS[1], 0);

    let message = NftEvent::Owner {
        token_id: 0,
        owner: ActorId::from(USERS[1]),
    }
    .encode();
    assert!(res.contains(&(USERS[1], message)));
}

#[test]
fn owner_failure() {
    let sys = System::new();
    init_nft(&sys);
    let nft = sys.get_program(1);
    assert!(!mint(&nft, USERS[0], USERS[1].into()).main_failed());
    let res = owner_of(&nft, USERS[1], 1);
    assert!(res.main_failed());
}

#[test]
fn is_approved_to_success() {
    let sys = System::new();
    init_nft(&sys);
    let nft = sys.get_program(1);
    assert!(!mint(&nft, USERS[0], USERS[1].into()).main_failed());
    assert!(!approve(&nft, USERS[1], USERS[2], 0).main_failed());

    let res = is_approved_to(&nft, USERS[0], 0, USERS[2]);
    let message = NftEvent::CheckIfApproved {
        to: USERS[2].into(),
        token_id: 0,
        approved: true,
    }
    .encode();
    assert!(res.contains(&(USERS[0], message)));
}

#[test]
fn is_approved_to_failure() {
    let sys = System::new();
    init_nft(&sys);
    let nft = sys.get_program(1);
    assert!(!mint(&nft, USERS[0], USERS[1].into()).main_failed());
    assert!(!approve(&nft, USERS[1], USERS[2], 0).main_failed());
    let res = is_approved_to(&nft, USERS[1], 1, USERS[2]);
    assert!(res.main_failed());
}
