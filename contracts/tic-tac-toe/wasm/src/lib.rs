#![no_std]
include!(concat!(env!("OUT_DIR"), "/tic_tac_toe_client.rs"));
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

#[cfg(target_arch = "wasm32")]
pub use tic_tac_toe_app::wasm::*;
