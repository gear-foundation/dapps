#![no_std]
include!(concat!(env!("OUT_DIR"), "/galactic_express_client.rs"));
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

#[cfg(target_arch = "wasm32")]
pub use galactic_express_app::wasm::*;
