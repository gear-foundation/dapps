#![no_std]
#![allow(static_mut_refs)]
use randomness_client::randomness::io as randomness_io;
use sails_rs::calls::ActionIo;
use sails_rs::gstd::{exec, msg};
use sails_rs::prelude::*;

static mut ORACLE: Option<Oracle> = None;

#[derive(Debug, Default, Clone, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct Oracle {
    owner: ActorId,
    manager: ActorId,
    dns_info: Option<(ActorId, String)>,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum Event {
    NewValue { value: u128 },
    NewManager(ActorId),
}

struct OracleService(());

impl OracleService {
    pub async fn init(
        owner: ActorId,
        manager: ActorId,
        dns_id_and_name: Option<(ActorId, String)>,
    ) -> Self {
        unsafe {
            ORACLE = Some(Oracle {
                owner,
                manager,
                dns_info: dns_id_and_name.clone(),
            });
        }

        if let Some((id, name)) = dns_id_and_name {
            let request = [
                "Dns".encode(),
                "AddNewProgram".to_string().encode(),
                (name, exec::program_id()).encode(),
            ]
            .concat();

            msg::send_bytes_with_gas_for_reply(id, request, 5_000_000_000, 0, 0)
                .expect("Error in sending message")
                .await
                .expect("Error in `AddNewProgram`");
        }
        Self(())
    }
    pub fn get_mut(&mut self) -> &'static mut Oracle {
        unsafe { ORACLE.as_mut().expect("Oracle is not initialized") }
    }
    pub fn get(&self) -> &'static Oracle {
        unsafe { ORACLE.as_ref().expect("Oracle is not initialized") }
    }
}

#[sails_rs::service(events = Event)]
impl OracleService {
    pub fn new() -> Self {
        Self(())
    }

    pub async fn request_value(&mut self) -> u128 {
        let request = randomness_io::GetLastRoundWithRandomValue::encode_call();
        let bytes_reply = msg::send_bytes_for_reply(self.get().manager, request, 0, 0)
            .expect("Unable to send message to `manager`.")
            .await
            .expect("Unable to decode reply payload from `manager`.");
        let (_, value) =
            randomness_io::GetLastRoundWithRandomValue::decode_reply(bytes_reply).unwrap();
        self.notify_on(Event::NewValue { value })
            .expect("Notification Error");
        value
    }

    pub fn change_manager(&mut self, new_manager: ActorId) {
        let oracle = self.get_mut();
        oracle.manager = new_manager;
        self.notify_on(Event::NewManager(new_manager))
            .expect("Notification Error");
    }

    pub fn get_owner(&self) -> ActorId {
        self.get().owner
    }
    pub fn get_manager(&self) -> ActorId {
        self.get().manager
    }
    pub fn get_dns_info(&self) -> Option<(ActorId, String)> {
        self.get().dns_info.clone()
    }
}

pub struct OracleProgram(());

#[sails_rs::program]
impl OracleProgram {
    // Program's constructor
    pub async fn new(
        owner: ActorId,
        manager: ActorId,
        dns_id_and_name: Option<(ActorId, String)>,
    ) -> Self {
        OracleService::init(owner, manager, dns_id_and_name).await;
        Self(())
    }

    // Exposed service
    pub fn oracle(&self) -> OracleService {
        OracleService::new()
    }
}
