pub mod utils;

use nft_marketplace_io::*;
use utils::prelude::*;

// TODO: fix test
#[test]
#[ignore]
fn buy_with_fungible_tokens() {
    let system = utils::initialize_system();

    let (ft_program, nft_program, market) = utils::initialize_programs(&system);

    market
        .add_market_data(
            &system,
            SELLER,
            nft_program.actor_id(),
            Some(ft_program.actor_id()),
            TOKEN_ID.into(),
            Some(NFT_PRICE),
        )
        .succeed((nft_program.actor_id(), TOKEN_ID.into(), Some(NFT_PRICE)));

    let tx_id: u64 = 100;
    ft_program.mint(BUYER, tx_id, NFT_PRICE);

    market
        .buy_item(BUYER, nft_program.actor_id(), TOKEN_ID.into(), 0)
        .succeed((BUYER.into(), nft_program.actor_id(), TOKEN_ID.into()));

    // Check balance of SELLER
    let treasury_fee = NFT_PRICE * ((TREASURY_FEE * BASE_PERCENT) as u128) / 10_000u128;
    ft_program
        .balance_of(SELLER)
        .check(NFT_PRICE - treasury_fee);

    // Check balance of TREASURY_ID
    ft_program.balance_of(TREASURY_ID).check(treasury_fee);
}

// TODO: fix test
#[test]
#[ignore]
fn buy_with_fungible_tokens_failures() {
    let system = utils::initialize_system();

    let (ft_program, nft_program, market) = utils::initialize_programs(&system);

    // Must fail since item does not exist
    market
        .buy_item(BUYER, nft_program.actor_id(), TOKEN_ID.into(), 0)
        .failed(MarketErr::ItemDoesNotExists);

    market
        .add_market_data(
            &system,
            SELLER,
            nft_program.actor_id(),
            Some(ft_program.actor_id()),
            TOKEN_ID.into(),
            None,
        )
        .succeed((nft_program.actor_id(), TOKEN_ID.into(), None));

    // Must fail since item isn't on sale
    market
        .buy_item(BUYER, nft_program.actor_id(), TOKEN_ID.into(), 0)
        .failed(MarketErr::ItemIsNotOnSale);

    market
        .create_auction(
            &system,
            SELLER,
            (nft_program.actor_id(), TOKEN_ID.into(), None),
            NFT_PRICE,
            BID_PERIOD,
            DURATION,
        )
        .succeed((nft_program.actor_id(), TOKEN_ID.into(), NFT_PRICE));

    // Must fail since auction has started on that item
    market
        .buy_item(BUYER, nft_program.actor_id(), TOKEN_ID.into(), 0)
        .failed(MarketErr::ItemOnAuction);
}

// TODO: fix test
#[test]
#[ignore]
fn buy_with_native_tokens() {
    let system = utils::initialize_system();

    let (_, nft_program, market) = utils::initialize_programs(&system);

    market
        .add_market_data(
            &system,
            SELLER,
            nft_program.actor_id(),
            None,
            TOKEN_ID.into(),
            Some(NFT_PRICE),
        )
        .succeed((nft_program.actor_id(), TOKEN_ID.into(), Some(NFT_PRICE)));

    system.mint_to(BUYER, NFT_PRICE * 2);

    // Must fail since not enough value was attached to the message
    market
        .buy_item(
            BUYER,
            nft_program.actor_id(),
            TOKEN_ID.into(),
            NFT_PRICE - 1000,
        )
        .failed(MarketErr::WrongPrice);

    market
        .buy_item(BUYER, nft_program.actor_id(), TOKEN_ID.into(), NFT_PRICE)
        .succeed((BUYER.into(), nft_program.actor_id(), TOKEN_ID.into()));

    let treasury_fee = NFT_PRICE * ((TREASURY_FEE * BASE_PERCENT) as u128) / 10_000u128;

    // Check balance of SELLER
    system.claim_value_from_mailbox(SELLER);
    assert_eq!(system.balance_of(SELLER), NFT_PRICE - treasury_fee);

    // Check balance of TREASURY_ID
    system.claim_value_from_mailbox(TREASURY_ID);
    assert_eq!(system.balance_of(TREASURY_ID), treasury_fee);
}
