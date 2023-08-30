mod utils_gclient;

use gclient::GearApi;
use gstd::prelude::*;
use vara_man_io::{Level, Status};

#[tokio::test]
#[ignore]
async fn success_register_player() -> gclient::Result<()> {
    let api = GearApi::dev_from_path("../target/tmp/gear").await?;

    let vara_man_id = utils_gclient::common::init(&api).await?;
    utils_gclient::vara_man::change_status(&api, &vara_man_id, Status::Started, false).await?;

    {
        let api = api.with("//Peter")?;
        utils_gclient::vara_man::register_player(&api, &vara_man_id, "Peter", false).await?;

        let state = utils_gclient::vara_man::get_state(&api, &vara_man_id).await?;

        assert!(!state.players.is_empty());
        assert!(state.games.is_empty());
        assert_eq!(state.players[0].1.name, "Peter".to_owned());
    }

    Ok(())
}

#[tokio::test]
#[ignore]
async fn success_start_game() -> gclient::Result<()> {
    let api = GearApi::dev_from_path("../target/tmp/gear").await?;

    let vara_man_id = utils_gclient::common::init(&api).await?;
    utils_gclient::vara_man::change_status(&api, &vara_man_id, Status::Started, false).await?;

    {
        let api = api.with("//Peter")?;
        utils_gclient::vara_man::register_player(&api, &vara_man_id, "Peter", false).await?;
        utils_gclient::vara_man::start_game(&api, &vara_man_id, Level::Easy, u64::MAX, false)
            .await?;

        let state = utils_gclient::vara_man::get_state(&api, &vara_man_id).await?;
        assert_eq!(state.games.len(), 1);
    }

    Ok(())
}
