#![no_std]

#[cfg(not(feature = "binary-vendor"))]
pub mod contract;
pub mod ft_messages;
pub mod utils;

// See `Cargo.toml` for the description of the "binary-vendor" feature.
#[cfg(feature = "binary-vendor")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));
