#![no_std]
#![allow(clippy::new_without_default)]
#![allow(clippy::comparison_chain)]

use sails_rs::prelude::*;
mod services;
use crate::services::game::utils::Config;
use services::game::Service;
use services::session::SessionService;
pub struct Program(());

#[program]
impl Program {
    pub async fn new(config: Config, dns_id_and_name: Option<(ActorId, String)>) -> Self {
        Service::init(config, dns_id_and_name).await;
        SessionService::init();
        Self(())
    }

    pub fn vara_man(&self) -> Service {
        Service::new()
    }
    pub fn session(&self) -> SessionService {
        SessionService::new()
    }
}
