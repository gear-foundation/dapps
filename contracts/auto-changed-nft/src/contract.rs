use auto_changed_nft_io::{Collection, InitNFT2, IoNFT, NFTAction, NFTEvent, NFTMetadata, Nft2, NFTState2};
use gear_lib::non_fungible_token::{io::NFTTransfer, nft_core::*, state::*, token::*};
use gear_lib_derive::{NFTCore, NFTMetaState, NFTStateKeeper};
use gmeta::Metadata;
use gstd::{
    errors::Result as GstdResult,
    exec::{self},
    msg::{self, send_delayed, send_delayed_from_reservation},
    prelude::*,
    ActorId, MessageId, ReservationId,
};
use hashbrown::HashMap;
use primitive_types::{H256, U256};

const RESERVATION_AMOUNT: u64 = 240_000_000_000;
const GAS_FOR_UPDATE: u64 = 4_000_000_000;

#[derive(Debug, Default, NFTStateKeeper, NFTCore, NFTMetaState)]
pub struct AutoChangedNft {
    #[NFTStateField]
    pub token: NFTState,
    pub token_id: TokenId,
    pub owner: ActorId,
    pub transactions: HashMap<H256, NFTEvent>,
    pub urls: HashMap<TokenId, Vec<String>>,
    pub rest_updates_count: u32,
    pub update_period: u32,
    pub collection: Collection,
}

static mut CONTRACT: Option<AutoChangedNft> = None;
static mut RESERVATION: Vec<ReservationId> = vec![];

#[no_mangle]
unsafe extern "C" fn init() {
    let config: InitNFT2 = msg::load().expect("Unable to decode InitNFT");
    let nft = AutoChangedNft {
        token: NFTState {
            name: config.collection.name.clone(),
            symbol: "".to_string(),
            base_uri: "".to_string(),
            royalties: None,
            ..Default::default()
        },
        collection: config.collection,
        owner: msg::source(),
        ..Default::default()
    };

    CONTRACT = Some(nft);
}

#[no_mangle]
unsafe extern "C" fn handle() {
    let action: NFTAction = msg::load().expect("Could not load NFTAction");
    let nft = CONTRACT.get_or_insert(Default::default());
    gstd::debug!("AZOYAN NFTAction: {:?}", &action);
    match action {
        NFTAction::Mint {
            transaction_id,
            token_metadata,
        } => {
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
        NFTAction::ReserveGas => nft.reserve_gas(),
        NFTAction::Clear { transaction_hash } => nft.clear(transaction_hash),
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
            gstd::debug!(
                "AZOYAN Update rest_updates_count: {}, token_ids: {:?}",
                rest_updates_count,
                token_ids
            );
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
            gstd::debug!("AZOYAN Update. gas_available: {}", gas_available);
            if gas_available <= GAS_FOR_UPDATE {
                let reservations = unsafe { &mut RESERVATION };
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
            // let gas1 = exec::gas_available();
            let message_id = send_delayed(exec::program_id(), &payload, 0, update_period)
                .expect("Can't send delayed");
            nft.reserve_gas();
            // let gas2 = exec::gas_available();
            gstd::debug!(
                "AZOYAN send_delayed payload: message_id: {:?}, {:?}, update_period: {} token_ids: {:?}",
                message_id, payload,
                update_period,
                token_ids
            );
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

    fn update_media(&mut self, token_ids: &Vec<TokenId>) {
        for token_id in token_ids {
            if let Some(Some(meta)) = self.token.token_metadata_by_id.get_mut(token_id) {
                let urls_for_token = &self.urls[token_id];
                let index = self.rest_updates_count as usize % urls_for_token.len();
                let media = urls_for_token[index].clone();
                gstd::debug!(
                    "AZOYAN update_media(): urls.len(): {}, token_id: {}, index: {}, media: {}",
                    urls_for_token.len(),
                    token_id,
                    index,
                    media
                );
                meta.media = media
            }
        }
    }
    fn reserve_gas(&self) {
        let reservations = unsafe { &mut RESERVATION };
        let reservation_id =
            ReservationId::reserve(RESERVATION_AMOUNT, 600).expect("reservation across executions");
        reservations.push(reservation_id);
        // msg::reply(NFTEvent::GasReserved, 0).expect("");
    }
}

#[no_mangle]
extern "C" fn metahash() {
    let metahash: [u8; 32] = include!("../.metahash");
    reply(metahash).expect("Failed to encode or reply with `[u8; 32]` from `metahash()`");
}

fn static_mut_state() -> &'static AutoChangedNft {
    unsafe { CONTRACT.get_or_insert(Default::default()) }
}

fn common_state() -> <NFTMetadata as Metadata>::State {
    static_mut_state().into()
}

#[no_mangle]
extern "C" fn state() {
    reply(common_state())
        .expect("Failed to encode or reply with `<NFTMetadata as Metadata>::State` from `state()`");
}

fn reply(payload: impl Encode) -> GstdResult<MessageId> {
    msg::reply(payload, 0)
}

pub fn get_hash(account: &ActorId, transaction_id: u64) -> H256 {
    let account: [u8; 32] = (*account).into();
    let transaction_id = transaction_id.to_be_bytes();
    sp_core_hashing::blake2_256(&[account.as_slice(), transaction_id.as_slice()].concat()).into()
}

impl From<&AutoChangedNft> for IoNFT {
    fn from(value: &AutoChangedNft) -> Self {
        let AutoChangedNft {
            token,
            token_id,
            owner,
            transactions,
            urls,
            rest_updates_count: update_number,
            update_period: _,
            collection: _,
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
            token: token.into(),
            token_id: *token_id,
            owner: *owner,
            transactions,
            urls,
            update_number: *update_number,
        }
    }
}

impl From<&AutoChangedNft> for NFTState2 {
    fn from(value: &AutoChangedNft) -> Self {
        let AutoChangedNft {
            token,
            token_id,
            owner,
            transactions,
            urls,
            rest_updates_count: update_number,
            update_period: _,
            collection,
        } = value;

        // let transactions = transactions
        //     .iter()
        //     .map(|(key, event)| (*key, event.clone()))
        //     .collect();
        // let urls = urls
        //     .iter()
        //     .map(|(token_id, urls)| (*token_id, urls.clone()))
        //     .collect();

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
                let nft = Nft2 {
                    owner: token.owner_by_id.get(id).unwrap().clone(),
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
            collection: collection.clone(),
            nonce: token_id.clone(),
            owners,
        }
    }
}
