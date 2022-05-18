use escrow_io::*;
use ft_io::*;
use gstd::Encode;
use gstd::String;
use gtest::{Program, System};

pub const FT: u64 = 2;
pub const FOREIGN_USER: u64 = 1337;
pub const BUYER: [u64; 2] = [12, 34];
pub const SELLER: [u64; 2] = [56, 78];
pub const AMOUNT: [u128; 2] = [12345, 54321];
pub const CONTRACT: [u128; 2] = [0, 1];

pub fn init_system() -> System {
    let system = System::new();
    system.init_logger();

    system
}

pub fn init_fungible_tokens(sys: &System) -> Program {
    let ft_program = Program::from_file(
        &sys,
        "../target/wasm32-unknown-unknown/release/fungible_token.wasm",
    );

    assert!(ft_program
        .send(
            FOREIGN_USER,
            InitConfig {
                name: String::from("MyToken"),
                symbol: String::from("MTK"),
            },
        )
        .log()
        .is_empty());

    ft_program
}

pub fn init_escrow(sys: &System) -> Program {
    let escrow_program = Program::current(&sys);

    assert!(escrow_program
        .send(
            FOREIGN_USER,
            InitEscrow {
                ft_program_id: FT.into(),
            },
        )
        .log()
        .is_empty());

    escrow_program
}

pub fn create(
    escrow_program: &Program,
    contract_id: u128,
    from: u64,
    buyer: u64,
    seller: u64,
    amount: u128,
) {
    assert!(escrow_program
        .send(
            from,
            EscrowAction::Create {
                buyer: buyer.into(),
                seller: seller.into(),
                amount,
            },
        )
        .contains(&(from, EscrowEvent::Created { contract_id }.encode())));
}

pub fn create_fail(escrow_program: &Program, from: u64, buyer: u64, seller: u64, amount: u128) {
    assert!(escrow_program
        .send(
            from,
            EscrowAction::Create {
                buyer: buyer.into(),
                seller: seller.into(),
                amount,
            },
        )
        .main_failed());
}

pub fn deposit(escrow_program: &Program, contract_id: u128, buyer: u64, amount: u128) {
    assert!(escrow_program
        .send(buyer, EscrowAction::Deposit { contract_id })
        .contains(&(
            buyer,
            EscrowEvent::Deposited {
                buyer: buyer.into(),
                amount,
            }
            .encode()
        )));
}

pub fn deposit_fail(escrow_program: &Program, contract_id: u128, from: u64) {
    assert!(escrow_program
        .send(from, EscrowAction::Deposit { contract_id })
        .main_failed());
}

pub fn confirm(escrow_program: &Program, contract_id: u128, buyer: u64, seller: u64, amount: u128) {
    assert!(escrow_program
        .send(buyer, EscrowAction::Confirm { contract_id })
        .contains(&(
            buyer,
            EscrowEvent::Confirmed {
                seller: seller.into(),
                amount,
            }
            .encode()
        )));
}

pub fn confirm_fail(escrow_program: &Program, contract_id: u128, from: u64) {
    assert!(escrow_program
        .send(from, EscrowAction::Confirm { contract_id })
        .main_failed());
}

pub fn refund(escrow_program: &Program, contract_id: u128, buyer: u64, seller: u64, amount: u128) {
    assert!(escrow_program
        .send(seller, EscrowAction::Refund { contract_id })
        .contains(&(
            seller,
            EscrowEvent::Refunded {
                buyer: buyer.into(),
                amount
            }
            .encode()
        )));
}

pub fn refund_fail(escrow_program: &Program, contract_id: u128, from: u64) {
    assert!(escrow_program
        .send(from, EscrowAction::Refund { contract_id })
        .main_failed());
}

pub fn cancel(
    escrow_program: &Program,
    contract_id: u128,
    from: u64,
    buyer: u64,
    seller: u64,
    amount: u128,
) {
    assert!(escrow_program
        .send(from, EscrowAction::Cancel { contract_id })
        .contains(&(
            from,
            EscrowEvent::Cancelled {
                buyer: buyer.into(),
                seller: seller.into(),
                amount
            }
            .encode()
        )));
}

pub fn cancel_fail(escrow_program: &Program, contract_id: u128, from: u64) {
    assert!(escrow_program
        .send(from, EscrowAction::Cancel { contract_id })
        .main_failed());
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
