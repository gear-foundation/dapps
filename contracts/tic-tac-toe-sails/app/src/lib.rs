#![no_std]
#![allow(clippy::new_without_default)]

use sails_rs::prelude::*;
mod services;
use crate::services::game::utils::Config;
use services::game::Service;
pub struct Program(());

#[program]
impl Program {
    pub fn new(config: Config) -> Self {
        Service::init(config);
        Self(())
    }

    pub fn tic_tac_toe(&self) -> Service {
        Service::new()
    }
}
