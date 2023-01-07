use ft_logic_io::{Action, FTLogicEvent};
use ft_main_io::*;
use gstd::{prelude::*, ActorId};
use gtest::{Program, System};
use sp_core::sr25519::Signature;

pub trait FToken {
    fn ftoken(system: &System) -> Program;
    fn mint(&self, transaction_id: u64, from: u64, account: u64, amount: u128, error: bool);
    fn check_balance(&self, account: impl Into<ActorId>, expected_amount: u128);
    fn check_permit_id(&self, account: [u8; 32], expected_permit_id: u128);
    fn burn(&self, transaction_id: u64, from: u64, account: u64, amount: u128, error: bool);
    fn transfer(
        &self,
        transaction_id: u64,
        from: u64,
        sender: impl Into<ActorId>,
        recipient: impl Into<ActorId>,
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

    #[allow(clippy::too_many_arguments)]
    fn permit(
        &self,
        transaction_id: u64,
        from: u64,
        owner: ActorId,
        approved_account: ActorId,
        amount: u128,
        permit_id: u128,
        sign: Signature,
        error: bool,
    );
    fn send_message_and_check_res(&self, from: u64, payload: FTokenAction, error: bool);
}

const HARDCODED_ACCOUNT: u64 = 100;

impl FToken for Program<'_> {
    fn ftoken(system: &System) -> Program {
        let ftoken = Program::current(system);
        let storage_code_hash: [u8; 32] = system
            .submit_code("../target/wasm32-unknown-unknown/release/ft_storage.opt.wasm")
            .into();

        let ft_logic_code_hash: [u8; 32] = system
            .submit_code("../target/wasm32-unknown-unknown/release/ft_logic.opt.wasm")
            .into();

        let res = ftoken.send(
            HARDCODED_ACCOUNT,
            InitFToken {
                storage_code_hash: storage_code_hash.into(),
                ft_logic_code_hash: ft_logic_code_hash.into(),
            },
        );
        assert!(!res.main_failed());
        ftoken
    }

    fn mint(&self, transaction_id: u64, from: u64, account: u64, amount: u128, error: bool) {
        let payload = Action::Mint {
            recipient: account.into(),
            amount,
        }
        .encode();
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
        let payload = Action::Burn {
            sender: account.into(),
            amount,
        }
        .encode();
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
        sender: impl Into<ActorId>,
        recipient: impl Into<ActorId>,
        amount: u128,
        error: bool,
    ) {
        let payload = Action::Transfer {
            sender: sender.into(),
            recipient: recipient.into(),
            amount,
        }
        .encode();
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
        let payload = Action::Approve {
            approved_account: approved_account.into(),
            amount,
        }
        .encode();
        self.send_message_and_check_res(
            from,
            FTokenAction::Message {
                transaction_id,
                payload,
            },
            error,
        );
    }

    fn permit(
        &self,
        transaction_id: u64,
        from: u64,
        owner: ActorId,
        approved_account: ActorId,
        amount: u128,
        permit_id: u128,
        sign: Signature,
        error: bool,
    ) {
        let payload = Action::Permit {
            owner_account: owner,
            approved_account,
            amount,
            permit_id,
            sign: sign.into(),
        }
        .encode();
        self.send_message_and_check_res(
            from,
            FTokenAction::Message {
                transaction_id,
                payload,
            },
            error,
        );
    }

    fn check_balance(&self, account: impl Into<ActorId>, expected_amount: u128) {
        let res = self.send(HARDCODED_ACCOUNT, FTokenAction::GetBalance(account.into()));
        let reply = FTLogicEvent::Balance(expected_amount).encode();
        assert!(res.contains(&(HARDCODED_ACCOUNT, reply)));
    }

    fn check_permit_id(&self, account: [u8; 32], expected_permit_id: u128) {
        let res = self.send(HARDCODED_ACCOUNT, FTokenAction::GetPermitId(account.into()));
        let reply = FTLogicEvent::PermitId(expected_permit_id).encode();
        assert!(res.contains(&(HARDCODED_ACCOUNT, reply)));
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
