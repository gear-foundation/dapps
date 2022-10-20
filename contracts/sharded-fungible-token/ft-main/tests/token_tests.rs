pub mod utils;
use gtest::{Program, System};
use utils::*;

#[test]
fn mint() {
    let system = System::new();
    system.init_logger();
    let transaction_id: u64 = 0;
    let account: u64 = 100;
    let amount: u128 = 100_000;
    let ftoken = Program::ftoken(&system);

    // check balance before
    ftoken.check_balance(account, 0);
    // mint tokens
    ftoken.mint(transaction_id, account, account, amount, false);
    // check balance after
    ftoken.check_balance(account, amount);

    // try to mint again with the same transaction id
    ftoken.mint(transaction_id, account, account, amount, false);
    // check balance
    ftoken.check_balance(account, amount);

    // mint again
    ftoken.mint(transaction_id + 1, account, account, amount, false);
    // check balance
    ftoken.check_balance(account, 2 * amount);
}

#[test]
fn burn() {
    let system = System::new();
    system.init_logger();
    let transaction_id: u64 = 0;
    let account: u64 = 100;
    let wrong_account: u64 = 101;
    let amount: u128 = 100_000;
    let ftoken = Program::ftoken(&system);

    // mint tokens
    ftoken.mint(transaction_id, account, account, amount, false);
    // check balance
    ftoken.check_balance(account, amount);

    // burn token
    ftoken.burn(transaction_id + 1, account, account, amount / 2, false);
    // check balance
    ftoken.check_balance(account, amount / 2);

    // must fail since not approved account tries to burn tokens
    ftoken.burn(
        transaction_id + 2,
        wrong_account,
        account,
        amount / 10,
        true,
    );

    // must fail since account has no enough tokens to burn
    ftoken.burn(transaction_id + 3, account, account, amount, true);
}

#[test]
fn transfer() {
    let system = System::new();
    system.init_logger();
    let transaction_id: u64 = 0;
    let sender: u64 = 100;
    let recipient: u64 = 200;
    let wrong_account: u64 = 101;
    let amount: u128 = 100_000;
    let ftoken = Program::ftoken(&system);

    // mint tokens
    ftoken.mint(transaction_id, sender, sender, amount, false);
    // check balance
    ftoken.check_balance(sender, amount);

    ftoken.transfer(
        transaction_id + 1,
        sender,
        sender,
        recipient,
        amount / 10,
        false,
    );

    // check balance
    ftoken.check_balance(sender, amount - amount / 10);
    ftoken.check_balance(recipient, amount / 10);

    // must fail since not approved account tries to transfer the tokens
    ftoken.transfer(
        transaction_id + 2,
        wrong_account,
        sender,
        recipient,
        amount / 10,
        true,
    );
}

#[test]
fn approve() {
    let system = System::new();
    system.init_logger();
    let transaction_id: u64 = 0;
    let sender: u64 = 100;
    let recipient: u64 = 200;
    let approved_account: u64 = 300;
    let amount: u128 = 100_000;
    let ftoken = Program::ftoken(&system);

    // mint tokens
    ftoken.mint(transaction_id, sender, sender, amount, false);
    // check balance
    ftoken.check_balance(sender, amount);

    ftoken.approve(
        transaction_id + 1,
        sender,
        approved_account,
        amount / 2,
        false,
    );

    ftoken.transfer(
        transaction_id + 2,
        approved_account,
        sender,
        recipient,
        amount / 10,
        false,
    );

    // check balance
    ftoken.check_balance(sender, amount - amount / 10);
    ftoken.check_balance(recipient, amount / 10);

    // must fail since approved account tries to transfer more token than allowed amount
    ftoken.transfer(
        transaction_id + 3,
        approved_account,
        sender,
        recipient,
        amount / 2,
        true,
    );

    // approve one more time
    ftoken.approve(
        transaction_id + 4,
        sender,
        approved_account,
        amount / 10,
        false,
    );

    ftoken.transfer(
        transaction_id + 5,
        approved_account,
        sender,
        recipient,
        amount / 2,
        false,
    );

    // check balance
    ftoken.check_balance(sender, amount - amount / 10 - amount / 2);
    ftoken.check_balance(recipient, amount / 10 + amount / 2);

    // approve one more time for burn
    ftoken.approve(
        transaction_id + 6,
        sender,
        approved_account,
        amount / 10,
        false,
    );

    // must fail since sender has no enough tokens
    ftoken.burn(
        transaction_id + 7,
        approved_account,
        sender,
        amount / 10,
        false,
    );

    ftoken.check_balance(sender, amount - amount / 5 - amount / 2);
}
