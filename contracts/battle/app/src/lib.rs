#![no_std]
#![warn(clippy::new_without_default)]
mod services;

use crate::services::{
    game::session::{Config as SessionConfig, SessionService},
    game::utils::Config,
};
use services::game::BattleService;
use session_service::*;

pub struct Program(());

pub struct BattleProgram(());

#[sails_rs::program]
impl BattleProgram {
    pub fn new(config: Config, session_config: SessionConfig) -> Self {
        BattleService::init(config);
        SessionService::init(session_config);
        Self(())
    }

    pub fn battle(&self) -> BattleService {
        BattleService::new()
    }

    pub fn session(&self) -> SessionService {
        SessionService::new()
    }
}
