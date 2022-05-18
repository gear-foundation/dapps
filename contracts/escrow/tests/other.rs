pub mod utils;
use utils::*;

#[test]
fn two_different_escrows() {
    const AMOUNT_REMAINDER: u128 = 20000;

    let system = init_system();
    let escrow_program = init_escrow(&system);
    let ft_program = init_fungible_tokens(&system);

    mint(&ft_program, BUYER[0], AMOUNT[0] + AMOUNT_REMAINDER);
    mint(&ft_program, BUYER[1], AMOUNT[1] + AMOUNT_REMAINDER);

    create(
        &escrow_program,
        CONTRACT[0],
        SELLER[0],
        BUYER[0],
        SELLER[0],
        AMOUNT[0],
    );
    create(
        &escrow_program,
        CONTRACT[1],
        SELLER[1],
        BUYER[1],
        SELLER[1],
        AMOUNT[1],
    );

    deposit(&escrow_program, CONTRACT[0], BUYER[0], AMOUNT[0]);
    deposit(&escrow_program, CONTRACT[1], BUYER[1], AMOUNT[1]);

    confirm(&escrow_program, CONTRACT[0], BUYER[0], SELLER[0], AMOUNT[0]);
    confirm(&escrow_program, CONTRACT[1], BUYER[1], SELLER[1], AMOUNT[1]);

    check_balance(&ft_program, BUYER[0], AMOUNT_REMAINDER);
    check_balance(&ft_program, BUYER[1], AMOUNT_REMAINDER);

    check_balance(&ft_program, SELLER[0], AMOUNT[0]);
    check_balance(&ft_program, SELLER[1], AMOUNT[1]);
}

#[test]
fn reuse_after_refund() {
    let system = init_system();
    let escrow_program = init_escrow(&system);
    let ft_program = init_fungible_tokens(&system);

    mint(&ft_program, BUYER[0], AMOUNT[0]);
    create(
        &escrow_program,
        CONTRACT[0],
        SELLER[0],
        BUYER[0],
        SELLER[0],
        AMOUNT[0],
    );
    deposit(&escrow_program, CONTRACT[0], BUYER[0], AMOUNT[0]);

    refund(&escrow_program, CONTRACT[0], BUYER[0], SELLER[0], AMOUNT[0]);
    check_balance(&ft_program, BUYER[0], AMOUNT[0]);

    deposit(&escrow_program, CONTRACT[0], BUYER[0], AMOUNT[0]);
    confirm(&escrow_program, CONTRACT[0], BUYER[0], SELLER[0], AMOUNT[0]);
}

#[test]
fn interact_with_non_existend_contract() {
    const NONEXISTEND_CONTRACT: u128 = 999999;

    let system = init_system();
    let escrow_program = init_escrow(&system);

    deposit_fail(&escrow_program, NONEXISTEND_CONTRACT, BUYER[0]);
    cancel_fail(&escrow_program, NONEXISTEND_CONTRACT, BUYER[0]);
    refund_fail(&escrow_program, NONEXISTEND_CONTRACT, BUYER[0]);
    confirm_fail(&escrow_program, NONEXISTEND_CONTRACT, BUYER[0]);
}
