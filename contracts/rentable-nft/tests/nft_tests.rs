use gear_lib::non_fungible_token::io::NFTTransfer;
use gstd::{ActorId, Encode};
use gtest::System;
use std::time::{Duration, Instant};
mod utils;
use hex_literal::hex;
use nft_io::NFTEvent;
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
fn set_user() {
    let sys = System::new();
    init_nft(&sys);
    let nft = sys.get_program(1);
    let mut transaction_id = 0u64;
    assert!(!mint(&nft, transaction_id, USERS[0]).main_failed());
    transaction_id += 1;
    assert!(!approve(&nft, transaction_id, USERS[0], USERS[1], 0).main_failed());
    let pair = Sr25519Pair::from_seed(&hex!(
        "9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60"
    ));
    let owner_id = pair.public().0;
    transaction_id += 1;
    let duration = Duration::from_secs(1);

    let expires = duration.as_secs();
    println!("expires = {expires}");

    let res = utils::set_user(
        &nft,
        USERS[1],
        ActorId::from(owner_id),
        0u64.into(),
        expires,
        transaction_id,
    );

    println!("{:?}", res.decoded_log::<NFTEvent>());
    let message = NFTEvent::UpdateUser {
        token_id: 0.into(),
        address: ActorId::from(owner_id),
        expires,
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
fn user_of() {
    let sys = System::new();
    init_nft(&sys);
    let nft = sys.get_program(1);
    let mut transaction_id = 0u64;
    assert!(!mint(&nft, transaction_id, USERS[0]).main_failed());
    transaction_id += 1;
    assert!(!approve(&nft, transaction_id, USERS[0], USERS[1], 0).main_failed());
    let token_id = 0.into();
    let pair = Sr25519Pair::from_seed(&hex!(
        "9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60"
    ));
    let owner_id = pair.public().0;
    transaction_id += 1;
    let expires = (Instant::now() + Duration::from_secs(1))
        .elapsed()
        .as_secs();
    let res = utils::set_user(
        &nft,
        USERS[1],
        ActorId::from(owner_id),
        0u64.into(),
        expires,
        transaction_id,
    );
    println!("{:?}", res.decoded_log::<NFTEvent>());
    let message = NFTEvent::UpdateUser {
        token_id: 0.into(),
        address: ActorId::from(owner_id),
        expires,
    }
    .encode();
    assert!(res.contains(&(USERS[1], message)));
    let res = utils::user_of(&nft, USERS[1], token_id);
    println!("{:?}", res.decoded_log::<NFTEvent>());
    let message = NFTEvent::UserOf {
        address: ActorId::from(owner_id),
    }
    .encode();
    assert!(res.contains(&(USERS[1], message)));
}

#[test]
fn user_expires() {
    let sys = System::new();
    init_nft(&sys);
    let nft = sys.get_program(1);
    let mut transaction_id = 0u64;
    assert!(!mint(&nft, transaction_id, USERS[0]).main_failed());
    transaction_id += 1;
    assert!(!approve(&nft, transaction_id, USERS[0], USERS[1], 0).main_failed());
    let token_id = 0.into();
    let pair = Sr25519Pair::from_seed(&hex!(
        "9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60"
    ));
    let owner_id = pair.public().0;
    transaction_id += 1;

    let expires = (Instant::now() + Duration::from_secs(1))
        .elapsed()
        .as_secs();
    let res = utils::set_user(
        &nft,
        USERS[1],
        ActorId::from(owner_id),
        0u64.into(),
        expires,
        transaction_id,
    );
    println!("{:?}", res.decoded_log::<NFTEvent>());
    let message = NFTEvent::UpdateUser {
        token_id: 0.into(),
        address: ActorId::from(owner_id),
        expires,
    }
    .encode();
    assert!(res.contains(&(USERS[1], message)));

    let res = utils::user_expires(&nft, USERS[1], token_id);
    println!("{:?}", res.decoded_log::<NFTEvent>());
    let message = NFTEvent::UserExpires { expires }.encode();
    assert!(res.contains(&(USERS[1], message)));
}
