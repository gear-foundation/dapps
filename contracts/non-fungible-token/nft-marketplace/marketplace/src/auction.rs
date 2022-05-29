use crate::{
    nft_messages::{nft_approve, nft_payouts, nft_transfer},
    payment::{check_attached_value, transfer_payment},
    Item, Market, MarketEvent,
};
use gstd::{exec, msg, prelude::*, ActorId};
use market_io::*;
use primitive_types::{H256, U256};
const MIN_BID_PERIOD: u64 = 60_000;

impl Market {
    /// Creates an auction for selected item
    /// If item isn't listed on the marketplace it will add item
    /// Requirements:
    /// * Only the item owner can start auction
    /// * `nft_contract_id` must be added to `approved_nft_contracts`
    /// *  there must be no active auction
    /// Arguments:
    /// * `nft_contract_id`: the NFT contract address
    /// * `ft_contract_id`: the fungible token contract address that can be used for trading
    /// * `token_id`: the NFT id
    /// * `min_price`: the starting price
    /// * `bid_period`: the time that the auction lasts until another bid occurs
    pub async fn create_auction(
        &mut self,
        nft_contract_id: &ActorId,
        ft_contract_id: Option<ActorId>,
        token_id: U256,
        min_price: u128,
        bid_period: u64,
        duration: u64,
    ) {
        self.check_approved_nft_contract(nft_contract_id);
        self.check_approved_ft_contract(ft_contract_id);
        let contract_and_token_id =
            format!("{}{}", H256::from_slice(nft_contract_id.as_ref()), token_id);
        self.on_auction(&contract_and_token_id);
        if bid_period < MIN_BID_PERIOD || duration < MIN_BID_PERIOD {
            panic!("bid period or auction duration can't be less than 1 minute");
        }
        if min_price == 0 {
            panic!("price can't be equal to zero");
        }
        // approve nft to trade on the marketplace
        nft_approve(nft_contract_id, &exec::program_id(), token_id).await;

        let auction = Auction {
            bid_period,
            started_at: exec::block_timestamp(),
            ended_at: exec::block_timestamp() + duration,
            current_price: min_price,
            bids: vec![],
        };
        self.items
            .entry(contract_and_token_id)
            .and_modify(|item| {
                item.price = None;
                item.auction = Some(auction.clone());
                item.ft_contract_id = ft_contract_id
            })
            .or_insert(Item {
                owner_id: msg::source(),
                nft_contract_id: *nft_contract_id,
                ft_contract_id,
                token_id,
                price: None,
                auction: Some(auction),
                offers: Vec::new(),
            });

        msg::reply(
            MarketEvent::AuctionCreated {
                nft_contract_id: *nft_contract_id,
                token_id,
                price: min_price,
            },
            0,
        )
        .unwrap();
    }

    /// Settles the auction
    /// Requirements:
    /// * The auction must be over
    /// Arguments:
    /// * `nft_contract_id`: the NFT contract address
    /// * `token_id`: the NFT id
    pub async fn settle_auction(&mut self, nft_contract_id: &ActorId, token_id: U256) {
        let contract_and_token_id =
            format!("{}{}", H256::from_slice(nft_contract_id.as_ref()), token_id);
        let item = self
            .items
            .get_mut(&contract_and_token_id)
            .expect("Item does not exist");

        let auction = item.auction.clone().expect("Auction doesn not exist");

        if auction.ended_at > exec::block_timestamp() {
            panic!("Auction is not over");
        }
        item.auction = None;
        let mut bids = auction.bids;

        if bids.is_empty() {
            msg::reply(
                MarketEvent::AuctionCancelled {
                    nft_contract_id: *nft_contract_id,
                    token_id,
                },
                0,
            )
            .unwrap();

            return;
        }

        let highest_bid = &bids[bids.len() - 1].clone();
        bids.pop();
        for bid in bids {
            transfer_payment(&exec::program_id(), &bid.id, item.ft_contract_id, bid.price).await;
        }
        // fee for treasury
        let treasury_fee = highest_bid.price * self.treasury_fee / 10_000u128;
        transfer_payment(
            &exec::program_id(),
            &self.treasury_id,
            item.ft_contract_id,
            treasury_fee,
        )
        .await;
        let payouts = nft_payouts(
            nft_contract_id,
            &item.owner_id,
            highest_bid.price - treasury_fee,
        )
        .await;
        for (account, amount) in payouts.iter() {
            transfer_payment(&exec::program_id(), account, item.ft_contract_id, *amount).await;
        }

        item.owner_id = highest_bid.id;
        // transfer NFT
        nft_transfer(nft_contract_id, &highest_bid.id, token_id).await;
        msg::reply(
            MarketEvent::AuctionSettled {
                nft_contract_id: *nft_contract_id,
                token_id,
                price: highest_bid.price,
            },
            0,
        )
        .unwrap();
    }

    /// Adds a bid to an ongoing auction
    /// Requirements:
    /// * The auction must be on
    /// * The caller must have enough balance for the offered price
    /// Arguments:
    /// * `nft_contract_id`: the NFT contract address
    /// * `token_id`: the NFT id
    /// * `price`: the offered price
    pub async fn add_bid(&mut self, nft_contract_id: &ActorId, token_id: U256, price: u128) {
        let contract_and_token_id =
            format!("{}{}", H256::from_slice(nft_contract_id.as_ref()), token_id);

        let item = self
            .items
            .get_mut(&contract_and_token_id)
            .expect("Item does not exist");

        let mut auction = item.auction.clone().expect("Auction doesn not exist");
        if auction.ended_at < exec::block_timestamp() {
            panic!("Auction has already ended");
        }

        check_attached_value(item.ft_contract_id, price);

        if price <= auction.current_price {
            panic!("Cant offer less or equal to the current bid price")
        }

        transfer_payment(
            &msg::source(),
            &exec::program_id(),
            item.ft_contract_id,
            price,
        )
        .await;
        auction.bids.push(Bid {
            id: msg::source(),
            price,
        });
        if auction.ended_at <= exec::block_timestamp() + auction.bid_period {
            auction.ended_at = exec::block_timestamp() + auction.bid_period;
        }
        auction.current_price = price;
        item.auction = Some(auction);
        msg::reply(
            MarketEvent::BidAdded {
                nft_contract_id: *nft_contract_id,
                token_id,
                price,
            },
            0,
        )
        .unwrap();
    }

    // checks that there is an active auction
    pub fn on_auction(&self, contract_and_token_id: &String) {
        if let Some(item) = self.items.get(contract_and_token_id) {
            if item.auction.is_some() {
                panic!("There is an opened auction");
            }
        }
    }
}
