#![no_std]
#![allow(clippy::new_without_default)]

use sails_rs::prelude::*;
pub mod services;
use services::game::GameService;
pub struct SyndoteProgram(());

#[program]
impl SyndoteProgram {
    pub async fn new(dns_id_and_name: Option<(ActorId, String)>) -> Self {
        GameService::init(dns_id_and_name).await;
        Self(())
    }

    pub fn syndote(&self) -> GameService {
        GameService::new()
    }
}
