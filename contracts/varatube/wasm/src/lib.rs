#![no_std]

#[cfg(target_arch = "wasm32")]
pub use varatube_app::wasm::*;
