pub mod utils;

use gstd::{collections::BTreeMap, ActorId};
use nft_marketplace_io::*;
use utils::prelude::*;

// TODO: fix test
#[test]
#[ignore]
fn offers() {
    let system = utils::initialize_system();

    let (ft_program, nft_program, market) = utils::initialize_programs(&system);

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

    let mut offers: BTreeMap<(Option<ContractId>, Price), ActorId> = BTreeMap::new();
    for i in 0..10 {
        let offered_price = 200_000_000_000_000 * (i + 1) as u128;
        system.mint_to(BUYER, offered_price);
        market
            .add_offer(
                BUYER.into(),
                nft_program.actor_id(),
                TOKEN_ID.into(),
                None,
                offered_price,
                offered_price,
            )
            .succeed((nft_program.actor_id(), None, TOKEN_ID.into(), offered_price));
        offers.insert((None, offered_price), BUYER.into());
    }
    let mut tx_id: u64 = 100;

    for i in 10..20 {
        let offered_price = 200_000_000_000_000 * (i + 1) as u128;
        tx_id += 1;
        ft_program.mint(tx_id, BUYER, offered_price);
        tx_id += 1;
        ft_program.approve(tx_id, BUYER, market.actor_id(), offered_price);
        market
            .add_offer(
                BUYER.into(),
                nft_program.actor_id(),
                TOKEN_ID.into(),
                Some(ft_program.actor_id()),
                offered_price,
                0,
            )
            .succeed((
                nft_program.actor_id(),
                Some(ft_program.actor_id()),
                TOKEN_ID.into(),
                offered_price,
            ));
        offers.insert((Some(ft_program.actor_id()), offered_price), BUYER.into());
    }

    let market_state = market.meta_state().state().0;
    assert!(market_state
        .items
        .contains_key(&(nft_program.actor_id(), TOKEN_ID.into())));

    // Accept offer (for fungible tokens)
    let accepted_price = 200_000_000_000_000 * 15;
    market
        .accept_offer(
            SELLER.into(),
            nft_program.actor_id(),
            TOKEN_ID.into(),
            Some(ft_program.actor_id()),
            accepted_price,
        )
        .succeed((
            nft_program.actor_id(),
            TOKEN_ID.into(),
            BUYER.into(),
            accepted_price,
        ));

    let treasury_fee = accepted_price * ((TREASURY_FEE * BASE_PERCENT) as u128) / 10_000u128;

    // Check balance of SELLER
    ft_program
        .balance_of(SELLER)
        .check(accepted_price - treasury_fee);

    // Check balance of TREASURY_ID
    ft_program.balance_of(TREASURY_ID).check(treasury_fee);

    let market_state = market.meta_state().state().0;
    assert!(!market_state
        .items
        .get(&(nft_program.actor_id(), TOKEN_ID.into()))
        .expect("Unexpected invalid item.")
        .offers
        .contains_key(&(Some(ft_program.actor_id()), accepted_price)));

    // Withdraw tokens
    let withdrawn_tokens = 2_200_000_000_000_000;
    market
        .withdraw(
            BUYER.into(),
            nft_program.actor_id(),
            TOKEN_ID.into(),
            Some(ft_program.actor_id()),
            withdrawn_tokens,
        )
        .succeed((nft_program.actor_id(), TOKEN_ID.into(), withdrawn_tokens));

    // Withdraw native tokens
    let withdrawn_tokens = 200_000_000_000_000 * 2_u128;
    market
        .withdraw(
            BUYER.into(),
            nft_program.actor_id(),
            TOKEN_ID.into(),
            None,
            withdrawn_tokens,
        )
        .succeed((nft_program.actor_id(), TOKEN_ID.into(), withdrawn_tokens));

    // Check balance of SELLER after tokens withdrawal
    system.claim_value_from_mailbox(BUYER);
    assert_eq!(system.balance_of(BUYER), withdrawn_tokens);

    // Previous owner makes offer for native value
    let offered_value = 20_000_000_000_000_000;
    let buyer_balance = system.balance_of(BUYER);
    let treasury_fee = offered_value * ((TREASURY_FEE * BASE_PERCENT) as u128) / 10_000u128;

    system.mint_to(SELLER, offered_value);
    market
        .add_offer(
            SELLER.into(),
            nft_program.actor_id(),
            TOKEN_ID.into(),
            None,
            offered_value,
            offered_value,
        )
        .succeed((nft_program.actor_id(), None, TOKEN_ID.into(), offered_value));

    // New owner accepts offer
    market
        .accept_offer(
            BUYER.into(),
            nft_program.actor_id(),
            TOKEN_ID.into(),
            None,
            offered_value,
        )
        .succeed((
            nft_program.actor_id(),
            TOKEN_ID.into(),
            SELLER.into(),
            offered_value,
        ));

    // Check balance of BUYER
    system.claim_value_from_mailbox(BUYER);
    assert_eq!(
        system.balance_of(BUYER),
        buyer_balance + offered_value - treasury_fee
    );

    // Check balance of TREASURY_ID
    system.claim_value_from_mailbox(TREASURY_ID);
    assert_eq!(system.balance_of(TREASURY_ID), treasury_fee);
}

// TODO: fix test
#[test]
#[ignore]
fn offers_failures() {
    let system = utils::initialize_system();

    let (ft_program, nft_program, market) = utils::initialize_programs_without_ft_approve(&system);

    market
        .add_market_data(
            &system,
            SELLER,
            nft_program.actor_id(),
            None,
            TOKEN_ID.into(),
            None,
        )
        .succeed((nft_program.actor_id(), TOKEN_ID.into(), None));

    // Must fail since the fungible token contract is not approved
    market
        .add_offer(
            BUYER.into(),
            nft_program.actor_id(),
            TOKEN_ID.into(),
            Some(ft_program.actor_id()),
            NFT_PRICE,
            0,
        )
        .failed(MarketErr::ContractNotApproved);

    market
        .add_ft_contract(ADMIN, ft_program.actor_id())
        .succeed(ft_program.actor_id());

    // Must fail since the price is zero
    market
        .add_offer(
            BUYER.into(),
            nft_program.actor_id(),
            TOKEN_ID.into(),
            Some(ft_program.actor_id()),
            0,
            0,
        )
        .failed(MarketErr::WrongPrice);

    system.mint_to(BUYER, 4 * NFT_PRICE);

    // Must fail since the attached value is not equal to the offered price
    market
        .add_offer(
            BUYER.into(),
            nft_program.actor_id(),
            TOKEN_ID.into(),
            None,
            NFT_PRICE,
            NFT_PRICE - 100_000_000_000,
        )
        .failed(MarketErr::WrongPrice);

    // Add offer
    market
        .add_offer(
            BUYER.into(),
            nft_program.actor_id(),
            TOKEN_ID.into(),
            None,
            NFT_PRICE,
            NFT_PRICE,
        )
        .succeed((nft_program.actor_id(), None, TOKEN_ID.into(), NFT_PRICE));

    // Must fail since the offers with these params already exists
    market
        .add_offer(
            BUYER.into(),
            nft_program.actor_id(),
            TOKEN_ID.into(),
            None,
            NFT_PRICE,
            NFT_PRICE,
        )
        .failed(MarketErr::OfferAlreadyExists);

    // Accept offer

    // Must fail since only owner can accept offer
    market
        .accept_offer(
            BUYER.into(),
            nft_program.actor_id(),
            TOKEN_ID.into(),
            None,
            NFT_PRICE,
        )
        .failed(MarketErr::OfferShouldAcceptedByOwner);

    // Must fail since the offer with the params doesn't exist
    market
        .accept_offer(
            SELLER.into(),
            nft_program.actor_id(),
            TOKEN_ID.into(),
            None,
            2 * NFT_PRICE,
        )
        .failed(MarketErr::OfferIsNotExists);

    // Start auction
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

    // Must fail since auction is on
    market
        .add_offer(
            BUYER.into(),
            nft_program.actor_id(),
            TOKEN_ID.into(),
            None,
            NFT_PRICE - 100_000_000_000,
            NFT_PRICE - 100_000_000_000,
        )
        .failed(MarketErr::AuctionIsAlreadyExists);

    market
        .accept_offer(
            SELLER.into(),
            nft_program.actor_id(),
            TOKEN_ID.into(),
            None,
            NFT_PRICE,
        )
        .failed(MarketErr::AuctionIsOpened);

    // Withdraw failures

    // Must fail since the caller isn't the offer author
    market
        .withdraw(
            SELLER.into(),
            nft_program.actor_id(),
            TOKEN_ID.into(),
            None,
            NFT_PRICE,
        )
        .failed(MarketErr::InvalidCaller);

    // Must fail since the indicated offer hash doesn't exist
    market
        .withdraw(
            BUYER.into(),
            nft_program.actor_id(),
            TOKEN_ID.into(),
            None,
            2 * NFT_PRICE,
        )
        .failed(MarketErr::OfferIsNotExists);
}
