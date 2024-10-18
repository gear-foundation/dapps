#![no_std]
#![allow(clippy::new_without_default)]

use sails_rs::prelude::*;
mod services;
use services::syndote::Service;
pub struct Program(());

#[program]
impl Program {
    pub fn new() -> Self {
        Service::seed();
        Self(())
    }

    pub fn syndote(&self) -> Service {
        Service::new()
    }
}
