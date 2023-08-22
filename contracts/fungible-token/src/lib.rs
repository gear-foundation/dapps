#![no_std]

#[cfg(test)]
mod tests;

#[cfg(not(feature = "binary-vendor"))]
mod contract;

#[cfg(feature = "binary-vendor")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));
