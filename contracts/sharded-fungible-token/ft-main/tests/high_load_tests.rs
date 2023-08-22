pub mod utils;
use gtest::{Program, System};
use utils::*;

// Change this parameter to scale test
const ACCOUNTS_AMOUNT: u64 = 1000;

#[ignore]
#[test]
fn high_load_mint() {
    const FIRST_ID: u64 = 100;
    let system = System::new();
    system.init_logger();
    let mut transaction_id: u64 = FIRST_ID;
    let amount: u128 = 100_000;
    let ftoken = Program::ftoken(&system);

    while transaction_id < FIRST_ID + ACCOUNTS_AMOUNT {
        // Mint tokens to account and check it
        println!("id is {transaction_id}");
        ftoken.mint(
            transaction_id,
            transaction_id,
            transaction_id,
            amount,
            false,
        );
        transaction_id += 1;
    }
}

#[test]
fn high_load_transfer() {
    const FIRST_ID: u64 = 100;

    // Change this parameter to control ratio Transfer/Mint
    const MINT_TRANSFER_RATIO: u64 = 10;

    let system = System::new();
    system.init_logger();
    let mut transaction_id: u64 = FIRST_ID;
    let amount: u128 = 100_000;
    let ftoken = Program::ftoken(&system);
    let transfer_amount = amount / MINT_TRANSFER_RATIO as u128;

    while transaction_id < FIRST_ID + ACCOUNTS_AMOUNT / MINT_TRANSFER_RATIO {
        // Mint tokens to account and check it
        println!("id is {transaction_id}");
        ftoken.mint(
            transaction_id,
            transaction_id,
            transaction_id,
            amount,
            false,
        );
        transaction_id += 1;
    }

    let last_id = transaction_id + ACCOUNTS_AMOUNT;
    let mut sender_id = FIRST_ID;
    while transaction_id < last_id {
        // Each account with minted tokens transfers tokens MINT_TRANSFER_RATIO times
        println!("user {sender_id} sending to {transaction_id}");
        ftoken.transfer(
            transaction_id,
            sender_id,
            sender_id,
            transaction_id,
            transfer_amount,
            false,
        );

        transaction_id += 1;
        if transaction_id % MINT_TRANSFER_RATIO == 0 {
            sender_id += 1;
        }
    }
}
