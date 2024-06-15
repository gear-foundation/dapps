#![no_std]

mod contract;

include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));
