mod utils_gclient;

use gclient::GearApi;

#[tokio::test]
async fn gclient_success() -> gclient::Result<()> {
    let nft = 1337u64;
    let nft_id = nft.into();

    let api = GearApi::dev_from_path("../target/tmp/gear").await?;

    let nft_master = utils_gclient::common::init(&api).await?;
    let state = utils_gclient::nft_master::get_state(&api, &nft_master).await?;

    assert!(state.nfts.is_empty());
    assert!(!state.operators.is_empty());

    utils_gclient::nft_master::add_nft_contract(&api, &nft_master, &nft_id, "1", false).await?;
    let state = utils_gclient::nft_master::get_state(&api, &nft_master).await?;

    assert_eq!(state.nfts[0], (nft_id, "1".to_owned()));

    Ok(())
}
