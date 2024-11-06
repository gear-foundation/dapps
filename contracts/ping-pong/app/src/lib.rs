#![no_std]

use sails_rs::prelude::*;

static mut PING_COUNTER: Option<U256> = None;

struct PingPongService(());

impl PingPongService {
    pub fn new() -> Self {
        Self(())
    }
    pub fn get_mut(&mut self) -> &'static mut U256 {
        unsafe { PING_COUNTER.as_mut().expect("Ping counter is not initialized") }
    }
    pub fn get(&self) -> &'static U256 {
        unsafe { PING_COUNTER.as_ref().expect("Ping counter is not initialized") }
    }
}

#[sails_rs::service]
impl PingPongService {
    fn init() -> Self {
        unsafe {
            PING_COUNTER = Some(U256::zero());
        }
        Self(())
    }
    // Service's method (command)
    pub fn ping(&mut self) -> String {
        let ping_counter = self.get_mut();
        *ping_counter += U256::one();
        "Pong!".to_string()
    }

    // Service's query
    pub fn get_ping_count(&self) -> U256 {
        *self.get()
    }    
}

pub struct PingPongProgram(());

#[sails_rs::program]
impl PingPongProgram {
    // Program's constructor
    pub fn new() -> Self {
        PingPongService::init();
        Self(())
    }

    // Exposed service
    pub fn ping_pong(&self) -> PingPongService {
        PingPongService::new()
    }
}
