#![no_std]

#[cfg(target_arch = "wasm32")]
pub use car_strategy_app_3::wasm::*;

#[cfg(feature = "wasm-binary")]
#[cfg(not(target_arch = "wasm32"))]
pub use code::WASM_BINARY_OPT as WASM_BINARY;

#[cfg(feature = "wasm-binary")]
#[cfg(not(target_arch = "wasm32"))]
mod code {
    include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));
}
