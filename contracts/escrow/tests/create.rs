pub mod utils;
use utils::*;

#[test]
fn foreign_user_create() {
    let system = init_system();
    let escrow_program = init_escrow(&system);

    // Should fail because not a future buyer/seller for this wallet tries to create it.
    fail::create(
        &escrow_program,
        FOREIGN_USER,
        BUYER[0],
        SELLER[0],
        AMOUNT[0],
    );
}
