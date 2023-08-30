use crate::{
    nft_messages::*,
    payment::*,
    {BASE_PERCENT, MINIMUM_VALUE},
};
use gstd::{exec, msg, prelude::*, ActorId};
use nft_marketplace_io::*;

#[async_trait::async_trait]
pub trait SaleHandler {
    async fn buy_item(
        &mut self,
        nft_contract_id: &ContractId,
        token_id: TokenId,
    ) -> Result<MarketEvent, MarketErr>;
}

#[async_trait::async_trait]
impl SaleHandler for Market {
    async fn buy_item(
        &mut self,
        nft_contract_id: &ContractId,
        token_id: TokenId,
    ) -> Result<MarketEvent, MarketErr> {
        let contract_and_token_id = (*nft_contract_id, token_id);

        if let Some(item) = self.items.get_mut(&contract_and_token_id) {
            if item.auction.is_some() {
                return Err(MarketErr::ItemOnAuction);
            }
            assert!(item.auction.is_none(), "There is an opened auction");

            let Some(price) = item.price else {
                return Err(MarketErr::ItemIsNotOnSale);
            };

            // calculate fee for treasury
            let treasury_fee = price * (self.treasury_fee * BASE_PERCENT) as u128 / 10_000u128;

            // payouts for NFT sale (includes royalty accounts and seller)
            let mut payouts = payouts(nft_contract_id, &item.owner, price - treasury_fee).await;
            payouts.insert(self.treasury_id, treasury_fee);

            if let Some((tx_id, tx)) = item.tx.clone() {
                match tx {
                    MarketTx::Sale { buyer } => {
                        if buyer != msg::source() {
                            return Err(MarketErr::WrongTransaction);
                        }
                        return buy_item_tx(
                            tx_id,
                            item,
                            nft_contract_id,
                            &buyer,
                            token_id,
                            &payouts,
                        )
                        .await;
                    }
                    _ => {
                        return Err(MarketErr::WrongTransaction);
                    }
                }
            }
            let buyer = msg::source();
            let tx_id = self.tx_id;
            item.tx = Some((tx_id, MarketTx::Sale { buyer }));
            buy_item_tx(tx_id, item, nft_contract_id, &buyer, token_id, &payouts).await
        } else {
            Err(MarketErr::ItemDoesNotExists)
        }
    }
}

async fn buy_item_tx(
    mut tx_id: TransactionId,
    item: &mut Item,
    nft_contract_id: &ContractId,
    new_owner: &ActorId,
    token_id: TokenId,
    payouts: &Payout,
) -> Result<MarketEvent, MarketErr> {
    let ft_id = if let Some(ft_contract_id) = item.ft_contract_id {
        ft_contract_id
    } else {
        return buy_item_tx_with_value(tx_id, item, nft_contract_id, new_owner, token_id, payouts)
            .await;
    };

    // transfer NFT to the marketplace account
    if nft_transfer(tx_id, nft_contract_id, &exec::program_id(), token_id)
        .await
        .is_err()
    {
        item.tx = None;
        return Err(MarketErr::NFTTransferFailed);
    }

    let price = item.price.expect("Can't be None");

    // transfer tokens to the marketplace account
    if transfer_tokens(tx_id, &ft_id, new_owner, &exec::program_id(), price)
        .await
        .is_err()
    {
        // if there is a fail during the token transfer
        // we transfer NFT back to the seller
        tx_id = tx_id.wrapping_add(1);
        if nft_transfer(tx_id, nft_contract_id, &item.owner, token_id)
            .await
            .is_err()
        {
            return Err(MarketErr::RerunTransaction);
        }
        item.tx = None;
        return Err(MarketErr::TokenTransferFailed);
    }
    // send tokens to the seller, royalties and tresuary account
    // since tokens are on the marketplace account, the error can be only due the lack of gas
    for (account, amount) in payouts.iter() {
        tx_id = tx_id.wrapping_add(1);
        if transfer_tokens(tx_id, &ft_id, &exec::program_id(), account, *amount)
            .await
            .is_err()
        {
            return Err(MarketErr::RerunTransaction);
        };
    }

    // transfer NFT to the buyer
    if nft_transfer(tx_id, nft_contract_id, new_owner, token_id)
        .await
        .is_err()
    {
        return Err(MarketErr::RerunTransaction);
    }

    item.owner = *new_owner;
    item.price = None;
    item.tx = None;

    Ok(MarketEvent::ItemSold {
        owner: *new_owner,
        nft_contract_id: *nft_contract_id,
        token_id,
    })
}

pub async fn buy_item_tx_with_value(
    tx_id: TransactionId,
    item: &mut Item,
    nft_contract_id: &ContractId,
    new_owner: &ActorId,
    token_id: TokenId,
    payouts: &Payout,
) -> Result<MarketEvent, MarketErr> {
    let price = item.price.expect("Can't be None");
    if msg::value() < price {
        return Err(MarketErr::WrongPrice);
    }

    // transfer NFT to the
    if nft_transfer(tx_id, nft_contract_id, new_owner, token_id)
        .await
        .is_err()
    {
        item.tx = None;
        return Err(MarketErr::NFTTransferFailed);
    }

    // send tokens to the seller, royalties and tresuary account
    // since tokens are on the marketplace account, the error can be only due the lack of gas
    for (account, amount) in payouts.iter() {
        if account != &exec::program_id() && price > MINIMUM_VALUE.into() {
            msg::send(*account, "", *amount).expect("Error in sending value");
        }
    }

    item.owner = *new_owner;
    item.price = None;
    item.tx = None;

    Ok(MarketEvent::ItemSold {
        owner: *new_owner,
        nft_contract_id: *nft_contract_id,
        token_id,
    })
}
