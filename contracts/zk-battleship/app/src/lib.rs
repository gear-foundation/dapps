#![no_std]
#![allow(dead_code)]
#![allow(clippy::new_without_default)]
#![allow(clippy::result_unit_err)]
#![allow(clippy::should_implement_trait)]
use gstd::{msg, ActorId};
use sails_rtl::gstd::gprogram;
use services::{admin, multiple, session, single, verify::VerifyingKeyBytes};
pub mod services;

pub struct Program(());

#[gprogram]
impl Program {
    pub fn new(
        builtin_bls381: ActorId,
        verification_key_for_start: VerifyingKeyBytes,
        verification_key_for_move: VerifyingKeyBytes,
    ) -> Self {
        admin::GstdDrivenService::seed(
            msg::source(),
            builtin_bls381,
            verification_key_for_start,
            verification_key_for_move,
        );
        session::GstdDrivenService::seed();
        single::GstdDrivenService::seed();
        multiple::GstdDrivenService::seed();
        Self(())
    }
    pub fn admin(&self) -> admin::GstdDrivenService {
        admin::GstdDrivenService::new()
    }
    pub fn single(&self) -> single::GstdDrivenService {
        single::GstdDrivenService::new()
    }
    pub fn multiple(&self) -> multiple::GstdDrivenService {
        multiple::GstdDrivenService::new()
    }
    pub fn session(&self) -> session::GstdDrivenService {
        session::GstdDrivenService::new()
    }
}
