use sails_rs::{collections::HashMap, gstd::{msg, exec}, ActorId};
use crate::{Item, Price, TokenId, ContractId, Market, Auction, MarketEvent, get_owner, nft_transfer, transfer_tokens, MINIMUM_VALUE, MIN_BID_PERIOD};
use crate::sale::buy_item_with_value;

pub async fn add_market_data(market: &mut Market, nft_contract_id: ContractId, ft_contract_id: Option<ContractId>, token_id: TokenId, price: Option<Price>) {
    market.check_approved_nft_contract(&nft_contract_id);
    market.check_approved_ft_contract(ft_contract_id);
    let contract_and_token_id = (nft_contract_id, token_id);

    let owner = get_owner(&nft_contract_id, token_id).await;
    assert_eq!(
        owner,
        msg::source(),
        "Only owner has a right to add NFT to the marketplace"
    );
    nft_transfer(&nft_contract_id, &owner, &exec::program_id(), token_id).await;
    market.items
        .entry(contract_and_token_id)
        .and_modify(|item| {
            item.price = price;
            item.ft_contract_id = ft_contract_id
        })
        .or_insert(Item {
            token_id,
            owner,
            ft_contract_id,
            price,
            auction: None,
            offers: HashMap::new(),
        });
}


pub async fn buy_item(market: &mut Market, nft_contract_id: &ContractId, token_id: TokenId, msg_src: ActorId) {
    let contract_and_token_id = (*nft_contract_id, token_id);

    if let Some(item) = market.items.get_mut(&contract_and_token_id) {
        if item.auction.is_some() {
            panic!("Item on auction");
        }
        if item.price.is_none() {
            panic!("Item is not on sale");
        };
        buy_item_impl(item, nft_contract_id, &msg_src, token_id).await
    } else {
        panic!("Item does not exists");
    }
}

async fn buy_item_impl(
    item: &mut Item,
    nft_contract_id: &ContractId,
    new_owner: &ActorId,
    token_id: TokenId,
) {
    let program_id = exec::program_id();
    let ft_id = if let Some(ft_contract_id) = item.ft_contract_id {
        ft_contract_id
    } else {
        return buy_item_with_value(item, nft_contract_id, &program_id, new_owner, token_id)
            .await;
    };

    let price = item.price.expect("Can't be None");

    transfer_tokens(&ft_id, new_owner, &item.owner, price.into()).await;

    // transfer NFT to the buyer
    nft_transfer(nft_contract_id, &program_id, new_owner, token_id).await;
    item.owner = *new_owner;
    item.price = None;
}

pub async fn add_offer(
    market: &mut Market,
    nft_contract_id: &ContractId,
    ft_contract_id: Option<ContractId>,
    token_id: TokenId,
    price: Price,
) {
    let contract_and_token_id = (*nft_contract_id, token_id);

    if let Some(ft_contract_id) = &ft_contract_id {
        let is_ft_approved = market.approved_ft_contracts.contains(ft_contract_id);
        if !is_ft_approved {
            panic!("Contract not approved");
        }
    }

    if ft_contract_id.is_some() && price <= 0
        || ft_contract_id.is_none() && price <= MINIMUM_VALUE.into()
    {
        panic!("Wrong price");
    }

    if ft_contract_id.is_none() && msg::value() != price {
        panic!("Wrong price");
    }

    let item = market
        .items
        .get_mut(&contract_and_token_id)
        .expect("Item does not exists");

    if item.auction.is_some() {
        panic!("Auction is opened");
    }

    if item.offers.contains_key(&(ft_contract_id, price)) {
        panic!("Offer already exists");
    };
    let msg_source = msg::source();
    let ft_id = if let Some(ft_id) = ft_contract_id {
        ft_id
    } else {
        item.offers.insert((None, price), msg_source);
        return;
    };

    transfer_tokens(
        &ft_id,
        &msg_source,
        &exec::program_id(),
        price.into(),
    )
    .await;

    item.offers.insert((Some(ft_id), price), msg_source);
}


pub async fn accept_offer(
    market: &mut Market,
    nft_contract_id: &ContractId,
    ft_contract_id: Option<ContractId>,
    token_id: TokenId,
    price: Price,
) -> ActorId {
    let contract_and_token_id = (*nft_contract_id, token_id);

    let item = market
        .items
        .get_mut(&contract_and_token_id)
        .expect("Item does not exists");

    if item.auction.is_some() {
        panic!("Auction is opened");
    }

    if item.owner != msg::source() {
        panic!("Offer should accepted by owner");
    }

    assert!(
        item.price.is_none(),
        "Remove the item from the sale when accepting the offer"
    );
    let offers = item.offers.clone();

    let account = offers
        .get(&(ft_contract_id, price))
        .expect("Offer is not exists");

    let program_id = exec::program_id();

    let ft_id = if let Some(ft_contract_id) = ft_contract_id {
        ft_contract_id
    } else {
        accept_offer_with_value(
            item,
            nft_contract_id,
            account,
            token_id,
            price,
            &program_id,
        )
        .await;
        return *account;
    };

    // Transfer NFT to the buyer
    nft_transfer(nft_contract_id, &program_id, account, token_id).await;

    transfer_tokens(&ft_id, &program_id, account, price.into()).await;

    item.owner = *account;
    item.price = None;
    item.offers.remove(&(ft_contract_id, price));

    *account
}

pub async fn accept_offer_with_value(
    item: &mut Item,
    nft_contract_id: &ContractId,
    new_owner: &ActorId,
    token_id: TokenId,
    price: Price,
    program_id: &ActorId,
) {
    // transfer NFT
    nft_transfer(nft_contract_id, program_id, new_owner, token_id).await;


    item.owner = *new_owner;
    item.price = None;

    item.offers.remove(&(None, price));

}

pub async fn withdraw(
    market: &mut Market,
    nft_contract_id: &ContractId,
    ft_contract_id: Option<ContractId>,
    token_id: TokenId,
    price: Price,
) {
    let contract_and_token_id = (*nft_contract_id, token_id);

    let item = market
        .items
        .get_mut(&contract_and_token_id)
        .expect("Item does not exists");

    let account = if let Some(account) = item.offers.get(&(ft_contract_id, price)) {
        *account
    } else {
        panic!("Offer is not Exists");
    };

    if account != msg::source() {
        panic!("Invalid caller");
    }

    let ft_id = if let Some(ft_id) = ft_contract_id {
        ft_id
    } else {
        msg::send_with_gas(account, "", 0, price).expect("Error in sending value");
        return;
    };

    transfer_tokens(&ft_id, &exec::program_id(), &account, price.into()).await;
    item.offers.remove(&(Some(ft_id), price));

}


pub async fn create_auction(
    market: &mut Market,
    nft_contract_id: &ContractId,
    ft_contract_id: Option<ContractId>,
    token_id: TokenId,
    min_price: Price,
    bid_period: u64,
    duration: u64,
) {
    market.check_approved_nft_contract(nft_contract_id);
    market.check_approved_ft_contract(ft_contract_id);
    let contract_and_token_id = (*nft_contract_id, token_id);
    let item = market
        .items
        .get_mut(&contract_and_token_id)
        .expect("Item does not exists");

    assert_eq!(
        item.owner,
        msg::source(),
        "Only owner has a right to add NFT to the marketplace and start the auction"
    );

    if item.auction.is_some() {
        panic!("Auction is already exists");
    }

    assert!(
        item.price.is_none(),
        "Remove the item from the sale before starting the auction"
    );

    if bid_period < MIN_BID_PERIOD || duration < MIN_BID_PERIOD {
        panic!("Auction bid period or duration is invalid");
    }

    if min_price <= 0 {
        panic!("Auction min price is zero");
    }

    nft_transfer(nft_contract_id, &item.owner,&exec::program_id(), token_id).await;
    item.ft_contract_id = ft_contract_id;
    item.auction = Some(Auction {
        bid_period,
        started_at: exec::block_timestamp(),
        ended_at: exec::block_timestamp() + duration,
        current_price: min_price,
        current_winner: ActorId::zero(),
    });
}

pub async fn add_bid(
    market: &mut Market,
    nft_contract_id: &ContractId,
    token_id: TokenId,
    price: Price,
) {
    let msg_src = msg::source();
    let contract_and_token_id = (*nft_contract_id, token_id);
    let item = market
        .items
        .get_mut(&contract_and_token_id)
        .expect("Item does not exists");

    let auction: &mut Auction = item.auction.as_mut().expect("Auction does not exists");

    if auction.ended_at < exec::block_timestamp() {
        panic!("Auction is already ended");
    }

    let ft_id = match item.ft_contract_id {
        Some(ft_id) => ft_id,
        None => {
            if price <= auction.current_price {
                panic!("Wrong price");
            }

            assert!(msg::value() == price, "Not enough attached value");

            msg::send_with_gas(
                auction.current_winner,
                "",
                0,
                auction.current_price,
            )
            .expect("Error in sending value");

            auction.current_price = price;
            auction.current_winner = msg_src;

            return;
        }
    };

    if price <= auction.current_price {
        panic!("Wrong price");
    }

    transfer_tokens(&ft_id, &msg_src, &exec::program_id(), price.into()).await;

    if !auction.current_winner.is_zero() {
        transfer_tokens(
            &ft_id,
            &exec::program_id(),
            &auction.current_winner,
            auction.current_price.into(),
        )
        .await;
    }

    auction.current_price = price;
    auction.current_winner = msg_src;
}

pub async fn settle_auction(
    market: &mut Market,
    nft_contract_id: &ContractId,
    token_id: TokenId,
) -> MarketEvent {
    let program_id = exec::program_id();
    let contract_and_token_id = (*nft_contract_id, token_id);
    let item = market
        .items
        .get_mut(&contract_and_token_id)
        .expect("Item does not exists");

    let auction: &mut Auction = item.auction.as_mut().expect("Auction does not exists");

    if auction.ended_at > exec::block_timestamp() {
        panic!("Auction is not over");
    }

    let price = auction.current_price;

    let winner = if auction.current_winner.is_zero() {
        nft_transfer(nft_contract_id, &exec::program_id(), &item.owner, token_id).await;
        item.auction = None;
        return MarketEvent::AuctionCancelled {
            nft_contract_id: *nft_contract_id,
            token_id,
        };
    } else {
        auction.current_winner
    };

    if let Some(ft_id) = item.ft_contract_id {
        transfer_tokens(&ft_id, &exec::program_id(), &item.owner, auction.current_price.into()).await;
    } else {
        msg::send_with_gas(item.owner, "", 0, auction.current_price)
            .expect("Error in sending value");
    }

    nft_transfer(nft_contract_id, &program_id, &winner, token_id).await;

    item.auction = None;
    item.owner = winner;

    MarketEvent::AuctionSettled {
        nft_contract_id: *nft_contract_id,
        token_id,
        price,
    }
}
