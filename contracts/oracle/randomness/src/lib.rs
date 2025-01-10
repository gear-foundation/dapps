#![no_std]
#![allow(static_mut_refs)]
use gstd::{async_main, msg, prelude::*, ActorId};
use oracle_randomness_io::{
    state::{self, RandomnessOracle},
    Action, Event, InitConfig,
};

static mut RANDOMNESS_ORACLE: Option<RandomnessOracle> = None;

#[async_trait::async_trait]
pub trait RandomnessOracleHandler {
    fn set_random_value(&mut self, round: u128, value: &state::Random);
    fn update_manager(&mut self, new_manager: &ActorId);
    fn get_value(&self, round: u128) -> state::Random;
    fn get_values(&self) -> Vec<(u128, state::Random)>;
    fn get_random_value(&self, round: u128) -> state::RandomSeed;
    fn assert_manager(&self);
    fn assert_owner(&self);
}

#[async_trait::async_trait]
impl RandomnessOracleHandler for RandomnessOracle {
    fn set_random_value(&mut self, round: u128, value: &state::Random) {
        self.assert_manager();

        if round <= self.last_round {
            panic!("Invalid round!");
        }

        self.last_round = round;

        if self.values.insert(round, value.clone()).is_some() {
            panic!("Unable to update existing value!");
        }

        msg::reply(
            Event::NewRandomValue {
                round,
                value: value.clone(),
            },
            0,
        )
        .expect("Unable to reply!");
    }

    fn update_manager(&mut self, new_manager: &ActorId) {
        self.assert_owner();

        self.manager = *new_manager;
        msg::reply(Event::NewManager(*new_manager), 0).expect("Unable to reply!");
    }

    fn get_value(&self, round: u128) -> state::Random {
        self.values
            .get(&round)
            .expect("Unable to find round!")
            .clone()
    }

    fn get_values(&self) -> Vec<(u128, state::Random)> {
        self.values
            .iter()
            .map(|(round, value)| (*round, value.clone()))
            .collect()
    }

    fn get_random_value(&self, round: u128) -> state::RandomSeed {
        self.get_value(round).randomness
    }

    fn assert_manager(&self) {
        if msg::source() != self.manager {
            panic!("Only manager allowed to call this!");
        }
    }

    fn assert_owner(&self) {
        if msg::source() != self.owner {
            panic!("Only owner allowed to call this!");
        }
    }
}

#[async_main]
async fn main() {
    let action: Action = msg::load().expect("Unable to decode Action.");
    let randomness_oracle: &mut RandomnessOracle =
        unsafe { RANDOMNESS_ORACLE.get_or_insert(RandomnessOracle::default()) };

    match action {
        Action::SetRandomValue { round, value } => {
            randomness_oracle.set_random_value(round, &value)
        }
        Action::GetLastRoundWithRandomValue => {
            let round = randomness_oracle.last_round;
            let random_value = randomness_oracle.get_random_value(round);

            msg::reply(
                Event::LastRoundWithRandomValue {
                    round,
                    random_value,
                },
                0,
            )
            .expect("Unable to reply!");
        }
        Action::UpdateManager(new_manager) => randomness_oracle.update_manager(&new_manager),
    }
}

#[no_mangle]
unsafe extern fn init() {
    let config: InitConfig = msg::load().expect("Unable to decode InitConfig.");
    let randomness_oracle = RandomnessOracle {
        owner: msg::source(),
        manager: config.manager,
        ..Default::default()
    };

    RANDOMNESS_ORACLE = Some(randomness_oracle);
}

#[no_mangle]
extern fn state() {
    msg::reply(
        unsafe {
            RANDOMNESS_ORACLE
                .clone()
                .expect("Uninitialized randomness oracle state.")
        },
        0,
    )
    .expect("Failed to encode or reply with `<AppMetadata as Metadata>::State` from `state()`.");
}
