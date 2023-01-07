pub mod utils;
use ft_logic_io::PermitUnsigned;
use gstd::Encode;
use gtest::{Program, System};
use hex_literal::hex;
use sp_core::{sr25519::Pair as Sr25519Pair, Pair};
use utils::*;

#[test]
fn mint() {
    let system = System::new();
    system.init_logger();
    let mut transaction_id: u64 = 0;
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
    transaction_id += 1;

    // mint again
    ftoken.mint(transaction_id, account, account, amount, false);
    // check balance
    ftoken.check_balance(account, 2 * amount);
}

#[test]
fn burn() {
    let system = System::new();
    system.init_logger();
    let mut transaction_id: u64 = 0;
    let account: u64 = 100;
    let wrong_account: u64 = 101;
    let amount: u128 = 100_000;
    let ftoken = Program::ftoken(&system);

    // mint tokens
    ftoken.mint(transaction_id, account, account, amount, false);
    // check balance
    ftoken.check_balance(account, amount);
    transaction_id += 1;

    // burn token
    ftoken.burn(transaction_id, account, account, amount / 2, false);
    // check balance
    ftoken.check_balance(account, amount / 2);
    transaction_id += 1;

    // must fail since not approved account tries to burn tokens
    ftoken.burn(transaction_id, wrong_account, account, amount / 10, true);
    transaction_id += 1;

    // must fail since account has no enough tokens to burn
    ftoken.burn(transaction_id, account, account, amount, true);
}

#[test]
fn transfer() {
    let system = System::new();
    system.init_logger();
    let mut transaction_id: u64 = 0;
    let sender: u64 = 100;
    let recipient: u64 = 200;
    let wrong_account: u64 = 101;
    let amount: u128 = 100_000;
    let ftoken = Program::ftoken(&system);

    // mint tokens
    ftoken.mint(transaction_id, sender, sender, amount, false);
    // check balance
    ftoken.check_balance(sender, amount);
    transaction_id += 1;

    ftoken.transfer(
        transaction_id,
        sender,
        sender,
        recipient,
        amount / 10,
        false,
    );
    transaction_id += 1;

    // check balance
    ftoken.check_balance(sender, amount - amount / 10);
    ftoken.check_balance(recipient, amount / 10);

    // must fail since not approved account tries to transfer the tokens
    ftoken.transfer(
        transaction_id,
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
    let mut transaction_id: u64 = 0;
    let sender: u64 = 100;
    let recipient: u64 = 200;
    let approved_account: u64 = 300;
    let amount: u128 = 100_000;
    let ftoken = Program::ftoken(&system);

    // mint tokens
    ftoken.mint(transaction_id, sender, sender, amount, false);
    // check balance
    ftoken.check_balance(sender, amount);
    transaction_id += 1;

    ftoken.approve(transaction_id, sender, approved_account, amount / 2, false);
    transaction_id += 1;

    ftoken.transfer(
        transaction_id,
        approved_account,
        sender,
        recipient,
        amount / 10,
        false,
    );
    transaction_id += 1;

    // check balance
    ftoken.check_balance(sender, amount - amount / 10);
    ftoken.check_balance(recipient, amount / 10);

    // must fail since approved account tries to transfer more token than allowed amount
    ftoken.transfer(
        transaction_id,
        approved_account,
        sender,
        recipient,
        amount / 2,
        true,
    );
    transaction_id += 1;

    // approve one more time
    ftoken.approve(transaction_id, sender, approved_account, amount / 10, false);
    transaction_id += 1;

    ftoken.transfer(
        transaction_id,
        approved_account,
        sender,
        recipient,
        amount / 2,
        false,
    );
    transaction_id += 1;

    // check balance
    ftoken.check_balance(sender, amount - amount / 10 - amount / 2);
    ftoken.check_balance(recipient, amount / 10 + amount / 2);

    // approve one more time for burn
    ftoken.approve(transaction_id, sender, approved_account, amount / 10, false);
    transaction_id += 1;

    // must fail since sender has no enough tokens
    ftoken.burn(transaction_id, approved_account, sender, amount / 10, false);

    ftoken.check_balance(sender, amount - amount / 5 - amount / 2);
}

#[test]
fn permit() {
    let system = System::new();
    system.init_logger();
    let mut transaction_id: u64 = 0;
    let mut permit_id: u128 = 0;
    let sender: u64 = 100; // those who send permit
    let approved: u64 = 200; // those who is permitted spend
    let amount: u128 = 100_000;
    let ftoken = Program::ftoken(&system);

    let pair = Sr25519Pair::from_seed(&hex!(
        "9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60"
    ));
    let owner = pair.public().0;

    ftoken.mint(transaction_id, sender, sender, amount, false);
    ftoken.check_balance(sender, amount);
    transaction_id += 1;

    let signature;
    // Check that signing algorithm matches
    {
        let action_permit = PermitUnsigned {
            owner_account: owner.into(),
            approved_account: approved.into(),
            amount,
            permit_id,
        };
        let message_vec = action_permit.encode();
        let message_bytes = message_vec.as_slice();

        signature = pair.sign(message_bytes);

        assert!(light_sr25519::verify(signature.encode().as_slice(), message_bytes, owner).is_ok());
    }

    ftoken.check_permit_id(owner, 0);

    /*
     * sender   -> 100k tokens
     * owner    -> 0 tokens, permit_id 0
     * approved -> 0 tokens
     */
    // Failing 'cause of invalid signature
    ftoken.permit(
        transaction_id,
        sender,
        owner.into(),
        approved.into(),
        amount * 2,
        permit_id,
        signature.clone(),
        true,
    );
    transaction_id += 1;

    // Sending tokens to owner_id and aprrove
    ftoken.transfer(transaction_id, sender, sender, owner, amount, false);
    transaction_id += 1;
    ftoken.check_balance(owner, amount);
    ftoken.check_permit_id(owner, permit_id);

    /*
     * sender   -> 0 tokens
     * owner    -> 100k tokens, permit_id 0
     * approved -> 0 tokens
     */
    ftoken.permit(
        transaction_id,
        sender,
        owner.into(),
        approved.into(),
        amount,
        permit_id,
        signature.clone(),
        false,
    );
    transaction_id += 1;
    ftoken.transfer(transaction_id, approved, owner, sender, amount / 2, false);
    /*
     * sender   -> 50k tokens
     * owner    -> 50k tokens, permit_id 1
     * approved -> 0 tokens
     */
    ftoken.check_balance(sender, amount / 2);
    ftoken.check_balance(owner, amount / 2);
    ftoken.check_permit_id(owner, 1);

    // Failing cause of current permit_id is already executed
    ftoken.permit(
        transaction_id,
        sender,
        owner.into(),
        approved.into(),
        amount,
        permit_id,
        signature.clone(),
        true,
    );
    transaction_id += 1;

    // Failing cause of invalid permit_id sign
    permit_id += 1;
    ftoken.permit(
        transaction_id,
        sender,
        owner.into(),
        approved.into(),
        amount,
        permit_id,
        signature,
        true,
    );
    transaction_id += 1;
    /*
     * sender   -> 50k tokens
     * owner    -> 50k tokens, permit_id 1
     * approved -> 0 tokens
     */
    ftoken.check_permit_id(owner, 1);

    let new_signature;
    {
        let action_permit = PermitUnsigned {
            owner_account: owner.into(),
            approved_account: approved.into(),
            amount,
            permit_id,
        };
        let message_vec = action_permit.encode();
        let message_bytes = message_vec.as_slice();

        new_signature = pair.sign(message_bytes);

        assert!(
            light_sr25519::verify(new_signature.encode().as_slice(), message_bytes, owner).is_ok()
        );
    }
    ftoken.permit(
        transaction_id,
        sender,
        owner.into(),
        approved.into(),
        amount,
        permit_id,
        new_signature,
        false,
    );
    ftoken.check_permit_id(owner, 2);
}
