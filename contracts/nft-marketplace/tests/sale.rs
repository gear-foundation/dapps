use ft_io::*;
use gear_lib::non_fungible_token::token::*;
use gstd::Encode;
use market_io::*;
use nft_io::*;

use gtest::System;
mod utils;
pub use utils::*;

fn before_each_test(sys: &System) {
    init_ft(sys);
    init_nft(sys);
    init_market(sys);
    let market = sys.get_program(3);
    let res = market.send(USERS[0], MarketAction::AddFTContract(1.into()));
    assert!(res.log().is_empty());
    let res = market.send(USERS[0], MarketAction::AddNftContract(2.into()));
    assert!(res.log().is_empty());
}

#[test]
fn buy() {
    let sys = System::new();
    sys.init_logger();
    before_each_test(&sys);
    let nft = sys.get_program(2);
    // mint nft
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
    // add nft to market
    let market = sys.get_program(3);
    add_market_data(&market, None, USERS[0], 0, Some(100_000));

    sys.mint_to(USERS[1], 100_000);
    let res = market.send_with_value(
        USERS[1],
        MarketAction::BuyItem {
            nft_contract_id: 2.into(),
            token_id: 0.into(),
        },
        100_000,
    );
    assert!(res.contains(&(
        USERS[1],
        MarketEvent::ItemSold {
            owner: USERS[1].into(),
            nft_contract_id: 2.into(),
            token_id: 0.into(),
        }
        .encode()
    )));

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
            owner_id: USERS[1].into(),
            ft_contract_id: None,
            price: None,
            auction: None,
            offers: vec![],
        })
        .encode()
    )));
}

#[test]
fn buy_with_tokens() {
    let sys = System::new();
    sys.init_logger();
    before_each_test(&sys);
    let ft = sys.get_program(1);
    // mint ft
    let res = ft.send(USERS[1], FTAction::Mint(10_000));
    assert!(!res.main_failed());
    let nft = sys.get_program(2);
    // mint nft
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
    // add nft to market
    let market = sys.get_program(3);
    add_market_data(&market, Some(1.into()), USERS[0], 0, Some(1_000));

    let res = market.send(
        USERS[1],
        MarketAction::BuyItem {
            nft_contract_id: 2.into(),
            token_id: 0.into(),
        },
    );
    assert!(res.contains(&(
        USERS[1],
        MarketEvent::ItemSold {
            owner: USERS[1].into(),
            nft_contract_id: 2.into(),
            token_id: 0.into(),
        }
        .encode()
    )));

    // check the owner balance
    let res = ft.send(USERS[0], FTAction::BalanceOf(USERS[0].into()));
    assert!(res.contains(&(USERS[0], FTEvent::Balance(990).encode())));

    // check the treasury id address
    let res = ft.send(USERS[0], FTAction::BalanceOf(TREASURY_ID.into()));
    assert!(res.contains(&(USERS[0], FTEvent::Balance(10).encode())));
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
            owner_id: USERS[1].into(),
            ft_contract_id: Some(1.into()),
            price: None,
            auction: None,
            offers: vec![],
        })
        .encode()
    )));
}
#[test]
fn buy_failures() {
    let sys = System::new();
    sys.init_logger();
    before_each_test(&sys);
    let nft = sys.get_program(2);
    let market = sys.get_program(3);
    // mint nft
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
    // must since item does not exist
    let res = market.send(
        USERS[1],
        MarketAction::BuyItem {
            nft_contract_id: 2.into(),
            token_id: 0.into(),
        },
    );
    assert!(res.main_failed());
    // must fail since item isn't on sale
    add_market_data(&market, Some(1.into()), USERS[0], 0, None);
    let res = market.send(
        USERS[1],
        MarketAction::BuyItem {
            nft_contract_id: 2.into(),
            token_id: 0.into(),
        },
    );

    assert!(res.main_failed());

    add_market_data(&market, None, USERS[0], 0, Some(1_000));
    // must fail since that the attached value is not equal to the indicated price
    sys.mint_to(USERS[1], 990);
    let res = market.send_with_value(
        USERS[1],
        MarketAction::BuyItem {
            nft_contract_id: 2.into(),
            token_id: 0.into(),
        },
        990,
    );
    assert!(res.main_failed());
}
