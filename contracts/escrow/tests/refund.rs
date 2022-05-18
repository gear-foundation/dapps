pub mod utils;
use utils::*;

#[test]
fn refund_not_paid() {
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
    // Should fail because a seller tries to refund an unpaid contract
    refund_fail(&escrow_program, CONTRACT[0], SELLER[0]);
}

#[test]
fn not_seller_refund() {
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
    // Should fail because not a seller saved in a contract tries to refund
    refund_fail(&escrow_program, CONTRACT[0], FOREIGN_USER);
    refund_fail(&escrow_program, CONTRACT[0], BUYER[0]);
    refund_fail(&escrow_program, CONTRACT[0], SELLER[1]);
}
