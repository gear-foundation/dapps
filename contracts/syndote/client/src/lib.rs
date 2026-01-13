#![no_std]
#![allow(clippy::type_complexity)]

// Incorporate code generated based on the IDL file
include!(concat!(env!("OUT_DIR"), "/syndote_client.rs"));
