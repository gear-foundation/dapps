#![no_std]

use gstd::{debug, ActorId};
use sails_rtl::gstd::gprogram;
use services::{multiple, single};
pub mod services;

pub struct Program(());

#[gprogram]
impl Program {
    pub fn new(builtin_bls381: ActorId) -> Self {
        debug!("new");
        single::GstdDrivenService::seed(builtin_bls381);
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
