#![no_std]
#![allow(static_mut_refs)]
use sails_rs::{gstd::msg, prelude::*};

static mut STATE: Option<State> = None;
pub const REPLY_DEPOSIT: u64 = 10_000_000_000;

#[derive(Default)]
struct State {
    logic_address: ActorId,
    admin: ActorId,
}

struct UpgradeProxyService(());

impl UpgradeProxyService {
    pub fn init(logic_address: ActorId, admin: ActorId) -> Self {
        unsafe {
            STATE = Some(State {
                logic_address,
                admin,
            })
        }
        Self(())
    }
    pub fn get_mut(&mut self) -> &'static mut State {
        unsafe { STATE.as_mut().expect("STATE is not initialized") }
    }
    pub fn get(&self) -> &'static State {
        unsafe { STATE.as_ref().expect("STATE is not initialized") }
    }
}

#[sails_rs::service]
impl UpgradeProxyService {
    pub fn new() -> Self {
        Self(())
    }

    pub async fn execute_msg(&mut self, bytes: Vec<u8>) -> Vec<u8> {
        let original_sender = Some(msg::source());
        let sender_encoded = original_sender.encode();
        let mut new_payload = bytes.clone();
        new_payload.extend(sender_encoded);
        msg::send_bytes_for_reply(
            self.get().logic_address,
            new_payload,
            msg::value(),
            REPLY_DEPOSIT,
        )
        .expect("Error during message sending")
        .await
        .expect("Error during getting the reply")
    }

    pub async fn read_state(&self, bytes: Vec<u8>) -> Vec<u8> {
        msg::send_bytes_for_reply(self.get().logic_address, bytes, 0, 0)
            .expect("Error during message sending")
            .await
            .expect("Error during getting the reply")
    }

    pub fn update_logic(&mut self, new_logic_address: ActorId) {
        assert_eq!(
            self.get().admin,
            msg::source(),
            "Only admin can update the logic address"
        );
        self.get_mut().logic_address = new_logic_address;
    }

    pub fn get_logic_address(&self) -> ActorId {
        self.get().logic_address
    }

    pub fn get_admin(&self) -> ActorId {
        self.get().admin
    }

    // This function must check whether the new logic contract (new_logic_address) is compatible with the requirements of the proxy.
    pub fn check_compatibility(&mut self, _new_logic_address: ActorId) -> bool {
        // TODO: Implement compatibility checks for the new logic contract
        true
    }
}

pub struct UpgradeProxyProgram(());

#[sails_rs::program]
impl UpgradeProxyProgram {
    // Program's constructor
    pub fn new(logic_address: ActorId, admin: ActorId) -> Self {
        UpgradeProxyService::init(logic_address, admin);
        Self(())
    }

    // Exposed service
    pub fn proxy(&self) -> UpgradeProxyService {
        UpgradeProxyService::new()
    }
}
