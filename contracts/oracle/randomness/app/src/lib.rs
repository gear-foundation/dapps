#![no_std]
#![allow(static_mut_refs)]
use sails_rs::collections::HashMap;
use sails_rs::gstd::msg;
use sails_rs::prelude::*;

static mut RANDOMNESS_ORACLE: Option<RandomnessOracle> = None;

#[derive(Debug, Default, Clone)]
pub struct RandomnessOracle {
    pub owner: ActorId,
    pub values: HashMap<u128, Random>,
    pub last_round: u128,
    pub manager: ActorId,
}

#[derive(Debug, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum Event {
    NewManager(ActorId),
    NewRandomValue { round: u128, value: Random },
    LastRoundWithRandomValue { round: u128, random_value: u128 },
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct Random {
    pub randomness: u128,
    pub signature: String,
    pub prev_signature: String,
}

struct RandomnessService(());

impl RandomnessService {
    pub async fn init(manager: ActorId) -> Self {
        unsafe {
            RANDOMNESS_ORACLE = Some(RandomnessOracle {
                owner: msg::source(),
                manager,
                ..Default::default()
            });
        }
        Self(())
    }
    pub fn get_mut(&mut self) -> &'static mut RandomnessOracle {
        unsafe {
            RANDOMNESS_ORACLE
                .as_mut()
                .expect("RandomnessOracle is not initialized")
        }
    }
    pub fn get(&self) -> &'static RandomnessOracle {
        unsafe {
            RANDOMNESS_ORACLE
                .as_ref()
                .expect("RandomnessOracle is not initialized")
        }
    }
}

#[sails_rs::service(events = Event)]
impl RandomnessService {
    pub fn new() -> Self {
        Self(())
    }

    pub fn set_random_value(&mut self, round: u128, value: Random) {
        let randomness_oracle = self.get_mut();
        self.assert_manager();

        if round <= randomness_oracle.last_round {
            panic!("Invalid round!");
        }

        randomness_oracle.last_round = round;

        if randomness_oracle
            .values
            .insert(round, value.clone())
            .is_some()
        {
            panic!("Unable to update existing value!");
        }

        self.emit_event(Event::NewRandomValue { round, value })
            .expect("Notification Error");
    }

    pub fn get_last_round_with_random_value(&mut self) -> (u128, u128) {
        let randomness_oracle = self.get_mut();
        let round = randomness_oracle.last_round;
        let random_value = self.get_random_value(round);

        self.emit_event(Event::LastRoundWithRandomValue {
            round,
            random_value,
        })
        .expect("Notification Error");
        (round, random_value)
    }

    pub fn update_manager(&mut self, new_manager: ActorId) {
        let randomness_oracle = self.get_mut();
        self.assert_owner();
        randomness_oracle.manager = new_manager;
        self.emit_event(Event::NewManager(new_manager))
            .expect("Notification Error");
    }

    fn get_value(&self, round: u128) -> Random {
        self.get()
            .values
            .get(&round)
            .expect("Unable to find round!")
            .clone()
    }

    fn get_random_value(&self, round: u128) -> u128 {
        self.get_value(round).randomness
    }

    fn assert_manager(&self) {
        if msg::source() != self.get().manager {
            panic!("Only manager allowed to call this!");
        }
    }
    fn assert_owner(&self) {
        if msg::source() != self.get().owner {
            panic!("Only owner allowed to call this!");
        }
    }

    pub fn get_owner(&self) -> ActorId {
        self.get().owner
    }

    pub fn get_last_round(&self) -> u128 {
        self.get().last_round
    }

    pub fn get_manager(&self) -> ActorId {
        self.get().manager
    }

    pub fn get_values(&self) -> Vec<(u128, Random)> {
        self.get().values.clone().into_iter().collect()
    }
}

pub struct RandomnessProgram(());

#[sails_rs::program]
impl RandomnessProgram {
    // Program's constructor
    pub async fn new(manager: ActorId) -> Self {
        RandomnessService::init(manager).await;
        Self(())
    }

    // Exposed service
    pub fn randomness(&self) -> RandomnessService {
        RandomnessService::new()
    }
}
