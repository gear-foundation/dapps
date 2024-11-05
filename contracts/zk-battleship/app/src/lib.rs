#![no_std]
#![allow(dead_code)]
#![allow(clippy::new_without_default)]
#![allow(clippy::result_unit_err)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::too_many_arguments)]
use gstd::{msg, ActorId};
use sails_rs::gstd::program;
use services::{admin, multiple, session, single, verify::VerifyingKeyBytes};
pub mod services;

pub struct Program(());

#[program]
impl Program {
    pub fn new(
        builtin_bls381: ActorId,
        verification_key_for_start: VerifyingKeyBytes,
        verification_key_for_move: VerifyingKeyBytes,
        config: admin::storage::configuration::Configuration,
    ) -> Self {
        admin::AdminService::seed(
            msg::source(),
            builtin_bls381,
            verification_key_for_start,
            verification_key_for_move,
            config,
        );
        session::SessionService::seed();
        single::SingleService::seed();
        multiple::MultipleService::seed();
        Self(())
    }
    pub fn admin(&self) -> admin::AdminService {
        admin::AdminService::new()
    }
    pub fn single(&self) -> single::SingleService {
        single::SingleService::new()
    }
    pub fn multiple(&self) -> multiple::MultipleService {
        multiple::MultipleService::new()
    }
    pub fn session(&self) -> session::SessionService {
        session::SessionService::new()
    }
}
