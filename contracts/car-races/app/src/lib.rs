#![no_std]

use sails_rs::{gstd::GStdExecContext, prelude::*};
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
        CarRacesService::<GStdExecContext>::init(
            init_config,
            GStdExecContext::new(),
            dns_id_and_name,
        )
        .await;
        SessionService::init(session_config);
        Self
    }

    pub fn car_races_service(&self) -> CarRacesService<GStdExecContext> {
        CarRacesService::new(GStdExecContext::new())
    }

    pub fn session(&self) -> SessionService {
        SessionService::new()
    }
}
