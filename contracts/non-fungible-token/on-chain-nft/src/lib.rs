#![no_std]

use gear_lib::non_fungible_token::{nft_core::*, state::*, token::*};
use gear_lib_derive::{NFTCore, NFTMetaState, NFTStateKeeper};
use gstd::{msg, prelude::*, ActorId};
use on_chain_nft_io::*;
use primitive_types::U256;

#[derive(Debug, Default, NFTStateKeeper, NFTCore, NFTMetaState)]
pub struct OnChainNFT {
    #[NFTStateField]
    pub token: NFTState,
    pub token_id: TokenId,
    pub owner: ActorId,
    pub base_image: String,
    pub layers: BTreeMap<LayerId, Vec<String>>,
    pub nfts: BTreeMap<TokenId, Vec<ItemId>>,
    pub nfts_existence: BTreeSet<String>,
}

static mut CONTRACT: Option<OnChainNFT> = None;

#[no_mangle]
pub unsafe extern "C" fn init() {
    let config: InitOnChainNFT = msg::load().expect("Unable to decode InitOnChainNFT");
    let nft = OnChainNFT {
        token: NFTState {
            name: config.name,
            symbol: config.symbol,
            base_uri: config.base_uri,
            ..Default::default()
        },
        owner: msg::source(),
        base_image: config.base_image,
        layers: config.layers,
        ..Default::default()
    };
    CONTRACT = Some(nft);
}

#[no_mangle]
pub unsafe extern "C" fn handle() {
    let action: OnChainNFTAction = msg::load().expect("Could not load OnChainNFTAction");
    let nft = CONTRACT.get_or_insert(Default::default());
    match action {
        OnChainNFTAction::Mint {
            description,
            token_metadata,
        } => OnChainNFTCore::mint(nft, description, token_metadata),
        OnChainNFTAction::Burn { token_id } => OnChainNFTCore::burn(nft, token_id),
        OnChainNFTAction::Transfer { to, token_id } => NFTCore::transfer(nft, &to, token_id),
        OnChainNFTAction::TransferPayout {
            to,
            token_id,
            amount,
        } => NFTCore::transfer_payout(nft, &to, token_id, amount),
        OnChainNFTAction::Approve { to, token_id } => NFTCore::approve(nft, &to, token_id),
    }
}

#[no_mangle]
pub unsafe extern "C" fn meta_state() -> *mut [i32; 2] {
    let query: OnChainNFTQuery = msg::load().expect("failed to decode input argument");
    let nft = CONTRACT.get_or_insert(OnChainNFT::default());
    match query {
        OnChainNFTQuery::TokenURI { token_id } => {
            let encoded = OnChainNFTCore::token_uri(nft, token_id)
                .expect("Error in reading OnChainNFT contract state");
            gstd::util::to_leak_ptr(encoded)
        }
        OnChainNFTQuery::Base(query) => {
            let encoded =
                NFTMetaState::proc_state(nft, query).expect("Error in reading NFT contract state");
            gstd::util::to_leak_ptr(encoded)
        }
    }
}

pub trait OnChainNFTCore: NFTCore {
    fn mint(&mut self, description: Vec<ItemId>, metadata: TokenMetadata);
    fn burn(&mut self, token_id: TokenId);
    fn token_uri(&mut self, token_id: TokenId) -> Option<Vec<u8>>;
}

impl OnChainNFTCore for OnChainNFT {
    /// Mint an NFT on chain.
    /// `description` - is the vector of ids ,
    ///  where each index represents a layer id, and element represents a layer item id.
    /// `metadata` - is the default metadata provided by gear-lib.
    fn mint(&mut self, description: Vec<ItemId>, metadata: TokenMetadata) {
        // precheck if the layers actually exist
        for (layer_id, layer_item_id) in description.iter().enumerate() {
            if layer_id > self.layers.len() {
                panic!("No such layer");
            }
            if *layer_item_id
                > self
                    .layers
                    .get(&(layer_id as u128))
                    .expect("No such layer")
                    .len() as u128
            {
                panic!("No such item");
            }
        }

        // also check if description has all layers provided
        if description.len() != self.layers.len() {
            panic!("The number of layers must be equal to the number of layers in the contract");
        }

        // precheck if there is already an nft with such description
        let key = description
            .clone()
            .into_iter()
            .map(|i| i.to_string())
            .collect::<String>();
        if self.nfts_existence.contains(&key) {
            panic!("Such nft already exists");
        }
        self.nfts_existence.insert(key);
        NFTCore::mint(self, &msg::source(), self.token_id, Some(metadata));
        self.nfts.insert(self.token_id, description);
        self.token_id = self.token_id.saturating_add(U256::one());
    }

    /// Burns an NFT.
    /// `token_id` - is the id of a token. MUST exist.
    fn burn(&mut self, token_id: TokenId) {
        NFTCore::burn(self, token_id);
        let key = self
            .nfts
            .get(&token_id)
            .expect("No such token")
            .iter()
            .map(|i| i.to_string())
            .collect::<String>();
        self.nfts.remove(&token_id);
        self.nfts_existence.remove(&key);
    }

    /// Returns token information - metadata and all the content of all the layers for the NFT.
    /// `token_id` - is the id of a token. MUST exist.
    fn token_uri(&mut self, token_id: TokenId) -> Option<Vec<u8>> {
        let mut metadata = TokenMetadata::default();
        if let Some(Some(mtd)) = self.token.token_metadata_by_id.get(&token_id) {
            metadata = mtd.clone();
        }
        // construct media
        let mut content: Vec<String> = Vec::new();
        // check if exists
        let nft = self.nfts.get(&token_id).expect("No such nft");
        for (layer_id, layer_item_id) in nft.iter().enumerate() {
            let layer_content = self
                .layers
                .get(&(layer_id as u128))
                .expect("No such layer")
                .iter()
                .nth(*layer_item_id as usize)
                .expect("No such layer item");
            content.push(layer_content.clone());
        }
        Some(TokenURI { metadata, content }.encode())
    }
}

gstd::metadata! {
    title: "OnChainNFT",
    init:
        input: InitOnChainNFT,
    handle:
        input: OnChainNFTAction,
        output: Vec<u8>,
    state:
        input: OnChainNFTQuery,
        output: Vec<u8>,
}
