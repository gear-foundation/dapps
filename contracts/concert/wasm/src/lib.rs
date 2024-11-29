#![no_std]
#![allow(clippy::type_complexity)]

include!(concat!(env!("OUT_DIR"), "/concert_client.rs"));
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

#[cfg(target_arch = "wasm32")]
pub use concert_app::wasm::*;
