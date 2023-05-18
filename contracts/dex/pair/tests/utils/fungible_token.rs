use super::{
    Program, RunResult, TransactionalProgram, FOREIGN_USER, FT_LOGIC, FT_MAIN, FT_STORAGE,
};
use ft_main_io::{FTokenAction, FTokenEvent, InitFToken, LogicAction};
use gstd::{prelude::*, ActorId};
use gtest::{Log, Program as InnerProgram, RunResult as InnerRunResult, System};

pub struct FungibleToken<'a>(InnerProgram<'a>, u64);

impl Program for FungibleToken<'_> {
    fn inner_program(&self) -> &InnerProgram {
        &self.0
    }
}

impl TransactionalProgram for FungibleToken<'_> {
    fn previous_mut_transaction_id(&mut self) -> &mut u64 {
        &mut self.1
    }
}

impl<'a> FungibleToken<'a> {
    #[track_caller]
    pub fn initialize(system: &'a System) -> Self {
        let program = InnerProgram::from_file(system, FT_MAIN);
        let storage_code_id: [u8; 32] = system.submit_code(FT_STORAGE).into();
        let logic_code_id: [u8; 32] = system.submit_code(FT_LOGIC).into();

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

    #[track_caller]
    pub fn mint(&mut self, recipient: impl Into<ActorId>, amount: u128) {
        let transaction_id = self.transaction_id();

        assert_ft_token_event_ok(self.0.send(
            FOREIGN_USER,
            FTokenAction::Message {
                transaction_id,
                payload: LogicAction::Mint {
                    recipient: recipient.into(),
                    amount,
                },
            },
        ))
    }

    #[track_caller]
    pub fn approve(&mut self, from: u64, approved_account: impl Into<ActorId>, amount: u128) {
        let transaction_id = self.transaction_id();

        assert_ft_token_event_ok(self.0.send(
            from,
            FTokenAction::Message {
                transaction_id,
                payload: LogicAction::Approve {
                    approved_account: approved_account.into(),
                    amount,
                },
            },
        ));
    }

    #[track_caller]
    pub fn balance(&self, actor_id: impl Into<ActorId>) -> RunResult<u128, (), FTokenEvent, ()> {
        RunResult::new(
            self.0
                .send(FOREIGN_USER, FTokenAction::GetBalance(actor_id.into())),
            |event, balance| {
                if let FTokenEvent::Balance(true_balance) = event {
                    assert_eq!(balance, true_balance)
                } else {
                    unreachable!()
                }
            },
        )
    }
}

fn assert_ft_token_event_ok(run_result: InnerRunResult) {
    assert!(run_result.contains(&Log::builder().payload(FTokenEvent::Ok)))
}
