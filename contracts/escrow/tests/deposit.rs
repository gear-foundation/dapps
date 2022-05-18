pub mod utils;
use utils::*;

#[test]
fn not_enougn_tokens() {
    let system = init_system();

    let escrow_program = init_escrow(&system);
    let _ft_program = init_fungible_tokens(&system);

    create(
        &escrow_program,
        CONTRACT[0],
        SELLER[0],
        BUYER[0],
        SELLER[0],
        AMOUNT[0],
    );
    // Should fail because a buyer doesn't have enought tokens to deposit
    deposit_fail(&escrow_program, CONTRACT[0], BUYER[0]);
}

#[test]
fn double_deposit() {
    let system = init_system();
    let escrow_program = init_escrow(&system);
    let ft_program = init_fungible_tokens(&system);

    // Purposely make it possible for a buyer to pay twice
    mint(&ft_program, BUYER[0], AMOUNT[0] * 2);
    create(
        &escrow_program,
        CONTRACT[0],
        SELLER[0],
        BUYER[0],
        SELLER[0],
        AMOUNT[0],
    );
    deposit(&escrow_program, CONTRACT[0], BUYER[0], AMOUNT[0]);
    // Should fail because a buyer tries to make a deposit twice
    deposit_fail(&escrow_program, CONTRACT[0], BUYER[0]);
    check_balance(&ft_program, BUYER[0], AMOUNT[0]);
}

#[test]
fn not_buyer_deposit() {
    let system = init_system();
    let escrow_program = init_escrow(&system);

    create(
        &escrow_program,
        CONTRACT[0],
        SELLER[0],
        BUYER[0],
        SELLER[0],
        AMOUNT[0],
    );
    // Should fail because not a buyer saved in contract tries to make a deposit
    deposit_fail(&escrow_program, CONTRACT[0], FOREIGN_USER);
    deposit_fail(&escrow_program, CONTRACT[0], BUYER[1]);
    deposit_fail(&escrow_program, CONTRACT[0], SELLER[0]);
}
