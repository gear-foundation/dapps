mod utils;

use gstd::prelude::*;
use gtest::{Program, System};
use mt_logic_io::{TokenId, NFT_BIT};
use std::mem;
use utils::{MToken, USER_ACCOUNTS};

#[test]
fn success_create_and_mint_batch_nft() {
    let system = System::new();
    system.init_logger();

    let mut tx_id = 0;
    // Abstract `collection` id
    let token_id: TokenId = 1 << (mem::size_of::<TokenId>() * 8 / 2) | NFT_BIT;
    // Abstract `edition`(copy) id
    let minted_id_1: TokenId = token_id | 1;
    let minted_id_2: TokenId = token_id | 2;

    let mtoken = Program::mtoken(&system);

    mtoken.create(
        tx_id,
        USER_ACCOUNTS[0],
        0,
        String::from("https://example.com"),
        true,
        false,
    );
    tx_id += 1;

    mtoken.mint_batch_nft(
        tx_id,
        USER_ACCOUNTS[0],
        token_id,
        vec![USER_ACCOUNTS[1], USER_ACCOUNTS[2]],
        false,
    );
    assert_eq!(mtoken.get_balance(minted_id_1, USER_ACCOUNTS[1]), 1);
    assert_eq!(mtoken.get_balance(minted_id_2, USER_ACCOUNTS[2]), 1);
}

#[test]
fn success_transfer_nft() {
    let system = System::new();
    system.init_logger();

    let mut tx_id = 0;
    // Abstract `collection` id
    let token_id: TokenId = 1 << (mem::size_of::<TokenId>() * 8 / 2) | NFT_BIT;
    // Abstract `edition`(copy) id
    let minted_id_1: TokenId = token_id | 1;
    let minted_id_2: TokenId = token_id | 2;

    let mtoken = Program::mtoken(&system);

    mtoken.create(
        tx_id,
        USER_ACCOUNTS[0],
        0,
        String::from("https://example.com"),
        true,
        false,
    );
    tx_id += 1;

    mtoken.mint_batch_nft(
        tx_id,
        USER_ACCOUNTS[0],
        token_id,
        vec![USER_ACCOUNTS[1], USER_ACCOUNTS[2]],
        false,
    );
    tx_id += 1;

    mtoken.transfer(
        tx_id,
        USER_ACCOUNTS[2],
        minted_id_2,
        USER_ACCOUNTS[1],
        0,
        false,
    );
    assert_eq!(mtoken.get_balance(minted_id_2, USER_ACCOUNTS[2]), 0);
    assert_eq!(mtoken.get_balance(minted_id_2, USER_ACCOUNTS[1]), 1);
    tx_id += 1;

    mtoken.transfer(
        tx_id,
        USER_ACCOUNTS[1],
        minted_id_1,
        USER_ACCOUNTS[2],
        0,
        false,
    );
    assert_eq!(mtoken.get_balance(minted_id_1, USER_ACCOUNTS[1]), 0);
    assert_eq!(mtoken.get_balance(minted_id_1, USER_ACCOUNTS[2]), 1);
}

#[test]
fn success_burn_nft() {
    let system = System::new();
    system.init_logger();

    let mut tx_id = 0;
    // Abstract `collection` id
    let token_id: TokenId = 1 << (mem::size_of::<TokenId>() * 8 / 2) | NFT_BIT;
    // Abstract `edition`(copy) id
    let minted_id_1: TokenId = token_id | 1;
    let minted_id_2: TokenId = token_id | 2;

    let mtoken = Program::mtoken(&system);

    mtoken.create(
        tx_id,
        USER_ACCOUNTS[0],
        0,
        String::from("https://example.com"),
        true,
        false,
    );
    tx_id += 1;

    mtoken.mint_batch_nft(
        tx_id,
        USER_ACCOUNTS[0],
        token_id,
        vec![USER_ACCOUNTS[1], USER_ACCOUNTS[2]],
        false,
    );
    assert_eq!(mtoken.get_balance(minted_id_1, USER_ACCOUNTS[1]), 1);
    assert_eq!(mtoken.get_balance(minted_id_2, USER_ACCOUNTS[2]), 1);
    tx_id += 1;

    mtoken.burn_nft(
        tx_id,
        USER_ACCOUNTS[1],
        minted_id_1,
        USER_ACCOUNTS[1],
        false,
    );
    assert_eq!(mtoken.get_balance(minted_id_1, USER_ACCOUNTS[1]), 0);
    tx_id += 1;

    mtoken.burn_nft(
        tx_id,
        USER_ACCOUNTS[2],
        minted_id_2,
        USER_ACCOUNTS[2],
        false,
    );
    assert_eq!(mtoken.get_balance(minted_id_2, USER_ACCOUNTS[2]), 0);
}
