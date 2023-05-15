use super::{ADMIN, FT_ID};
use ft_main_io::*;
use gstd::prelude::*;
use gtest::{Program, System};

pub trait FToken {
    fn ftoken(system: &System) -> Program;
    fn mint(&self, transaction_id: u64, from: u64, account: u64, amount: u128);
    fn check_balance(&self, account: u64, expected_amount: u128);
    fn approve(&self, transaction_id: u64, from: u64, approved_account: u64, amount: u128);
    fn send_message_and_check_res(&self, from: u64, payload: FTokenAction);
}

impl FToken for Program<'_> {
    fn ftoken(system: &System) -> Program {
        let ftoken = Program::from_file_with_id(system, FT_ID, "../target/ft_main.wasm");
        let storage_code_hash: [u8; 32] = system.submit_code("../target/ft_storage.wasm").into();
        let ft_logic_code_hash: [u8; 32] = system.submit_code("../target/ft_logic.wasm").into();

        let res = ftoken.send(
            ADMIN,
            InitFToken {
                storage_code_hash: storage_code_hash.into(),
                ft_logic_code_hash: ft_logic_code_hash.into(),
            },
        );
        assert!(!res.main_failed());
        ftoken
    }

    fn mint(&self, transaction_id: u64, from: u64, account: u64, amount: u128) {
        let payload = LogicAction::Mint {
            recipient: account.into(),
            amount,
        };

        self.send_message_and_check_res(
            from,
            FTokenAction::Message {
                transaction_id,
                payload,
            },
        );
    }

    fn approve(&self, transaction_id: u64, from: u64, approved_account: u64, amount: u128) {
        let payload = LogicAction::Approve {
            approved_account: approved_account.into(),
            amount,
        };

        self.send_message_and_check_res(
            from,
            FTokenAction::Message {
                transaction_id,
                payload,
            },
        );
    }

    fn check_balance(&self, account: u64, expected_amount: u128) {
        let res = self.send(ADMIN, FTokenAction::GetBalance(account.into()));
        let reply = FTokenEvent::Balance(expected_amount).encode();
        assert!(res.contains(&(ADMIN, reply)));
    }

    fn send_message_and_check_res(&self, from: u64, payload: FTokenAction) {
        let res = self.send(from, payload);
        assert!(res.contains(&(from, FTokenEvent::Ok.encode())));
    }
}
