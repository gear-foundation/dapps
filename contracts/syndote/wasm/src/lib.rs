#![no_std]
#![allow(clippy::type_complexity)]

include!(concat!(env!("OUT_DIR"), "/syndote_client.rs"));
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

use syndote_app::services::game::Event;
#[cfg(target_arch = "wasm32")]
pub use syndote_app::wasm::*;
