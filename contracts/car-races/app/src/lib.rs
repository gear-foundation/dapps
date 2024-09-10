#![no_std]

use sails_rs::{gstd::GStdExecContext, prelude::*};
pub mod services;
use services::game::{CarRacesService, InitConfig};
use services::session::SessionService;
#[derive(Default)]
pub struct Program;

#[program]
impl Program {
    pub fn new(init_config: InitConfig) -> Self {
        CarRacesService::<GStdExecContext>::seed(init_config, GStdExecContext::new());
        SessionService::init();
        Self
    }

    pub fn car_races_service(&self) -> CarRacesService<GStdExecContext> {
        CarRacesService::new(GStdExecContext::new())
    }

    pub fn session(&self) -> SessionService {
        SessionService::new()
    }
}
