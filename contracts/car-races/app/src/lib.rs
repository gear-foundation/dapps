#![no_std]

use core::cell::RefCell;
use sails_rs::prelude::*;

pub mod services;
use services::{
    session::{SessionConfig, SessionService, SessionStorage},
    CarRacesService, InitConfig,
};

pub struct CarRacesProgram {
    /// Storage used by the generated session system (sessions map + config)
    session_storage: RefCell<SessionStorage>,
}

#[program]
impl CarRacesProgram {
    pub async fn new(
        init_config: InitConfig,
        session_config: SessionConfig,
        dns_id_and_name: Option<(ActorId, String)>,
    ) -> Self {
        CarRacesService::init(init_config, dns_id_and_name).await;
        Self {
            session_storage: RefCell::new(SessionStorage::new(session_config)),
        }
    }

    pub fn car_races_service(&self) -> CarRacesService<'_> {
        CarRacesService::new(&self.session_storage)
    }

    pub fn session(&self) -> SessionService<'_> {
        SessionService::new(&self.session_storage)
    }
}
