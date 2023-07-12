use gear_lib::non_fungible_token::delegated::DelegatedApproveMessage;
use gear_lib::non_fungible_token::io::*;
use gstd::{ActorId, Encode};
use gtest::System;
mod utils;
use hex_literal::hex;
use nft_io::*;
use sp_core::{sr25519::Pair as Sr25519Pair, Pair};
use utils::*;

const USERS: &[u64] = &[3, 4, 5];
const ZERO_ID: u64 = 0;

#[test]
fn mint_success() {
    let sys = System::new();
    init_nft(&sys);
    let nft = sys.get_program(1);
    let transaction_id: u64 = 0;
    let res = mint(&nft, transaction_id, USERS[0]);
    let message = NFTEvent::Transfer(NFTTransfer {
        from: ZERO_ID.into(),
        to: USERS[0].into(),
        token_id: 0.into(),
    })
    .encode();
    assert!(res.contains(&(USERS[0], message)));
}

#[test]
fn mint_limit_exceed() {
    let sys = System::new();
    sys.init_logger();
    let nft = gtest::Program::current(&sys);

    let collection = Collection {
        name: String::from("MyToken"),
        description: String::from("My token"),
    };

    let init_nft = InitNFT {
        collection,
        royalties: None,
        constraints: Constraints {
            max_mint_count: Some(1),
            authorized_minters: vec![USERS[0].into()],
        },
    };

    let res = nft.send(USERS[0], init_nft);

    assert!(!res.main_failed());

    let nft = sys.get_program(1);
    let transaction_id: u64 = 0;
    let res = mint(&nft, transaction_id, USERS[0]);
    assert!(!res.main_failed());
    let res = mint(&nft, transaction_id + 1, USERS[1]);
    assert!(res.main_failed())
}

#[test]
fn mint_authorized() {
    let sys = System::new();
    sys.init_logger();
    let nft = gtest::Program::current(&sys);

    let collection = Collection {
        name: String::from("MyToken"),
        description: String::from("My token"),
    };

    let authorized_minters: Vec<ActorId> = vec![USERS[0].into()];
    let init_nft = InitNFT {
        collection,
        royalties: None,
        constraints: Constraints {
            max_mint_count: None,
            authorized_minters,
        },
    };

    let res = nft.send(USERS[0], init_nft);

    assert!(!res.main_failed());

    let nft = sys.get_program(1);
    let transaction_id: u64 = 0;
    let res = mint(&nft, transaction_id, USERS[0]);
    assert!(!res.main_failed());
    let res = mint(&nft, transaction_id + 1, USERS[0]);
    assert!(!res.main_failed())
}

#[test]
fn mint_not_authorized() {
    let sys = System::new();
    sys.init_logger();
    let nft = gtest::Program::current(&sys);

    let collection = Collection {
        name: String::from("MyToken"),
        description: String::from("My token"),
    };

    let authorized_minters: Vec<ActorId> = vec![USERS[0].into()];
    let init_nft = InitNFT {
        collection,
        royalties: None,
        constraints: Constraints {
            max_mint_count: None,
            authorized_minters,
        },
    };

    let res = nft.send(USERS[0], init_nft);

    assert!(!res.main_failed());

    let nft = sys.get_program(1);
    let transaction_id: u64 = 0;
    let res = mint(&nft, transaction_id, USERS[0]);
    assert!(!res.main_failed());
    let res = mint(&nft, transaction_id + 1, USERS[1]);
    assert!(res.main_failed())
}

#[test]
fn mint_added() {
    let sys = System::new();
    sys.init_logger();
    let nft = gtest::Program::current(&sys);

    let collection = Collection {
        name: String::from("MyToken"),
        description: String::from("My token"),
    };

    let authorized_minters: Vec<ActorId> = vec![USERS[0].into()];
    let init_nft = InitNFT {
        collection,
        royalties: None,
        constraints: Constraints {
            max_mint_count: None,
            authorized_minters,
        },
    };

    let res = nft.send(USERS[0], init_nft);

    assert!(!res.main_failed());

    let nft = sys.get_program(1);
    let transaction_id: u64 = 0;
    let res = add_minter(&nft, transaction_id, USERS[1].into(), USERS[0]);
    assert!(!res.main_failed());
    let res = add_minter(&nft, transaction_id + 1, USERS[2].into(), USERS[1]);
    assert!(!res.main_failed());

    let res = add_minter(&nft, transaction_id + 1, 5.into(), 7);
    assert!(res.main_failed())
}

#[test]
fn burn_success() {
    let sys = System::new();
    init_nft(&sys);
    let nft = sys.get_program(1);
    let mut transaction_id: u64 = 0;
    assert!(!mint(&nft, transaction_id, USERS[0]).main_failed());
    transaction_id += 1;
    let res = burn(&nft, transaction_id, USERS[0], 0);
    let message = NFTEvent::Transfer(NFTTransfer {
        from: USERS[0].into(),
        to: ZERO_ID.into(),
        token_id: 0.into(),
    })
    .encode();
    assert!(res.contains(&(USERS[0], message)));
}

#[test]
fn burn_failures() {
    let sys = System::new();
    init_nft(&sys);
    let nft = sys.get_program(1);
    let mut transaction_id: u64 = 0;
    assert!(!mint(&nft, transaction_id, USERS[0]).main_failed());
    // must fail since the token doesn't exist
    transaction_id += 1;
    assert!(burn(&nft, transaction_id, USERS[0], 1).main_failed());
    // must fail since the caller is not the token owner
    transaction_id += 1;
    assert!(burn(&nft, transaction_id, USERS[1], 0).main_failed());
}

#[test]
fn transfer_success() {
    let sys = System::new();
    init_nft(&sys);
    let nft = sys.get_program(1);
    let mut transaction_id: u64 = 0;
    assert!(!mint(&nft, transaction_id, USERS[0]).main_failed());
    transaction_id += 1;
    let res = transfer(&nft, transaction_id, USERS[0], USERS[1], 0);
    let message = NFTEvent::Transfer(NFTTransfer {
        from: USERS[0].into(),
        to: USERS[1].into(),
        token_id: 0.into(),
    })
    .encode();
    assert!(res.contains(&(USERS[0], message)));
}

#[test]
fn transfer_failures() {
    let sys = System::new();
    init_nft(&sys);
    let nft = sys.get_program(1);
    let mut transaction_id: u64 = 0;
    assert!(!mint(&nft, transaction_id, USERS[0]).main_failed());

    // must fail since the token doesn't exist
    transaction_id += 1;
    assert!(transfer(&nft, transaction_id, USERS[0], USERS[1], 1).main_failed());
    // must fail since the caller is not the token owner
    transaction_id += 1;
    assert!(transfer(&nft, transaction_id, USERS[1], USERS[0], 0).main_failed());
    // must fail since transfer to the zero address
    transaction_id += 1;
    assert!(transfer(&nft, transaction_id, USERS[1], ZERO_ID, 0).main_failed());
}

#[test]
fn owner_success() {
    let sys = System::new();
    init_nft(&sys);
    let nft = sys.get_program(1);
    let mut transaction_id: u64 = 0;
    assert!(!mint(&nft, transaction_id, USERS[0]).main_failed());
    transaction_id += 1;
    assert!(!approve(&nft, transaction_id, USERS[0], USERS[1], 0).main_failed());
    let res = owner_of(&nft, USERS[1], 0);
    println!("{:?}", res.decoded_log::<NFTEvent>());
    let message = NFTEvent::Owner {
        token_id: 0.into(),
        owner: ActorId::from(USERS[0]),
    }
    .encode();
    assert!(res.contains(&(USERS[1], message)));
}

#[test]
fn is_approved_to_success() {
    let sys = System::new();
    init_nft(&sys);
    let nft = sys.get_program(1);
    let mut transaction_id: u64 = 0;
    assert!(!mint(&nft, transaction_id, USERS[0]).main_failed());
    transaction_id += 1;
    assert!(!approve(&nft, transaction_id, USERS[0], USERS[1], 0).main_failed());

    let res = is_approved_to(&nft, USERS[1], 0, USERS[1]);
    println!("{:?}", res.decoded_log::<NFTEvent>());
    let message = NFTEvent::IsApproved {
        to: USERS[1].into(),
        token_id: 0.into(),
        approved: true,
    }
    .encode();
    assert!(res.contains(&(USERS[1], message)));

    let res = is_approved_to(&nft, USERS[1], 0, USERS[0]);
    println!("{:?}", res.decoded_log::<NFTEvent>());
    let message = NFTEvent::IsApproved {
        to: USERS[0].into(),
        token_id: 0.into(),
        approved: false,
    }
    .encode();
    assert!(res.contains(&(USERS[1], message)));
}

#[test]
fn is_approved_to_failure() {
    let sys = System::new();
    init_nft(&sys);
    let nft = sys.get_program(1);
    let mut transaction_id: u64 = 0;
    assert!(!mint(&nft, transaction_id, USERS[0]).main_failed());
    transaction_id += 1;
    assert!(!approve(&nft, transaction_id, USERS[0], USERS[1], 0).main_failed());
    let res = is_approved_to(&nft, USERS[1], 1, USERS[1]);
    println!("{:?}", res.decoded_log::<NFTEvent>());
    assert!(res.main_failed());
}

#[test]
fn approve_success() {
    let sys = System::new();
    init_nft(&sys);
    let nft = sys.get_program(1);
    let mut transaction_id: u64 = 0;
    assert!(!mint(&nft, transaction_id, USERS[0]).main_failed());
    transaction_id += 1;
    let res = approve(&nft, transaction_id, USERS[0], USERS[1], 0);
    let message = NFTEvent::Approval(NFTApproval {
        owner: USERS[0].into(),
        approved_account: USERS[1].into(),
        token_id: 0.into(),
    })
    .encode();
    assert!(res.contains(&(USERS[0], message)));
    transaction_id += 1;
    assert!(!transfer(&nft, transaction_id, USERS[1], USERS[2], 0).main_failed());
}

#[test]
fn approve_failures() {
    let sys = System::new();
    init_nft(&sys);
    let nft = sys.get_program(1);
    let mut transaction_id: u64 = 0;
    assert!(!mint(&nft, transaction_id, USERS[0]).main_failed());
    transaction_id += 1;
    // must fail since the token doesn't exist
    assert!(approve(&nft, transaction_id, USERS[0], USERS[1], 1).main_failed());
    transaction_id += 1;
    // must fail since the caller is not the token owner
    assert!(approve(&nft, transaction_id, USERS[1], USERS[0], 0).main_failed());
    transaction_id += 1;
    // must fail since approval to the zero address
    assert!(approve(&nft, transaction_id, USERS[1], ZERO_ID, 0).main_failed());

    //approve
    transaction_id += 1;
    assert!(!approve(&nft, transaction_id, USERS[0], USERS[1], 0).main_failed());
    //transfer
    transaction_id += 1;
    assert!(!transfer(&nft, transaction_id, USERS[1], USERS[2], 0).main_failed());
    //must fail since approval was removed after transferring
    transaction_id += 1;
    assert!(transfer(&nft, transaction_id, USERS[1], USERS[0], 0).main_failed());
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

    let mut transaction_id: u64 = 0;
    assert!(!add_minter(&nft, transaction_id, owner_id.into(), USERS[0]).main_failed());
    assert!(!mint_to_actor(&nft, transaction_id + 1, owner_id).main_failed());

    let message = DelegatedApproveMessage {
        token_owner_id: owner_id.into(),
        approved_actor_id: USERS[1].into(),
        nft_program_id: 1.into(),
        token_id: 0.into(),
        expiration_timestamp: sys.block_timestamp() + 10,
    };
    let signature = pair.sign(message.encode().as_slice());

    transaction_id += 1;
    let res = delegated_approve(&nft, transaction_id, USERS[1], message, signature.0);
    let message = NFTEvent::Approval(NFTApproval {
        owner: owner_id.into(),
        approved_account: USERS[1].into(),
        token_id: 0.into(),
    })
    .encode();
    assert!(res.contains(&(USERS[1], message)));
    assert!(!transfer(&nft, transaction_id, USERS[1], USERS[2], 0).main_failed());
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

    let mut transaction_id: u64 = 0;
    assert!(!add_minter(&nft, transaction_id, owner_id.into(), USERS[0]).main_failed());
    transaction_id += 1;
    assert!(!mint_to_actor(&nft, transaction_id, owner_id).main_failed());
    transaction_id += 1;
    assert!(!mint(&nft, transaction_id, USERS[0]).main_failed());
    transaction_id += 1;
    assert!(!mint_to_actor(&nft, transaction_id, owner_id).main_failed());

    // Not owner can't approve nft
    let message = DelegatedApproveMessage {
        token_owner_id: owner_id.into(),
        approved_actor_id: USERS[1].into(),
        nft_program_id: 1.into(),
        token_id: 1.into(),
        expiration_timestamp: sys.block_timestamp() + 10,
    };
    let signature = pair.sign(message.encode().as_slice());
    assert!(delegated_approve(&nft, transaction_id, USERS[1], message, signature.0).main_failed());

    // Only approved actor in delegated approve can send delegated approve action
    let message = DelegatedApproveMessage {
        token_owner_id: owner_id.into(),
        approved_actor_id: USERS[1].into(),
        nft_program_id: 1.into(),
        token_id: 0.into(),
        expiration_timestamp: sys.block_timestamp() + 10,
    };
    let signature = pair.sign(message.encode().as_slice());

    assert!(delegated_approve(&nft, transaction_id, USERS[0], message, signature.0).main_failed());
    // Must fail if user tries to approve token in wrong contract
    init_nft(&sys);
    let second_nft = sys.get_program(2);
    transaction_id += 1;
    assert!(!add_minter(&second_nft, transaction_id, owner_id.into(), USERS[0]).main_failed());
    transaction_id += 1;
    assert!(!mint_to_actor(&second_nft, transaction_id, owner_id).main_failed());

    let message = DelegatedApproveMessage {
        token_owner_id: owner_id.into(),
        approved_actor_id: USERS[1].into(),
        nft_program_id: 1.into(),
        token_id: 0.into(),
        expiration_timestamp: sys.block_timestamp() + 10,
    };
    let signature = pair.sign(message.encode().as_slice());

    assert!(
        delegated_approve(&second_nft, transaction_id, USERS[1], message, signature.0)
            .main_failed()
    );

    // Must fail if user tries to approve token to zero_id
    let message = DelegatedApproveMessage {
        token_owner_id: owner_id.into(),
        approved_actor_id: 0.into(),
        nft_program_id: 1.into(),
        token_id: 0.into(),
        expiration_timestamp: sys.block_timestamp() + 10,
    };
    let signature = pair.sign(message.encode().as_slice());
    assert!(delegated_approve(&nft, transaction_id, 0, message, signature.0).main_failed());

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
    assert!(
        delegated_approve(&nft, transaction_id, USERS[1], wrong_message, signature.0).main_failed()
    );

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
    assert!(delegated_approve(&nft, transaction_id, USERS[1], message, signature.0).main_failed());
}
