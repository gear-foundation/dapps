use super::{prelude::*, MetaStateReply};
use gstd::ActorId;
use gtest::{Log, Program as InnerProgram, System};
use sharded_fungible_token_io::{FTokenAction, FTokenEvent, InitFToken, LogicAction};

pub struct FungibleToken<'a>(InnerProgram<'a>);

impl Program for FungibleToken<'_> {
    fn inner_program(&self) -> &InnerProgram<'_> {
        &self.0
    }
}

impl<'a> FungibleToken<'a> {
    pub fn initialize(system: &'a System) -> Self {
        let program = InnerProgram::from_file(
            system,
            "../target/wasm32-unknown-unknown/release/sharded_fungible_token.opt.wasm",
        );
        let storage_code_hash: [u8; 32] = system
            .submit_code_file(
                "../target/wasm32-unknown-unknown/release/sharded_fungible_token_storage.opt.wasm",
            )
            .into();
        let ft_logic_code_hash: [u8; 32] = system
            .submit_code_file(
                "../target/wasm32-unknown-unknown/release/sharded_fungible_token_logic.opt.wasm",
            )
            .into();

        assert!(!program
            .send(
                ADMIN,
                InitFToken {
                    storage_code_hash: storage_code_hash.into(),
                    ft_logic_code_hash: ft_logic_code_hash.into(),
                },
            )
            .main_failed());

        Self(program)
    }

    pub fn mint(&self, transaction_id: u64, from: u64, amount: u128) {
        let payload = LogicAction::Mint {
            recipient: from.into(),
            amount,
        };

        assert!(self
            .0
            .send(
                from,
                FTokenAction::Message {
                    transaction_id,
                    payload,
                }
            )
            .contains(&Log::builder().payload(FTokenEvent::Ok)));
    }

    pub fn balance_of(&self, actor: u64) -> MetaStateReply<u128> {
        let payload = FTokenAction::GetBalance(actor.into()).encode();
        let result = self.0.send_bytes(ADMIN, payload);
        assert!(!result.main_failed());

        let amount = result
            .log()
            .iter()
            .find_map(|log| {
                let mut payload = log.payload();
                if let Ok(FTokenEvent::Balance(amount)) = FTokenEvent::decode(&mut payload) {
                    Some(amount)
                } else {
                    None
                }
            })
            .expect("Invalid balance reply!");

        MetaStateReply(amount)
    }

    pub fn approve(&self, transaction_id: u64, from: u64, to: ActorId, amount: u128) {
        let payload = LogicAction::Approve {
            approved_account: to,
            amount,
        };

        assert!(self
            .0
            .send(
                from,
                FTokenAction::Message {
                    transaction_id,
                    payload,
                }
            )
            .contains(&Log::builder().payload(FTokenEvent::Ok)));
    }
}
