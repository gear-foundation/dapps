use super::{prelude::*, StateReply};
use deploy::NFT_BINARY;
use gear_lib::non_fungible_token::token::{Token, TokenId};
use gstd::ActorId;
use gtest::{Program as InnerProgram, System};
use nft_io::InitNFT;

pub struct NonFungibleToken<'a>(InnerProgram<'a>, u64);

impl Program for NonFungibleToken<'_> {
    fn inner_program(&self) -> &InnerProgram {
        &self.0
    }
}

impl<'a> NonFungibleToken<'a> {
    pub fn initialize(system: &'a System) -> Self {
        let program = InnerProgram::from_file(system, NFT_BINARY);

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
                    gclient::code_from_os("target/nft-state.wasm").unwrap(),
                    Some(TokenId::from(token_id)),
                )
                .unwrap(),
        )
    }
}
