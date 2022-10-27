pub mod utils;
use utils::*;

#[test]
fn cancel_paid() {
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
    // Should fail because the buyer/seller tries to cancel the deal with the paid wallet.
    fail::cancel(&escrow_program, WALLET[0], BUYER[0]);
    fail::cancel(&escrow_program, WALLET[0], SELLER[0]);
}

#[test]
fn foreign_user_cancel() {
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
    // Should fail because not the buyer/seller for this wallet tries to cancel the deal and close the wallet.
    fail::cancel(&escrow_program, WALLET[0], FOREIGN_USER);
}

#[test]
fn interact_after_cancel() {
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
    check::cancel(&escrow_program, WALLET[0], BUYER[0]);

    // All of this should fail because nobody can interact with a closed wallet.
    fail::deposit(&escrow_program, WALLET[0], BUYER[0], false);
    fail::refund(&escrow_program, WALLET[0], SELLER[0]);
    fail::confirm(&escrow_program, WALLET[0], BUYER[0]);
    fail::cancel(&escrow_program, WALLET[0], SELLER[0]);
}
