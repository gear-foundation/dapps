use convert::identity;
use fmt::Debug;
use gstd::{prelude::*, ActorId};
use gtest::{Log, Program as InnerProgram, RunResult as InnerRunResult, System};
use marker::PhantomData;

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

pub trait TransactionalProgram {
    fn previous_mut_transaction_id(&mut self) -> &mut u64;

    fn transaction_id(&mut self) -> u64 {
        let tx_id = self.previous_mut_transaction_id();

        *tx_id = tx_id.wrapping_add(1);

        *tx_id
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
pub struct RunResult<T, R, E> {
    pub result: InnerRunResult,
    event: fn(T) -> R,
    ghost_data: PhantomData<E>,
}

impl<T, R: Encode, E: Encode> RunResult<T, R, E> {
    pub fn new(result: InnerRunResult, event: fn(T) -> R) -> Self {
        Self {
            result,
            event,
            ghost_data: PhantomData,
        }
    }

    #[track_caller]
    fn assert_contains(self, payload: impl Encode) {
        assert_contains(&self.result, payload);
    }

    #[track_caller]
    pub fn failed(self, error: E) {
        self.assert_contains(Err::<R, E>(error));
    }

    #[track_caller]
    fn common_succeed<V: Encode>(self, value: T, wrap: fn(R) -> V) {
        let event = (self.event)(value);

        self.assert_contains(wrap(event));
    }

    #[track_caller]
    pub fn succeed(self, value: T) {
        self.common_succeed(value, Ok::<R, E>);
    }

    #[track_caller]
    pub fn contains(self, value: T) {
        self.common_succeed(value, identity);
    }
}

#[must_use]
pub struct InitResult<T, E> {
    contract_instance: T,
    pub result: InnerRunResult,
    pub is_active: bool,
    ghost_data: PhantomData<E>,
}

impl<T, E: Encode> InitResult<T, E> {
    pub fn new(contract_instance: T, result: InnerRunResult, is_active: bool) -> Self {
        Self {
            contract_instance,
            result,
            is_active,
            ghost_data: PhantomData,
        }
    }

    fn assert_contains(&self, payload: impl Encode) {
        assert_contains(&self.result, payload);
    }

    #[track_caller]
    pub fn failed(self, error: E) {
        assert!(!self.is_active);
        self.assert_contains(Err::<(), E>(error));
    }

    #[track_caller]
    pub fn succeed(self) -> T {
        assert!(self.is_active);
        self.assert_contains(Ok::<_, E>(()));

        self.contract_instance
    }
}

#[track_caller]
fn assert_contains(result: &InnerRunResult, payload: impl Encode) {
    assert!(result.contains(&Log::builder().payload(payload)));
}
