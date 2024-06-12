use gstd::{fmt::Debug, marker::PhantomData, prelude::*};
use gtest::{Log, RunResult as InnerRunResult, System};

pub fn initialize_system() -> System {
    let system = System::new();

    system.init_logger();

    system
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
    pub fn failed(self, error: Error, index: usize) {
        assert_eq!(
            decode::<Result<Event, Error>>(&self.result, index).unwrap_err(),
            error
        );
    }

    #[track_caller]
    pub fn succeed(self, value: Check, index: usize) -> CheckResult {
        (self.check)(
            decode::<Result<Event, Error>>(&self.result, index).unwrap(),
            value,
        )
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

fn decode<T: Decode>(result: &InnerRunResult, index: usize) -> T {
    match T::decode(&mut result.log()[index].payload()) {
        Ok(ok) => ok,
        Err(_) => std::panic!("{}", String::from_utf8_lossy(result.log()[0].payload())),
    }
}
