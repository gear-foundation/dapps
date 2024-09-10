#![no_std]

use sails_rs::{collections::BTreeMap, prelude::*};
struct CarStrategyService(());

#[sails_rs::service]
impl CarStrategyService {
    pub fn new() -> Self {
        Self(())
    }

    // this car only accelerates
    pub fn make_move(&mut self, cars: BTreeMap<ActorId, Car>) -> StrategyAction {
        let my_car_id = gstd::exec::program_id();
        let my_car = cars.get(&my_car_id).expect("Unable to get my car");
        let my_position = my_car.position;
        let mut cars_vec: Vec<(ActorId, Car)> = cars
            .iter()
            .map(|(car_id, car)| (*car_id, car.clone()))
            .collect();
        cars_vec.sort_by(|a, b| b.1.position.cmp(&a.1.position));
        // If I'm the first skip
        if cars_vec[0].0 == my_car_id {
            return StrategyAction::Skip;
        }
        // if I'm the second
        if cars_vec[1].0 == my_car_id {
            // if the distance is small, then just buy acceleration
            if (cars_vec[0].1.position - my_position) <= 1000 {
                return StrategyAction::BuyShell;
            } else {
                // else buy shells
                return StrategyAction::BuyAcceleration;
            }
        }
        // if I'm the third just buy shell
        if cars_vec[2].0 == my_car_id {
            return StrategyAction::BuyAcceleration;
        }
        StrategyAction::Skip
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
