#![no_std]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(clippy::new_without_default)]
use gstd::ActorId;
use sails_rtl::gstd::gprogram;
use services::{multiple, single, verify::VerifyingKeyBytes};
pub mod services;

pub struct Program(());

#[gprogram]
impl Program {
    pub fn new(
        builtin_bls381: ActorId,
        verification_key_for_start: VerifyingKeyBytes,
        verification_key_for_move: VerifyingKeyBytes,
    ) -> Self {
        single::GstdDrivenService::seed(
            builtin_bls381,
            verification_key_for_start,
            verification_key_for_move,
        );
        multiple::GstdDrivenService::seed();
        Self(())
    }
    pub fn single(&self) -> single::GstdDrivenService {
        single::GstdDrivenService::new()
    }
    pub fn multiple(&self) -> multiple::GstdDrivenService {
        multiple::GstdDrivenService::new()
    }
}
