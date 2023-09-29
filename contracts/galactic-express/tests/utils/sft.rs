use super::{prelude::*, RunResult};
use gstd::{prelude::*, ActorId};
use gtest::{Program as InnerProgram, System};
use sharded_fungible_token_io::{FTokenAction, FTokenEvent, InitFToken};

pub struct Sft<'a>(InnerProgram<'a>, u64);

impl Program for Sft<'_> {
    fn inner_program(&self) -> &InnerProgram<'_> {
        &self.0
    }
}

impl<'a> Sft<'a> {
    #[track_caller]
    pub fn initialize(system: &'a System) -> Self {
        let program = InnerProgram::from_file(
            system,
            "../target/wasm32-unknown-unknown/debug/sharded_fungible_token.opt.wasm",
        );
        let storage_code_id: [u8; 32] = system
            .submit_code(
                "../target/wasm32-unknown-unknown/debug/sharded_fungible_token_storage.opt.wasm",
            )
            .into();
        let logic_code_id: [u8; 32] = system
            .submit_code(
                "../target/wasm32-unknown-unknown/debug/sharded_fungible_token_logic.opt.wasm",
            )
            .into();

        assert!(!program
            .send(
                FOREIGN_USER,
                InitFToken {
                    storage_code_hash: storage_code_id.into(),
                    ft_logic_code_hash: logic_code_id.into(),
                },
            )
            .main_failed());

        Self(program, 0)
    }

    pub fn balance(&self, actor_id: impl Into<ActorId>) -> RunResult<u128, (), FTokenEvent, Error> {
        RunResult::new(
            self.0
                .send(FOREIGN_USER, FTokenAction::GetBalance(actor_id.into())),
            |event, bal| assert_eq!(FTokenEvent::Balance(bal), event),
        )
    }
}
