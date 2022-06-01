#![no_std]

use codec::{Decode, Encode};
use gstd::{exec, msg, prelude::*, ActorId};
pub use market_io::*;
use primitive_types::{H256, U256};
use scale_info::TypeInfo;
pub mod nft_messages;
use nft_messages::*;
pub mod auction;
pub mod offers;
pub mod payment;
pub mod sale;

pub type ContractAndTokenId = String;

#[derive(Debug, Default, Encode, Decode, TypeInfo)]
pub struct Market {
    pub owner_id: ActorId,
    pub treasury_id: ActorId,
    pub treasury_fee: u128,
    pub items: BTreeMap<ContractAndTokenId, Item>,
    pub approved_nft_contracts: Vec<ActorId>,
    pub approved_ft_contracts: Vec<ActorId>,
    pub offer_history_length: u8,
}

static mut MARKET: Option<Market> = None;

impl Market {
    /// Adds nft contract addresses that can be listed on marketplace
    /// Requirements:
    /// Only admin can add approved nft addresses
    /// Arguments:
    /// * `nft_contract_id`: the NFT contract address
    fn add_nft_contract(&mut self, nft_contract_id: &ActorId) {
        self.check_owner();
        self.approved_nft_contracts.push(*nft_contract_id);
    }

    /// Adds the contract addresses of fungible tokens with which users can pay for NFT
    /// Requirements:
    /// Only admin can add approved ft addresses
    /// Arguments:
    /// * `ft_contract_id`: the FT contract address
    fn add_ft_contract(&mut self, ft_contract_id: &ActorId) {
        self.check_owner();
        self.approved_ft_contracts.push(*ft_contract_id);
    }

    /// Add data on market item
    /// If NFT is not listed on the marketplace then it will be listed
    /// Requirements
    /// * `msg::source()` must be the NFT owner
    /// * `nft_contract_id` must be added to `approved_nft_contracts`
    /// * if item already exists, then it cannot be changed if there is an active auction
    /// Arguments:
    /// * `nft_contract_id`: the NFT contract address
    /// * `token_id`: the NFT id
    /// * `price`: the NFT price (if it is `None` then the item is not on the sale)
    pub async fn add_market_data(
        &mut self,
        nft_contract_id: &ActorId,
        ft_contract_id: Option<ActorId>,
        token_id: U256,
        price: Option<u128>,
    ) {
        self.check_approved_nft_contract(nft_contract_id);
        self.check_approved_ft_contract(ft_contract_id);
        let contract_and_token_id =
            format!("{}{}", H256::from_slice(nft_contract_id.as_ref()), token_id);
        self.on_auction(&contract_and_token_id);

        nft_approve(nft_contract_id, &exec::program_id(), token_id).await;

        self.items
            .entry(contract_and_token_id)
            .and_modify(|item| {
                item.price = price;
                item.ft_contract_id = ft_contract_id
            })
            .or_insert(Item {
                owner_id: msg::source(),
                nft_contract_id: *nft_contract_id,
                ft_contract_id,
                token_id,
                price,
                auction: None,
                offers: Vec::new(),
            });

        msg::reply(
            MarketEvent::MarketDataAdded {
                nft_contract_id: *nft_contract_id,
                owner: msg::source(),
                token_id,
                price,
            },
            0,
        )
        .unwrap();
    }

    pub fn check_owner(&self) {
        if msg::source() != self.owner_id {
            panic!("Only owner can make that action");
        }
    }

    pub fn check_approved_nft_contract(&self, nft_contract_id: &ActorId) {
        if !self.approved_nft_contracts.contains(nft_contract_id) {
            panic!("that nft contract is not approved");
        }
    }

    pub fn check_approved_ft_contract(&self, ft_contract_id: Option<ActorId>) {
        if ft_contract_id.is_some()
            && !self
                .approved_ft_contracts
                .contains(&ft_contract_id.unwrap())
        {
            panic!("that ft contract is not approved");
        }
    }
}

gstd::metadata! {
    title: "NFTMarketplace",
        init:
            input: InitMarket,
        handle:
            input: MarketAction,
            output: MarketEvent,
}

#[gstd::async_main]
async unsafe fn main() {
    let action: MarketAction = msg::load().expect("Could not load Action");
    let market: &mut Market = unsafe { MARKET.get_or_insert(Market::default()) };
    match action {
        MarketAction::AddNftContract(nft_contract_id) => {
            market.add_nft_contract(&nft_contract_id);
        }
        MarketAction::AddFTContract(nft_contract_id) => {
            market.add_ft_contract(&nft_contract_id);
        }
        MarketAction::AddMarketData {
            nft_contract_id,
            ft_contract_id,
            token_id,
            price,
        } => {
            market
                .add_market_data(&nft_contract_id, ft_contract_id, token_id, price)
                .await;
        }
        MarketAction::BuyItem {
            nft_contract_id,
            token_id,
        } => {
            market.buy_item(&nft_contract_id, token_id).await;
        }
        MarketAction::Item {
            nft_contract_id,
            token_id,
        } => {
            let contract_and_token_id =
                format!("{}{}", H256::from_slice(nft_contract_id.as_ref()), token_id);
            let item = market
                .items
                .get(&contract_and_token_id)
                .unwrap_or(&Item::default())
                .clone();
            msg::reply(MarketEvent::ItemInfo(item), 0).unwrap();
        }
        MarketAction::AddOffer {
            nft_contract_id,
            ft_contract_id,
            token_id,
            price,
        } => {
            market
                .add_offer(&nft_contract_id, ft_contract_id, token_id, price)
                .await
        }
        MarketAction::AcceptOffer {
            nft_contract_id,
            token_id,
            offer_hash,
        } => {
            market
                .accept_offer(&nft_contract_id, token_id, offer_hash)
                .await
        }
        MarketAction::Withdraw {
            nft_contract_id,
            token_id,
            hash,
        } => market.withdraw(&nft_contract_id, token_id, hash).await,
        MarketAction::CreateAuction {
            nft_contract_id,
            ft_contract_id,
            token_id,
            min_price,
            bid_period,
            duration,
        } => {
            market
                .create_auction(
                    &nft_contract_id,
                    ft_contract_id,
                    token_id,
                    min_price,
                    bid_period,
                    duration,
                )
                .await;
        }
        MarketAction::AddBid {
            nft_contract_id,
            token_id,
            price,
        } => market.add_bid(&nft_contract_id, token_id, price).await,

        MarketAction::SettleAuction {
            nft_contract_id,
            token_id,
        } => {
            market.settle_auction(&nft_contract_id, token_id).await;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn init() {
    let config: InitMarket = msg::load().expect("Unable to decode InitConfig");
    let market = Market {
        owner_id: config.owner_id,
        treasury_id: config.treasury_id,
        treasury_fee: config.treasury_fee,
        ..Market::default()
    };
    MARKET = Some(market);
}
