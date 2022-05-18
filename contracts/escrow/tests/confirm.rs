pub mod utils;
use utils::*;

#[test]
fn not_buyer_confirm() {
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
    // Should fail because not a buyer saved in a contract tries to confirm
    confirm_fail(&escrow_program, CONTRACT[0], FOREIGN_USER);
    confirm_fail(&escrow_program, CONTRACT[0], BUYER[1]);
    confirm_fail(&escrow_program, CONTRACT[0], SELLER[0]);
    check_balance(&ft_program, SELLER[0], 0);
}

#[test]
fn double_confirm() {
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
    confirm(&escrow_program, CONTRACT[0], BUYER[0], SELLER[0], AMOUNT[0]);
    // Should fail because a buyer tries to confirm twice
    confirm_fail(&escrow_program, CONTRACT[0], BUYER[0]);
    check_balance(&ft_program, SELLER[0], AMOUNT[0]);
}

#[test]
fn confirm_before_deposit() {
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
    // Should fail because a buyer tries to confirm before making a deposit
    confirm_fail(&escrow_program, CONTRACT[0], BUYER[0]);
    check_balance(&ft_program, SELLER[0], 0);
}

#[test]
fn interact_after_confirm() {
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
    confirm(&escrow_program, CONTRACT[0], BUYER[0], SELLER[0], AMOUNT[0]);

    // All of this should fail because nobody can interact with a contract after confirm
    deposit_fail(&escrow_program, CONTRACT[0], BUYER[0]);
    refund_fail(&escrow_program, CONTRACT[0], SELLER[0]);
    confirm_fail(&escrow_program, CONTRACT[0], BUYER[0]);
    cancel_fail(&escrow_program, CONTRACT[0], SELLER[0]);
}
