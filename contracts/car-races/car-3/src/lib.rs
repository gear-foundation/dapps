#![no_std]
use gstd::{collections::BTreeMap, debug, exec, msg, prelude::*, ActorId};

#[derive(Encode, Decode, TypeInfo)]
pub enum CarAction {
    YourTurn(BTreeMap<ActorId, Car>),
}
#[derive(Encode, Decode, TypeInfo, Clone, Debug)]
pub struct Car {
    pub balance: u32,
    pub position: u32,
    pub speed: u32,
    pub penalty: u8,
    pub car_actions: Vec<RoundAction>,
    pub round_result: Option<RoundAction>,
}
#[derive(Encode, Decode, TypeInfo)]
pub enum StrategyAction {
    BuyAcceleration,
    BuyShell,
    Skip,
}

#[derive(Encode, Decode, TypeInfo, Debug, Clone)]
pub enum RoundAction {
    Accelerated(u32),
    SlowedDown(u32),
}

#[no_mangle]
extern fn handle() {
    let msg: CarAction = msg::load().expect("Unable to load the message");
    let CarAction::YourTurn(cars) = msg;
    let my_car_id = exec::program_id();
    let my_car = cars.get(&my_car_id).expect("Unable to get my car");
    let my_position = my_car.position;
    let mut cars_vec: Vec<(ActorId, Car)> = cars
        .iter()
        .map(|(car_id, car)| (*car_id, car.clone()))
        .collect();
    cars_vec.sort_by(|a, b| b.1.position.cmp(&a.1.position));
    // If I'm the first skip
    if cars_vec[0].0 == my_car_id {
        msg::reply(StrategyAction::Skip, 0).expect("Error in sending a message");
        return;
    }
    debug!("CAR 2: SKIP");
    // if I'm the second
    if cars_vec[1].0 == my_car_id {
        debug!("CAR 2 I AM THE SECOND");
        // if the distance is small, then just buy acceleration
        if (cars_vec[0].1.position - my_position) <= 1000 {
            debug!("ACC");
            msg::reply(StrategyAction::BuyShell, 0).expect("Error in sending a message");
        } else {
            // else buy shells
            debug!("SHELL");
            msg::reply(StrategyAction::BuyAcceleration, 0).expect("Error in sending a message");
        }
        return;
    }
    debug!("CAR 2: SKIP");
    // if I'm the third just buy shell
    if cars_vec[2].0 == my_car_id {
        debug!("CAR 2 I AM THE THIRD");
        msg::reply(StrategyAction::BuyAcceleration, 0).expect("Error in sending a message");
        return;
    }

    debug!("CAR 2: SKIP");
}
