#![no_std]
#![allow(clippy::new_without_default)]

use sails_rs::prelude::*;
mod services;
use crate::services::game::utils::Config;
use services::game::GameService;
use services::session::SessionService;
pub struct Program(());

#[program]
impl Program {
    pub fn new(config: Config) -> Self {
        GameService::init(config);
        SessionService::init();
        Self(())
    }

    pub fn tic_tac_toe(&self) -> GameService {
        GameService::new()
    }

    pub fn session(&self) -> SessionService {
        SessionService::new()
    }
}
