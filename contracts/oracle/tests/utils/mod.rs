use gtest::{Program, System};

pub const OWNER: u64 = 3;
pub const MANAGER: u64 = 4;
pub const NEW_MANAGER: u64 = 5;
pub const USER: u64 = 6;
pub const FAKE_OWNER: u64 = 7;
pub const FAKE_MANAGER: u64 = 8;

pub const OWNER_GCLIENT: &str = "//Bob";
pub const MANAGER_GCLIENT: &str = "//Mike";
pub const NEW_MANAGER_GCLIENT: &str = "//Josh";
pub const FAKE_OWNER_GCLIENT: &str = "//Keks";

pub const RANDOM_VALUE: u128 = 1337;

pub fn load_program(sys: &System) -> Program<'_> {
    sys.init_logger();

    Program::current(sys)
}

pub fn load_randomness_program(sys: &System) -> Program<'_> {
    sys.init_logger();

    Program::from_file(
        sys,
        "../target/wasm32-unknown-unknown/debug/oracle_randomness.opt.wasm",
    )
}
