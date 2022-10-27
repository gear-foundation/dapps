use ft_io::*;
use gstd::{ActorId, Encode};
use gtest::{Program, System};
use market_io::*;
use nft_io::*;

pub const USERS: &[u64] = &[4, 5, 6, 7];
pub const TREASURY_ID: u64 = 8;

pub fn init_ft(sys: &System) {
    let ft = Program::from_file(sys, "./target/fungible_token.wasm");

    let res = ft.send(
        USERS[0],
        InitConfig {
            name: String::from("MyToken"),
            symbol: String::from("MTK"),
            decimals: 18,
        },
    );

    assert!(res.log().is_empty());
}

pub fn init_nft(sys: &System) {
    sys.init_logger();
    let nft = Program::from_file(sys, "./target/nft.wasm");

    let res = nft.send(
        USERS[0],
        InitNFT {
            name: String::from("MyToken"),
            symbol: String::from("MTK"),
            base_uri: "".to_string(),
            royalties: None,
        },
    );
    assert!(res.log().is_empty());
}

pub fn init_market(sys: &System) {
    sys.init_logger();
    let market = Program::current(sys);
    let res = market.send(
        USERS[0],
        InitMarket {
            admin_id: USERS[0].into(),
            treasury_id: TREASURY_ID.into(),
            treasury_fee: 1,
        },
    );
    assert!(res.log().is_empty());
}

pub fn add_market_data(
    market: &Program,
    ft_contract_id: Option<ActorId>,
    user: u64,
    token_id: u128,
    price: Option<u128>,
) {
    // lists nft on the market
    let res = market.send(
        user,
        MarketAction::AddMarketData {
            nft_contract_id: 2.into(),
            ft_contract_id,
            token_id: token_id.into(),
            price,
        },
    );
    assert!(res.contains(&(
        user,
        MarketEvent::MarketDataAdded {
            nft_contract_id: 2.into(),
            owner: user.into(),
            token_id: token_id.into(),
            price,
        }
        .encode()
    )));
}
