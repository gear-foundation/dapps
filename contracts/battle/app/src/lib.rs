#![no_std]
#![warn(clippy::new_without_default)]
mod services;

use crate::services::game::utils::Config;
use services::game::BattleService;

pub struct Program(());

pub struct BattleProgram(());

#[sails_rs::program]
impl BattleProgram {
    pub fn new(config: Config) -> Self {
        BattleService::init(config);
        Self(())
    }

    pub fn battle(&self) -> BattleService {
        BattleService::new()
    }
}
