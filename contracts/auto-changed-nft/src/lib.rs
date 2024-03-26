#![no_std]

use auto_changed_nft_io::*;
use gear_lib_derive::{NFTCore, NFTMetaState, NFTStateKeeper};
use gear_lib_old::non_fungible_token::{io::NFTTransfer, nft_core::*, state::*, token::*};
use gstd::{
    collections::HashMap,
    exec::{self},
    msg::{self, send_delayed, send_delayed_from_reservation},
    prelude::*,
    ActorId, ReservationId,
};
use primitive_types::{H256, U256};

include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

const RESERVATION_AMOUNT: u64 = 240_000_000_000;
const GAS_FOR_UPDATE: u64 = 4_000_000_000;

#[derive(Debug, Default, NFTStateKeeper, NFTCore, NFTMetaState)]
pub struct AutoChangedNft {
    #[NFTStateField]
    pub token: NFTState,
    pub token_id: TokenId,
    pub owner: ActorId,
    pub transactions: HashMap<H256, NFTEvent>,
    pub collection: Collection,
    pub config: Config,
    pub urls: HashMap<TokenId, Vec<String>>,
    pub rest_updates_count: u32,
    pub update_period: u32,
}

static mut CONTRACT: Option<AutoChangedNft> = None;
static mut RESERVATION: Vec<ReservationId> = vec![];

#[no_mangle]
unsafe extern fn init() {
    let config: InitNFT = msg::load().expect("Unable to decode InitNFT");
    if config.royalties.is_some() {
        config.royalties.as_ref().expect("Unable to g").validate();
    }
    let nft = AutoChangedNft {
        token: NFTState {
            name: config.collection.name.clone(),
            royalties: config.royalties,
            ..Default::default()
        },
        collection: config.collection,
        config: config.config,
        owner: msg::source(),
        ..Default::default()
    };

    CONTRACT = Some(nft);
}

#[no_mangle]
extern fn handle() {
    let action: NFTAction = msg::load().expect("Could not load NFTAction");
    let nft = unsafe { CONTRACT.get_or_insert(Default::default()) };
    match action {
        NFTAction::Mint {
            transaction_id,
            token_metadata,
        } => {
            nft.check_config();
            msg::reply(
                nft.process_transaction(transaction_id, |nft| {
                    NFTEvent::Transfer(MyNFTCore::mint(nft, token_metadata))
                }),
                0,
            )
            .expect("Error during replying with `NFTEvent::Transfer`");
        }
        NFTAction::Burn {
            transaction_id,
            token_id,
        } => {
            msg::reply(
                nft.process_transaction(transaction_id, |nft| {
                    NFTEvent::Transfer(NFTCore::burn(nft, token_id))
                }),
                0,
            )
            .expect("Error during replying with `NFTEvent::Transfer`");
        }
        NFTAction::Transfer {
            transaction_id,
            to,
            token_id,
        } => {
            msg::reply(
                nft.process_transaction(transaction_id, |nft| {
                    NFTEvent::Transfer(NFTCore::transfer(nft, &to, token_id))
                }),
                0,
            )
            .expect("Error during replying with `NFTEvent::Transfer`");
        }
        NFTAction::TransferPayout {
            transaction_id,
            to,
            token_id,
            amount,
        } => {
            msg::reply(
                nft.process_transaction(transaction_id, |nft| {
                    NFTEvent::TransferPayout(NFTCore::transfer_payout(nft, &to, token_id, amount))
                }),
                0,
            )
            .expect("Error during replying with `NFTEvent::TransferPayout`");
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
            msg::reply(
                nft.process_transaction(transaction_id, |nft| {
                    NFTEvent::Approval(NFTCore::approve(nft, &to, token_id))
                }),
                0,
            )
            .expect("Error during replying with `NFTEvent::Approval`");
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
            msg::reply(
                nft.process_transaction(transaction_id, |nft| {
                    NFTEvent::Approval(NFTCore::delegated_approve(nft, message, signature))
                }),
                0,
            )
            .expect("Error during replying with `NFTEvent::Approval`");
        }
        NFTAction::Clear { transaction_hash } => nft.clear(transaction_hash),
        NFTAction::AddMinter {
            transaction_id,
            minter_id,
        } => {
            nft.check_config();
            msg::reply(
                nft.process_transaction(transaction_id, |nft| {
                    nft.config.authorized_minters.push(minter_id);
                    NFTEvent::MinterAdded { minter_id }
                }),
                0,
            )
            .expect("Error during replying with `NFTEvent::Approval`");
        }
        NFTAction::ReserveGas => nft.reserve_gas(),
        NFTAction::AddMedia { token_id, media } => {
            if let Some(urls) = nft.urls.get_mut(&token_id) {
                urls.push(media);
            } else {
                nft.urls.insert(token_id, vec![media]);
            }
        }
        NFTAction::Update {
            rest_updates_count,
            token_ids,
        } => {
            nft.rest_updates_count = rest_updates_count - 1;
            nft.update_media(&token_ids);
            if nft.rest_updates_count == 0 {
                return;
            }
            let action = NFTAction::Update {
                rest_updates_count: nft.rest_updates_count,
                token_ids,
            };
            let gas_available = exec::gas_available();
            if gas_available <= GAS_FOR_UPDATE {
                let reservations: &mut Vec<ReservationId> = unsafe { RESERVATION.as_mut() };
                let reservation_id = reservations.pop().expect("Need more gas");
                send_delayed_from_reservation(
                    reservation_id,
                    exec::program_id(),
                    action,
                    0,
                    nft.update_period,
                )
                .expect("Can't send delayed from reservation");
            } else {
                send_delayed(exec::program_id(), action, 0, nft.update_period)
                    .expect("Can't send delayed");
            }
        }
        NFTAction::StartAutoChanging {
            updates_count,
            update_period,
            token_ids,
        } => {
            nft.rest_updates_count = updates_count;
            nft.update_period = update_period;

            nft.update_media(&token_ids);

            let payload = NFTAction::Update {
                rest_updates_count: updates_count,
                token_ids: token_ids.clone(),
            };
            send_delayed(exec::program_id(), &payload, 0, update_period)
                .expect("Can't send delayed");
            nft.reserve_gas();
        }
    };
}

pub trait MyNFTCore: NFTCore {
    fn mint(&mut self, token_metadata: TokenMetadata) -> NFTTransfer;
}

impl MyNFTCore for AutoChangedNft {
    fn mint(&mut self, token_metadata: TokenMetadata) -> NFTTransfer {
        let transfer = NFTCore::mint(self, &msg::source(), self.token_id, Some(token_metadata));
        self.token_id = self.token_id.saturating_add(U256::one());
        transfer
    }
}

impl AutoChangedNft {
    fn process_transaction(
        &mut self,
        transaction_id: u64,
        action: impl FnOnce(&mut AutoChangedNft) -> NFTEvent,
    ) -> NFTEvent {
        let transaction_hash = get_hash(&msg::source(), transaction_id);

        if let Some(nft_event) = self.transactions.get(&transaction_hash) {
            nft_event.clone()
        } else {
            let nft_event = action(self);

            self.transactions
                .insert(transaction_hash, nft_event.clone());

            nft_event
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
    fn check_config(&self) {
        if let Some(max_mint_count) = self.config.max_mint_count {
            if max_mint_count <= self.token.token_metadata_by_id.len() as u32 {
                panic!(
                    "Mint impossible because max minting count {} limit exceeded",
                    max_mint_count
                );
            }
        }

        let current_minter = msg::source();
        let is_authorized_minter = self
            .config
            .authorized_minters
            .iter()
            .any(|authorized_minter| authorized_minter.eq(&current_minter));

        if !is_authorized_minter {
            panic!(
                "Current minter {:?} is not authorized at initialization",
                current_minter
            );
        }
    }

    fn update_media(&mut self, token_ids: &Vec<TokenId>) {
        for token_id in token_ids {
            if let Some(Some(meta)) = self.token.token_metadata_by_id.get_mut(token_id) {
                let urls_for_token = &self.urls[token_id];
                let index = self.rest_updates_count as usize % urls_for_token.len();
                let media = urls_for_token[index].clone();
                meta.media = media
            }
        }
    }
    fn reserve_gas(&self) {
        let reservations = unsafe { &mut RESERVATION };
        let reservation_id =
            ReservationId::reserve(RESERVATION_AMOUNT, 600).expect("reservation across executions");
        reservations.push(reservation_id);
    }
}

#[no_mangle]
extern fn state() {
    let contract = unsafe { CONTRACT.take().expect("Unexpected error in taking state") };
    msg::reply::<State>(contract.into(), 0)
        .expect("Failed to encode or reply with `IoNFT` from `state()`");
}

pub fn get_hash(account: &ActorId, transaction_id: u64) -> H256 {
    let account: [u8; 32] = (*account).into();
    let transaction_id = transaction_id.to_be_bytes();
    sp_core_hashing::blake2_256(&[account.as_slice(), transaction_id.as_slice()].concat()).into()
}

impl From<AutoChangedNft> for IoNFT {
    fn from(value: AutoChangedNft) -> Self {
        let AutoChangedNft {
            token,
            token_id,
            owner,
            transactions,
            urls,
            rest_updates_count: update_number,
            ..
        } = value;

        let transactions = transactions
            .iter()
            .map(|(key, event)| (*key, event.clone()))
            .collect();
        let urls = urls
            .iter()
            .map(|(token_id, urls)| (*token_id, urls.clone()))
            .collect();
        Self {
            token: (&token).into(),
            token_id,
            owner,
            transactions,
            urls,
            update_number,
        }
    }
}

impl From<AutoChangedNft> for State {
    fn from(value: AutoChangedNft) -> Self {
        let AutoChangedNft {
            token,
            token_id,
            collection,
            ..
        } = value;

        let owners = token
            .owner_by_id
            .iter()
            .map(|(hash, actor_id)| (*actor_id, *hash))
            .collect();

        let token_metadata_by_id = token
            .token_metadata_by_id
            .iter()
            .map(|(id, metadata)| {
                let metadata = metadata.as_ref().unwrap();
                let nft = Nft {
                    owner: *token.owner_by_id.get(id).unwrap(),
                    name: metadata.name.clone(),
                    description: metadata.description.clone(),
                    media_url: metadata.media.clone(),
                    attrib_url: metadata.reference.clone(),
                };
                (*id, nft)
            })
            .collect();

        Self {
            tokens: token_metadata_by_id,
            collection,
            nonce: token_id,
            owners,
        }
    }
}
