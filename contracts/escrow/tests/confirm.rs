pub mod utils;
use utils::*;

#[test]
fn not_buyer_confirm() {
    let system = init_system();
    let escrow_program = init_escrow(&system);
    let ft_program = Program::ftoken(WALLET[0] as u64, FT_PROGRAM_ID, &system);

    ft_program.mint(0, WALLET[0] as u64, BUYER[0], AMOUNT[0], false);
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
    // Should fail because not the buyer for this wallet tries to confirm the deal.
    fail::confirm(&escrow_program, WALLET[0], FOREIGN_USER);
    fail::confirm(&escrow_program, WALLET[0], BUYER[1]);
    fail::confirm(&escrow_program, WALLET[0], SELLER[0]);
    ft_program.check_balance(SELLER[0], 0);
}

#[test]
fn double_confirm() {
    let system = init_system();
    let escrow_program = init_escrow(&system);
    let ft_program = Program::ftoken(WALLET[0] as u64, FT_PROGRAM_ID, &system);

    ft_program.mint(0, WALLET[0] as u64, BUYER[0], AMOUNT[0], false);
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
    check::confirm(&escrow_program, WALLET[0], BUYER[0], 1);
    // Should fail because the buyer tries to confirm the deal twice.
    fail::confirm(&escrow_program, WALLET[0], BUYER[0]);
    ft_program.check_balance(SELLER[0], AMOUNT[0]);
}

#[test]
fn confirm_before_deposit() {
    let system = init_system();
    let escrow_program = init_escrow(&system);
    let ft_program = Program::ftoken(WALLET[0] as u64, FT_PROGRAM_ID, &system);

    ft_program.mint(0, WALLET[0] as u64, BUYER[0], AMOUNT[0], false);
    ft_program.approve(1, BUYER[0], ESCROW_PROGRAM_ID, AMOUNT[0], false);
    check::create(
        &escrow_program,
        WALLET[0],
        SELLER[0],
        BUYER[0],
        SELLER[0],
        AMOUNT[0],
    );
    // Should fail because the buyer tries to confirm the deal before depositing.
    fail::confirm(&escrow_program, WALLET[0], BUYER[0]);
    ft_program.check_balance(SELLER[0], 0);
}

#[test]
fn interact_after_confirm() {
    let system = init_system();
    let escrow_program = init_escrow(&system);
    let ft_program = Program::ftoken(WALLET[0] as u64, FT_PROGRAM_ID, &system);

    ft_program.mint(0, WALLET[0] as u64, BUYER[0], AMOUNT[0], false);
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
    check::confirm(&escrow_program, WALLET[0], BUYER[0], 1);

    // All of this should fail because nobody can interact with a wallet after confirming a deal.
    fail::deposit(&escrow_program, WALLET[0], BUYER[0], false);
    fail::refund(&escrow_program, WALLET[0], SELLER[0]);
    fail::confirm(&escrow_program, WALLET[0], BUYER[0]);
    fail::cancel(&escrow_program, WALLET[0], SELLER[0]);
}
