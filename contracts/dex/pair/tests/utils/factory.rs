use super::{common::StateReply, InitResult, Program, RunResult, FOREIGN_USER};
use dex_factory::WASM_BINARY_OPT;
use dex_factory_io::*;
use dex_factory_state::{WASM_BINARY, WASM_EXPORTS};
use gstd::{prelude::*, ActorId};
use gtest::{Program as InnerProgram, System};

type FactoryRunResult<T, R> = RunResult<T, R, Event, Error>;

pub struct Factory<'a>(InnerProgram<'a>);

impl Program for Factory<'_> {
    fn inner_program(&self) -> &InnerProgram {
        &self.0
    }
}

impl<'a> Factory<'a> {
    pub fn initialize(
        system: &'a System,
        fee_to: u64,
        fee_to_setter: u64,
        id: u64,
    ) -> InitResult<Self, Error> {
        let program =
            InnerProgram::from_opt_and_meta_code_with_id(system, id, WASM_BINARY_OPT.into(), None);
        let pair_code_id: [u8; 32] = system
            .submit_code("../target/wasm32-unknown-unknown/debug/dex_pair.opt.wasm")
            .into();

        let result = program.send(
            FOREIGN_USER,
            Initialize {
                fee_to: fee_to.into(),
                fee_to_setter: fee_to_setter.into(),
                pair: pair_code_id.into(),
            },
        );
        let is_active = system.is_active_program(program.id());

        InitResult::new(Self(program), result, is_active)
    }

    pub fn create_pair(
        &mut self,
        pair: (ActorId, ActorId),
    ) -> FactoryRunResult<((ActorId, ActorId), u32), [u8; 32]> {
        RunResult::new(
            self.0
                .send(FOREIGN_USER, Action::CreatePair(pair.0, pair.1)),
            |event, (token_pair, pair_number)| {
                if let Event::PairCreated {
                    token_pair: true_token_pair,
                    pair_actor,
                    pair_number: true_pair_number,
                } = event
                {
                    assert_eq!(token_pair, true_token_pair);
                    assert_eq!(pair_number, true_pair_number);

                    pair_actor.into()
                } else {
                    unreachable!()
                }
            },
        )
    }

    pub fn fee_to(&mut self, from: u64, to: impl Into<ActorId>) -> FactoryRunResult<ActorId, ()> {
        RunResult::new(
            self.0.send(from, Action::FeeTo(to.into())),
            |event, fee_to| {
                assert_eq!(event, Event::FeeToSet(fee_to));
            },
        )
    }

    pub fn fee_to_setter(
        &mut self,
        from: u64,
        to: impl Into<ActorId>,
    ) -> FactoryRunResult<u64, ()> {
        RunResult::new(
            self.0.send(from, Action::FeeToSetter(to.into())),
            |event, fee_to_setter| assert!(event == Event::FeeToSetterSet(fee_to_setter.into())),
        )
    }

    pub fn state(&self) -> FactoryState {
        FactoryState(&self.0)
    }
}

pub struct FactoryState<'a>(&'a InnerProgram<'a>);

impl FactoryState<'_> {
    fn query_state_common<A: Encode, T: Decode>(
        self,
        fn_index: usize,
        argument: Option<A>,
    ) -> StateReply<T> {
        StateReply(
            self.0
                .read_state_using_wasm(WASM_EXPORTS[fn_index], WASM_BINARY.into(), argument)
                .unwrap(),
        )
    }

    fn query_state_with_argument<A: Encode, T: Decode>(
        self,
        fn_index: usize,
        argument: A,
    ) -> StateReply<T> {
        self.query_state_common(fn_index, Some(argument))
    }

    fn query_state<T: Decode>(self, fn_index: usize) -> StateReply<T> {
        self.query_state_common::<(), _>(fn_index, None)
    }

    pub fn fee_to(self) -> StateReply<ActorId> {
        self.query_state(1)
    }

    pub fn fee_to_setter(self) -> StateReply<ActorId> {
        self.query_state(2)
    }

    pub fn pair(self, pair: (ActorId, ActorId)) -> StateReply<ActorId> {
        self.query_state_with_argument(3, pair)
    }

    pub fn all_pairs_length(self) -> StateReply<u32> {
        self.query_state(4)
    }

    pub fn all_pairs(self) -> StateReply<Vec<((ActorId, ActorId), ActorId)>> {
        self.query_state(5)
    }
}
