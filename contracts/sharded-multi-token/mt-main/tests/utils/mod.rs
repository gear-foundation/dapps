use gstd::{prelude::*, ActorId};
use gtest::{Program, System};
use mt_main_io::{InitMToken, LogicAction, MTokenAction, MTokenEvent, TokenId};

pub const ROOT_ACCOUNT: u64 = 100;

#[allow(unused)]
pub const USER_ACCOUNTS: [u64; 3] = [200, 300, 400];

pub trait MToken {
    fn mtoken(system: &System) -> Program;

    fn transfer(
        &self,
        tx_id: u64,
        from: u64,
        token_id: TokenId,
        to: u64,
        amount: u128,
        error: bool,
    );

    fn approve(&self, tx_id: u64, from: u64, account: u64, is_approved: bool, error: bool);

    fn create(
        &self,
        tx_id: u64,
        from: u64,
        initial_amount: u128,
        uri: String,
        is_nft: bool,
        error: bool,
    );

    fn mint_batch_ft(
        &self,
        tx_id: u64,
        from: u64,
        token_id: TokenId,
        to: Vec<u64>,
        amounts: Vec<u128>,
        error: bool,
    );

    fn mint_batch_nft(&self, tx_id: u64, from: u64, token_id: TokenId, to: Vec<u64>, error: bool);

    fn burn_batch_ft(
        &self,
        tx_id: u64,
        from: u64,
        token_id: TokenId,
        burn_from: Vec<u64>,
        amounts: Vec<u128>,
        error: bool,
    );

    fn burn_nft(&self, tx_id: u64, from: u64, token_id: TokenId, burn_from: u64, error: bool);

    fn send_message_and_check_res(&self, from: u64, payload: MTokenAction, error: bool);

    fn get_balance(&self, token_id: TokenId, account: u64) -> u128;

    fn get_approval(&self, account: u64, approval_target: u64) -> bool;
}

impl MToken for Program<'_> {
    fn mtoken(system: &System) -> Program {
        let mtoken = Program::current(system);

        let storage_code_hash: [u8; 32] = system
            .submit_code("../target/wasm32-unknown-unknown/debug/mt_storage.wasm")
            .into();
        let mt_logic_code_hash: [u8; 32] = system
            .submit_code("../target/wasm32-unknown-unknown/debug/mt_logic.wasm")
            .into();

        let res = mtoken.send(
            ROOT_ACCOUNT,
            InitMToken {
                storage_code_hash: storage_code_hash.into(),
                mt_logic_code_hash: mt_logic_code_hash.into(),
            },
        );

        assert!(!res.main_failed());
        mtoken
    }

    fn transfer(
        &self,
        tx_id: u64,
        from: u64,
        token_id: TokenId,
        to: u64,
        amount: u128,
        error: bool,
    ) {
        let payload = LogicAction::Transfer {
            token_id,
            sender: from.into(),
            recipient: to.into(),
            amount,
        };

        self.send_message_and_check_res(
            from,
            MTokenAction::Message {
                transaction_id: tx_id,
                payload,
            },
            error,
        );
    }

    fn approve(&self, tx_id: u64, from: u64, account: u64, is_approved: bool, error: bool) {
        let payload = LogicAction::Approve {
            account: account.into(),
            is_approved,
        };

        self.send_message_and_check_res(
            from,
            MTokenAction::Message {
                transaction_id: tx_id,
                payload,
            },
            error,
        );
    }

    fn create(
        &self,
        tx_id: u64,
        from: u64,
        initial_amount: u128,
        uri: String,
        is_nft: bool,
        error: bool,
    ) {
        let payload = LogicAction::Create {
            initial_amount,
            uri,
            is_nft,
        };

        self.send_message_and_check_res(
            from,
            MTokenAction::Message {
                transaction_id: tx_id,
                payload,
            },
            error,
        );
    }

    fn mint_batch_ft(
        &self,
        tx_id: u64,
        from: u64,
        token_id: TokenId,
        to: Vec<u64>,
        amounts: Vec<u128>,
        error: bool,
    ) {
        let payload = LogicAction::MintBatchFT {
            token_id,
            to: to.iter().map(|id| Into::<ActorId>::into(*id)).collect(),
            amounts,
        };

        self.send_message_and_check_res(
            from,
            MTokenAction::Message {
                transaction_id: tx_id,
                payload,
            },
            error,
        );
    }

    fn mint_batch_nft(&self, tx_id: u64, from: u64, token_id: TokenId, to: Vec<u64>, error: bool) {
        let payload = LogicAction::MintBatchNFT {
            token_id,
            to: to.iter().map(|id| Into::<ActorId>::into(*id)).collect(),
        };

        self.send_message_and_check_res(
            from,
            MTokenAction::Message {
                transaction_id: tx_id,
                payload,
            },
            error,
        );
    }

    fn burn_batch_ft(
        &self,
        tx_id: u64,
        from: u64,
        token_id: TokenId,
        burn_from: Vec<u64>,
        amounts: Vec<u128>,
        error: bool,
    ) {
        let payload = LogicAction::BurnBatchFT {
            token_id,
            burn_from: burn_from
                .iter()
                .map(|id| Into::<ActorId>::into(*id))
                .collect(),
            amounts,
        };

        self.send_message_and_check_res(
            from,
            MTokenAction::Message {
                transaction_id: tx_id,
                payload,
            },
            error,
        );
    }

    fn burn_nft(&self, tx_id: u64, from: u64, token_id: TokenId, burn_from: u64, error: bool) {
        let payload = LogicAction::BurnNFT {
            token_id,
            from: burn_from.into(),
        };

        self.send_message_and_check_res(
            from,
            MTokenAction::Message {
                transaction_id: tx_id,
                payload,
            },
            error,
        );
    }

    fn send_message_and_check_res(&self, from: u64, payload: MTokenAction, error: bool) {
        let res = self.send(from, payload);
        let reply = if error {
            MTokenEvent::Err.encode()
        } else {
            MTokenEvent::Ok.encode()
        };

        assert!(res.contains(&(from, reply)));
    }

    fn get_balance(&self, token_id: TokenId, account: u64) -> u128 {
        let res = self.send(
            account,
            MTokenAction::GetBalance {
                token_id,
                account: account.into(),
            },
        );
        assert!(!res.main_failed());

        let balance = res
            .log()
            .iter()
            .find_map(|log| {
                if let Ok(MTokenEvent::Balance(balance)) = MTokenEvent::decode(&mut log.payload()) {
                    Some(balance)
                } else {
                    None
                }
            })
            .expect("`MTokenEvent::Balance` not found in reply.");

        balance
    }

    fn get_approval(&self, account: u64, approval_target: u64) -> bool {
        let res = self.send(
            account,
            MTokenAction::GetApproval {
                account: account.into(),
                approval_target: approval_target.into(),
            },
        );
        assert!(!res.main_failed());

        let approval = res
            .log()
            .iter()
            .find_map(|log| {
                if let Ok(MTokenEvent::Approval(approval)) = MTokenEvent::decode(&mut log.payload())
                {
                    Some(approval)
                } else {
                    None
                }
            })
            .expect("`MTokenEvent::Approval` not found in reply.");

        approval
    }
}
