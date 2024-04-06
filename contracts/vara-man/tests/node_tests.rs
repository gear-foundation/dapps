mod utils_gclient;

use gclient::GearApi;
use gstd::prelude::*;
use vara_man_io::{Level, Status, VaraManError};

#[tokio::test]
async fn gclient_success_register_player() -> gclient::Result<()> {
    let api = GearApi::dev_from_path("../target/tmp/gear").await?;
    let vara_man_id = utils_gclient::common::init(&api).await?;
    utils_gclient::vara_man::change_status(&api, &vara_man_id, Status::Started, None).await?;

    {
        let api = api.with("//Peter")?;
        utils_gclient::vara_man::register_player(&api, &vara_man_id, "Peter", None).await?;

        let state = utils_gclient::vara_man::get_state(&api, &vara_man_id)
            .await
            .expect("Unexpected invalid state.");
        assert!(!state.players.is_empty());
        assert!(state.games.is_empty());
        assert_eq!(state.players[0].1.name, "Peter".to_owned());
    }

    Ok(())
}

#[tokio::test]
async fn gclient_failures_register_player() -> gclient::Result<()> {
    let api = GearApi::dev_from_path("../target/tmp/gear").await?;
    let vara_man_id = utils_gclient::common::init(&api).await?;

    {
        let player_api = api.clone().with("//Peter")?;
        utils_gclient::vara_man::register_player(
            &player_api,
            &vara_man_id,
            "Peter",
            Some(VaraManError::WrongStatus),
        )
        .await?;
        utils_gclient::vara_man::change_status(&api, &vara_man_id, Status::Started, None).await?;
        utils_gclient::vara_man::register_player(
            &player_api,
            &vara_man_id,
            "",
            Some(VaraManError::EmptyName),
        )
        .await?;
        utils_gclient::vara_man::register_player(&player_api, &vara_man_id, "Peter", None).await?;
        utils_gclient::vara_man::register_player(
            &player_api,
            &vara_man_id,
            "Peter",
            Some(VaraManError::AlreadyRegistered),
        )
        .await?;
    }

    Ok(())
}

#[tokio::test]
async fn gclient_success_start_game() -> gclient::Result<()> {
    let api = GearApi::dev_from_path("../target/tmp/gear").await?;
    let vara_man_id = utils_gclient::common::init(&api).await?;
    utils_gclient::vara_man::change_status(&api, &vara_man_id, Status::Started, None).await?;

    {
        let api = api.with("//Peter")?;
        utils_gclient::vara_man::register_player(&api, &vara_man_id, "Peter", None).await?;
        utils_gclient::vara_man::start_game(&api, &vara_man_id, Level::Easy, None).await?;

        let state = utils_gclient::vara_man::get_state(&api, &vara_man_id)
            .await
            .expect("Unexpected invalid state.");
        assert_eq!(state.games.len(), 1);
    }

    Ok(())
}

#[tokio::test]
async fn gclient_failures_start_game() -> gclient::Result<()> {
    let api = GearApi::dev_from_path("../target/tmp/gear").await?;
    let vara_man_id = utils_gclient::common::init(&api).await?;

    {
        let player_api = api.clone().with("//Peter")?;
        utils_gclient::vara_man::start_game(
            &player_api,
            &vara_man_id,
            Level::Easy,
            Some(VaraManError::WrongStatus),
        )
        .await?;
        utils_gclient::vara_man::change_status(&api, &vara_man_id, Status::Started, None).await?;
        utils_gclient::vara_man::start_game(
            &player_api,
            &vara_man_id,
            Level::Easy,
            Some(VaraManError::NotRegistered),
        )
        .await?;
        utils_gclient::vara_man::register_player(&player_api, &vara_man_id, "Peter", None).await?;
        utils_gclient::vara_man::start_game(&player_api, &vara_man_id, Level::Easy, None).await?;
        utils_gclient::vara_man::start_game(
            &player_api,
            &vara_man_id,
            Level::Easy,
            Some(VaraManError::AlreadyStartGame),
        )
        .await?;
    }

    Ok(())
}

#[tokio::test]
async fn gclient_success_claim_reward() -> gclient::Result<()> {
    let api = GearApi::dev_from_path("../target/tmp/gear").await?;
    let vara_man_id = utils_gclient::common::init(&api).await?;
    utils_gclient::vara_man::change_status(&api, &vara_man_id, Status::Started, None).await?;
    let balance = api.total_balance(api.account_id()).await?;
    api.transfer(vara_man_id.encode().as_slice().into(), balance / 10)
        .await?;

    {
        let api = api.with("//Peter")?;
        utils_gclient::vara_man::register_player(&api, &vara_man_id, "Peter", None).await?;
        utils_gclient::vara_man::start_game(&api, &vara_man_id, Level::Easy, None).await?;
        utils_gclient::vara_man::claim_reward(&api, &vara_man_id, 10, 1, None).await?;

        let state = utils_gclient::vara_man::get_state(&api, &vara_man_id)
            .await
            .expect("Unexpected invalid state.");

        assert_eq!(state.players[0].1.claimed_gold_coins, 1);
        assert_eq!(state.players[0].1.claimed_silver_coins, 10);
        assert_eq!(state.players[0].1.lives, 2);
    }

    Ok(())
}

#[tokio::test]
async fn gclient_failures_claim_reward() -> gclient::Result<()> {
    let api = GearApi::dev_from_path("../target/tmp/gear").await?;
    let vara_man_id = utils_gclient::common::init(&api).await?;
    utils_gclient::vara_man::change_status(&api, &vara_man_id, Status::Started, None).await?;

    {
        let player_api = api.clone().with("//Peter")?;
        utils_gclient::vara_man::claim_reward(
            &player_api,
            &vara_man_id,
            10,
            1,
            Some(VaraManError::GameDoesNotExist),
        )
        .await?;
        utils_gclient::vara_man::register_player(&player_api, &vara_man_id, "Peter", None).await?;
        utils_gclient::vara_man::start_game(&player_api, &vara_man_id, Level::Easy, None).await?;
        utils_gclient::vara_man::change_status(&api, &vara_man_id, Status::Paused, None).await?;
        utils_gclient::vara_man::claim_reward(
            &player_api,
            &vara_man_id,
            10,
            1,
            Some(VaraManError::WrongStatus),
        )
        .await?;
        utils_gclient::vara_man::change_status(&api, &vara_man_id, Status::Started, None).await?;

        utils_gclient::vara_man::claim_reward(
            &player_api,
            &vara_man_id,
            10,
            10,
            Some(VaraManError::AmountGreaterThanAllowed),
        )
        .await?;
        utils_gclient::vara_man::claim_reward(
            &player_api,
            &vara_man_id,
            10,
            1,
            Some(VaraManError::TransferFailed),
        )
        .await?;

        let balance = api.total_balance(api.account_id()).await?;
        api.transfer(vara_man_id.encode().as_slice().into(), balance / 10)
            .await?;

        utils_gclient::vara_man::claim_reward(&player_api, &vara_man_id, 10, 1, None).await?;
        utils_gclient::vara_man::claim_reward(
            &player_api,
            &vara_man_id,
            10,
            1,
            Some(VaraManError::GameDoesNotExist),
        )
        .await?;
    }

    Ok(())
}
