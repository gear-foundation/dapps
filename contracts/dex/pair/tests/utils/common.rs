use gstd::{fmt::Debug, marker::PhantomData, prelude::*, ActorId};
use gtest::{Log, Program as InnerProgram, RunResult as InnerRunResult, System};
use pretty_assertions::assert_eq;

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
pub struct StateReply<T>(pub T);

impl<T: Debug + PartialEq> StateReply<T> {
    #[track_caller]
    pub fn eq(self, value: T) {
        assert_eq!(self.0, value);
    }
}

#[must_use]
pub struct RunResult<Check, CheckResult, Event, Error> {
    pub result: InnerRunResult,
    check: fn(Event, Check) -> CheckResult,
    ghost_data: PhantomData<(Event, Error)>,
}

impl<Check, CheckResult, Event: Decode + Debug, Error: Decode + Debug + PartialEq>
    RunResult<Check, CheckResult, Event, Error>
{
    pub fn new(result: InnerRunResult, check: fn(Event, Check) -> CheckResult) -> Self {
        Self {
            result,
            check,
            ghost_data: PhantomData,
        }
    }

    #[track_caller]
    pub fn failed(self, error: Error) {
        assert_eq!(
            decode::<Result<Event, Error>>(&self.result).unwrap_err(),
            error
        );
    }

    #[track_caller]
    pub fn succeed(self, value: Check) -> CheckResult {
        (self.check)(decode::<Result<Event, Error>>(&self.result).unwrap(), value)
    }

    #[track_caller]
    pub fn contains(self, value: Check) {
        (self.check)(decode::<Event>(&self.result), value);
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

fn decode<T: Decode>(result: &InnerRunResult) -> T {
    match T::decode(&mut result.log()[0].payload()) {
        Ok(ok) => ok,
        Err(_) => panic!("{}", String::from_utf8_lossy(result.log()[0].payload())),
    }
}
