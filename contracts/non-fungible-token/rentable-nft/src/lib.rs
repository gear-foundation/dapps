#![no_std]

use core::time::Duration;
use gear_lib::non_fungible_token::{
    nft_core::NFTCore,
    state::{NFTMetaState, NFTQuery, NFTQueryReply, NFTState, NFTStateKeeper},
    token::TokenId,
};
use gear_lib_derive::{NFTCore, NFTMetaState, NFTStateKeeper};
use gstd::{exec, exec::block_timestamp, msg, prelude::*, ActorId, BTreeSet, ToString};
use hashbrown::HashMap;
use io::{InitNFT, NFTAction, NFTEvent};
use primitive_types::H256;

const DELAY: u32 = 600_000;

#[derive(Debug, Default, NFTStateKeeper, NFTCore, NFTMetaState)]
pub struct NFT {
    #[NFTStateField]
    pub token: NFTState,
    pub token_id: TokenId,
    pub owner: ActorId,
    pub transactions: BTreeSet<H256>,
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
            "Not allowed to clear transactions"
        );
        self.transactions.remove(&transaction_hash);
    }
}

#[derive(Debug, Default)]
pub struct UserInfo {
    pub address: ActorId, // address of user role
    pub expires: u64,     // unix timestamp
}

#[derive(Debug, Default)]
pub struct RentableNFT {
    pub nft: NFT,
    pub users_info: HashMap<TokenId, UserInfo>,
}

static mut CONTRACT: Option<RentableNFT> = None;

#[no_mangle]
unsafe extern "C" fn handle() {
    let action: NFTAction = msg::load().expect("Could not load NFTAction");
    let rentable_nft = CONTRACT.get_or_insert(Default::default());
    let nft = &mut rentable_nft.nft;
    match action {
        NFTAction::Mint {
            transaction_id,
            token_metadata,
        } => {
            if !nft.transaction_made(transaction_id) {
                msg::reply(
                    NFTEvent::Transfer(NFTCore::mint(
                        nft,
                        &msg::source(),
                        nft.token_id,
                        Some(token_metadata),
                    )),
                    0,
                )
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
        NFTAction::NFTPayout { owner, amount } => {
            msg::reply(
                NFTEvent::NFTPayout(NFTCore::nft_payout(nft, &owner, amount)),
                0,
            )
            .expect("Error during replying with `NFTEvent::NFTPayout`");
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
        NFTAction::SetUser {
            token_id,
            address,
            duration_in_secs,
            transaction_id,
        } => {
            let now = Duration::from_millis(exec::block_timestamp());
            let duration = Duration::from_secs(duration_in_secs);
            let expires = now + duration;
            let expires: u64 = expires.as_secs();

            if let Some(transaction_id) = transaction_id {
                if !rentable_nft.nft.transaction_made(transaction_id) {
                    msg::reply(
                        NFTEvent::UpdateUser {
                            token_id,
                            address,
                            expires,
                        }
                        .encode(),
                        0,
                    )
                    .expect("Error during replying with `NFTEvent::SetUser`");
                }
            }
            rentable_nft.set_user(address, token_id, expires, transaction_id);

            let payload = NFTEvent::UpdateUser {
                token_id,
                address,
                expires,
            };
            msg::reply(payload, 0).expect("Error during replying with `NFTEvent::SetUser`");
        }
        NFTAction::UserOf { token_id } => {
            let address = rentable_nft.user_of(&token_id);
            let payload = NFTEvent::UserOf { address };
            msg::reply(payload, 0).expect("Error during replying with `NFTEvent::UserOf`");
        }
        NFTAction::UserExpires { token_id } => {
            let expires = rentable_nft.user_expires(&token_id);
            let payload = NFTEvent::UserExpires { expires };
            msg::reply(payload, 0).expect("Error during replying with `NFTEvent::UserExpires`");
        }
    };
}

gstd::metadata! {
    title: "RentableNFT",
    init:
        input: InitNFT,
    handle:
        input: NFTAction,
        output: NFTEvent,
    state:
        input: NFTQuery,
        output: NFTQueryReply,
}

impl RentableNFT {
    fn set_user(
        &mut self,
        address: ActorId,
        token_id: TokenId,
        expires: u64,
        _transaction_id: Option<u64>,
    ) {
        self.nft.assert_zero_address(&address);

        let owner = &self.nft.owner;

        // is Approved or Owner
        if !self.nft.is_approved_to(&msg::source(), token_id) {
            self.nft.assert_owner(owner);
        }

        self.users_info
            .entry(token_id)
            .and_modify(|user_info| user_info.expires = expires)
            .or_insert(UserInfo { address, expires });
    }

    fn user_of(&self, token_id: &TokenId) -> ActorId {
        if let Some(user_info) = self.users_info.get(token_id) {
            if user_info.expires < block_timestamp() {
                return user_info.address;
            }
        }

        ActorId::zero()
    }

    fn user_expires(&self, token_id: &TokenId) -> u64 {
        if let Some(user_info) = self.users_info.get(token_id) {
            user_info.expires
        } else {
            0u64
        }
    }
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
    .expect("Error in sending a delayed message `FTStorageAction::Clear`");
}
