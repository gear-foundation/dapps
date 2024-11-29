#![no_std]
include!(concat!(env!("OUT_DIR"), "/vara_man_client.rs"));
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

#[cfg(target_arch = "wasm32")]
pub use vara_man_app::wasm::*;
