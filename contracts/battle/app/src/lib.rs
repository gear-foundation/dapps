#![no_std]
#![warn(clippy::new_without_default)]
mod services;

use crate::services::{
    game::session::{SessionConfig, SessionService, SessionStorage},
    game::utils::Config,
};
use core::cell::RefCell;
use sails_rs::prelude::*;
use services::game::BattleService;

pub struct BattleProgram {
    session_storage: RefCell<SessionStorage>,
}

#[sails_rs::program]
impl BattleProgram {
    pub fn new(config: Config, session_config: SessionConfig) -> Self {
        BattleService::init(config);
        Self {
            session_storage: RefCell::new(SessionStorage::new(session_config)),
        }
    }

    pub fn battle(&self) -> BattleService<'_> {
        BattleService::new(&self.session_storage)
    }

    pub fn session(&self) -> SessionService<'_> {
        SessionService::new(&self.session_storage)
    }
}
