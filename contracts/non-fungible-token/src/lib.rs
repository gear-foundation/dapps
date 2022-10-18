#![no_std]

use gear_lib::non_fungible_token::{io::NFTTransfer, nft_core::*, state::*, token::*};
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
unsafe extern "C" fn init() {
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
unsafe extern "C" fn handle() {
    let action: NFTAction = msg::load().expect("Could not load NFTAction");
    let nft = CONTRACT.get_or_insert(Default::default());
    match action {
        NFTAction::Mint { token_metadata } => {
            msg::reply(NFTEvent::Transfer(MyNFTCore::mint(nft, token_metadata)), 0)
        }
        NFTAction::Burn { token_id } => {
            msg::reply(NFTEvent::Transfer(NFTCore::burn(nft, token_id)), 0)
        }
        NFTAction::Transfer { to, token_id } => {
            msg::reply(NFTEvent::Transfer(NFTCore::transfer(nft, &to, token_id)), 0)
        }
        NFTAction::TransferPayout {
            to,
            token_id,
            amount,
        } => msg::reply(
            NFTEvent::TransferPayout(NFTCore::transfer_payout(nft, &to, token_id, amount)),
            0,
        ),
        NFTAction::Approve { to, token_id } => {
            msg::reply(NFTEvent::Approval(NFTCore::approve(nft, &to, token_id)), 0)
        }
        NFTAction::Owner { token_id } => msg::reply(
            NFTEvent::Owner {
                owner: NFTCore::owner_of(nft, token_id),
                token_id,
            },
            0,
        ),
        NFTAction::IsApproved { to, token_id } => msg::reply(
            NFTEvent::IsApproved {
                to,
                token_id,
                approved: NFTCore::is_approved_to(nft, &to, token_id),
            },
            0,
        ),
        NFTAction::DelegatedApprove { message, signature } => msg::reply(
            NFTEvent::Approval(NFTCore::delegated_approve(nft, message, signature)),
            0,
        ),
    }
    .expect("Error during replying with `NFTEvent`");
}

#[no_mangle]
unsafe extern "C" fn meta_state() -> *mut [i32; 2] {
    let query: NFTQuery = msg::load().expect("failed to decode input argument");
    let nft = CONTRACT.get_or_insert(NFT::default());
    let encoded =
        NFTMetaState::proc_state(nft, query).expect("Error in reading NFT contract state");
    gstd::util::to_leak_ptr(encoded)
}

pub trait MyNFTCore: NFTCore {
    fn mint(&mut self, token_metadata: TokenMetadata) -> NFTTransfer;
}

impl MyNFTCore for NFT {
    fn mint(&mut self, token_metadata: TokenMetadata) -> NFTTransfer {
        let transfer = NFTCore::mint(self, &msg::source(), self.token_id, Some(token_metadata));
        self.token_id = self.token_id.saturating_add(U256::one());
        transfer
    }
}

gstd::metadata! {
    title: "NFT",
    init:
        input: InitNFT,
    handle:
        input: NFTAction,
        output: NFTEvent,
    state:
        input: NFTQuery,
        output: NFTQueryReply,
}
