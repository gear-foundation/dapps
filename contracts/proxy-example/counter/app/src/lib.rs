#![no_std]
#![allow(static_mut_refs)]
use sails_rs::{
    collections::BTreeMap,
    gstd::{exec, msg},
    prelude::*,
};

static mut COUNTER: Option<CounterState> = None;

#[derive(Default)]
struct CounterState {
    admin: ActorId,
    proxy_address: Option<ActorId>,
    value: u128,
    limit: u128,
    contributions: BTreeMap<ActorId, u128>,
}

struct CounterService(());

impl CounterService {
    pub fn init(limit: u128) -> Self {
        unsafe {
            COUNTER = Some(CounterState {
                admin: msg::source(),
                limit,
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
        self.only_admin(msg::source());
        self.get_mut().proxy_address = proxy_address;
    }

    pub fn contribute(&mut self, msg_source: Option<ActorId>) -> u128 {
        self.check_if_proxy();
        let msg_source = self.get_msg_source(msg_source);
        let amount = msg::value();
        assert!(amount > 0, "Contribution must be greater than zero");
        let state = self.get_mut();
        let contribution = state.contributions.entry(msg_source).or_insert(0);
        *contribution += amount;
        state.value += amount;
        state.value
    }

    pub fn distribute(&mut self, msg_source: Option<ActorId>) {
        self.check_if_proxy();
        let msg_source = self.get_msg_source(msg_source);
        self.only_admin(msg_source);
        let state = self.get_mut();

        assert!(
            state.value >= state.limit,
            "Counter has not reached the limit"
        );

        let total_contributions: u128 = state.contributions.values().sum();
        assert!(total_contributions > 0, "No contributions to distribute");

        for (user, contribution) in state.contributions.iter() {
            let reward = state.value * *contribution / total_contributions;
            msg::send_with_gas(*user, b"", 0, reward).expect("Failed to send reward");
        }

        state.value = 0;
        state.contributions.clear();
    }

    /// This function is used in the old program to export the migration state.
    /// It serializes only the essential fields (`value`, `limit`, and `contributions`)
    /// needed for transferring to the new program.
    pub fn export_migration_state(&self) -> Vec<u8> {
        let state = self.get();
        let export_data = (state.value, state.limit, state.contributions.clone());
        export_data.encode()
    }

    /// This function is used in the new program to import the migration state.
    /// It decodes the provided serialized data and updates the program's internal state
    /// with the `value`, `limit`, and `contributions` from the previous program.
    pub fn import_migration_state(&mut self, encoded_state: Vec<u8>) {
        self.only_admin(msg::source());
        let (value, limit, contributions) =
            <(u128, u128, BTreeMap<ActorId, u128>)>::decode(&mut encoded_state.as_ref())
                .expect("Failed to decode migration state");

        let state = self.get_mut();

        state.value = value;
        state.limit = limit;
        state.contributions = contributions;
    }

    /// Stops the execution of the current program and transfers its remaining `value`
    /// (balance) to an indicated address (for example, new inheritor program). This can be used, for example, when deploying
    /// a new version of the program.
    pub async fn kill(&mut self, inheritor: ActorId, msg_source: Option<ActorId>) {
        self.check_if_proxy();
        let msg_source = self.get_msg_source(msg_source);
        self.only_admin(msg_source);
        exec::exit(inheritor);
    }

    pub fn get_value(&self) -> u128 {
        self.get().value
    }

    fn get_msg_source(&self, msg_source: Option<ActorId>) -> ActorId {
        if self.get().proxy_address.is_some() {
            msg_source.expect("msg_source must be set through proxy")
        } else {
            msg::source()
        }
    }
    fn check_if_proxy(&self) {
        if let Some(proxy_address) = self.get().proxy_address {
            assert_eq!(msg::source(), proxy_address, "Only proxy can send messages")
        }
    }

    fn only_admin(&self, msg_source: ActorId) {
        assert_eq!(
            msg_source,
            self.get().admin,
            "Only proxy can send this message"
        )
    }
}

pub struct CounterProgram(());

#[allow(clippy::new_without_default)]
#[sails_rs::program]
impl CounterProgram {
    pub fn new(limit: u128) -> Self {
        CounterService::init(limit);
        Self(())
    }

    pub fn counter(&self) -> CounterService {
        CounterService::new()
    }
}
