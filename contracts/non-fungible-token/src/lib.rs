#![no_std]

use gear_lib::non_fungible_token::{io::NFTTransfer, nft_core::*, state::*, token::*};
use gear_lib_derive::{NFTCore, NFTMetaState, NFTStateKeeper};
use gstd::{exec, msg, prelude::*, ActorId};
use nft_io::*;
use primitive_types::{H256, U256};

const DELAY: u32 = 600_000;

#[derive(Debug, Default, NFTStateKeeper, NFTCore, NFTMetaState)]
pub struct NFT {
    #[NFTStateField]
    pub token: NFTState,
    pub token_id: TokenId,
    pub owner: ActorId,
    pub transactions: BTreeSet<H256>,
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
        NFTAction::Mint {
            transaction_id,
            token_metadata,
        } => {
            if !nft.transaction_made(transaction_id) {
                msg::reply(NFTEvent::Transfer(MyNFTCore::mint(nft, token_metadata)), 0)
                    .expect("Error during replying with `NFTEvent::Transfer`");
            }
        }
        NFTAction::Burn {
            transaction_id,
            token_id,
        } => {
            if !nft.transaction_made(transaction_id) {
                msg::reply(NFTEvent::Transfer(NFTCore::burn(nft, token_id)), 0)
                    .expect("Error during replying with `NFTEvent::Transfer`");
            }
        }
        NFTAction::Transfer {
            transaction_id,
            to,
            token_id,
        } => {
            if !nft.transaction_made(transaction_id) {
                msg::reply(NFTEvent::Transfer(NFTCore::transfer(nft, &to, token_id)), 0)
                    .expect("Error during replying with `NFTEvent::Transfer`");
            }
        }
        NFTAction::TransferPayout {
            transaction_id,
            to,
            token_id,
            amount,
        } => {
            if !nft.transaction_made(transaction_id) {
                msg::reply(
                    NFTEvent::TransferPayout(NFTCore::transfer_payout(nft, &to, token_id, amount)),
                    0,
                )
                .expect("Error during replying with `NFTEvent::TransferPayout`");
            }
        }
        NFTAction::Approve {
            transaction_id,
            to,
            token_id,
        } => {
            if !nft.transaction_made(transaction_id) {
                msg::reply(NFTEvent::Approval(NFTCore::approve(nft, &to, token_id)), 0)
                    .expect("Error during replying with `NFTEvent::Approval`");
            }
        }
        NFTAction::Owner { token_id } => {
            msg::reply(
                NFTEvent::Owner {
                    owner: NFTCore::owner_of(nft, token_id),
                    token_id,
                },
                0,
            )
            .expect("Error during replying with `NFTEvent::Owner`");
        }
        NFTAction::IsApproved { to, token_id } => {
            msg::reply(
                NFTEvent::IsApproved {
                    to,
                    token_id,
                    approved: NFTCore::is_approved_to(nft, &to, token_id),
                },
                0,
            )
            .expect("Error during replying with `NFTEvent::IsApproved`");
        }
        NFTAction::DelegatedApprove {
            transaction_id,
            message,
            signature,
        } => {
            if !nft.transaction_made(transaction_id) {
                msg::reply(
                    NFTEvent::Approval(NFTCore::delegated_approve(nft, message, signature)),
                    0,
                )
                .expect("Error during replying with `NFTEvent::Approval`");
            }
        }
        NFTAction::Clear { transaction_hash } => nft.clear(transaction_hash),
    };
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

impl NFT {
    fn transaction_made(&mut self, transaction_id: u64) -> bool {
        let transaction_hash = get_hash(&msg::source(), transaction_id);
        send_delayed_clear(transaction_hash);
        if self.transactions.insert(transaction_hash) {
            false
        } else {
            msg::reply(NFTEvent::TransactionMade, 0)
                .expect("Error during replying with `NFTEvent::TransactionMade`");
            true
        }
    }

    fn clear(&mut self, transaction_hash: H256) {
        assert_eq!(
            msg::source(),
            exec::program_id(),
            "Not allowed to creal transactions"
        );
        self.transactions.remove(&transaction_hash);
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

pub fn get_hash(account: &ActorId, transaction_id: u64) -> H256 {
    let account: [u8; 32] = (*account).into();
    let transaction_id = transaction_id.to_be_bytes();
    sp_core_hashing::blake2_256(&[account.as_slice(), transaction_id.as_slice()].concat()).into()
}

fn send_delayed_clear(transaction_hash: H256) {
    msg::send_delayed(
        exec::program_id(),
        NFTAction::Clear { transaction_hash },
        0,
        DELAY,
    )
    .expect("Error in sending a delayled message `FTStorageAction::Clear`");
}
