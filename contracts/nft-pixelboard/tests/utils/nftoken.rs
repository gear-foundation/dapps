use super::{common::MetaStateReply, prelude::*};
use gear_lib::non_fungible_token::{
    state::{NFTQuery, NFTQueryReply},
    token::Token,
};
use gstd::ActorId;
use gtest::{Program as InnerProgram, System};
use nft_io::InitNFT;

pub struct NonFungibleToken<'a>(InnerProgram<'a>);

impl Program for NonFungibleToken<'_> {
    fn inner_program(&self) -> &InnerProgram {
        &self.0
    }
}

impl<'a> NonFungibleToken<'a> {
    pub fn initialize(system: &'a System) -> Self {
        let program = InnerProgram::from_file(system, "./target/nft-0.1.0.wasm");

        assert!(!program
            .send(
                FOREIGN_USER,
                InitNFT {
                    base_uri: Default::default(),
                    name: Default::default(),
                    royalties: Default::default(),
                    symbol: Default::default(),
                }
            )
            .main_failed());

        Self(program)
    }

    pub fn meta_state(&self) -> NonFungibleTokenMetaState {
        NonFungibleTokenMetaState(&self.0)
    }
}

pub struct NonFungibleTokenMetaState<'a>(&'a InnerProgram<'a>);

impl NonFungibleTokenMetaState<'_> {
    pub fn owner(self, token_id: u128) -> MetaStateReply<ActorId> {
        MetaStateReply(self.token_metadata(token_id).0.owner_id)
    }

    pub fn token_metadata(self, token_id: u128) -> MetaStateReply<Token> {
        if let NFTQueryReply::Token { token: reply } = self
            .0
            .meta_state(NFTQuery::Token {
                token_id: token_id.into(),
            })
            .unwrap()
        {
            MetaStateReply(reply)
        } else {
            unreachable!();
        }
    }
}
