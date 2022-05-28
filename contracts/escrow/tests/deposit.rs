pub mod utils;
use utils::*;

#[test]
fn not_enougn_tokens() {
    let system = init_system();

    let escrow_program = init_escrow(&system);
    init_ft(&system);

    check::create(
        &escrow_program,
        WALLET[0],
        SELLER[0],
        BUYER[0],
        SELLER[0],
        AMOUNT[0],
    );
    // Should fail because the buyer doesn't have enough tokens to deposit.
    fail::deposit(&escrow_program, WALLET[0], BUYER[0]);
}

#[test]
fn double_deposit() {
    let system = init_system();
    let escrow_program = init_escrow(&system);
    let ft_program = init_ft(&system);

    // Purposely make it possible for the buyer to pay twice.
    mint(&ft_program, BUYER[0], AMOUNT[0] * 2);
    check::create(
        &escrow_program,
        WALLET[0],
        SELLER[0],
        BUYER[0],
        SELLER[0],
        AMOUNT[0],
    );
    check::deposit(&escrow_program, WALLET[0], BUYER[0], AMOUNT[0]);
    // Should fail because the buyer tries to deposit twice.
    fail::deposit(&escrow_program, WALLET[0], BUYER[0]);
    check_balance(&ft_program, BUYER[0], AMOUNT[0]);
}

#[test]
fn not_buyer_deposit() {
    let system = init_system();
    let escrow_program = init_escrow(&system);

    check::create(
        &escrow_program,
        WALLET[0],
        SELLER[0],
        BUYER[0],
        SELLER[0],
        AMOUNT[0],
    );
    // Should fail because not a buyer for this wallet tries to deposit.
    fail::deposit(&escrow_program, WALLET[0], FOREIGN_USER);
    fail::deposit(&escrow_program, WALLET[0], BUYER[1]);
    fail::deposit(&escrow_program, WALLET[0], SELLER[0]);
}
