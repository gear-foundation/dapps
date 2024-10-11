#![no_std]

use sails_rs::{prelude::*};
pub mod services;
use services::{
    session::{Config, SessionService},
    CarRacesService, InitConfig,
};

use session_service::*;

#[derive(Default)]
pub struct Program;

#[program]
impl Program {
    pub async fn new(
        init_config: InitConfig,
        session_config: Config,
        dns_id_and_name: Option<(ActorId, String)>,
    ) -> Self {
        CarRacesService::init(
            init_config,
            dns_id_and_name,
        )
        .await;
        SessionService::init(session_config);
        Self
    }

    pub fn car_races_service(&self) -> CarRacesService {
        CarRacesService::new()
    }

    pub fn session(&self) -> SessionService {
        SessionService::new()
    }
}
