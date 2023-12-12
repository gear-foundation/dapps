use gstd::{exec, prelude::*, ActorId};
static mut SEED: u8 = 0;

pub fn get_random_value(range: u8) -> u8 {
    if range == 0 {
        return 0;
    }
    let seed = unsafe { SEED };
    unsafe { SEED = SEED.wrapping_add(1) };
    let random_input: [u8; 32] = [seed; 32];
    let (random, _) = exec::random(random_input).expect("Error in getting random number");
    random[0] % range
}

pub fn generate_power(min_range: u16, max_range: u16, tmg_id: ActorId) -> u16 {
    let random_input: [u8; 32] = tmg_id.into();
    let (random, _) = exec::random(random_input).expect("Error in getting random number");
    let mut random_power = 5000;
    for i in 0..31 {
        let bytes: [u8; 2] = [random[i], random[i + 1]];
        random_power = u16::from_be_bytes(bytes) % max_range;
        if (min_range..=max_range).contains(&random_power) {
            break;
        }
    }
    random_power
}
pub fn generate_penalty_damage() -> u16 {
    let seed = unsafe { SEED };
    unsafe { SEED = SEED.wrapping_add(1) };
    let random_input: [u8; 32] = [seed; 32];
    let (random, _) = exec::random(random_input).expect("Error in getting random number");
    let bytes: [u8; 2] = [random[0], random[1]];
    u16::from_be_bytes(bytes) % 500
}
