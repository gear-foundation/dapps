#![no_std]

use gear_lib::non_fungible_token::{nft_core::*, state::*, token::*};
use gear_lib_derive::{NFTCore, NFTMetaState, NFTStateKeeper};
use gstd::{msg, prelude::*, ActorId};
use nft_io::*;
use primitive_types::U256;

#[derive(Debug, Default, NFTStateKeeper, NFTCore, NFTMetaState)]
pub struct NFT {
    #[NFTStateField]
    pub token: NFTState,
    pub token_id: TokenId,
    pub owner: ActorId,
}

static mut CONTRACT: Option<NFT> = None;

#[no_mangle]
pub unsafe extern "C" fn init() {
    let config: InitNFT = msg::load().expect("Unable to decode InitNFT");
    if config.royalties.is_some() {
        config.royalties.as_ref().unwrap().validate();
    }
    let nft = NFT {
        token: NFTState {
            name: config.name,
            symbol: config.symbol,
            base_uri: config.base_uri,
            royalties: config.royalties,
            ..Default::default()
        },
        owner: msg::source(),
        ..Default::default()
    };
    CONTRACT = Some(nft);
}

#[no_mangle]
pub unsafe extern "C" fn handle() {
    let action: NFTAction = msg::load().expect("Could not load NFTAction");
    let nft = CONTRACT.get_or_insert(Default::default());
    match action {
        NFTAction::Mint { token_metadata } => MyNFTCore::mint(nft, token_metadata),
        NFTAction::Burn { token_id } => NFTCore::burn(nft, token_id),
        NFTAction::Transfer { to, token_id } => NFTCore::transfer(nft, &to, token_id),
        NFTAction::TransferPayout {
            to,
            token_id,
            amount,
        } => NFTCore::transfer_payout(nft, &to, token_id, amount),
        NFTAction::Approve { to, token_id } => NFTCore::approve(nft, &to, token_id),
        NFTAction::Owner { token_id } => NFTCore::owner_of(nft, token_id),
        NFTAction::IsApproved { to, token_id } => NFTCore::is_approved_to(nft, &to, token_id),
        NFTAction::DelegatedApprove { message, signature } => {
            NFTCore::delegated_approve(nft, message, signature)
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn meta_state() -> *mut [i32; 2] {
    let query: NFTQuery = msg::load().expect("failed to decode input argument");
    let nft = CONTRACT.get_or_insert(NFT::default());
    let encoded =
        NFTMetaState::proc_state(nft, query).expect("Error in reading NFT contract state");
    gstd::util::to_leak_ptr(encoded)
}

pub trait MyNFTCore: NFTCore {
    fn mint(&mut self, token_metadata: TokenMetadata);
}

impl MyNFTCore for NFT {
    fn mint(&mut self, token_metadata: TokenMetadata) {
        NFTCore::mint(self, &msg::source(), self.token_id, Some(token_metadata));
        self.token_id = self.token_id.saturating_add(U256::one());
    }
}

gstd::metadata! {
    title: "NFT",
    init:
        input: InitNFT,
    handle:
        input: NFTAction,
        output: Vec<u8>,
    state:
        input: NFTQuery,
        output: NFTQueryReply,
}
