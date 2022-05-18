pub mod utils;
use utils::*;

#[test]
fn foreign_user_create() {
    let system = init_system();
    let escrow_program = init_escrow(&system);

    // Should fail because not a buyer/seller who will be saved in a contract tries to create
    create_fail(
        &escrow_program,
        FOREIGN_USER,
        BUYER[0],
        SELLER[0],
        AMOUNT[0],
    );
}
