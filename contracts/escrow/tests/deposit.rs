pub mod utils;
use utils::*;

#[test]
fn not_enougn_tokens() {
    let system = init_system();

    let escrow_program = init_escrow(&system);
    Program::ftoken(WALLET[0] as u64, FT_PROGRAM_ID, &system);

    check::create(
        &escrow_program,
        WALLET[0],
        SELLER[0],
        BUYER[0],
        SELLER[0],
        AMOUNT[0],
    );
    // Should fail because the buyer doesn't have enough tokens to deposit.
    fail::deposit(&escrow_program, WALLET[0], BUYER[0], true);
}

#[test]
fn double_deposit() {
    let system = init_system();
    let escrow_program = init_escrow(&system);
    let ft_program = Program::ftoken(WALLET[0] as u64, FT_PROGRAM_ID, &system);

    // Purposely make it possible for the buyer to pay twice.
    ft_program.mint(0, WALLET[0] as u64, BUYER[0], AMOUNT[0] * 2, false);
    ft_program.approve(1, BUYER[0], ESCROW_PROGRAM_ID, AMOUNT[0], false);
    check::create(
        &escrow_program,
        WALLET[0],
        SELLER[0],
        BUYER[0],
        SELLER[0],
        AMOUNT[0],
    );
    check::deposit(&escrow_program, WALLET[0], BUYER[0], 0);
    // Should fail because the buyer tries to deposit twice.
    fail::deposit(&escrow_program, WALLET[0], BUYER[0], false);
    ft_program.check_balance(BUYER[0], AMOUNT[0]);
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
    fail::deposit(&escrow_program, WALLET[0], FOREIGN_USER, false);
    fail::deposit(&escrow_program, WALLET[0], BUYER[1], false);
    fail::deposit(&escrow_program, WALLET[0], SELLER[0], false);
}
