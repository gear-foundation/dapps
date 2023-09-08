pub mod utils;
use utils::*;

#[test]
fn refund_not_paid() {
    let system = init_system();
    let escrow_program = init_escrow(&system);
    let ft_program = Program::ftoken(WALLET[0] as u64, FT_PROGRAM_ID, &system);

    ft_program.mint(0, WALLET[0] as u64, BUYER[0], AMOUNT[0], false);
    check::create(
        &escrow_program,
        WALLET[0],
        SELLER[0],
        BUYER[0],
        SELLER[0],
        AMOUNT[0],
    );
    // Should fail because the seller tries to refund tokens from the unpaid wallet.
    fail::refund(&escrow_program, WALLET[0], SELLER[0]);
}

#[test]
fn not_seller_refund() {
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
    // Should fail because not the seller for this wallet tries to refund.
    fail::refund(&escrow_program, WALLET[0], FOREIGN_USER);
    fail::refund(&escrow_program, WALLET[0], BUYER[0]);
    fail::refund(&escrow_program, WALLET[0], SELLER[1]);
}
