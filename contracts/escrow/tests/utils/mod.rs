pub use escrow_io::*;
use gstd::prelude::*;
pub use gtest::{Program, System};
pub use token::*;

pub mod check;
pub mod fail;
pub mod token;

pub const FT_PROGRAM_ID: u64 = 2;
pub const ESCROW_PROGRAM_ID: u64 = 13370;
pub const FOREIGN_USER: u64 = 1337;
pub const BUYER: [u64; 2] = [12, 34];
pub const SELLER: [u64; 2] = [56, 78];
pub const AMOUNT: [u128; 2] = [12345, 54321];
pub const WALLET: [u128; 2] = [0, 1];
pub const AMOUNT_REMAINDER: u128 = 20000;
pub const NONEXISTENT_WALLET: u128 = 999999;

pub fn init_system() -> System {
    let system = System::new();
    system.init_logger();

    system
}

pub fn init_escrow(sys: &System) -> Program {
    let escrow_program = Program::current_with_id(sys, ESCROW_PROGRAM_ID);

    assert!(escrow_program
        .send(
            FOREIGN_USER,
            InitEscrow {
                ft_program_id: FT_PROGRAM_ID.into(),
            },
        )
        .log()
        .is_empty());

    escrow_program
}
