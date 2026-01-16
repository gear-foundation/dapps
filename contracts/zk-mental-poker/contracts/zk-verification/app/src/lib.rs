#![no_std]
pub mod services;
use sails_rs::prelude::*;
use services::{VerifyingKeyBytes, ZkVerificationService};
pub struct ZkVerificationProgram(());

#[sails_rs::program]
impl ZkVerificationProgram {
    // Program's constructor
    pub fn new(vk_shuffle_bytes: VerifyingKeyBytes) -> Self {
        ZkVerificationService::init(vk_shuffle_bytes);
        Self(())
    }

    // Exposed service
    pub fn zk_verification(&self) -> ZkVerificationService {
        ZkVerificationService::new()
    }
}
