use codec::Encode;
use ft_io::*;
use gear_lib::non_fungible_token::token::*;
use gstd::ActorId;
use gtest::{Program, System};
use market_io::*;
use nft_io::*;
mod utils;
use nft_marketplace::offers::get_hash;
use utils::*;

fn before_each_test(sys: &System) {
    init_ft(sys);
    init_nft(sys);
    init_market(sys);
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

    let market = sys.get_program(3);
    let res = market.send(USERS[0], MarketAction::AddFTContract(1.into()));
    assert!(res.log().is_empty());
    let res = market.send(USERS[0], MarketAction::AddNftContract(2.into()));
    assert!(res.log().is_empty());
}

fn offer(market: &Program, user: u64, ft_contract_id: Option<ActorId>, price: u128) {
    let res = if ft_contract_id.is_none() {
        market.send_with_value(
            user,
            MarketAction::AddOffer {
                nft_contract_id: 2.into(),
                ft_contract_id,
                token_id: 0.into(),
                price,
            },
            price,
        )
    } else {
        market.send(
            user,
            MarketAction::AddOffer {
                nft_contract_id: 2.into(),
                ft_contract_id,
                token_id: 0.into(),
                price,
            },
        )
    };
    assert!(res.contains(&(
        user,
        MarketEvent::OfferAdded {
            nft_contract_id: 2.into(),
            ft_contract_id,
            token_id: 0.into(),
            price,
        }
        .encode()
    )));
}

//#[test]
fn add_offer() {
    let sys = System::new();
    sys.init_logger();
    before_each_test(&sys);
    let market = sys.get_program(3);
    add_market_data(&market, None, USERS[0], 0, Some(100_000));
    let mut offers = vec![];
    for i in 0..9 {
        sys.mint_to(USERS[1], 1000 * (i + 1));
        offer(&market, USERS[1], None, 1000 * (i + 1));
        let hash = get_hash(None, 1_000 * (i + 1));
        offers.push(Offer {
            hash,
            id: USERS[1].into(),
            ft_contract_id: None,
            price: 1_000 * (i + 1),
        });
    }
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
            nft_contract_id: 2.into(),
            token_id: 0.into(),
            owner_id: USERS[0].into(),
            ft_contract_id: None,
            price: Some(100_000),
            auction: None,
            offers,
        })
        .encode()
    )));
}

//#[test]
fn add_offer_with_tokens() {
    let sys = System::new();
    sys.init_logger();
    before_each_test(&sys);
    let market = sys.get_program(3);
    add_market_data(&market, None, USERS[0], 0, Some(100_000));

    let ft = sys.get_program(1);
    let res = ft.send(USERS[1], FTAction::Mint(100_000));
    assert!(!res.main_failed());
    offer(&market, USERS[1], Some(1.into()), 10_000);

    // check the market balance
    let res = ft.send(USERS[0], FTAction::BalanceOf(3.into()));
    assert!(res.contains(&(USERS[0], FTEvent::Balance(10_000).encode())));
}

//#[test]
fn add_offer_failures() {
    let sys = System::new();
    sys.init_logger();
    before_each_test(&sys);
    let market = sys.get_program(3);
    add_market_data(&market, None, USERS[0], 0, Some(100_000));

    // must fail since the fungible token contract is not approved
    let res = market.send(
        USERS[1],
        MarketAction::AddOffer {
            nft_contract_id: 2.into(),
            ft_contract_id: Some(11.into()),
            token_id: 0.into(),
            price: 0,
        },
    );
    assert!(res.main_failed());

    // must fail since the price is zero
    let res = market.send(
        USERS[1],
        MarketAction::AddOffer {
            nft_contract_id: 2.into(),
            ft_contract_id: Some(1.into()),
            token_id: 0.into(),
            price: 0,
        },
    );
    assert!(res.main_failed());

    // add offer
    let ft = sys.get_program(1);
    let res = ft.send(USERS[1], FTAction::Mint(100_000));
    assert!(!res.main_failed());
    let res = market.send(
        USERS[1],
        MarketAction::AddOffer {
            nft_contract_id: 2.into(),
            ft_contract_id: Some(1.into()),
            token_id: 0.into(),
            price: 100,
        },
    );
    assert!(!res.main_failed());

    // must fail since the offers with these params already exists
    let res = market.send(
        USERS[3],
        MarketAction::AddOffer {
            nft_contract_id: 2.into(),
            ft_contract_id: Some(1.into()),
            token_id: 0.into(),
            price: 100,
        },
    );
    assert!(res.main_failed());

    // must fail since the attached value is not equal to the offered price
    sys.mint_to(USERS[1], 10001);
    let res = market.send_with_value(
        USERS[1],
        MarketAction::AddOffer {
            nft_contract_id: 2.into(),
            ft_contract_id: None,
            token_id: 0.into(),
            price: 10000,
        },
        10001,
    );
    assert!(res.main_failed());
}

//#[test]
fn accept_offer() {
    let sys = System::new();
    sys.init_logger();
    before_each_test(&sys);

    let ft = sys.get_program(1);
    let market = sys.get_program(3);
    let res = ft.send(USERS[2], FTAction::Mint(100_000));
    assert!(!res.main_failed());
    add_market_data(&market, None, USERS[0], 0, Some(100_000));
    sys.mint_to(USERS[1], 100_000);
    offer(&market, USERS[1], None, 100_000);
    offer(&market, USERS[2], Some(1.into()), 1_000);

    let hash = get_hash(Some(1.into()), 1_000);

    let res = market.send(
        USERS[0],
        MarketAction::AcceptOffer {
            nft_contract_id: 2.into(),
            token_id: 0.into(),
            offer_hash: hash,
        },
    );
    assert!(res.contains(&(
        USERS[0],
        MarketEvent::OfferAccepted {
            nft_contract_id: 2.into(),
            token_id: 0.into(),
            new_owner: USERS[2].into(),
            price: 1_000,
        }
        .encode()
    )));

    // check the seller balance
    let res = ft.send(USERS[0], FTAction::BalanceOf(USERS[0].into()));
    assert!(res.contains(&(USERS[0], FTEvent::Balance(990).encode())));

    let offer = Offer {
        hash: get_hash(None, 100_000),
        id: USERS[1].into(),
        ft_contract_id: None,
        price: 100_000,
    };
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
            nft_contract_id: 2.into(),
            token_id: 0.into(),
            owner_id: USERS[2].into(),
            ft_contract_id: None,
            price: None,
            auction: None,
            offers: vec![offer],
        })
        .encode()
    )));

    let res = market.send(
        USERS[2],
        MarketAction::AcceptOffer {
            nft_contract_id: 2.into(),
            token_id: 0.into(),
            offer_hash: get_hash(None, 100_000),
        },
    );
    assert!(res.contains(&(
        USERS[2],
        MarketEvent::OfferAccepted {
            nft_contract_id: 2.into(),
            token_id: 0.into(),
            new_owner: USERS[1].into(),
            price: 100_000,
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
            nft_contract_id: 2.into(),
            token_id: 0.into(),
            owner_id: USERS[1].into(),
            ft_contract_id: None,
            price: None,
            auction: None,
            offers: vec![],
        })
        .encode()
    )));
}

//#[test]
fn accept_offer_failures() {
    let sys = System::new();
    sys.init_logger();
    before_each_test(&sys);

    let ft = sys.get_program(1);
    let market = sys.get_program(3);
    let res = ft.send(USERS[2], FTAction::Mint(100_000));
    assert!(!res.main_failed());
    add_market_data(&market, None, USERS[0], 0, Some(100_000));
    sys.mint_to(USERS[1], 100_000);
    offer(&market, USERS[1], None, 100_000);
    // must fail since only owner can accept offer
    let res = market.send(
        USERS[1],
        MarketAction::AcceptOffer {
            nft_contract_id: 2.into(),
            token_id: 0.into(),
            offer_hash: get_hash(Some(1.into()), 1_000),
        },
    );
    assert!(res.main_failed());

    // must fail since the offer with the indicated hash doesn't exist
    let res = market.send(
        USERS[1],
        MarketAction::AcceptOffer {
            nft_contract_id: 2.into(),
            token_id: 0.into(),
            offer_hash: get_hash(Some(1.into()), 10_000),
        },
    );
    assert!(res.main_failed());
}

//#[test]
fn withdraw() {
    let sys = System::new();
    sys.init_logger();
    before_each_test(&sys);

    let ft = sys.get_program(1);
    let market = sys.get_program(3);
    let res = ft.send(USERS[2], FTAction::Mint(100_000));
    assert!(!res.main_failed());
    add_market_data(&market, None, USERS[0], 0, Some(100_000));
    sys.mint_to(USERS[1], 100_000);
    offer(&market, USERS[1], None, 100_000);
    offer(&market, USERS[2], Some(1.into()), 1_000);

    let res = market.send(
        USERS[2],
        MarketAction::Withdraw {
            nft_contract_id: 2.into(),
            token_id: 0.into(),
            hash: get_hash(Some(1.into()), 1_000),
        },
    );
    assert!(res.contains(&(
        USERS[2],
        MarketEvent::TokensWithdrawn {
            nft_contract_id: 2.into(),
            token_id: 0.into(),
            price: 1_000,
        }
        .encode()
    )));

    // check the user balance
    let res = ft.send(USERS[0], FTAction::BalanceOf(USERS[2].into()));
    assert!(res.contains(&(USERS[0], FTEvent::Balance(100_000).encode())));

    let offer = Offer {
        hash: get_hash(None, 100_000),
        id: USERS[1].into(),
        ft_contract_id: None,
        price: 100_000,
    };
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
            nft_contract_id: 2.into(),
            token_id: 0.into(),
            owner_id: USERS[0].into(),
            ft_contract_id: None,
            price: Some(100_000),
            auction: None,
            offers: vec![offer],
        })
        .encode()
    )));

    let res = market.send(
        USERS[1],
        MarketAction::Withdraw {
            nft_contract_id: 2.into(),
            token_id: 0.into(),
            hash: get_hash(None, 100_000),
        },
    );
    assert!(res.contains(&(
        USERS[1],
        MarketEvent::TokensWithdrawn {
            nft_contract_id: 2.into(),
            token_id: 0.into(),
            price: 100_000,
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
            nft_contract_id: 2.into(),
            token_id: 0.into(),
            owner_id: USERS[0].into(),
            ft_contract_id: None,
            price: Some(100_000),
            auction: None,
            offers: vec![],
        })
        .encode()
    )));
}

//#[test]
fn withdraws_failure() {
    let sys = System::new();
    sys.init_logger();
    before_each_test(&sys);

    let ft = sys.get_program(1);
    let market = sys.get_program(3);
    let res = ft.send(USERS[2], FTAction::Mint(100_000));
    assert!(!res.main_failed());
    add_market_data(&market, None, USERS[0], 0, Some(100_000));
    sys.mint_to(USERS[1], 100_000);
    offer(&market, USERS[1], None, 100_000);
    offer(&market, USERS[2], Some(1.into()), 1_000);

    // must fail since the caller isn't the offer author
    let res = market.send(
        USERS[1],
        MarketAction::Withdraw {
            nft_contract_id: 2.into(),
            token_id: 0.into(),
            hash: get_hash(Some(1.into()), 1_000),
        },
    );
    assert!(res.main_failed());

    // must fail since the indicated offer hash doesn't exist
    let res = market.send(
        USERS[2],
        MarketAction::Withdraw {
            nft_contract_id: 2.into(),
            token_id: 0.into(),
            hash: get_hash(Some(1.into()), 1_010),
        },
    );
    assert!(res.main_failed());
}
