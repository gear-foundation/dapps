#![no_std]

#[cfg(target_arch = "wasm32")]
pub use syndote_player_app::wasm::*;
