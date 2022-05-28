use escrow_io::{EscrowAction, EscrowEvent, InitEscrow};
use ft_io::{FTAction, FTEvent, InitConfig as InitFT};
use gstd::prelude::*;
use gtest::{Program, System};

pub mod check;
pub mod fail;

pub const FT_PROGRAM_ID: u64 = 2;
pub const FOREIGN_USER: u64 = 1337;
pub const BUYER: [u64; 2] = [12, 34];
pub const SELLER: [u64; 2] = [56, 78];
pub const AMOUNT: [u128; 2] = [12345, 54321];
pub const WALLET: [u128; 2] = [0, 1];

pub fn init_system() -> System {
    let system = System::new();
    system.init_logger();

    system
}

pub fn init_ft(sys: &System) -> Program {
    let ft_program = Program::from_file(sys, "./target/fungible_token.wasm");

    assert!(ft_program
        .send(
            FOREIGN_USER,
            InitFT {
                name: String::from("MyToken"),
                symbol: String::from("MTK"),
            },
        )
        .log()
        .is_empty());

    ft_program
}

pub fn init_escrow(sys: &System) -> Program {
    let escrow_program = Program::current(sys);

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

pub fn check_balance(ft_program: &Program, from: u64, amount: u128) {
    assert!(ft_program
        .send(from, FTAction::BalanceOf(from.into()))
        .contains(&(from, FTEvent::Balance(amount).encode())));
}

pub fn mint(ft_program: &Program, from: u64, amount: u128) {
    assert!(ft_program.send(from, FTAction::Mint(amount)).contains(&(
        from,
        FTEvent::Transfer {
            from: 0.into(),
            to: from.into(),
            amount,
        }
        .encode()
    )));
}
