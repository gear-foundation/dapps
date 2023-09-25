mod utils_gclient;

use gclient::GearApi;

#[tokio::test]
async fn gclient_success() -> gclient::Result<()> {
    let api = GearApi::dev_from_path("../target/tmp/gear").await?;

    let student_nft = utils_gclient::common::init(&api).await?;

    let state = utils_gclient::student_nft::get_state(&api, &student_nft).await?;
    assert!(state.nfts.is_empty());
    assert_eq!(state.nft_nonce, 0);
    assert!(state.nft_owners.is_empty());

    utils_gclient::student_nft::mint(&api, &student_nft, false).await?;

    let state = utils_gclient::student_nft::get_state(&api, &student_nft).await?;
    assert!(!state.nfts.is_empty());
    assert_eq!(state.nft_nonce, 1);
    assert!(!state.nft_owners.is_empty());

    Ok(())
}
