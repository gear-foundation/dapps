use super::prelude::*;
use core::fmt::Debug;
use gstd::ActorId;
use gtest::{Log, Program as InnerProgram, RunResult, System};

pub fn initialize_system() -> System {
    let system = System::new();
    system.init_logger();
    system
}

pub trait Program {
    fn inner_program(&self) -> &InnerProgram;

    fn actor_id(&self) -> ActorId {
        let bytes: [u8; 32] = self.inner_program().id().into();
        bytes.into()
    }
}

pub struct MetaStateReply<T>(pub T);

impl<T: Debug + PartialEq> MetaStateReply<T> {
    #[track_caller]
    pub fn check(self, value: T) {
        assert_eq!(self.0, value);
    }
}

pub struct Action<T, R>(pub RunResult, pub fn(T) -> R);

impl<T, R> Action<T, R> {
    #[track_caller]
    pub fn check(self, value: T)
    where
        R: Encode,
    {
        assert!(self.0.contains(&Log::builder().payload(self.1(value))));
    }

    #[track_caller]
    pub fn failed(self) {
        assert!(self.0.main_failed())
    }
}
