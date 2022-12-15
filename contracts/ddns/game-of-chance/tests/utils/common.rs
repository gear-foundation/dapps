use fmt::Debug;
use gstd::{prelude::*, ActorId};
use gtest::{Log, Program as InnerProgram, RunResult as InnerRunResult, System};

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

pub trait TransactionProgram {
    fn previous_mut_transaction_id(&mut self) -> &mut u64;

    fn transaction_id(&mut self) -> u64 {
        let transaction_id = self.previous_mut_transaction_id();

        *transaction_id = transaction_id.wrapping_add(1);

        *transaction_id
    }
}

#[must_use]
pub struct MetaStateReply<T>(pub T);

impl<T: Debug + PartialEq> MetaStateReply<T> {
    #[track_caller]
    pub fn eq(self, value: T) {
        assert_eq!(self.0, value);
    }
}

#[must_use]
pub struct RunResult<T, R>(pub InnerRunResult, pub fn(T) -> R);

impl<T, R> RunResult<T, R> {
    #[track_caller]
    pub fn contains(self, value: T)
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

#[must_use]
pub struct InitResult<T>(pub T, pub bool);

impl<T> InitResult<T> {
    #[track_caller]
    pub fn failed(self) {
        assert!(self.1)
    }

    #[track_caller]
    pub fn succeed(self) -> T {
        assert!(!self.1);

        self.0
    }
}
