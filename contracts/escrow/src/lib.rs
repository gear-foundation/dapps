#![no_std]

#[cfg(not(feature = "binary-vendor"))]
mod contract;

// See `Cargo.toml` for the description of the "binary-vendor" feature.
#[cfg(feature = "binary-vendor")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));
