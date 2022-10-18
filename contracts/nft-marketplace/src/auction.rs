use crate::{
    nft_messages::{nft_approve, nft_transfer},
    payment::{check_attached_value, transfer_payment},
    Item, Market, MarketEvent, BASE_PERCENT,
};
use gstd::{debug, exec, msg, prelude::*, ActorId};
use market_io::*;
use primitive_types::{H256, U256};
const MIN_BID_PERIOD: u64 = 60_000;
const ZERO_ID: ActorId = ActorId::new([0u8; 32]);

impl Market {
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
            current_winner: ZERO_ID,
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
                ft_contract_id,
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
        .expect("Error in reply [MarketEvent::AuctionCreated]");
    }

    /// Settles the auction.
    ///
    /// Requirements:
    /// * The auction must be over.
    ///
    /// Arguments:
    /// * `nft_contract_id`: the NFT contract address
    /// * `token_id`: the NFT id
    ///   
    /// On success auction replies [`MarketEvent::AuctionSettled`].
    /// If no bids were made replies [`MarketEvent::AuctionCancelled`].

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
        let winner = auction.current_winner;
        let price = auction.current_price;

        if winner == ZERO_ID {
            msg::reply(
                MarketEvent::AuctionCancelled {
                    nft_contract_id: *nft_contract_id,
                    token_id,
                },
                0,
            )
            .expect("Error in reply [MarketEvent::AuctionCancelled]");

            return;
        }

        // fee for treasury
        let treasury_fee = price * (self.treasury_fee * BASE_PERCENT) as u128 / 10_000u128;
        transfer_payment(
            &exec::program_id(),
            &self.treasury_id,
            item.ft_contract_id,
            treasury_fee,
        )
        .await;

        // transfer NFT and pay royalties
        let payouts = nft_transfer(nft_contract_id, &winner, token_id, price - treasury_fee).await;
        debug!("payouts {:?}", payouts);

        for (account, amount) in payouts.iter() {
            debug!("account {:?} amount {:?}", account, amount);
            transfer_payment(&exec::program_id(), account, item.ft_contract_id, *amount).await;
        }

        item.owner_id = winner;
        item.auction = None;
        msg::reply(
            MarketEvent::AuctionSettled {
                nft_contract_id: *nft_contract_id,
                token_id,
                price,
            },
            0,
        )
        .expect("Error in reply [MarketEvent::AuctionSettled]");
    }

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

        let previous_price = auction.current_price;
        let previous_winner = auction.current_winner;

        if price <= previous_price {
            panic!("Cant offer less or equal to the current bid price")
        }

        if auction.ended_at <= exec::block_timestamp() + auction.bid_period {
            auction.ended_at = exec::block_timestamp() + auction.bid_period;
        }

        auction.current_price = price;
        auction.current_winner = msg::source();
        item.auction = Some(auction);
        // transfer payment from the current account to the marketplace contract
        transfer_payment(
            &msg::source(),
            &exec::program_id(),
            item.ft_contract_id,
            price,
        )
        .await;

        if previous_winner != ZERO_ID {
            // transfer payment back to the previous winner
            transfer_payment(
                &exec::program_id(),
                &previous_winner,
                item.ft_contract_id,
                previous_price,
            )
            .await;
        }

        msg::reply(
            MarketEvent::BidAdded {
                nft_contract_id: *nft_contract_id,
                token_id,
                price,
            },
            0,
        )
        .expect("Error in reply [MarketEvent::BidAdded]");
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
