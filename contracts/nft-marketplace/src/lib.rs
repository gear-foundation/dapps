#![no_std]

#[cfg(not(feature = "binary-vendor"))]
mod auction;
#[cfg(not(feature = "binary-vendor"))]
pub mod contract;
#[cfg(not(feature = "binary-vendor"))]
mod nft_messages;
#[cfg(not(feature = "binary-vendor"))]
mod offers;
#[cfg(not(feature = "binary-vendor"))]
mod payment;
#[cfg(not(feature = "binary-vendor"))]
mod sale;

// See `Cargo.toml` for the description of the "binary-vendor" feature.
#[cfg(feature = "binary-vendor")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));
