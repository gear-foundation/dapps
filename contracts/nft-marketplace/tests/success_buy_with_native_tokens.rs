pub mod utils_gclient;

use gstd::prelude::*;
use utils_gclient::{
    common::{self, gear_api_from_path, init_gear_api_from_path},
    marketplace,
};

// TODO: fix test
#[tokio::test]
#[ignore]
async fn gclient_success_buy_with_native_tokens() -> gclient::Result<()> {
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
            common::NFT_PRICE - 1_000_000_000_000,
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
