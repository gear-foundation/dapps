mod utils_gclient;

use gstd::prelude::*;
use nft_marketplace::contract::BASE_PERCENT;
use utils_gclient::{
    common::{self, gear_api_from_path, init_gear_api_from_path},
    ft, marketplace, nft,
};

#[tokio::test]
#[ignore]
async fn success_offers_gclient() -> gclient::Result<()> {
    let api = init_gear_api_from_path().await?;

    let (ft_contract, nft_contract, marketplace_contract) = common::init(&api).await?;

    {
        let seller_api = gear_api_from_path().with(common::SELLER)?;
        let mut listener = seller_api.subscribe().await?;

        marketplace::add_market_data(
            &seller_api,
            &mut listener,
            &marketplace_contract,
            &nft_contract,
            Some(ft_contract),
            common::TOKEN_ID.into(),
            None,
            false,
        )
        .await?;
    }

    {
        let buyer_api = gear_api_from_path().with(common::BUYER)?;
        let mut listener = buyer_api.subscribe().await?;

        for i in 0..10 {
            let offered_price = 10_000 * (i + 1) as u128;

            marketplace::add_offer(
                &buyer_api,
                &mut listener,
                &marketplace_contract,
                &nft_contract,
                None,
                common::TOKEN_ID.into(),
                offered_price,
                offered_price,
                false,
            )
            .await?;
        }

        let mut tx_id: u64 = 100;
        for i in 10..20 {
            let offered_price = 10_000 * (i + 1) as u128;

            tx_id += 1;
            ft::mint(
                &buyer_api,
                &mut listener,
                &ft_contract,
                tx_id,
                &common::get_current_actor_id(&buyer_api),
                offered_price,
            )
            .await?;
            tx_id += 1;

            let marketplace_id: common::Hash = marketplace_contract
                .encode()
                .try_into()
                .expect("Unexpected invalid program id.");

            ft::approve(
                &buyer_api,
                &mut listener,
                &ft_contract,
                tx_id,
                &marketplace_id.into(),
                offered_price,
            )
            .await?;

            marketplace::add_offer(
                &buyer_api,
                &mut listener,
                &marketplace_contract,
                &nft_contract,
                Some(ft_contract),
                common::TOKEN_ID.into(),
                offered_price,
                0,
                false,
            )
            .await?;
        }
    }

    let market_state = marketplace::state(&api, &marketplace_contract).await?;
    assert!(market_state
        .items
        .contains_key(&(nft_contract, common::TOKEN_ID.into())));

    let accepted_price = 10_000 * 15;
    {
        let seller_api = gear_api_from_path().with(common::SELLER)?;
        let mut listener = seller_api.subscribe().await?;

        marketplace::accept_offer(
            &seller_api,
            &mut listener,
            &marketplace_contract,
            &nft_contract,
            Some(ft_contract),
            common::TOKEN_ID.into(),
            accepted_price,
            false,
        )
        .await?;
    }

    let treasury_fee =
        accepted_price * ((common::TREASURY_FEE * BASE_PERCENT) as u128) / 10_000u128;

    let mut listener = api.subscribe().await?;

    assert_eq!(
        ft::balance_of(
            &api,
            &mut listener,
            &ft_contract,
            &common::get_user_to_actor_id(common::SELLER).await?
        )
        .await?,
        accepted_price - treasury_fee
    );

    assert_eq!(
        ft::balance_of(
            &api,
            &mut listener,
            &ft_contract,
            &common::get_user_to_actor_id(common::TREASURY).await?
        )
        .await?,
        treasury_fee
    );

    let market_state = marketplace::state(&api, &marketplace_contract).await?;
    assert!(!market_state
        .items
        .get(&(nft_contract, common::TOKEN_ID.into()))
        .expect("Unexpected invalid item.")
        .offers
        .contains_key(&(Some(ft_contract), accepted_price)));

    {
        let buyer_api = gear_api_from_path().with(common::BUYER)?;
        let mut listener = buyer_api.subscribe().await?;

        let withdrawn_tokens = 110_000;
        marketplace::withdraw(
            &buyer_api,
            &mut listener,
            &marketplace_contract,
            &nft_contract,
            Some(ft_contract),
            common::TOKEN_ID.into(),
            withdrawn_tokens,
            false,
        )
        .await?;

        let withdrawn_tokens = 10_000 * 2_u128;
        marketplace::withdraw(
            &buyer_api,
            &mut listener,
            &marketplace_contract,
            &nft_contract,
            None,
            common::TOKEN_ID.into(),
            withdrawn_tokens,
            false,
        )
        .await?;
    }

    let offered_value = 1_000_000;

    {
        let seller_api = gear_api_from_path().with(common::SELLER)?;
        let mut listener = seller_api.subscribe().await?;

        marketplace::add_offer(
            &seller_api,
            &mut listener,
            &marketplace_contract,
            &nft_contract,
            None,
            common::TOKEN_ID.into(),
            offered_value,
            offered_value,
            false,
        )
        .await?;
    }

    {
        let buyer_api = gear_api_from_path().with(common::BUYER)?;
        let mut listener = buyer_api.subscribe().await?;

        marketplace::accept_offer(
            &buyer_api,
            &mut listener,
            &marketplace_contract,
            &nft_contract,
            None,
            common::TOKEN_ID.into(),
            offered_value,
            false,
        )
        .await?;
    }

    Ok(())
}

#[tokio::test]
#[ignore]
async fn fail_offers_gclient() -> gclient::Result<()> {
    let api = init_gear_api_from_path().await?;

    let (ft_contract, nft_contract, marketplace_contract) = common::init(&api).await?;

    {
        let seller_api = gear_api_from_path().with(common::SELLER)?;
        let mut listener = seller_api.subscribe().await?;

        marketplace::add_market_data(
            &seller_api,
            &mut listener,
            &marketplace_contract,
            &nft_contract,
            None,
            common::TOKEN_ID.into(),
            None,
            false,
        )
        .await?;
    }

    {
        let buyer_api = gear_api_from_path().with(common::BUYER)?;
        let mut listener = buyer_api.subscribe().await?;

        marketplace::add_offer(
            &buyer_api,
            &mut listener,
            &marketplace_contract,
            &nft_contract,
            Some(ft_contract),
            common::TOKEN_ID.into(),
            0,
            0,
            true,
        )
        .await?;

        marketplace::add_offer(
            &buyer_api,
            &mut listener,
            &marketplace_contract,
            &nft_contract,
            None,
            common::TOKEN_ID.into(),
            common::NFT_PRICE,
            common::NFT_PRICE - 1000,
            true,
        )
        .await?;

        marketplace::add_offer(
            &buyer_api,
            &mut listener,
            &marketplace_contract,
            &nft_contract,
            None,
            common::TOKEN_ID.into(),
            common::NFT_PRICE,
            common::NFT_PRICE,
            false,
        )
        .await?;

        marketplace::add_offer(
            &buyer_api,
            &mut listener,
            &marketplace_contract,
            &nft_contract,
            None,
            common::TOKEN_ID.into(),
            common::NFT_PRICE,
            common::NFT_PRICE,
            true,
        )
        .await?;

        marketplace::accept_offer(
            &buyer_api,
            &mut listener,
            &marketplace_contract,
            &nft_contract,
            None,
            common::TOKEN_ID.into(),
            common::NFT_PRICE,
            true,
        )
        .await?;
    }

    {
        let seller_api = gear_api_from_path().with(common::SELLER)?;
        let mut listener = seller_api.subscribe().await?;

        marketplace::accept_offer(
            &seller_api,
            &mut listener,
            &marketplace_contract,
            &nft_contract,
            None,
            common::TOKEN_ID.into(),
            2 * common::NFT_PRICE,
            true,
        )
        .await?;

        nft::approve(
            &seller_api,
            &mut listener,
            &nft_contract,
            123,
            &marketplace_contract,
            common::TOKEN_ID.into(),
        )
        .await?;

        marketplace::create_auction(
            &seller_api,
            &mut listener,
            &marketplace_contract,
            &nft_contract,
            None,
            common::TOKEN_ID.into(),
            common::NFT_PRICE,
            common::BID_PERIOD,
            common::DURATION,
            false,
        )
        .await?;
    }

    {
        let buyer_api = gear_api_from_path().with(common::BUYER)?;
        let mut listener = buyer_api.subscribe().await?;

        marketplace::add_offer(
            &buyer_api,
            &mut listener,
            &marketplace_contract,
            &nft_contract,
            None,
            common::TOKEN_ID.into(),
            common::NFT_PRICE - 1000,
            common::NFT_PRICE - 1000,
            true,
        )
        .await?;
    }

    {
        let seller_api = gear_api_from_path().with(common::SELLER)?;
        let mut listener = seller_api.subscribe().await?;

        marketplace::accept_offer(
            &seller_api,
            &mut listener,
            &marketplace_contract,
            &nft_contract,
            None,
            common::TOKEN_ID.into(),
            common::NFT_PRICE,
            true,
        )
        .await?;

        marketplace::withdraw(
            &seller_api,
            &mut listener,
            &marketplace_contract,
            &nft_contract,
            None,
            common::TOKEN_ID.into(),
            common::NFT_PRICE,
            true,
        )
        .await?;
    }

    {
        let buyer_api = gear_api_from_path().with(common::BUYER)?;
        let mut listener = buyer_api.subscribe().await?;

        marketplace::withdraw(
            &buyer_api,
            &mut listener,
            &marketplace_contract,
            &nft_contract,
            None,
            common::TOKEN_ID.into(),
            2 * common::NFT_PRICE,
            true,
        )
        .await?;
    }

    Ok(())
}

#[tokio::test]
#[ignore]
async fn success_buy_with_ft_gclient() -> gclient::Result<()> {
    let api = init_gear_api_from_path().await?;

    let (ft_contract, nft_contract, marketplace_contract) = common::init(&api).await?;

    {
        let seller_api = gear_api_from_path().with(common::SELLER)?;
        let mut listener = seller_api.subscribe().await?;

        marketplace::add_market_data(
            &seller_api,
            &mut listener,
            &marketplace_contract,
            &nft_contract,
            Some(ft_contract),
            common::TOKEN_ID.into(),
            Some(common::NFT_PRICE),
            false,
        )
        .await?;
    }

    {
        let buyer_api = gear_api_from_path().with(common::BUYER)?;
        let mut listener = buyer_api.subscribe().await?;

        ft::mint(
            &buyer_api,
            &mut listener,
            &ft_contract,
            100,
            &common::get_current_actor_id(&buyer_api),
            common::NFT_PRICE,
        )
        .await?;

        marketplace::buy_item(
            &buyer_api,
            &mut listener,
            &marketplace_contract,
            &nft_contract,
            common::TOKEN_ID.into(),
            0,
            false,
        )
        .await?;
    }

    let mut listener = api.subscribe().await?;
    let treasury_fee =
        common::NFT_PRICE * ((common::TREASURY_FEE * BASE_PERCENT) as u128) / 10_000u128;
    assert_eq!(
        ft::balance_of(
            &api,
            &mut listener,
            &ft_contract,
            &common::get_user_to_actor_id(common::SELLER).await?,
        )
        .await?,
        common::NFT_PRICE - treasury_fee
    );

    assert_eq!(
        ft::balance_of(
            &api,
            &mut listener,
            &ft_contract,
            &common::get_user_to_actor_id(common::TREASURY).await?,
        )
        .await?,
        treasury_fee
    );

    Ok(())
}

#[tokio::test]
#[ignore]
async fn success_buy_with_native_tokens_gclient() -> gclient::Result<()> {
    let api = init_gear_api_from_path().await?;

    let (_, nft_contract, marketplace_contract) = common::init(&api).await?;

    {
        let seller_api = gear_api_from_path().with(common::SELLER)?;
        let mut listener = seller_api.subscribe().await?;

        marketplace::add_market_data(
            &seller_api,
            &mut listener,
            &marketplace_contract,
            &nft_contract,
            None,
            common::TOKEN_ID.into(),
            Some(common::NFT_PRICE),
            false,
        )
        .await?;
    }

    {
        let buyer_api = gear_api_from_path().with(common::BUYER)?;
        let mut listener = buyer_api.subscribe().await?;

        marketplace::buy_item(
            &buyer_api,
            &mut listener,
            &marketplace_contract,
            &nft_contract,
            common::TOKEN_ID.into(),
            common::NFT_PRICE - 1000,
            true,
        )
        .await?;

        marketplace::buy_item(
            &buyer_api,
            &mut listener,
            &marketplace_contract,
            &nft_contract,
            common::TOKEN_ID.into(),
            common::NFT_PRICE,
            false,
        )
        .await?;
    }

    Ok(())
}

#[tokio::test]
#[ignore]
async fn fail_buy_with_ft_gclient() -> gclient::Result<()> {
    let api = init_gear_api_from_path().await?;

    let (ft_contract, nft_contract, marketplace_contract) = common::init(&api).await?;

    {
        let buyer_api = gear_api_from_path().with(common::BUYER)?;
        let mut listener = buyer_api.subscribe().await?;

        marketplace::buy_item(
            &buyer_api,
            &mut listener,
            &marketplace_contract,
            &nft_contract,
            common::TOKEN_ID.into(),
            0,
            true,
        )
        .await?;
    }

    {
        let seller_api = gear_api_from_path().with(common::SELLER)?;
        let mut listener = seller_api.subscribe().await?;

        marketplace::add_market_data(
            &seller_api,
            &mut listener,
            &marketplace_contract,
            &nft_contract,
            Some(ft_contract),
            common::TOKEN_ID.into(),
            None,
            false,
        )
        .await?;
    }

    {
        let buyer_api = gear_api_from_path().with(common::BUYER)?;
        let mut listener = buyer_api.subscribe().await?;

        marketplace::buy_item(
            &buyer_api,
            &mut listener,
            &marketplace_contract,
            &nft_contract,
            common::TOKEN_ID.into(),
            0,
            true,
        )
        .await?;
    }

    {
        let seller_api = gear_api_from_path().with(common::SELLER)?;
        let mut listener = seller_api.subscribe().await?;

        nft::approve(
            &seller_api,
            &mut listener,
            &nft_contract,
            123,
            &marketplace_contract,
            common::TOKEN_ID.into(),
        )
        .await?;

        marketplace::create_auction(
            &seller_api,
            &mut listener,
            &marketplace_contract,
            &nft_contract,
            None,
            common::TOKEN_ID.into(),
            common::NFT_PRICE,
            common::BID_PERIOD,
            common::DURATION,
            false,
        )
        .await?;
    }

    {
        let buyer_api = gear_api_from_path().with(common::BUYER)?;
        let mut listener = buyer_api.subscribe().await?;

        marketplace::buy_item(
            &buyer_api,
            &mut listener,
            &marketplace_contract,
            &nft_contract,
            common::TOKEN_ID.into(),
            0,
            true,
        )
        .await?;
    }

    Ok(())
}
