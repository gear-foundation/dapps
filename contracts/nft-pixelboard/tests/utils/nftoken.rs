use super::{prelude::*, MetaStateReply, FOREIGN_USER};
use gear_lib::non_fungible_token::token::{Token, TokenId};
use gstd::ActorId;
use gtest::{Program as InnerProgram, System};
use nft_io::InitNFT;
use std::fs;

pub struct NonFungibleToken<'a>(InnerProgram<'a>, u64);

impl Program for NonFungibleToken<'_> {
    fn inner_program(&self) -> &InnerProgram {
        &self.0
    }
}

impl<'a> NonFungibleToken<'a> {
    pub fn initialize(system: &'a System) -> Self {
        let program = InnerProgram::from_file(system, "target/nft.opt.wasm");

        assert!(!program
            .send(
                FOREIGN_USER,
                InitNFT {
                    name: Default::default(),
                    symbol: Default::default(),
                    base_uri: Default::default(),
                    royalties: Default::default()
                }
            )
            .main_failed());

        Self(program, 0)
    }

    pub fn meta_state(&self) -> NonFungibleTokenMetaState {
        NonFungibleTokenMetaState(&self.0)
    }
}

pub struct NonFungibleTokenMetaState<'a>(&'a InnerProgram<'a>);

impl NonFungibleTokenMetaState<'_> {
    pub fn owner_id(self, token_id: u128) -> MetaStateReply<ActorId> {
        MetaStateReply(self.token(token_id).0.owner_id)
    }

    pub fn token(self, token_id: u128) -> MetaStateReply<Token> {
        if let Ok(token) = self.0.read_state_using_wasm::<TokenId, Token>(
            "token",
            fs::read("target/nft_state.wasm").unwrap(),
            Some(token_id.into()),
        ) {
            MetaStateReply(token)
        } else {
            unreachable!();
        }
    }
}
