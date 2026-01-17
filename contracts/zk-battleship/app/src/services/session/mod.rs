use self::storage::SessionsStorage;
use crate::services;
use core::fmt::Debug;
use gstd::{ActorId, Decode, Encode, TypeInfo, msg, prelude::*};
use sails_rs::gstd::service;
use sails_rs::{event, export};
pub use utils::*;

use super::admin::storage::configuration::ConfigurationStorage;

pub mod funcs;
pub mod sr25519;
pub mod storage;
pub(crate) mod utils;

#[event]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum Event {
    SessionCreated,
    SessionDeleted,
}

#[derive(Clone)]
pub struct SessionService(());

impl SessionService {
    pub fn new() -> Self {
        Self(())
    }
    pub fn seed() -> Self {
        let _res = SessionsStorage::default();
        debug_assert!(_res.is_ok());
        Self(())
    }
}

#[service(events = Event)]
impl SessionService {
    #[export]
    pub fn create_session(&mut self, signature_data: SignatureData, signature: Option<Vec<u8>>) {
        services::utils::panicking(move || {
            funcs::create_session(
                SessionsStorage::as_mut(),
                ConfigurationStorage::get(),
                signature_data,
                signature,
            )
        });
        self.emit_event(Event::SessionCreated)
            .expect("Notification Error");
    }

    #[export]
    pub fn delete_session_from_program(&mut self, session_for_account: ActorId) {
        services::utils::panicking(move || {
            funcs::delete_session_from_program(SessionsStorage::as_mut(), session_for_account)
        });
        self.emit_event(Event::SessionDeleted)
            .expect("Notification Error");
    }

    #[export]
    pub fn delete_session_from_account(&mut self) {
        services::utils::panicking(move || {
            funcs::delete_session_from_account(SessionsStorage::as_mut(), msg::source())
        });
        self.emit_event(Event::SessionDeleted)
            .expect("Notification Error");
    }

    #[export]
    pub fn sessions(&self) -> Vec<(ActorId, Session)> {
        SessionsStorage::as_ref()
            .into_iter()
            .map(|(actor_id, session)| (*actor_id, session.clone()))
            .collect()
    }

    #[export]
    pub fn session_for_the_account(&self, account: ActorId) -> Option<Session> {
        SessionsStorage::as_ref().get(&account).cloned()
    }
}
