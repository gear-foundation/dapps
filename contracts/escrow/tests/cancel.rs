pub mod utils;
use utils::*;

#[test]
fn cancel_paid() {
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
    // Should fail because a buyer/seller tries to cancel a paid contract
    cancel_fail(&escrow_program, CONTRACT[0], BUYER[0]);
    cancel_fail(&escrow_program, CONTRACT[0], SELLER[0]);
}

#[test]
fn foreign_user_cancel() {
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
    // Should fail because not a buyer/seller saved in a contract tries to cancel
    cancel_fail(&escrow_program, CONTRACT[0], FOREIGN_USER);
}

#[test]
fn interact_after_cancel() {
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
    cancel(
        &escrow_program,
        CONTRACT[0],
        BUYER[0],
        BUYER[0],
        SELLER[0],
        AMOUNT[0],
    );

    // All of this should fail because nobody can interact with a contract after cancel
    deposit_fail(&escrow_program, CONTRACT[0], BUYER[0]);
    refund_fail(&escrow_program, CONTRACT[0], SELLER[0]);
    confirm_fail(&escrow_program, CONTRACT[0], BUYER[0]);
    cancel_fail(&escrow_program, CONTRACT[0], SELLER[0]);
}
