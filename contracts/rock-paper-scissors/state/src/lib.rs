#![no_std]

#[cfg(not(feature = "binary-vendor"))]
pub mod state;

#[cfg(feature = "binary-vendor")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));
