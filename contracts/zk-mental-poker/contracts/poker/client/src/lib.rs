#![no_std]
#![allow(clippy::too_many_arguments)]
// Incorporate code generated based on the IDL file
include!(concat!(env!("OUT_DIR"), "/poker_client.rs"));
