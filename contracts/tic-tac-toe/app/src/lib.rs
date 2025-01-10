#![no_std]
#![allow(clippy::new_without_default)]

use sails_rs::prelude::*;
mod services;
use crate::services::game::utils::Config;
use services::{game::GameService, session::SessionService};
pub struct TicTacToeProgram(());

#[program]
impl TicTacToeProgram {
    pub async fn new(config: Config, dns_id_and_name: Option<(ActorId, String)>) -> Self {
        GameService::init(config, dns_id_and_name).await;
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
