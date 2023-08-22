use convert::identity;
use gstd::{prelude::*, ActorId};
use gtest::{Log, Program as InnerProgram, RunResult as InnerRunResult};
use marker::PhantomData;

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
    fn common_succeed<V: Encode>(self, value: T, wrap: fn(R) -> V) {
        let event = (self.event)(value);

        self.assert_contains(wrap(event));
    }

    #[track_caller]
    pub fn contains(self, value: T) {
        self.common_succeed(value, identity);
    }
}

#[track_caller]
fn assert_contains(result: &InnerRunResult, payload: impl Encode) {
    assert!(result.contains(&Log::builder().payload(payload)));
}
