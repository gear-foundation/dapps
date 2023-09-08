use crate::{
    nft_messages::{nft_transfer, payouts, Payout},
    payment::transfer_tokens,
    {MarketHandler, BASE_PERCENT, MINIMUM_VALUE},
};
use gstd::{exec, msg, prelude::*, ActorId};
use nft_marketplace_io::*;

const MIN_BID_PERIOD: u64 = 60_000;

#[async_trait::async_trait]
pub trait AuctionHandler {
    async fn create_auction(
        &mut self,
        nft_contract_id: &ContractId,
        ft_contract_id: Option<ContractId>,
        token_id: TokenId,
        min_price: Price,
        bid_period: u64,
        duration: u64,
    ) -> Result<MarketEvent, MarketErr>;

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
    #[allow(unused_must_use)]
    async fn settle_auction(
        &mut self,
        nft_contract_id: &ContractId,
        token_id: TokenId,
    ) -> Result<MarketEvent, MarketErr>;

    async fn add_bid(
        &mut self,
        nft_contract_id: &ContractId,
        token_id: TokenId,
        price: Price,
    ) -> Result<MarketEvent, MarketErr>;
}

#[async_trait::async_trait]
impl AuctionHandler for Market {
    async fn create_auction(
        &mut self,
        nft_contract_id: &ContractId,
        ft_contract_id: Option<ContractId>,
        token_id: TokenId,
        min_price: Price,
        bid_period: u64,
        duration: u64,
    ) -> Result<MarketEvent, MarketErr> {
        self.check_approved_nft_contract(nft_contract_id);
        self.check_approved_ft_contract(ft_contract_id);
        let contract_and_token_id = (*nft_contract_id, token_id);

        if let Some(item) = self.items.get_mut(&contract_and_token_id) {
            assert_eq!(
                item.owner,
                msg::source(),
                "Only owner has a right to add NFT to the marketplace and start the auction"
            );

            if item.auction.is_some() {
                return Err(MarketErr::AuctionIsAlreadyExists);
            }

            assert!(
                item.price.is_none(),
                "Remove the item from the sale before starting the auction"
            );

            if bid_period < MIN_BID_PERIOD || duration < MIN_BID_PERIOD {
                return Err(MarketErr::AuctionBidPeriodOrDurationIsInvalid);
            }

            #[allow(clippy::absurd_extreme_comparisons)]
            if min_price <= 0 {
                return Err(MarketErr::AuctionMinPriceIsZero);
            }

            if let Some((tx_id, tx)) = item.tx.clone() {
                if tx == MarketTx::CreateAuction {
                    return create_auction_tx(
                        tx_id,
                        item,
                        nft_contract_id,
                        ft_contract_id,
                        token_id,
                        min_price,
                        bid_period,
                        duration,
                    )
                    .await;
                } else {
                    return Err(MarketErr::WrongTransaction);
                }
            }

            let tx_id = self.tx_id;
            self.tx_id = self.tx_id.wrapping_add(1);
            item.tx = Some((tx_id, MarketTx::CreateAuction));

            create_auction_tx(
                tx_id,
                item,
                nft_contract_id,
                ft_contract_id,
                token_id,
                min_price,
                bid_period,
                duration,
            )
            .await
        } else {
            Err(MarketErr::ItemDoesNotExists)
        }
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
    #[allow(unused_must_use)]
    async fn settle_auction(
        &mut self,
        nft_contract_id: &ContractId,
        token_id: TokenId,
    ) -> Result<MarketEvent, MarketErr> {
        let contract_and_token_id = (*nft_contract_id, token_id);

        if let Some(item) = self.items.get_mut(&contract_and_token_id) {
            let Some(auction) = item.auction.clone() else {
                return Err(MarketErr::AuctionDoesNotExists);
            };

            if auction.ended_at > exec::block_timestamp() {
                return Err(MarketErr::AuctionIsNotOver);
            }

            if let Some((tx_id, tx)) = item.tx.clone() {
                match tx {
                    MarketTx::Bid { account, price } => {
                        let ft_id = item.ft_contract_id.expect("Can't be None");
                        add_bid_tx(
                            tx_id,
                            item,
                            nft_contract_id,
                            &ft_id,
                            token_id,
                            &account,
                            price,
                        )
                        .await;
                    }
                    MarketTx::SettleAuction => {
                        let price = auction.current_price;
                        // calculate fee for treasury
                        let treasury_fee =
                            price * (self.treasury_fee * BASE_PERCENT) as u128 / 10_000u128;

                        // payouts for NFT sale (includes royalty accounts and seller)
                        let mut payouts =
                            payouts(nft_contract_id, &item.owner, price - treasury_fee).await;
                        payouts.insert(self.treasury_id, treasury_fee);
                        return settle_auction_tx(
                            tx_id,
                            item,
                            &payouts,
                            nft_contract_id,
                            token_id,
                            price,
                        )
                        .await;
                    }
                    _ => {
                        return Err(MarketErr::WrongTransaction);
                    }
                }
            }

            let price = auction.current_price;
            // calculate fee for treasury
            let treasury_fee = price * (self.treasury_fee * BASE_PERCENT) as u128 / 10_000u128;

            // payouts for NFT sale (includes royalty accounts and seller)
            let mut payouts = payouts(nft_contract_id, &item.owner, price - treasury_fee).await;
            payouts.insert(self.treasury_id, treasury_fee);

            let tx_id = self.tx_id;
            self.tx_id = self.tx_id.wrapping_add(payouts.len() as u64);
            item.tx = Some((tx_id, MarketTx::SettleAuction));
            settle_auction_tx(tx_id, item, &payouts, nft_contract_id, token_id, price).await
        } else {
            Err(MarketErr::ItemDoesNotExists)
        }
    }

    async fn add_bid(
        &mut self,
        nft_contract_id: &ContractId,
        token_id: TokenId,
        price: Price,
    ) -> Result<MarketEvent, MarketErr> {
        let contract_and_token_id = (*nft_contract_id, token_id);

        if let Some(item) = self.items.get_mut(&contract_and_token_id) {
            if let Some(auction) = item.auction.as_mut() {
                if auction.ended_at < exec::block_timestamp() {
                    return Err(MarketErr::AuctionIsAlreadyEnded);
                }

                let ft_id = match item.ft_contract_id {
                    Some(ft_id) => ft_id,
                    None => {
                        if price <= auction.current_price {
                            return Err(MarketErr::WrongPrice);
                        }

                        assert!(msg::value() == price, "Not enough attached value");

                        msg::send(
                            auction.current_winner,
                            MarketEvent::TransferValue,
                            auction.current_price,
                        )
                        .expect("Error in sending value");

                        auction.current_price = price;
                        auction.current_winner = msg::source();

                        return Ok(MarketEvent::BidAdded {
                            nft_contract_id: *nft_contract_id,
                            token_id,
                            price,
                        });
                    }
                };

                if let Some((tx_id, tx)) = item.tx.clone() {
                    match tx {
                        MarketTx::Bid { account, price } => {
                            let new_price = price;
                            let result = add_bid_tx(
                                tx_id,
                                item,
                                nft_contract_id,
                                &ft_id,
                                token_id,
                                &account,
                                price,
                            )
                            .await;
                            if account == msg::source() && new_price == price {
                                return result;
                            }
                        }
                        _ => {
                            return Err(MarketErr::WrongTransaction);
                        }
                    }
                }

                let tx_id = self.tx_id;
                self.tx_id = self.tx_id.wrapping_add(2);
                item.tx = Some((
                    tx_id,
                    MarketTx::Bid {
                        account: msg::source(),
                        price,
                    },
                ));

                add_bid_tx(
                    tx_id,
                    item,
                    nft_contract_id,
                    &ft_id,
                    token_id,
                    &msg::source(),
                    price,
                )
                .await
            } else {
                Err(MarketErr::AuctionDoesNotExists)
            }
        } else {
            Err(MarketErr::ItemDoesNotExists)
        }
    }
}

#[allow(clippy::too_many_arguments)]
async fn create_auction_tx(
    tx_id: TransactionId,
    item: &mut Item,
    nft_contract_id: &ContractId,
    ft_contract_id: Option<ContractId>,
    token_id: TokenId,
    price: Price,
    bid_period: u64,
    duration: u64,
) -> Result<MarketEvent, MarketErr> {
    if nft_transfer(tx_id, nft_contract_id, &exec::program_id(), token_id)
        .await
        .is_err()
    {
        item.tx = None;
        return Err(MarketErr::NFTTransferFailed);
    }
    item.ft_contract_id = ft_contract_id;
    item.auction = Some(Auction {
        bid_period,
        started_at: exec::block_timestamp(),
        ended_at: exec::block_timestamp() + duration,
        current_price: price,
        current_winner: ActorId::zero(),
    });
    item.tx = None;
    Ok(MarketEvent::AuctionCreated {
        nft_contract_id: (*nft_contract_id),
        token_id,
        price,
    })
}

async fn add_bid_tx(
    mut tx_id: TransactionId,
    item: &mut Item,
    nft_contract_id: &ContractId,
    ft_contract_id: &ContractId,
    token_id: TokenId,
    account: &ActorId,
    price: Price,
) -> Result<MarketEvent, MarketErr> {
    let auction: &mut Auction = item.auction.as_mut().expect("Can't be None");
    if price <= auction.current_price {
        return Err(MarketErr::WrongPrice);
    }

    if transfer_tokens(tx_id, ft_contract_id, account, &exec::program_id(), price)
        .await
        .is_err()
    {
        item.tx = None;
        return Err(MarketErr::TokenTransferFailed);
    }

    tx_id += 1;
    if !auction.current_winner.is_zero()
        && transfer_tokens(
            tx_id,
            ft_contract_id,
            &exec::program_id(),
            &auction.current_winner,
            auction.current_price,
        )
        .await
        .is_err()
    {
        return Err(MarketErr::RerunTransaction);
    }

    item.tx = None;
    auction.current_price = price;
    auction.current_winner = *account;

    Ok(MarketEvent::BidAdded {
        nft_contract_id: *nft_contract_id,
        token_id,
        price,
    })
}

async fn settle_auction_tx(
    mut tx_id: TransactionId,
    item: &mut Item,
    payouts: &Payout,
    nft_contract_id: &ContractId,
    token_id: TokenId,
    price: Price,
) -> Result<MarketEvent, MarketErr> {
    let auction: &mut Auction = item.auction.as_mut().expect("Can't be None");
    let winner = if auction.current_winner.is_zero() {
        item.auction = None;
        item.tx = None;

        return Ok(MarketEvent::AuctionCancelled {
            nft_contract_id: *nft_contract_id,
            token_id,
        });
    } else {
        auction.current_winner
    };

    // send tokens to the seller, royalties and tresuary account
    // since tokens are on the marketplace account, the error can be only due the lack of gas
    if let Some(ft_id) = item.ft_contract_id {
        for (account, amount) in payouts.iter() {
            tx_id = tx_id.wrapping_add(1);
            if transfer_tokens(tx_id, &ft_id, &exec::program_id(), account, *amount)
                .await
                .is_err()
            {
                return Err(MarketErr::RerunTransaction);
            };
        }
    } else {
        for (account, amount) in payouts.iter() {
            if account != &exec::program_id() && price > MINIMUM_VALUE.into() {
                msg::send(*account, MarketEvent::TransferValue, *amount)
                    .expect("Error in sending value");
            }
        }
    }

    if nft_transfer(tx_id, nft_contract_id, &winner, token_id)
        .await
        .is_err()
    {
        return Err(MarketErr::RerunTransaction);
    }

    item.tx = None;
    item.auction = None;
    item.owner = winner;

    Ok(MarketEvent::AuctionSettled {
        nft_contract_id: *nft_contract_id,
        token_id,
        price,
    })
}
