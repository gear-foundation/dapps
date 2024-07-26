pub mod utils_gclient;

use gstd::prelude::*;
use utils_gclient::{
    common::{self, gear_api_from_path, init_gear_api_from_path},
    marketplace, nft,
};

// TODO: fix test
#[tokio::test]
#[ignore]
async fn gclient_fail_buy_with_ft() -> gclient::Result<()> {
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
