use super::{prelude::*, StateReply, TransactionalProgram};
use deploy::NFT_BINARY;
use gear_lib::non_fungible_token::{
    io::NFTApproval,
    token::{Token, TokenId},
};
use gstd::ActorId;
use gtest::{Log, Program as InnerProgram, System};
use nft_io::{Constraints, InitNFT, NFTAction, NFTEvent};

pub struct NonFungibleToken<'a>(InnerProgram<'a>, u64);

impl Program for NonFungibleToken<'_> {
    fn inner_program(&self) -> &InnerProgram {
        &self.0
    }
}

impl TransactionalProgram for NonFungibleToken<'_> {
    fn previous_mut_transaction_id(&mut self) -> &mut u64 {
        &mut self.1
    }
}

impl<'a> NonFungibleToken<'a> {
    pub fn initialize(system: &'a System) -> Self {
        let program = InnerProgram::from_file(system, NFT_BINARY);

        assert!(!program
            .send(
                FOREIGN_USER,
                InitNFT {
                    royalties: Default::default(),
                    collection: Default::default(),
                    constraints: Constraints {
                        authorized_minters: vec![FOREIGN_USER.into()],
                        ..Default::default()
                    },
                },
            )
            .main_failed());

        Self(program, 0)
    }

    pub fn add_minter(&mut self, actor: impl Into<ActorId>) {
        let transaction_id = self.transaction_id();
        let actor = actor.into();

        assert!(self
            .0
            .send(
                FOREIGN_USER,
                NFTAction::AddMinter {
                    transaction_id,
                    minter_id: actor,
                },
            )
            .contains(&Log::builder().payload(NFTEvent::MinterAdded { minter_id: actor })),)
    }

    pub fn approve(&mut self, from: u64, to: ActorId, token_id: u128) {
        let transaction_id = self.transaction_id();

        assert!(self
            .0
            .send(
                from,
                NFTAction::Approve {
                    transaction_id,
                    to,
                    token_id: token_id.into()
                },
            )
            .contains(&Log::builder().payload(NFTEvent::Approval(NFTApproval {
                owner: from.into(),
                approved_account: to,
                token_id: token_id.into()
            }))),)
    }

    pub fn meta_state(&self) -> NonFungibleTokenState {
        NonFungibleTokenState(&self.0)
    }
}

pub struct NonFungibleTokenState<'a>(&'a InnerProgram<'a>);

impl NonFungibleTokenState<'_> {
    pub fn owner_id(self, token_id: u128) -> StateReply<ActorId> {
        StateReply(self.token(token_id).0.owner_id)
    }

    pub fn token(self, token_id: u128) -> StateReply<Token> {
        StateReply(
            self.0
                .read_state_using_wasm(
                    "token",
                    gclient::code_from_os(
                        "target/wasm32-unknown-unknown/debug/nft_state.meta.wasm",
                    )
                    .unwrap(),
                    Some(TokenId::from(token_id)),
                )
                .unwrap(),
        )
    }
}
