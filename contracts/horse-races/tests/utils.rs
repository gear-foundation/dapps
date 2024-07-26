pub use gstd::{prelude::*, ActorId};
use gtest::{Program, ProgramBuilder, System};

pub const FEE_BPS: u16 = 200;
pub const NEW_FEE_BPS: u16 = 500;
pub const HORSE_RACES_ID: u64 = 3;
pub const ORACLE_ID: u64 = 4;
pub const TOKEN_ID: u64 = 5;
pub const OWNER: u64 = 6;
pub const MANAGER: u64 = 7;
pub const NEW_MANAGER: u64 = 8;
pub const NEW_ORACLE: u64 = 9;
pub const FAKE_MANAGER: u64 = 10;
pub const USER: u64 = 11;
pub const USER_1: u64 = 12;
pub const USER_2: u64 = 13;
pub const USER_3: u64 = 14;

pub fn get_programs(sys: &System) -> (Program<'_>, Program<'_>, Program<'_>) {
    sys.init_logger();

    let current_program = Program::current_with_id(sys, HORSE_RACES_ID);

    let oracle_program = ProgramBuilder::from_file(
        "../target/wasm32-unknown-unknown/release/oracle_randomness.opt.wasm",
    )
    .with_id(ORACLE_ID)
    .build(sys);
    let token_program = ProgramBuilder::from_file(
        "../target/wasm32-unknown-unknown/release/fungible_token.opt.wasm",
    )
    .with_id(TOKEN_ID)
    .build(sys);
    (current_program, oracle_program, token_program)
}

pub fn init_oracle<'a>(oracle_program: &'a Program<'a>) {
    let result = oracle_program.send(
        OWNER,
        oracle_randomness_io::InitConfig {
            manager: MANAGER.into(),
        },
    );
    assert!(!result.main_failed());
}

pub fn set_oracle_value<'a>(oracle_program: &'a Program<'a>, round: u128, value: u128) {
    oracle_program.send(
        MANAGER,
        oracle_randomness_io::Action::SetRandomValue {
            round,
            value: oracle_randomness_io::state::Random {
                randomness: (value, 0),
                signature: String::from("signature"),
                prev_signature: String::from("prev_signature"),
            },
        },
    );
}

pub fn init_token<'a>(token_program: &'a Program<'a>) {
    let result = token_program.send(
        OWNER,
        fungible_token_io::InitConfig {
            name: String::from("TestToken"),
            symbol: String::from("TST"),
            decimals: 18,
        },
    );

    assert!(!result.main_failed());
}

pub fn mint_token<'a>(token_program: &'a Program<'a>, user: u64, amount: u128) {
    let result = token_program.send(OWNER, fungible_token_io::FTAction::Mint(amount));
    assert!(!result.main_failed());

    let result = token_program.send(
        OWNER,
        fungible_token_io::FTAction::Transfer {
            from: OWNER.into(),
            to: user.into(),
            amount,
        },
    );
    assert!(!result.main_failed());
    assert!(!result.others_failed());

    let result = token_program.send(OWNER, fungible_token_io::FTAction::BalanceOf(user.into()));

    assert!(!result.main_failed());
    assert!(result.contains(&(OWNER, fungible_token_io::FTEvent::Balance(amount).encode())));
}

pub fn approve<'a>(token_program: &'a Program<'a>, from: u64, user: ActorId, amount: u128) {
    let result = token_program.send(
        from,
        fungible_token_io::FTAction::Approve { to: user, amount },
    );
    assert!(result.contains(&(
        from,
        fungible_token_io::FTEvent::Approve {
            from: from.into(),
            to: user,
            amount
        }
        .encode()
    )));
}

pub fn get_state() -> Vec<u8> {
    std::fs::read("../target/wasm32-unknown-unknown/release/horse_races_state.meta.wasm").unwrap()
}
