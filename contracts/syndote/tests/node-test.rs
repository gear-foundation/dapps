use gclient::{GearApi, Result};
use gstd::prelude::*;
use syndote_io::*;
use tokio::time::{sleep, Duration};

pub mod node_utils;
use node_utils::{
    get_game_session, get_owner_id, get_player_info, make_reservation, send_balances, send_message,
    upload_and_register_players, upload_syndote, ApiUtils,
};

#[tokio::test]
async fn successfull_game() -> Result<()> {
    let client = GearApi::dev().await?;
    let mut listener = client.subscribe().await?;

    let game_id = upload_syndote(
        &client,
        &mut listener,
        Config {
            reservation_amount: 700_000_000_000,
            reservation_duration_in_block: 1_000,
            time_for_step: 10,
            min_gas_limit: 5_000_000_000,
            gas_refill_timeout: 30,
            gas_for_step: 10_000_000_000,
        },
    )
    .await?;

    // create session
    let admin_id = client.get_actor_id();
    let exp_reply: Result<GameReply, GameError> = Ok(GameReply::GameSessionCreated { admin_id });
    assert_eq!(
        Ok(exp_reply),
        send_message(
            &client,
            &mut listener,
            game_id.into(),
            GameAction::CreateGameSession { entry_fee: None },
            10_000_000_000,
            false
        )
        .await?
    );

    let actor_id_to_suri = send_balances(&client).await?;

    upload_and_register_players(&client, &mut listener, admin_id, game_id).await?;

    make_reservation(&client, &mut listener, game_id, 1, admin_id).await?;

    // start game
    client
        .send_message(game_id, GameAction::Play { admin_id }, 730_000_000_000, 0)
        .await?;

    let mut game_session: GameState = get_game_session(&client, game_id, admin_id).await?;

    while game_session.game_status != GameStatus::Finished {
        sleep(Duration::from_secs(10)).await;
        game_session = get_game_session(&client, game_id, admin_id).await?;

        println!("{:?}", game_session.game_status);
        println!("{:?}", game_session.round);
        if let GameStatus::WaitingForGasForStrategy(strategy) = game_session.game_status {
            let owner_id = get_owner_id(&client, game_id, admin_id, strategy).await?;
            let suri = actor_id_to_suri
                .get(&owner_id)
                .expect("Suri does not exist");
            let client = client.clone().with(suri)?;
            // refill gas
            let exp_reply: Result<GameReply, GameError> = Ok(GameReply::GasForPlayerStrategyAdded);
            assert_eq!(
                Ok(exp_reply),
                send_message(
                    &client,
                    &mut listener,
                    game_id.into(),
                    GameAction::AddGasToPlayerStrategy { admin_id },
                    730_000_000_000,
                    false
                )
                .await?
            );

            // continue game
            client
                .send_message(game_id, GameAction::Play { admin_id }, 730_000_000_000, 0)
                .await?;
        }
        if game_session.game_status == GameStatus::WaitingForGasForGameContract {
            // continue game
            client
                .send_message(game_id, GameAction::Play { admin_id }, 730_000_000_000, 0)
                .await?;
        }
        if game_session.game_status == GameStatus::Finished {
            println!("{:?}", game_session);
            break;
        }
    }

    Ok(())
}

// the player does not add gas and is removed from the game after a certain number of blocks
#[tokio::test]
async fn gasless_player_timeout() -> Result<()> {
    let client = GearApi::dev().await?;
    let mut listener = client.subscribe().await?;

    let game_id = upload_syndote(
        &client,
        &mut listener,
        Config {
            reservation_amount: 700_000_000_000,
            reservation_duration_in_block: 1_000,
            time_for_step: 10,
            min_gas_limit: 5_000_000_000,
            gas_refill_timeout: 10,
            gas_for_step: 10_000_000_000,
        },
    )
    .await?;

    // create session
    let admin_id = client.get_actor_id();
    let exp_reply: Result<GameReply, GameError> = Ok(GameReply::GameSessionCreated { admin_id });
    assert_eq!(
        Ok(exp_reply),
        send_message(
            &client,
            &mut listener,
            game_id.into(),
            GameAction::CreateGameSession { entry_fee: None },
            10_000_000_000,
            false
        )
        .await?
    );

    send_balances(&client).await?;

    upload_and_register_players(&client, &mut listener, admin_id, game_id).await?;

    make_reservation(&client, &mut listener, game_id, 1, admin_id).await?;

    // start game
    client
        .send_message(game_id, GameAction::Play { admin_id }, 730_000_000_000, 0)
        .await?;

    let mut game_session: GameState = get_game_session(&client, game_id, admin_id).await?;

    while game_session.game_status != GameStatus::Finished {
        sleep(Duration::from_secs(10)).await;
        game_session = get_game_session(&client, game_id, admin_id).await?;

        println!("{:?}", game_session.game_status);
        println!("{:?}", game_session.round);
        if let GameStatus::WaitingForGasForStrategy(strategy) = game_session.game_status {
            // waiting 10 blocks = 30 sec
            sleep(Duration::from_secs(30)).await;

            // check that player was excluded from the game
            let owner_id = get_owner_id(&client, game_id, admin_id, strategy).await?;
            let player_info = get_player_info(&client, game_id, admin_id, owner_id).await?;
            assert!(player_info.lost);
        }
        if game_session.game_status == GameStatus::WaitingForGasForGameContract {
            // continue game
            client
                .send_message(game_id, GameAction::Play { admin_id }, 730_000_000_000, 0)
                .await?;
        }
        if game_session.game_status == GameStatus::Finished {
            println!("{:?}", game_session);
        }
    }

    Ok(())
}
