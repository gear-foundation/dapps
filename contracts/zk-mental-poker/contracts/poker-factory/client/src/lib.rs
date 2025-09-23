#![no_std]
#![allow(clippy::large_enum_variant)]
// Incorporate code generated based on the IDL file
include!(concat!(env!("OUT_DIR"), "/poker_factory_client.rs"));
