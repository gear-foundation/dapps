use crate::sale::buy_item_with_value;
use crate::{
    get_owner, nft_transfer, sale::buy_item_with_fungible_tokens, transfer_tokens, Auction,
    ContractId, Item, Market, MarketEvent, Price, TokenId, MINIMUM_VALUE,
};
use sails_rs::{
    collections::HashMap,
    gstd::{exec, msg},
    ActorId,
};

pub async fn add_market_data(
    market: &mut Market,
    nft_contract_id: ContractId,
    ft_contract_id: Option<ContractId>,
    token_id: TokenId,
    price: Option<Price>,
) {
    // Check approved nft and ft contract
    market.check_approved_nft_contract(&nft_contract_id);
    market.check_approved_ft_contract(ft_contract_id);

    // Check owner
    let owner = get_owner(&nft_contract_id, token_id).await;
    assert_eq!(
        owner,
        msg::source(),
        "Only owner has a right to add NFT to the marketplace"
    );
    // Transfer nft to marketplace
    nft_transfer(&nft_contract_id, &owner, &exec::program_id(), token_id).await;
    market
        .items
        .entry((nft_contract_id, token_id))
        .and_modify(|item| {
            item.price = price;
            item.ft_contract_id = ft_contract_id
        })
        .or_insert(Item {
            frozen: false,
            token_id,
            owner,
            ft_contract_id,
            price,
            auction: None,
            offers: HashMap::new(),
        });
}

pub async fn remove_market_data(
    market: &mut Market,
    nft_contract_id: &ContractId,
    token_id: TokenId,
    msg_src: ActorId,
) {
    let item = market
        .items
        .get_mut(&(*nft_contract_id, token_id))
        .expect("Item does not exists");

    if item.owner != msg_src {
        panic!("Wrong owner");
    }
    if item.frozen {
        panic!("Item is frozen");
    }

    if item.auction.is_some() {
        panic!("Item on auction");
    }
    item.frozen = true;
    let program_id = exec::program_id();
    nft_transfer(nft_contract_id, &program_id, &item.owner, token_id).await;
    for ((ft_id, price), account) in item.offers.iter() {
        if let Some(id) = ft_id {
            transfer_tokens(id, &program_id, account, (*price).into()).await;
        } else {
            msg::send_with_gas(*account, "", 0, *price).expect("Error in sending value");
        }
    }
    market
        .items
        .remove(&(*nft_contract_id, token_id))
        .expect("Item does not exists");
}

pub async fn buy_item(
    market: &mut Market,
    nft_contract_id: &ContractId,
    token_id: TokenId,
    msg_src: ActorId,
) {
    let item = market
        .items
        .get_mut(&(*nft_contract_id, token_id))
        .expect("Item does not exists");

    if item.frozen {
        panic!("Item is frozen");
    }

    if item.auction.is_some() {
        panic!("Item on auction");
    }
    if item.price.is_none() {
        panic!("Item is not on sale");
    };

    let program_id = exec::program_id();
    if let Some(ft_contract_id) = item.ft_contract_id {
        buy_item_with_fungible_tokens(
            item,
            nft_contract_id,
            &ft_contract_id,
            &program_id,
            &msg_src,
            token_id,
        )
        .await;
    } else {
        buy_item_with_value(item, nft_contract_id, &program_id, &msg_src, token_id).await;
    };
}

pub async fn add_offer(
    market: &mut Market,
    nft_contract_id: &ContractId,
    ft_contract_id: Option<ContractId>,
    token_id: TokenId,
    price: Price,
) {
    if let Some(ft_contract_id) = &ft_contract_id {
        if !market.approved_ft_contracts.contains(ft_contract_id) {
            panic!("Contract not approved");
        }
        if price == 0 {
            panic!("Wrong price");
        }
    } else if price <= MINIMUM_VALUE.into() || msg::value() != price {
        panic!("Wrong price");
    }

    let item = market
        .items
        .get_mut(&(*nft_contract_id, token_id))
        .expect("Item does not exists");

    if item.frozen {
        panic!("Item is frozen");
    }

    if item.auction.is_some() {
        panic!("Auction is opened");
    }

    if item.offers.contains_key(&(ft_contract_id, price)) {
        panic!("Offer already exists");
    };
    let msg_source = msg::source();
    if let Some(ft_id) = ft_contract_id {
        // Transfer fungible tokens to marketplace
        transfer_tokens(&ft_id, &msg_source, &exec::program_id(), price.into()).await;
    }
    item.offers.insert((ft_contract_id, price), msg_source);
}

pub async fn accept_offer(
    market: &mut Market,
    nft_contract_id: &ContractId,
    ft_contract_id: Option<ContractId>,
    token_id: TokenId,
    price: Price,
) -> ActorId {
    let item = market
        .items
        .get_mut(&(*nft_contract_id, token_id))
        .expect("Item does not exists");

    if item.frozen {
        panic!("Item is frozen");
    }

    if item.auction.is_some() {
        panic!("Auction is opened");
    }

    if item.owner != msg::source() {
        panic!("Offer should be accepted by owner");
    }

    assert!(
        item.price.is_none(),
        "Remove the item from the sale when accepting the offer"
    );

    let account = *item
        .offers
        .get(&(ft_contract_id, price))
        .expect("Offer is not exists");

    let program_id = exec::program_id();
    item.frozen = true;
    // Transfer NFT to the buyer
    nft_transfer(nft_contract_id, &program_id, &account, token_id).await;
    if let Some(ft_id) = ft_contract_id {
        // Transfer FT to the item owner
        transfer_tokens(&ft_id, &program_id, &item.owner, price.into()).await;
    } else {
        // Transfer value to the item owner
        msg::send_with_gas(item.owner, "", 0, price).expect("Error in sending value");
    };
    item.owner = account;
    item.price = None;
    item.frozen = false;
    item.offers.remove(&(ft_contract_id, price));
    account
}

pub async fn withdraw(
    market: &mut Market,
    nft_contract_id: &ContractId,
    ft_contract_id: Option<ContractId>,
    token_id: TokenId,
    price: Price,
) {
    let item = market
        .items
        .get_mut(&(*nft_contract_id, token_id))
        .expect("Item does not exists");

    if item.frozen {
        panic!("Item is frozen");
    }

    let account = if let Some(account) = item.offers.get(&(ft_contract_id, price)) {
        *account
    } else {
        panic!("Offer is not Exists");
    };

    if account != msg::source() {
        panic!("Invalid caller");
    }
    item.frozen = true;
    if let Some(ft_id) = ft_contract_id {
        transfer_tokens(&ft_id, &exec::program_id(), &account, price.into()).await;
    } else {
        msg::send_with_gas(account, "", 0, price).expect("Error in sending value");
    };
    item.offers.remove(&(ft_contract_id, price));
    item.frozen = false;
}

pub async fn create_auction(
    market: &mut Market,
    nft_contract_id: &ContractId,
    ft_contract_id: Option<ContractId>,
    token_id: TokenId,
    min_price: Price,
    duration: u64,
) {
    // Check approved nft and ft contract
    market.check_approved_nft_contract(nft_contract_id);
    market.check_approved_ft_contract(ft_contract_id);

    let item = market
        .items
        .get_mut(&(*nft_contract_id, token_id))
        .expect("Item does not exists");

    if item.frozen {
        panic!("Item is frozen");
    }

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

    if ft_contract_id.is_some() {
        if min_price == 0 {
            panic!("Auction min price is zero");
        }
    } else if min_price <= MINIMUM_VALUE.into() {
        panic!("Price is less than the minimum value: {:?}", MINIMUM_VALUE);
    }

    item.ft_contract_id = ft_contract_id;
    item.auction = Some(Auction {
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
    let item = market
        .items
        .get_mut(&(*nft_contract_id, token_id))
        .expect("Item does not exists");

    if item.frozen {
        panic!("Item is frozen");
    }

    let auction: &mut Auction = item.auction.as_mut().expect("Auction does not exists");

    if auction.ended_at < exec::block_timestamp() {
        panic!("Auction is already ended");
    }

    if price <= auction.current_price {
        panic!("Wrong price");
    }

    if let Some(ft_id) = item.ft_contract_id {
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
    } else {
        assert!(msg::value() == price, "Not enough attached value");
        if !auction.current_winner.is_zero() {
            msg::send_with_gas(auction.current_winner, "", 0, auction.current_price)
                .expect("Error in sending value");
        }
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
    let item = market
        .items
        .get_mut(&(*nft_contract_id, token_id))
        .expect("Item does not exists");

    if item.frozen {
        panic!("Item is frozen");
    }

    let auction: &mut Auction = item.auction.as_mut().expect("Auction does not exists");

    if auction.ended_at > exec::block_timestamp() {
        panic!("Auction is not over");
    }

    let price = auction.current_price;
    item.frozen = true;
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
        transfer_tokens(
            &ft_id,
            &exec::program_id(),
            &item.owner,
            auction.current_price.into(),
        )
        .await;
    } else {
        msg::send_with_gas(item.owner, "", 0, auction.current_price)
            .expect("Error in sending value");
    }

    nft_transfer(nft_contract_id, &program_id, &winner, token_id).await;

    item.auction = None;
    item.owner = winner;
    item.frozen = false;
    MarketEvent::AuctionSettled {
        nft_contract_id: *nft_contract_id,
        token_id,
        price,
    }
}
