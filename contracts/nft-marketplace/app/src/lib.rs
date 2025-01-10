#![no_std]
#![allow(static_mut_refs)]
use sails_rs::gstd::msg;
use sails_rs::prelude::*;

mod funcs;
mod nft_messages;
mod payment;
mod sale;
mod utils;

use funcs::*;
use nft_messages::*;
use payment::*;
use utils::*;

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
            admin_id,
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
    /// Adds data on market item.
    /// If the item of that NFT does not exist on the marketplace then it will be listed.
    /// If the item exists then that action is used to change the price or suspend the sale.
    ///
    /// # Requirements
    /// * [`msg::source()`](gstd::msg::source) must be the NFT owner
    /// * `nft_contract_id` must be added to `approved_nft_contracts`
    /// * if item already exists, then it cannot be changed if there is an active auction
    ///
    /// On success triggers the event [`MarketEvent::MarketDataAdded`].
    pub async fn add_market_data(
        &mut self,
        nft_contract_id: ContractId,
        ft_contract_id: Option<ContractId>,
        token_id: TokenId,
        price: Option<Price>,
    ) {
        let market = self.get_mut();
        add_market_data(market, nft_contract_id, ft_contract_id, token_id, price).await;
        self.notify_on(MarketEvent::MarketDataAdded {
            nft_contract_id,
            token_id,
            price,
        })
        .expect("Notification Error");
    }
    /// Removes data on market item.
    ///
    /// # Requirements
    /// * [`msg::source()`](gstd::msg::source) must be the  item.owner
    /// * There must be no open auction on the item.
    ///
    /// On success triggers the event [`MarketEvent::MarketDataRemoved`].
    pub async fn remove_market_data(&mut self, nft_contract_id: ContractId, token_id: TokenId) {
        let market = self.get_mut();
        let msg_src = msg::source();
        remove_market_data(market, &nft_contract_id, token_id, msg_src).await;
        self.notify_on(MarketEvent::MarketDataRemoved {
            owner: msg_src,
            nft_contract_id,
            token_id,
        })
        .expect("Notification Error");
    }
    /// Sells the NFT.
    ///
    /// # Requirements:
    /// * The NFT item must exist and be on sale.
    /// * If the NFT is sold for a native Vara token, then a buyer must attach a value equal to the price.
    /// * If the NFT is sold for fungible tokens then a buyer must have enough tokens in the fungible token contract.
    /// * There must be no open auction on the item.
    ///
    /// On success triggers the event [`MarketEvent::ItemSold`].
    pub async fn buy_item(&mut self, nft_contract_id: ContractId, token_id: TokenId) {
        let market = self.get_mut();
        let msg_src = msg::source();
        buy_item(market, &nft_contract_id, token_id, msg_src).await;
        self.notify_on(MarketEvent::ItemSold {
            owner: msg_src,
            nft_contract_id,
            token_id,
        })
        .expect("Notification Error");
    }
    pub async fn add_offer(
        &mut self,
        nft_contract_id: ContractId,
        ft_contract_id: Option<ContractId>,
        token_id: TokenId,
        price: u128,
    ) {
        let market = self.get_mut();
        add_offer(market, &nft_contract_id, ft_contract_id, token_id, price).await;
        self.notify_on(MarketEvent::OfferAdded {
            nft_contract_id,
            ft_contract_id,
            token_id,
            price,
        })
        .expect("Notification Error");
    }
    pub async fn accept_offer(
        &mut self,
        nft_contract_id: ContractId,
        ft_contract_id: Option<ContractId>,
        token_id: TokenId,
        price: u128,
    ) {
        let market = self.get_mut();
        let new_owner =
            accept_offer(market, &nft_contract_id, ft_contract_id, token_id, price).await;
        self.notify_on(MarketEvent::OfferAccepted {
            nft_contract_id,
            token_id,
            new_owner,
            price,
        })
        .expect("Notification Error");
    }
    pub async fn withdraw(
        &mut self,
        nft_contract_id: ContractId,
        ft_contract_id: Option<ContractId>,
        token_id: TokenId,
        price: u128,
    ) {
        let market = self.get_mut();
        withdraw(market, &nft_contract_id, ft_contract_id, token_id, price).await;
        self.notify_on(MarketEvent::Withdraw {
            nft_contract_id,
            token_id,
            price,
        })
        .expect("Notification Error");
    }

    /// Creates an auction for selected item.
    /// If the NFT item doesn't exist on the marketplace then it will be listed
    ///
    /// Requirements:
    /// * Only the item owner can start the auction.
    /// * `nft_contract_id` must be in the list of `approved_nft_contracts`
    /// *  There must be no active auction
    ///
    /// On success triggers the event [`MarketEvent::AuctionCreated`].
    pub async fn create_auction(
        &mut self,
        nft_contract_id: ContractId,
        ft_contract_id: Option<ContractId>,
        token_id: TokenId,
        min_price: u128,
        duration: u64,
    ) {
        let market = self.get_mut();
        create_auction(
            market,
            &nft_contract_id,
            ft_contract_id,
            token_id,
            min_price,
            duration,
        )
        .await;
        self.notify_on(MarketEvent::AuctionCreated {
            nft_contract_id,
            token_id,
            price: min_price,
        })
        .expect("Notification Error");
    }
    /// Adds a bid to an ongoing auction.
    ///
    /// # Requirements:
    /// * The item must exist.
    /// * The auction must exist on the item.
    /// * If the NFT is sold for a native Vara token, then a buyer must attach a value equal to the price indicated in the arguments.
    /// * If the NFT is sold for fungible tokens then a buyer must have   enough tokens in the fungible token contract.
    /// * `price` must be greater than the current offered price for that item.
    ///
    /// On success triggers the event [`MarketEvent::BidAdded`].
    pub async fn add_bid(&mut self, nft_contract_id: ContractId, token_id: TokenId, price: u128) {
        let market = self.get_mut();
        add_bid(market, &nft_contract_id, token_id, price).await;
        self.notify_on(MarketEvent::BidAdded {
            nft_contract_id,
            token_id,
            price,
        })
        .expect("Notification Error");
    }

    pub async fn settle_auction(&mut self, nft_contract_id: ContractId, token_id: TokenId) {
        let market = self.get_mut();
        let event = settle_auction(market, &nft_contract_id, token_id).await;
        self.notify_on(event).expect("Notification Error");
    }

    pub fn get_market(&self) -> MarketState {
        self.get().clone().into()
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
