#![no_std]
use sails_rs::gstd::msg;
use sails_rs::prelude::*;

mod utils;
mod nft_messages;
mod funcs;
mod sale;
mod payment;

use utils::*;
use nft_messages::*;
use funcs::*;
use payment::*;

static mut STORAGE: Option<Market> = None;

struct MarketService(());

impl MarketService {
    pub fn new() -> Self {
        Self(())
    }
    pub fn get_mut(&mut self) -> &'static mut Market {
        unsafe { STORAGE.as_mut().expect("Storage is not initialized") }
    }
    pub fn get(&self) -> &'static Market {
        unsafe { STORAGE.as_ref().expect("Storage is not initialized") }
    }
}

#[sails_rs::service(events = MarketEvent)]
impl MarketService {
    fn init(admin_id: ActorId) -> Self {    
        let market = Market {
            admin_id: admin_id,
            ..Default::default()
        };
        unsafe { STORAGE = Some(market) };
        Self(())
    }

    pub fn add_nft_contract(&mut self, nft_contract_id: ContractId) {
        let market = self.get_mut();
        market.check_admin();
        market.approved_nft_contracts.insert(nft_contract_id);
        self.notify_on(MarketEvent::NftContractAdded(nft_contract_id))
            .expect("Notification Error");
    }
    pub fn add_ft_contract(&mut self, ft_contract_id: ContractId) {
        let market = self.get_mut();
        market.check_admin();
        market.approved_ft_contracts.insert(ft_contract_id);
        self.notify_on(MarketEvent::FtContractAdded(ft_contract_id))
            .expect("Notification Error");
    }
    pub async fn add_market_data(&mut self, nft_contract_id: ContractId, ft_contract_id: Option<ContractId>, token_id: TokenId, price: Option<Price>) {
        let market = self.get_mut();
        
        add_market_data(market, nft_contract_id, ft_contract_id, token_id, price).await;
        self.notify_on(MarketEvent::MarketDataAdded { nft_contract_id, token_id, price })
            .expect("Notification Error");
    }

    pub async fn buy_item(&mut self, nft_contract_id: ContractId, token_id: TokenId) {
        let market = self.get_mut();
        let msg_src = msg::source();
        buy_item(market, &nft_contract_id, token_id, msg_src).await;
        self.notify_on(MarketEvent::ItemSold { owner: msg_src, nft_contract_id, token_id })
            .expect("Notification Error");
    }
    pub async fn add_offer(&mut self, nft_contract_id: ContractId, ft_contract_id: Option<ContractId>, token_id: TokenId, price: u128) {
        let market = self.get_mut();
        add_offer(market, &nft_contract_id, ft_contract_id, token_id, price).await;
        self.notify_on(MarketEvent::OfferAdded {
            nft_contract_id,
            ft_contract_id,
            token_id,
            price,
        }).expect("Notification Error");
    }
    pub async fn accept_offer(&mut self, nft_contract_id: ContractId, ft_contract_id: Option<ContractId>, token_id: TokenId, price: u128) {
        let market = self.get_mut();
        let new_owner = accept_offer(market, &nft_contract_id, ft_contract_id, token_id, price).await;
        self.notify_on(MarketEvent::OfferAccepted {
            nft_contract_id,
            token_id,
            new_owner,
            price,
        }).expect("Notification Error");
    }
    pub async fn withdraw(&mut self, nft_contract_id: ContractId, ft_contract_id: Option<ContractId>, token_id: TokenId, price: u128) {
        let market = self.get_mut();
        withdraw(market, &nft_contract_id, ft_contract_id, token_id, price).await;
        self.notify_on(MarketEvent::Withdraw {
            nft_contract_id,
            token_id,
            price,
        }).expect("Notification Error");
    }
    pub async fn create_auction(&mut self, nft_contract_id: ContractId, ft_contract_id: Option<ContractId>, token_id: TokenId, min_price: u128, bid_period: u64, duration: u64) {
        let market = self.get_mut();
        create_auction(market, &nft_contract_id, ft_contract_id, token_id, min_price, bid_period, duration).await;
        self.notify_on(MarketEvent::AuctionCreated {
            nft_contract_id,
            token_id,
            price: min_price,
        }).expect("Notification Error");
    }

    pub async fn add_bid(&mut self, nft_contract_id: ContractId, token_id: TokenId, price: u128) {
        let market = self.get_mut();
        add_bid(market, &nft_contract_id, token_id, price).await;
        self.notify_on(MarketEvent::BidAdded {
            nft_contract_id,
            token_id,
            price,
        }).expect("Notification Error");
    }
    
    pub async fn settle_auction(&mut self, nft_contract_id: ContractId, token_id: TokenId) {
        let market = self.get_mut();
        let event = settle_auction(market, &nft_contract_id, token_id).await;
        self.notify_on(event).expect("Notification Error");
    }

    
}

pub struct MarketProgram(());

#[allow(clippy::new_without_default)]
#[sails_rs::program]
impl MarketProgram {
    // Program's constructor
    pub fn new(admin_id: ActorId) -> Self {
        MarketService::init(admin_id);
        Self(())
    }

    // Exposed service
    pub fn nft_marketplace(&self) -> MarketService {
        MarketService::new()
    }
}
