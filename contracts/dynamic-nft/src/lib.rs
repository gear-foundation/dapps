#![no_std]

use dynamic_nft_io::*;
use gear_lib_derive::{NFTCore, NFTMetaState, NFTStateKeeper};
use gear_lib_old::non_fungible_token::{io::NFTTransfer, nft_core::*, state::*, token::*};
use gstd::{
    collections::HashMap,
    exec::{self},
    msg,
    prelude::*,
    ActorId,
};
use primitive_types::{H256, U256};

include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

#[derive(Debug, Default, NFTStateKeeper, NFTCore, NFTMetaState)]
pub struct DynamicNft {
    #[NFTStateField]
    pub token: NFTState,
    pub token_id: TokenId,
    pub owner: ActorId,
    pub transactions: HashMap<H256, NFTEvent>,
    pub collection: Collection,
    pub config: Config,
    pub dynamic_data: Vec<u8>,
}

static mut CONTRACT: Option<DynamicNft> = None;

#[no_mangle]
unsafe extern fn init() {
    let config: InitNFT = msg::load().expect("Unable to decode InitNFT");
    if config.royalties.is_some() {
        config.royalties.as_ref().expect("Unable to g").validate();
    }
    let nft = DynamicNft {
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
unsafe extern fn handle() {
    let action: NFTAction = msg::load().expect("Could not load NFTAction");
    let nft = CONTRACT.get_or_insert(Default::default());
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
        NFTAction::UpdateDynamicData {
            transaction_id,
            data,
        } => {
            let payload = nft.process_transaction(transaction_id, |nft| {
                let data_hash = H256::from(sp_core_hashing::blake2_256(&data));
                nft.dynamic_data = data;

                NFTEvent::Updated { data_hash }
            });
            msg::reply(payload, 0).expect("Error during replying with `NFTEvent::Updated`");
        }
    };
}

pub trait MyNFTCore: NFTCore {
    fn mint(&mut self, token_metadata: TokenMetadata) -> NFTTransfer;
}

impl MyNFTCore for DynamicNft {
    fn mint(&mut self, token_metadata: TokenMetadata) -> NFTTransfer {
        let transfer = NFTCore::mint(self, &msg::source(), self.token_id, Some(token_metadata));
        self.token_id = self.token_id.saturating_add(U256::one());
        transfer
    }
}

impl DynamicNft {
    fn process_transaction(
        &mut self,
        transaction_id: u64,
        action: impl FnOnce(&mut DynamicNft) -> NFTEvent,
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
}

#[no_mangle]
extern fn state() {
    let contract = unsafe { CONTRACT.take().expect("Unexpected error in taking state") };
    msg::reply::<IoNFT>(contract.into(), 0)
        .expect("Failed to encode or reply with `IoNFT` from `state()`");
}

pub fn get_hash(account: &ActorId, transaction_id: u64) -> H256 {
    let account: [u8; 32] = (*account).into();
    let transaction_id = transaction_id.to_be_bytes();
    sp_core_hashing::blake2_256(&[account.as_slice(), transaction_id.as_slice()].concat()).into()
}

impl From<DynamicNft> for IoNFT {
    fn from(value: DynamicNft) -> Self {
        let DynamicNft {
            token,
            token_id,
            owner,
            transactions,
            dynamic_data,
            ..
        } = value;

        let transactions = transactions
            .iter()
            .map(|(key, event)| (*key, event.clone()))
            .collect();

        Self {
            token: (&token).into(),
            token_id,
            owner,
            transactions,
            dynamic_data,
        }
    }
}
