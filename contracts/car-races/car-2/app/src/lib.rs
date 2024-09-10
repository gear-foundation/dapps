#![no_std]

use sails_rs::{collections::BTreeMap, prelude::*};
struct CarStrategyService(());

#[sails_rs::service]
impl CarStrategyService {
    pub fn new() -> Self {
        Self(())
    }

    pub fn make_move(&mut self, _cars: BTreeMap<ActorId, Car>) -> StrategyAction {
        let random_choice = get_random_value(10);
        match random_choice {
            0..=2 => StrategyAction::BuyAcceleration,
            3..=9 => StrategyAction::BuyShell,
            _ => {
                unreachable!()
            }
        }
    }
}

static mut SEED: u8 = 0;

pub fn get_random_value(range: u8) -> u8 {
    let seed = unsafe { SEED };
    unsafe { SEED = SEED.wrapping_add(1) };
    let mut random_input: [u8; 32] = gstd::exec::program_id().into();
    random_input[0] = random_input[0].wrapping_add(seed);
    let (random, _) = gstd::exec::random(random_input).expect("Error in getting random number");
    random[0] % range
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
