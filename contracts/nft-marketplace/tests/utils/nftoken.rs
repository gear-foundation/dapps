use super::{prelude::*, MetaStateReply};
use gear_lib_old::non_fungible_token::{io::*, token::Token};
use gstd::ActorId;
use gtest::{Log, Program as InnerProgram, System};
use nft_marketplace_io::*;
use non_fungible_token_io::{Collection, Config, InitNFT, NFTAction, NFTEvent};

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
            "../target/wasm32-unknown-unknown/release/non_fungible_token.opt.wasm",
        );

        assert!(!program
            .send(
                ADMIN,
                InitNFT {
                    royalties: Default::default(),
                    collection: Collection::default(),
                    config: Config {
                        authorized_minters: vec![ADMIN.into()],
                        ..Default::default()
                    }
                }
            )
            .main_failed());

        Self(program)
    }

    pub fn add_minter(&self, transaction_id: u64, to: u64) {
        assert!(self
            .0
            .send(
                ADMIN,
                NFTAction::AddMinter {
                    transaction_id,
                    minter_id: to.into()
                }
            )
            .contains(&Log::builder().payload(NFTEvent::MinterAdded {
                minter_id: to.into()
            })));
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

    pub fn meta_state(&self) -> NonFungibleTokenMetaState<'_> {
        NonFungibleTokenMetaState(&self.0)
    }
}

#[allow(dead_code)]
pub struct NonFungibleTokenMetaState<'a>(&'a InnerProgram<'a>);

impl NonFungibleTokenMetaState<'_> {
    pub fn owner_id(self, token_id: u64) -> MetaStateReply<ActorId> {
        MetaStateReply(self.token(token_id).0.owner_id)
    }

    pub fn token(self, _token_id: u64) -> MetaStateReply<Token> {
        unreachable!()
    }
}
