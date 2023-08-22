#![no_std]

#[cfg(feature = "std")]
mod code {
    include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));
}

#[cfg(feature = "std")]
pub use code::WASM_BINARY as META_WASM_V2;

#[cfg(feature = "std")]
pub use code::WASM_EXPORTS as META_EXPORTS_V2;

#[cfg(not(feature = "std"))]
mod wasm;
