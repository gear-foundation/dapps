use gstd::prelude::*;
use gtest::{Program, System};
use sharded_fungible_token_io::{FTokenAction, FTokenEvent, InitFToken, LogicAction};

pub trait FToken {
    fn ftoken(owner: u64, id: u64, system: &System) -> Program<'_>;
    fn mint(&self, transaction_id: u64, from: u64, account: u64, amount: u128, error: bool);
    fn check_balance(&self, account: u64, expected_amount: u128);
    fn burn(&self, transaction_id: u64, from: u64, account: u64, amount: u128, error: bool);
    fn transfer(
        &self,
        transaction_id: u64,
        from: u64,
        sender: u64,
        recipient: u64,
        amount: u128,
        error: bool,
    );
    fn approve(
        &self,
        transaction_id: u64,
        from: u64,
        approved_account: u64,
        amount: u128,
        error: bool,
    );
    fn send_message_and_check_res(&self, from: u64, payload: FTokenAction, error: bool);
}

impl FToken for Program<'_> {
    fn ftoken(owner: u64, id: u64, system: &System) -> Program<'_> {
        let ftoken = Program::from_file_with_id(
            system,
            id,
            "../target/wasm32-unknown-unknown/debug/sharded_fungible_token.opt.wasm",
        );
        let storage_code_hash: [u8; 32] = system
            .submit_code(
                "../target/wasm32-unknown-unknown/debug/sharded_fungible_token_storage.opt.wasm",
            )
            .into();
        let ft_logic_code_hash: [u8; 32] = system
            .submit_code(
                "../target/wasm32-unknown-unknown/debug/sharded_fungible_token_logic.opt.wasm",
            )
            .into();

        let res = ftoken.send(
            owner,
            InitFToken {
                storage_code_hash: storage_code_hash.into(),
                ft_logic_code_hash: ft_logic_code_hash.into(),
            },
        );

        assert!(!res.main_failed());
        ftoken
    }

    fn mint(&self, transaction_id: u64, from: u64, account: u64, amount: u128, error: bool) {
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
            error,
        );
    }

    fn burn(&self, transaction_id: u64, from: u64, account: u64, amount: u128, error: bool) {
        let payload = LogicAction::Burn {
            sender: account.into(),
            amount,
        };

        self.send_message_and_check_res(
            from,
            FTokenAction::Message {
                transaction_id,
                payload,
            },
            error,
        );
    }

    fn transfer(
        &self,
        transaction_id: u64,
        from: u64,
        sender: u64,
        recipient: u64,
        amount: u128,
        error: bool,
    ) {
        let payload = LogicAction::Transfer {
            sender: sender.into(),
            recipient: recipient.into(),
            amount,
        };

        self.send_message_and_check_res(
            from,
            FTokenAction::Message {
                transaction_id,
                payload,
            },
            error,
        );
    }

    fn approve(
        &self,
        transaction_id: u64,
        from: u64,
        approved_account: u64,
        amount: u128,
        error: bool,
    ) {
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
            error,
        );
    }

    fn check_balance(&self, account: u64, expected_amount: u128) {
        let res = self.send(100, FTokenAction::GetBalance(account.into()));
        let reply = FTokenEvent::Balance(expected_amount).encode();
        assert!(res.contains(&(100, reply)));
    }

    fn send_message_and_check_res(&self, from: u64, payload: FTokenAction, error: bool) {
        let res = self.send(from, payload);
        let reply = if error {
            FTokenEvent::Err.encode()
        } else {
            FTokenEvent::Ok.encode()
        };

        assert!(res.contains(&(from, reply)));
    }
}
