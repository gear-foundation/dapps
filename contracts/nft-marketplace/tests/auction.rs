pub mod utils;

use nft_marketplace_io::*;
use utils::prelude::*;

// TODO: fix test
#[test]
#[ignore]
fn auction_with_native_tokens() {
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

    nft_program.approve(0, SELLER, market.actor_id(), TOKEN_ID.into());

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

    for (i, &participant) in PARTICIPANTS.iter().enumerate() {
        let bid_price = (i as u128 + 2) * NFT_PRICE;
        system.mint_to(participant, bid_price);
        market
            .add_bid(
                participant,
                nft_program.actor_id(),
                TOKEN_ID.into(),
                bid_price,
                bid_price,
            )
            .succeed((nft_program.actor_id(), TOKEN_ID.into(), bid_price));

        // check that marketplace has returned funds to the previous participant
        if i != 0 {
            system.claim_value_from_mailbox(PARTICIPANTS[i - 1]);
            assert_eq!(
                system.balance_of(PARTICIPANTS[i - 1]),
                (i as u128 + 1) * NFT_PRICE
            );
        }
    }

    let winner_price = 6 * NFT_PRICE;
    let _winner = PARTICIPANTS[4];

    // check balance of nft marketplace contract
    assert_eq!(system.balance_of(MARKET_ID), winner_price);

    system.spend_blocks((DURATION / 1000) as u32);

    market
        .settle_auction(SELLER, nft_program.actor_id(), TOKEN_ID.into())
        .succeed(MarketEvent::AuctionSettled {
            nft_contract_id: nft_program.actor_id(),
            token_id: TOKEN_ID.into(),
            price: winner_price,
        });

    let treasury_fee = winner_price * ((TREASURY_FEE * BASE_PERCENT) as u128) / 10_000u128;

    // Check balance of SELLER
    system.claim_value_from_mailbox(SELLER);
    assert_eq!(system.balance_of(SELLER), winner_price - treasury_fee);

    // Check balance of TREASURY_ID
    system.claim_value_from_mailbox(TREASURY_ID);
    assert_eq!(system.balance_of(TREASURY_ID), treasury_fee);
}

// TODO: fix test
#[test]
#[ignore]
fn cancelled_auction() {
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

    nft_program.approve(0, SELLER, market.actor_id(), TOKEN_ID.into());

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

    system.spend_blocks((DURATION / 1000) as u32);

    market
        .settle_auction(SELLER, nft_program.actor_id(), TOKEN_ID.into())
        .succeed(MarketEvent::AuctionCancelled {
            nft_contract_id: nft_program.actor_id(),
            token_id: TOKEN_ID.into(),
        });
}

// TODO: fix test
#[test]
#[ignore]
fn auction_with_fungible_tokens() {
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

    nft_program.approve(0, SELLER, market.actor_id(), TOKEN_ID.into());

    market
        .create_auction(
            &system,
            SELLER,
            (
                nft_program.actor_id(),
                TOKEN_ID.into(),
                Some(ft_program.actor_id()),
            ),
            NFT_PRICE,
            BID_PERIOD,
            DURATION,
        )
        .succeed((nft_program.actor_id(), TOKEN_ID.into(), NFT_PRICE));

    let mut tx_id: u64 = 100;
    for (i, &participant) in PARTICIPANTS.iter().enumerate() {
        let bid_price = (i as u128 + 2) * NFT_PRICE;
        ft_program.approve(tx_id, participant, market.actor_id(), bid_price);
        tx_id += 1;
        ft_program.mint(tx_id, participant, bid_price);
        market
            .add_bid(
                participant,
                nft_program.actor_id(),
                TOKEN_ID.into(),
                bid_price,
                0,
            )
            .succeed((nft_program.actor_id(), TOKEN_ID.into(), bid_price));

        // Check that marketplace has returned funds to the previous participant
        if i != 0 {
            ft_program
                .balance_of(PARTICIPANTS[i - 1])
                .check((i as u128 + 1) * NFT_PRICE);
        }
    }

    let winner_price = 6 * NFT_PRICE;
    let _winner = PARTICIPANTS[4];

    // Check balance of nft marketplace contract
    ft_program.balance_of(MARKET_ID).check(winner_price);

    system.spend_blocks((DURATION / 1000) as u32);

    market
        .settle_auction(SELLER, nft_program.actor_id(), TOKEN_ID.into())
        .succeed(MarketEvent::AuctionSettled {
            nft_contract_id: nft_program.actor_id(),
            token_id: TOKEN_ID.into(),
            price: winner_price,
        });

    let treasury_fee = winner_price * ((TREASURY_FEE * BASE_PERCENT) as u128) / 10_000u128;

    // Check balance of SELLER
    ft_program
        .balance_of(SELLER)
        .check(winner_price - treasury_fee);

    // Check balance of TREASURY_ID
    ft_program.balance_of(TREASURY_ID).check(treasury_fee);
}

// TODO: fix test
#[test]
#[ignore]
fn auction_failures() {
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

    // Create auction failures

    // Must fail since the bid period is less than 1 minute
    market
        .create_auction(
            &system,
            SELLER,
            (nft_program.actor_id(), TOKEN_ID.into(), None),
            NFT_PRICE,
            MIN_BID_PERIOD - 100,
            DURATION,
        )
        .failed(MarketErr::AuctionBidPeriodOrDurationIsInvalid);

    // Must fail since the bid duration is less than 1 minute
    market
        .create_auction(
            &system,
            SELLER,
            (nft_program.actor_id(), TOKEN_ID.into(), None),
            NFT_PRICE,
            BID_PERIOD,
            MIN_BID_PERIOD - 100,
        )
        .failed(MarketErr::AuctionBidPeriodOrDurationIsInvalid);

    // Must fail since the min price is equal to zero
    market
        .create_auction(
            &system,
            SELLER,
            (nft_program.actor_id(), TOKEN_ID.into(), None),
            0,
            BID_PERIOD,
            DURATION,
        )
        .failed(MarketErr::AuctionMinPriceIsZero);

    nft_program.approve(0, SELLER, market.actor_id(), TOKEN_ID.into());

    // start auction
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

    // Must fail since the auction is already on
    market
        .create_auction(
            &system,
            SELLER,
            (nft_program.actor_id(), TOKEN_ID.into(), None),
            NFT_PRICE,
            BID_PERIOD,
            DURATION,
        )
        .failed(MarketErr::AuctionIsAlreadyExists);

    // add bid and create auction failures

    // Must fail since the price is equal to the current bid price
    system.mint_to(BUYER, NFT_PRICE * 2);
    market
        .add_bid(
            BUYER,
            nft_program.actor_id(),
            TOKEN_ID.into(),
            NFT_PRICE,
            NFT_PRICE,
        )
        .failed(MarketErr::WrongPrice);

    // Must fail since the auction is not over
    market
        .settle_auction(SELLER, nft_program.actor_id(), TOKEN_ID.into())
        .failed(MarketErr::AuctionIsNotOver);

    system.spend_blocks((DURATION as u32) / 1000 + 1);

    // Must fail since the auction has alredy ended
    market
        .add_bid(
            BUYER,
            nft_program.actor_id(),
            TOKEN_ID.into(),
            NFT_PRICE,
            NFT_PRICE,
        )
        .failed(MarketErr::AuctionIsAlreadyEnded);

    market
        .settle_auction(SELLER, nft_program.actor_id(), TOKEN_ID.into())
        .succeed(MarketEvent::AuctionCancelled {
            nft_contract_id: nft_program.actor_id(),
            token_id: TOKEN_ID.into(),
        });

    // Must fail since the auction doesn't exist
    market
        .settle_auction(SELLER, nft_program.actor_id(), TOKEN_ID.into())
        .failed(MarketErr::AuctionDoesNotExists);
}
