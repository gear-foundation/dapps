pub mod utils;
use utils::*;

#[test]
fn two_different_escrows() {
    const AMOUNT_REMAINDER: u128 = 20000;

    let system = init_system();
    let escrow_program = init_escrow(&system);
    let ft_program = init_ft(&system);

    mint(&ft_program, BUYER[0], AMOUNT[0] + AMOUNT_REMAINDER);
    mint(&ft_program, BUYER[1], AMOUNT[1] + AMOUNT_REMAINDER);

    check::create(
        &escrow_program,
        WALLET[0],
        SELLER[0],
        BUYER[0],
        SELLER[0],
        AMOUNT[0],
    );
    check::create(
        &escrow_program,
        WALLET[1],
        SELLER[1],
        BUYER[1],
        SELLER[1],
        AMOUNT[1],
    );

    check::deposit(&escrow_program, WALLET[0], BUYER[0], AMOUNT[0]);
    check::deposit(&escrow_program, WALLET[1], BUYER[1], AMOUNT[1]);

    check::confirm(&escrow_program, WALLET[0], BUYER[0], SELLER[0], AMOUNT[0]);
    check::confirm(&escrow_program, WALLET[1], BUYER[1], SELLER[1], AMOUNT[1]);

    check_balance(&ft_program, BUYER[0], AMOUNT_REMAINDER);
    check_balance(&ft_program, BUYER[1], AMOUNT_REMAINDER);

    check_balance(&ft_program, SELLER[0], AMOUNT[0]);
    check_balance(&ft_program, SELLER[1], AMOUNT[1]);
}

#[test]
fn reuse_after_refund() {
    let system = init_system();
    let escrow_program = init_escrow(&system);
    let ft_program = init_ft(&system);

    mint(&ft_program, BUYER[0], AMOUNT[0]);
    check::create(
        &escrow_program,
        WALLET[0],
        SELLER[0],
        BUYER[0],
        SELLER[0],
        AMOUNT[0],
    );
    check::deposit(&escrow_program, WALLET[0], BUYER[0], AMOUNT[0]);

    check::refund(&escrow_program, WALLET[0], BUYER[0], SELLER[0], AMOUNT[0]);
    check_balance(&ft_program, BUYER[0], AMOUNT[0]);

    check::deposit(&escrow_program, WALLET[0], BUYER[0], AMOUNT[0]);
    check::confirm(&escrow_program, WALLET[0], BUYER[0], SELLER[0], AMOUNT[0]);
}

#[test]
fn interact_with_non_existend_wallet() {
    const NONEXISTEND_WALLET: u128 = 999999;

    let system = init_system();
    let escrow_program = init_escrow(&system);

    fail::deposit(&escrow_program, NONEXISTEND_WALLET, BUYER[0]);
    fail::cancel(&escrow_program, NONEXISTEND_WALLET, BUYER[0]);
    fail::refund(&escrow_program, NONEXISTEND_WALLET, BUYER[0]);
    fail::confirm(&escrow_program, NONEXISTEND_WALLET, BUYER[0]);
}
