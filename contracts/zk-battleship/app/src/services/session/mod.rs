use self::storage::SessionsStorage;
use crate::services;
use core::fmt::Debug;
use gstd::{exec, msg, prelude::*, ActorId, Decode, Encode, TypeInfo};
use sails_rs::gstd::service;
use sails_rs::{format, Box};

pub use utils::*;

pub mod funcs;
pub mod storage;
pub(crate) mod utils;

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
    pub fn seed() -> Self {
        let _res = SessionsStorage::default();
        debug_assert!(_res.is_ok());
        Self(())
    }
}

#[service(events = Event)]
impl SessionService {
    pub fn new() -> Self {
        Self(())
    }
    pub fn create_session(
        &mut self,
        key: ActorId,
        duration: u64,
        allowed_actions: Vec<ActionsForSession>,
    ) {
        services::utils::panicking(move || {
            funcs::create_session(
                SessionsStorage::as_mut(),
                msg::source(),
                exec::block_timestamp(),
                key,
                duration,
                allowed_actions,
            )
        });
        let _unused = self.notify_on(Event::SessionCreated);
    }

    pub fn delete_session(&mut self) {
        services::utils::panicking(move || {
            funcs::delete_session(SessionsStorage::as_mut(), msg::source())
        });
        let _unused = self.notify_on(Event::SessionDeleted);
    }
    pub fn sessions(&self) -> Vec<(ActorId, Session)> {
        SessionsStorage::as_ref()
            .into_iter()
            .map(|(actor_id, session)| (*actor_id, session.clone()))
            .collect()
    }
}
