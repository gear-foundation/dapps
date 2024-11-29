#![no_std]
#![allow(clippy::new_without_default)]

use sails_rs::prelude::*;
mod services;
use services::dynamic_nft::ExtendedService;
pub struct DynamicNftProgram(());

#[program]
impl DynamicNftProgram {
    pub fn new(name: String, symbol: String, gas_for_one_time_updating: u64) -> Self {
        ExtendedService::init(name, symbol, gas_for_one_time_updating);
        Self(())
    }

    pub fn dynamic_nft(&self) -> ExtendedService {
        ExtendedService::new()
    }
}
