#![no_std]
#![allow(clippy::new_without_default)]

use sails_rs::prelude::*;
pub mod services;
use services::game::{Config, GameService};
pub struct Program(());

#[program]
impl Program {
    pub async fn new(config: Config, dns_id_and_name: Option<(ActorId, String)>) -> Self {
        GameService::init(config, dns_id_and_name).await;
        Self(())
    }

    pub fn syndote(&self) -> GameService {
        GameService::new()
    }
}
