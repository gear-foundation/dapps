#![no_std]

#[cfg(not(feature = "binary-vendor"))]
mod contract;
mod sr25519;

#[cfg(feature = "binary-vendor")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));
