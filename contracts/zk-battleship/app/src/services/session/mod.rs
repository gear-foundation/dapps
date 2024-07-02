use self::storage::SessionsStorage;
use crate::services;
use core::{fmt::Debug, marker::PhantomData};
use gstd::{exec, msg, prelude::*, ActorId, Decode, Encode, TypeInfo};
use sails_rtl::gstd::{
    events::{EventTrigger, GStdEventTrigger},
    gservice,
};

pub use utils::*;

pub mod funcs;
pub mod storage;
pub(crate) mod utils;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rtl::scale_codec)]
#[scale_info(crate = sails_rtl::scale_info)]
pub enum Event {
    SessionCreated,
    SessionDeleted,
}

pub type GstdDrivenService = Service<GStdEventTrigger<Event>>;

#[derive(Clone)]
pub struct Service<X>(PhantomData<X>);

impl<X> Service<X> {
    pub fn seed() -> Self {
        let _res = SessionsStorage::default();
        debug_assert!(_res.is_ok());
        Self(PhantomData)
    }
}

#[gservice]
impl<X> Service<X>
where
    X: EventTrigger<Event>,
{
    pub fn new() -> Self {
        Self(PhantomData)
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
        services::utils::deposit_event(Event::SessionCreated);
    }

    pub fn delete_session(&mut self) {
        services::utils::panicking(move || {
            funcs::delete_session(SessionsStorage::as_mut(), msg::source())
        });
        services::utils::deposit_event(Event::SessionDeleted);
    }
    pub fn sessions(&self) -> Vec<(ActorId, Session)> {
        SessionsStorage::as_ref()
            .into_iter()
            .map(|(actor_id, session)| (*actor_id, session.clone()))
            .collect()
    }
}
