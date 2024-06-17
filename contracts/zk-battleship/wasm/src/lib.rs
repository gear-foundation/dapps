#![no_std]

#[cfg(target_arch = "wasm32")]
pub use zk_battleship::wasm::*;
