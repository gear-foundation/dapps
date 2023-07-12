#![no_std]

mod auction;
pub mod contract;
mod nft_messages;
mod offers;
mod payment;
mod sale;

// See `Cargo.toml` for the description of the "binary-vendor" feature.
#[cfg(feature = "binary-vendor")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));
