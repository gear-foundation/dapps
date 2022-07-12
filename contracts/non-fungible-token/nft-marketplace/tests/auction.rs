use codec::Encode;
use ft_io::*;
use gstd::ActorId;
use gtest::{Program, RunResult, System};
use market_io::*;
use nft_io::*;
mod utils;
use gear_lib::non_fungible_token::token::*;
pub use utils::*;

fn before_each_test(sys: &System) {
    init_ft(sys);
    init_nft(sys);
    init_market(sys);
    let nft = sys.get_program(2);
    let market = sys.get_program(3);
    let res = market.send(USERS[0], MarketAction::AddFTContract(1.into()));
    assert!(res.log().is_empty());
    let res = market.send(USERS[0], MarketAction::AddNftContract(2.into()));
    assert!(res.log().is_empty());

    let res = nft.send(
        USERS[0],
        NFTAction::Mint {
            token_metadata: TokenMetadata {
                name: "CryptoKitty".to_string(),
                description: "Description".to_string(),
                media: "http://".to_string(),
                reference: "http://".to_string(),
            },
        },
    );
    assert!(!res.main_failed());
}

fn start_auction(
    market: &Program,
    ft_contract_id: Option<ActorId>,
    min_price: u128,
    bid_period: u64,
    duration: u64,
) -> RunResult {
    market.send(
        USERS[0],
        MarketAction::CreateAuction {
            nft_contract_id: 2.into(),
            ft_contract_id,
            token_id: 0.into(),
            min_price,
            bid_period,
            duration,
        },
    )
}

fn bid(market: &Program, user: u64, price: u128) -> RunResult {
    market.send_with_value(
        user,
        MarketAction::AddBid {
            nft_contract_id: 2.into(),
            token_id: 0.into(),
            price,
        },
        price,
    )
}

#[test]
fn create_auction() {
    let sys = System::new();
    sys.init_logger();
    before_each_test(&sys);
    let market = sys.get_program(3);
    let res = start_auction(&market, None, 1000, 60_000, 86_400_000);
    assert!(res.contains(&(
        USERS[0],
        MarketEvent::AuctionCreated {
            nft_contract_id: 2.into(),
            token_id: 0.into(),
            price: 1000,
        }
        .encode()
    )));
}

#[test]
fn create_auction_failures() {
    let sys = System::new();
    sys.init_logger();
    before_each_test(&sys);
    let market = sys.get_program(3);

    // must fail since the bid period is less than 1 minute
    let res = start_auction(&market, None, 1_000, 50_000, 86_400_000);
    assert!(res.main_failed());

    // must fail since the bid period is less than 1 minute
    let res = start_auction(&market, None, 1_000, 60_000, 50_000);
    assert!(res.main_failed());

    // must fail since the min price is equal to zero
    let res = start_auction(&market, None, 0, 60_000, 86_400_000);
    assert!(res.main_failed());

    // start auction
    let res = start_auction(&market, None, 1_000, 60_000, 86_400_000);
    assert!(!res.main_failed());

    // must fail since the auction is already on
    let res = start_auction(&market, None, 1_000, 60_000, 86_400_000);
    assert!(res.main_failed());
}

#[test]
fn add_bid() {
    let sys = System::new();
    sys.init_logger();
    before_each_test(&sys);
    let market = sys.get_program(3);
    // start auction
    let res = start_auction(&market, None, 100_000, 60_000, 86_400_000);
    assert!(!res.main_failed());

    let res = bid(&market, USERS[0], 100_001);
    assert!(res.contains(&(
        USERS[0],
        MarketEvent::BidAdded {
            nft_contract_id: 2.into(),
            token_id: 0.into(),
            price: 100_001,
        }
        .encode()
    )));
}

#[test]
fn add_bid_failures() {
    let sys = System::new();
    sys.init_logger();
    before_each_test(&sys);
    let market = sys.get_program(3);
    // start auction
    let res = start_auction(&market, None, 100_000, 60_000, 86_400_000);
    assert!(!res.main_failed());
    // must fail since the price is equal to the current bid price
    let res = bid(&market, USERS[0], 100_000);
    assert!(res.main_failed());

    sys.spend_blocks(86400001);

    // must fail since the auction has ended
    let res = bid(&market, USERS[0], 200_000);
    assert!(res.main_failed());
}

#[test]
fn settle_auction() {
    let sys = System::new();
    sys.init_logger();
    before_each_test(&sys);
    let market = sys.get_program(3);
    let res = start_auction(&market, None, 100_000, 60_000, 86_400_000);
    assert!(!res.main_failed());

    // Users add bids
    USERS.iter().enumerate().for_each(|(i, user)| {
        let res = bid(&market, *user, 100_001 + i as u128);
        assert!(!res.main_failed());
    });

    sys.spend_blocks(86400000);

    let res = market.send(
        USERS[0],
        MarketAction::SettleAuction {
            nft_contract_id: 2.into(),
            token_id: 0.into(),
        },
    );

    assert!(res.contains(&(
        USERS[0],
        MarketEvent::AuctionSettled {
            nft_contract_id: 2.into(),
            token_id: 0.into(),
            price: 100004,
        }
        .encode()
    )));

    // Checks NFT item on the marketplace
    let res = market.send(
        USERS[0],
        MarketAction::Item {
            nft_contract_id: 2.into(),
            token_id: 0.into(),
        },
    );
    assert!(res.contains(&(
        USERS[0],
        MarketEvent::ItemInfo(Item {
            owner_id: USERS[3].into(),
            ft_contract_id: None,
            price: None,
            auction: None,
            offers: vec![],
        })
        .encode()
    )));
}

#[test]
fn auction_is_cancelled() {
    let sys = System::new();
    sys.init_logger();
    before_each_test(&sys);
    let market = sys.get_program(3);
    let res = start_auction(&market, None, 100_000, 60_000, 86_400_000);
    assert!(!res.main_failed());

    sys.spend_blocks(86400001);

    let res = market.send(
        USERS[0],
        MarketAction::SettleAuction {
            nft_contract_id: 2.into(),
            token_id: 0.into(),
        },
    );

    assert!(res.contains(&(
        USERS[0],
        MarketEvent::AuctionCancelled {
            nft_contract_id: 2.into(),
            token_id: 0.into(),
        }
        .encode()
    )));
}

#[test]
fn settle_auction_failures() {
    let sys = System::new();
    sys.init_logger();
    before_each_test(&sys);
    let market = sys.get_program(3);
    let res = start_auction(&market, None, 100_000, 60_000, 86_400_000);
    assert!(!res.main_failed());

    // must fail since the auction is not over
    let res = market.send(
        USERS[0],
        MarketAction::SettleAuction {
            nft_contract_id: 2.into(),
            token_id: 0.into(),
        },
    );
    assert!(res.main_failed());

    let nft = sys.get_program(2);
    let res = nft.send(
        USERS[0],
        NFTAction::Mint {
            token_metadata: TokenMetadata {
                name: "CryptoKitty".to_string(),
                description: "Description".to_string(),
                media: "http://".to_string(),
                reference: "http://".to_string(),
            },
        },
    );
    assert!(!res.main_failed());

    // lists nft on the market
    let res = market.send(
        USERS[0],
        MarketAction::AddMarketData {
            nft_contract_id: 2.into(),
            ft_contract_id: None,
            token_id: 1.into(),
            price: None,
        },
    );
    assert!(!res.main_failed());
    // must fail since the auction doesn't exist
    let res = market.send(
        USERS[0],
        MarketAction::SettleAuction {
            nft_contract_id: 2.into(),
            token_id: 1.into(),
        },
    );
    assert!(res.main_failed());
}

#[test]
fn auction_with_ft_token() {
    let sys = System::new();
    sys.init_logger();
    before_each_test(&sys);
    let ft = sys.get_program(1);
    let market = sys.get_program(3);
    let res = market.send(USERS[0], MarketAction::AddFTContract(1.into()));
    assert!(res.log().is_empty());
    let res = start_auction(&market, Some(1.into()), 10_000, 60_000, 86_400_000);
    assert!(!res.main_failed());

    // Mints tokens for users
    USERS.iter().for_each(|user| {
        let res = ft.send(*user, FTAction::Mint(100_000));
        assert!(!res.main_failed());
    });

    // Users add bids
    USERS.iter().enumerate().for_each(|(i, user)| {
        let res = bid(&market, *user, 10_100 + 100 * i as u128);
        assert!(!res.main_failed());
    });

    sys.spend_blocks(86_400_000);

    let res = market.send(
        USERS[0],
        MarketAction::SettleAuction {
            nft_contract_id: 2.into(),
            token_id: 0.into(),
        },
    );

    assert!(res.contains(&(
        USERS[0],
        MarketEvent::AuctionSettled {
            nft_contract_id: 2.into(),
            token_id: 0.into(),
            price: 10400,
        }
        .encode()
    )));

    // check the balance of treasury account
    let res = ft.send(USERS[0], FTAction::BalanceOf(TREASURY_ID.into()));
    assert!(res.contains(&(USERS[0], FTEvent::Balance(104).encode())));

    // check the balance of seller
    let res = ft.send(USERS[0], FTAction::BalanceOf(USERS[0].into()));
    println!("{:?}", res.decoded_log::<FTEvent>());
    assert!(res.contains(&(USERS[0], FTEvent::Balance(110_296).encode())));

    // check the balance of buyer
    let res = ft.send(USERS[0], FTAction::BalanceOf(USERS[3].into()));
    assert!(res.contains(&(USERS[0], FTEvent::Balance(89_600).encode())));

    // check the balances of user who don't win auctions
    let res = ft.send(USERS[0], FTAction::BalanceOf(USERS[1].into()));
    assert!(res.contains(&(USERS[0], FTEvent::Balance(100_000).encode())));

    // Checks NFT item on the marketplace
    let res = market.send(
        USERS[0],
        MarketAction::Item {
            nft_contract_id: 2.into(),
            token_id: 0.into(),
        },
    );
    assert!(res.contains(&(
        USERS[0],
        MarketEvent::ItemInfo(Item {
            owner_id: USERS[3].into(),
            ft_contract_id: Some(1.into()),
            price: None,
            auction: None,
            offers: vec![],
        })
        .encode()
    )));
}
