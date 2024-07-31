pub mod utils_gclient;

use gstd::prelude::*;
use nft_marketplace::BASE_PERCENT;
use utils_gclient::{
    common::{self, gear_api_from_path, init_gear_api_from_path},
    ft, marketplace,
};

// TODO: fix test
#[tokio::test]
#[ignore]
async fn gclient_success_buy_with_ft() -> gclient::Result<()> {
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
