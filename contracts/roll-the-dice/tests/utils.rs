pub const OWNER: u64 = 3;
pub const MANAGER: u64 = 4;
pub const USER: u64 = 5;
pub const ORACLE_ID: u64 = 100;
pub const ROLL_DICE_ID: u64 = 200;

pub fn get_state() -> Vec<u8> {
    std::fs::read("../target/wasm32-unknown-unknown/release/roll_the_dice_state.meta.wasm").unwrap()
}
