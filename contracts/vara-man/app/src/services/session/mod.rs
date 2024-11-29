use super::game::GameStorage;
use crate::services;
use sails_rs::{collections::HashMap, gstd::service, prelude::*};
mod funcs;
pub mod utils;
use utils::*;

#[derive(Default)]
pub struct Storage(());

impl Storage {
    pub fn get_session_map() -> &'static SessionMap {
        unsafe { STORAGE.as_ref().expect("Storage is not initialized") }
    }
}

static mut STORAGE: Option<SessionMap> = None;

#[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum Event {
    SessionCreated,
    SessionDeleted,
}

#[derive(Clone)]
pub struct SessionService(());

impl SessionService {
    pub fn init() -> Self {
        unsafe {
            STORAGE = Some(HashMap::new());
        }
        Self(())
    }
    pub fn as_mut(&mut self) -> &'static mut SessionMap {
        unsafe { STORAGE.as_mut().expect("Storage is not initialized") }
    }
    pub fn as_ref(&self) -> &'static SessionMap {
        unsafe { STORAGE.as_ref().expect("Storage is not initialized") }
    }
}

#[service(events = Event)]
impl SessionService {
    pub fn new() -> Self {
        Self(())
    }
    pub fn create_session(&mut self, signature_data: SignatureData, signature: Option<Vec<u8>>) {
        let sessions = self.as_mut();
        let config = GameStorage::get_config();
        let event = services::utils::panicking(|| {
            funcs::create_session(sessions, config, signature_data, signature)
        });
        self.notify_on(event.clone()).expect("Notification Error");
    }

    pub fn delete_session_from_program(&mut self, session_for_account: ActorId) {
        let sessions = self.as_mut();
        let event = services::utils::panicking(|| {
            funcs::delete_session_from_program(sessions, session_for_account)
        });
        self.notify_on(event.clone()).expect("Notification Error");
    }

    pub fn delete_session_from_account(&mut self) {
        let sessions = self.as_mut();
        let event = services::utils::panicking(|| funcs::delete_session_from_account(sessions));
        self.notify_on(event.clone()).expect("Notification Error");
    }

    pub fn sessions(&self) -> Vec<(ActorId, SessionData)> {
        self.as_ref().clone().into_iter().collect()
    }

    pub fn session_for_the_account(&self, account: ActorId) -> Option<SessionData> {
        self.as_ref().get(&account).cloned()
    }
}
