use super::{prelude::*, MetaStateReply, FOREIGN_USER};
use gear_lib_old::non_fungible_token::token::{Token, TokenId};
use gstd::ActorId;
use gtest::{Program as InnerProgram, System};
use non_fungible_token_io::{Collection, Config, InitNFT};
use std::fs;

pub struct NonFungibleToken<'a>(InnerProgram<'a>);

impl Program for NonFungibleToken<'_> {
    fn inner_program(&self) -> &InnerProgram<'_> {
        &self.0
    }
}

impl<'a> NonFungibleToken<'a> {
    pub fn initialize(system: &'a System) -> Self {
        let program = InnerProgram::from_file(
            system,
            "../target/wasm32-unknown-unknown/debug/non_fungible_token.opt.wasm",
        );

        assert!(!program
            .send(
                FOREIGN_USER,
                InitNFT {
                    royalties: Default::default(),
                    collection: Collection::default(),
                    config: Config::default()
                }
            )
            .main_failed());

        Self(program)
    }

    pub fn meta_state(&self) -> NonFungibleTokenMetaState<'_> {
        NonFungibleTokenMetaState(&self.0)
    }
}

pub struct NonFungibleTokenMetaState<'a>(&'a InnerProgram<'a>);

impl NonFungibleTokenMetaState<'_> {
    pub fn owner_id(self, token_id: u128) -> MetaStateReply<ActorId> {
        MetaStateReply(self.token(token_id).0.owner_id)
    }

    pub fn token(self, token_id: u128) -> MetaStateReply<Token> {
        if let Ok(token) = self.0.read_state_using_wasm::<TokenId, _, Token>(
            0,
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
