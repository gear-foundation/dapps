#![no_std]
use gstd::{collections::BTreeMap, exec, msg, prelude::*, ActorId};

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum CarAction {
    YourTurn(BTreeMap<ActorId, Car>),
}
#[derive(Encode, Decode, TypeInfo, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct Car {
    pub balance: u32,
    pub position: u32,
    pub speed: u32,
    pub penalty: u8,
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum StrategyAction {
    BuyAcceleration,
    BuyShell,
    Skip,
}

#[no_mangle]
extern fn handle() {
    let random_choice = get_random_value(10);
    match random_choice {
        0 | 1 | 2 => {
            msg::reply(StrategyAction::BuyAcceleration, 0).expect("Error in sending a message");
        }
        3 | 4 | 5 | 6 | 7 | 8 | 9 => {
            msg::reply(StrategyAction::BuyShell, 0).expect("Error in sending a message");
        }
        _ => {
            unreachable!()
        }
    }
}

static mut SEED: u8 = 0;

pub fn get_random_value(range: u8) -> u8 {
    let seed = unsafe { SEED };
    unsafe { SEED = SEED.wrapping_add(1) };
    let mut random_input: [u8; 32] = exec::program_id().into();
    random_input[0] = random_input[0].wrapping_add(seed);
    let (random, _) = exec::random(random_input).expect("Error in getting random number");
    random[0] % range
}
