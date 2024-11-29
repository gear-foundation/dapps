#![no_std]
#![allow(clippy::new_without_default)]

use sails_rs::{collections::BTreeMap, prelude::*};
struct CarStrategyService(());

#[sails_rs::service]
impl CarStrategyService {
    pub fn new() -> Self {
        Self(())
    }

    // this car only accelerates
    pub fn make_move(&mut self, _cars: BTreeMap<ActorId, Car>) -> StrategyAction {
        StrategyAction::BuyAcceleration
    }
}

pub struct CarStrategyProgram(());

#[sails_rs::program]
impl CarStrategyProgram {
    // Program's constructor
    pub fn new() -> Self {
        Self(())
    }

    // Exposed service
    pub fn car_strategy(&self) -> CarStrategyService {
        CarStrategyService::new()
    }
}

#[derive(Encode, Decode, TypeInfo, Clone, Debug)]
pub struct Car {
    pub position: u32,
    pub speed: u32,
    pub car_actions: Vec<RoundAction>,
    pub round_result: Option<RoundAction>,
}

#[derive(Encode, Decode, TypeInfo, Debug)]
pub enum StrategyAction {
    BuyAcceleration,
    BuyShell,
    Skip,
}

#[derive(Encode, Decode, TypeInfo, Debug, Clone, PartialEq, Eq)]
pub enum RoundAction {
    Accelerated,
    SlowedDown,
    SlowedDownAndAccelerated,
}
