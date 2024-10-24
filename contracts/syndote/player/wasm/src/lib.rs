#![no_std]

#[cfg(target_arch = "wasm32")]
pub use player_app::wasm::*;
