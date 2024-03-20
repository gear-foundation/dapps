mod utils_gclient;
use gclient::GearApi;
use gstd::prelude::*;
use utils_gclient::{common::*, vara_man::*};
use vara_man_io::{Level, Stage, Status};

#[tokio::test]
async fn gclient_success_play_tournament() -> gclient::Result<()> {
    let api = GearApi::dev_from_path("../target/tmp/gear").await?;
    // let api = GearApi::dev().await?;
    let vara_man_id = utils_gclient::common::init(&api).await?;
    change_status(&api, &vara_man_id, Status::StartedWithNativeToken, None).await?;

    {
        let api = api.with("//Peter")?;
        create_tournament(
            &api,
            &vara_man_id,
            "tournament_name".to_string(),
            "tournament admin".to_string(),
            Level::Easy,
            30_000,
            None,
        )
        .await?;
        let state = get_state(&api, &vara_man_id)
            .await
            .expect("Unexpected invalid state.");
        assert_eq!(state.tournaments.len(), 1);
        assert_eq!(state.players_to_game_id.len(), 1);

        let api_alex = api.clone().with("//Alex")?;
        let admin_id = get_user_to_actor_id("//Peter").await?;
        register_for_tournament(
            &api_alex,
            &vara_man_id,
            admin_id,
            "player #1".to_string(),
            10_000_000_000_000,
            None,
        )
        .await?;

        let state = get_state(&api, &vara_man_id)
            .await
            .expect("Unexpected invalid state.");
        assert_eq!(state.players_to_game_id.len(), 2);

        start_tournament(&api, &vara_man_id, None).await?;

        record_tournament_result(&api_alex, &vara_man_id, 1_000, 1, 5, None).await?;
        record_tournament_result(&api, &vara_man_id, 1_000, 1, 5, None).await?;

        let state = get_state(&api, &vara_man_id)
            .await
            .expect("Unexpected invalid state.");
        println!("State: {:?}", state);
        assert_eq!(state.tournaments[0].1.participants[0].1.points, 10);

        let old_balance = api.total_balance(api_alex.account_id()).await?;

        std::thread::sleep(std::time::Duration::from_secs(15));

        let state = get_state(&api, &vara_man_id)
            .await
            .expect("Unexpected invalid state.");
        let alex_id = get_user_to_actor_id("//Alex").await?;
        assert_eq!(
            state.tournaments[0].1.stage,
            Stage::Finished(vec![alex_id, admin_id])
        );

        let new_balance = api.total_balance(api_alex.account_id()).await?;

        assert_eq!(new_balance - old_balance, 10_000_000_000_000);
    }

    Ok(())
}
