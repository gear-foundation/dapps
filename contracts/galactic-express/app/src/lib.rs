#![no_std]
#![allow(clippy::new_without_default)]

use sails_rs::prelude::*;
mod services;
use services::galactic_express::GameService;
pub struct Program(());

#[program]
impl Program {
    pub async fn new(dns_id_and_name: Option<(ActorId, String)>) -> Self {
        GameService::init(dns_id_and_name).await;
        Self(())
    }

    pub fn galactic_express(&self) -> GameService {
        GameService::new()
    }
}
