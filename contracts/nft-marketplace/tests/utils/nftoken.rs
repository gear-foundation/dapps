use super::{prelude::*, MetaStateReply};
use gear_lib::non_fungible_token::{io::*, token::Token};
use gstd::ActorId;
use gtest::{Log, Program as InnerProgram, System};
use market_io::*;
use nft_io::{InitNFT, NFTAction, NFTEvent};

pub struct NonFungibleToken<'a>(InnerProgram<'a>);

impl Program for NonFungibleToken<'_> {
    fn inner_program(&self) -> &InnerProgram {
        &self.0
    }
}

impl<'a> NonFungibleToken<'a> {
    pub fn initialize(system: &'a System) -> Self {
        let program = InnerProgram::from_file(system, "./target/nft.wasm");

        assert!(!program
            .send(
                ADMIN,
                InitNFT {
                    name: Default::default(),
                    symbol: Default::default(),
                    base_uri: Default::default(),
                    royalties: None
                }
            )
            .main_failed());

        Self(program)
    }

    pub fn mint(&self, transaction_id: u64, from: u64) {
        assert!(self
            .0
            .send(
                from,
                NFTAction::Mint {
                    transaction_id,
                    token_metadata: Default::default()
                }
            )
            .contains(&Log::builder().payload(NFTEvent::Transfer(NFTTransfer {
                from: ActorId::zero(),
                to: from.into(),
                token_id: TOKEN_ID.into(),
            }))));
    }

    pub fn approve(&self, transaction_id: u64, from: u64, to: ActorId, token_id: TokenId) {
        assert!(self
            .0
            .send(
                from,
                NFTAction::Approve {
                    transaction_id,
                    to,
                    token_id
                }
            )
            .contains(&Log::builder().payload(NFTEvent::Approval(NFTApproval {
                owner: from.into(),
                approved_account: to,
                token_id: TOKEN_ID.into(),
            }))));
    }

    pub fn meta_state(&self) -> NonFungibleTokenMetaState {
        NonFungibleTokenMetaState(&self.0)
    }
}

pub struct NonFungibleTokenMetaState<'a>(&'a InnerProgram<'a>);

impl NonFungibleTokenMetaState<'_> {
    pub fn owner_id(self, token_id: u64) -> MetaStateReply<ActorId> {
        MetaStateReply(self.token(token_id).0.owner_id)
    }

    pub fn token(self, _token_id: u64) -> MetaStateReply<Token> {
        unreachable!()
    }
}
