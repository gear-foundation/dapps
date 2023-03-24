mod utils;

use gstd::prelude::*;
use gtest::{Program, System};
use mt_logic_io::TokenId;
use std::mem;
use utils::{MToken, USER_ACCOUNTS};

#[test]
fn success_create_ft() {
    let system = System::new();
    system.init_logger();

    let mut tx_id = 0;
    let initial_amount = 1000000;
    let mtoken = Program::mtoken(&system);

    let token_id: TokenId = 1 << (mem::size_of::<TokenId>() * 8 / 2);
    mtoken.create(
        tx_id,
        USER_ACCOUNTS[0],
        initial_amount,
        String::from("https://example.com"),
        false,
        false,
    );
    assert_eq!(
        mtoken.get_balance(token_id, USER_ACCOUNTS[0]),
        initial_amount
    );
    tx_id += 1;

    let token_id: TokenId = 2 << (mem::size_of::<TokenId>() * 8 / 2);
    mtoken.create(
        tx_id,
        USER_ACCOUNTS[1],
        initial_amount * 2,
        String::from("https://example1.com"),
        false,
        false,
    );
    assert_eq!(
        mtoken.get_balance(token_id, USER_ACCOUNTS[1]),
        initial_amount * 2
    );
    tx_id += 1;

    let token_id: TokenId = 3 << (mem::size_of::<TokenId>() * 8 / 2);
    mtoken.create(
        tx_id,
        USER_ACCOUNTS[0],
        initial_amount / 10000,
        String::from("https://example1.com"),
        false,
        false,
    );
    assert_eq!(
        mtoken.get_balance(token_id, USER_ACCOUNTS[0]),
        initial_amount / 10000
    );
}

#[test]
fn success_mint_batch_ft() {
    let system = System::new();
    system.init_logger();

    let mut tx_id = 0;
    let base_amount = 133700;
    let initial_amount = 1000000;
    let token_id: TokenId = 1 << (mem::size_of::<TokenId>() * 8 / 2);
    let mtoken = Program::mtoken(&system);

    mtoken.create(
        tx_id,
        USER_ACCOUNTS[0],
        initial_amount,
        String::from("https://example.com"),
        false,
        false,
    );
    tx_id += 1;

    mtoken.mint_batch_ft(
        tx_id,
        USER_ACCOUNTS[0],
        token_id,
        vec![USER_ACCOUNTS[1]],
        vec![base_amount],
        false,
    );
    assert_eq!(mtoken.get_balance(token_id, USER_ACCOUNTS[1]), base_amount);
    tx_id += 1;

    mtoken.mint_batch_ft(
        tx_id,
        USER_ACCOUNTS[0],
        token_id,
        vec![USER_ACCOUNTS[1]],
        vec![base_amount * 2],
        false,
    );
    assert_eq!(
        mtoken.get_balance(token_id, USER_ACCOUNTS[1]),
        base_amount + base_amount * 2
    );
    tx_id += 1;

    mtoken.mint_batch_ft(
        tx_id,
        USER_ACCOUNTS[0],
        token_id,
        vec![USER_ACCOUNTS[0]],
        vec![base_amount],
        false,
    );
    assert_eq!(
        mtoken.get_balance(token_id, USER_ACCOUNTS[0]),
        initial_amount + base_amount
    );
}

#[test]
fn success_burn_batch_ft() {
    let system = System::new();
    system.init_logger();

    let mut tx_id = 0;
    let base_amount = 1337000;
    let initial_amount = 1000000;
    let burn_amount = 10000;
    let token_id: TokenId = 1 << (mem::size_of::<TokenId>() * 8 / 2);
    let mtoken = Program::mtoken(&system);

    mtoken.create(
        tx_id,
        USER_ACCOUNTS[0],
        initial_amount,
        String::from("https://example.com"),
        false,
        false,
    );
    tx_id += 1;

    mtoken.mint_batch_ft(
        tx_id,
        USER_ACCOUNTS[0],
        token_id,
        vec![USER_ACCOUNTS[1]],
        vec![base_amount],
        false,
    );
    tx_id += 1;

    mtoken.burn_batch_ft(
        tx_id,
        USER_ACCOUNTS[1],
        token_id,
        vec![USER_ACCOUNTS[1]],
        vec![burn_amount],
        false,
    );
    assert_eq!(
        mtoken.get_balance(token_id, USER_ACCOUNTS[1]),
        base_amount - burn_amount
    );
    tx_id += 1;

    mtoken.burn_batch_ft(
        tx_id,
        USER_ACCOUNTS[0],
        token_id,
        vec![USER_ACCOUNTS[0]],
        vec![burn_amount],
        false,
    );
    assert_eq!(
        mtoken.get_balance(token_id, USER_ACCOUNTS[0]),
        initial_amount - burn_amount
    );
    tx_id += 1;

    mtoken.approve(tx_id, USER_ACCOUNTS[1], USER_ACCOUNTS[0], true, false);
    tx_id += 1;

    mtoken.burn_batch_ft(
        tx_id,
        USER_ACCOUNTS[0],
        token_id,
        vec![USER_ACCOUNTS[1]],
        vec![burn_amount],
        false,
    );
    assert_eq!(
        mtoken.get_balance(token_id, USER_ACCOUNTS[1]),
        base_amount - burn_amount - burn_amount
    );
}

#[test]
fn success_approve_ft() {
    let system = System::new();
    system.init_logger();

    let mut tx_id = 0;
    let mtoken = Program::mtoken(&system);

    assert!(!mtoken.get_approval(USER_ACCOUNTS[0], USER_ACCOUNTS[1]));
    mtoken.approve(tx_id, USER_ACCOUNTS[0], USER_ACCOUNTS[1], true, false);
    assert!(mtoken.get_approval(USER_ACCOUNTS[0], USER_ACCOUNTS[1]));
    tx_id += 1;

    mtoken.approve(tx_id, USER_ACCOUNTS[0], USER_ACCOUNTS[1], false, false);
    assert!(!mtoken.get_approval(USER_ACCOUNTS[0], USER_ACCOUNTS[1]));
    tx_id += 1;

    mtoken.approve(tx_id, USER_ACCOUNTS[1], USER_ACCOUNTS[0], true, false);
    assert!(mtoken.get_approval(USER_ACCOUNTS[1], USER_ACCOUNTS[0]));
    tx_id += 1;

    mtoken.approve(tx_id, USER_ACCOUNTS[1], USER_ACCOUNTS[0], false, false);
    assert!(!mtoken.get_approval(USER_ACCOUNTS[1], USER_ACCOUNTS[0]));
}

#[test]
fn success_transfer_ft() {
    let system = System::new();
    system.init_logger();

    let mut tx_id = 0;
    let initial_amount = 1000000;
    let transfer_amount = 50000;
    let transfer_return_amount = transfer_amount / 2;
    let token_id: TokenId = 1 << (mem::size_of::<TokenId>() * 8 / 2);
    let mtoken = Program::mtoken(&system);

    mtoken.create(
        tx_id,
        USER_ACCOUNTS[0],
        initial_amount,
        String::from("https://example.com"),
        false,
        false,
    );
    assert_eq!(
        mtoken.get_balance(token_id, USER_ACCOUNTS[0]),
        initial_amount
    );
    assert_eq!(mtoken.get_balance(token_id, USER_ACCOUNTS[1]), 0);
    tx_id += 1;

    mtoken.transfer(
        tx_id,
        USER_ACCOUNTS[0],
        token_id,
        USER_ACCOUNTS[1],
        transfer_amount,
        false,
    );
    assert_eq!(
        mtoken.get_balance(token_id, USER_ACCOUNTS[0]),
        initial_amount - transfer_amount
    );
    assert_eq!(
        mtoken.get_balance(token_id, USER_ACCOUNTS[1]),
        transfer_amount
    );
    tx_id += 1;

    mtoken.transfer(
        tx_id,
        USER_ACCOUNTS[1],
        token_id,
        USER_ACCOUNTS[0],
        transfer_return_amount,
        false,
    );
    assert_eq!(
        mtoken.get_balance(token_id, USER_ACCOUNTS[0]),
        initial_amount - transfer_amount + transfer_return_amount
    );
    assert_eq!(
        mtoken.get_balance(token_id, USER_ACCOUNTS[1]),
        transfer_return_amount
    );
    tx_id += 1;

    mtoken.transfer(
        tx_id,
        USER_ACCOUNTS[1],
        token_id,
        USER_ACCOUNTS[0],
        transfer_return_amount,
        false,
    );
    assert_eq!(
        mtoken.get_balance(token_id, USER_ACCOUNTS[0]),
        initial_amount
    );
    assert_eq!(mtoken.get_balance(token_id, USER_ACCOUNTS[1]), 0);
}
