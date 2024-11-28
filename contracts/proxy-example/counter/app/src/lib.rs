#![no_std]

use sails_rs::{gstd::msg, prelude::*};

static mut COUNTER: Option<CounterState> = None;

#[derive(Default)]
struct CounterState {
    admin: ActorId,
    proxy_address: Option<ActorId>,
    value: u64,
}

struct CounterService(());

impl CounterService {
    pub fn init() -> Self {
        unsafe {
            COUNTER = Some(CounterState {
                admin: msg::source(),
                ..Default::default()
            });
        }
        Self(())
    }

    pub fn get_mut(&mut self) -> &'static mut CounterState {
        unsafe { COUNTER.as_mut().expect("COUNTER is not initialized") }
    }

    pub fn get(&self) -> &'static CounterState {
        unsafe { COUNTER.as_ref().expect("COUNTER is not initialized") }
    }
}

#[sails_rs::service]
impl CounterService {
    pub fn new() -> Self {
        Self(())
    }

    pub fn set_proxy(&mut self, proxy_address: Option<ActorId>) {
        self.only_admin();
        self.get_mut().proxy_address = proxy_address;
    }

    pub fn increment(&mut self) -> u64 {
        self.check_if_proxy();
        let counter = self.get_mut();
        counter.value += 1;
        counter.value
    }

    pub fn decrement(&mut self) -> u64 {
        self.check_if_proxy();
        let counter = self.get_mut();
        assert!(counter.value > 0, "Counter value cannot be negative");
        counter.value -= 1;
        counter.value
    }

    pub fn get_value(&self) -> u64 {
        self.get().value
    }

    fn check_if_proxy(&self) {
        if let Some(proxy_address) = self.get().proxy_address {
            assert_eq!(msg::source(), proxy_address, "Only proxy can send messages")
        }
    }

    fn only_admin(&self) {
        assert_eq!(
            msg::source(),
            self.get().admin,
            "Only proxy can send this message"
        )
    }
}

pub struct CounterProgram(());

#[sails_rs::program]
impl CounterProgram {
    pub fn new() -> Self {
        CounterService::init();
        Self(())
    }

    pub fn counter(&self) -> CounterService {
        CounterService::new()
    }
}
