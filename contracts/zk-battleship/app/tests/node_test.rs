use battleship::services::single::{Entity, Status};
use gclient::{EventProcessor, GearApi, Result};
use gstd::{ActorId, Encode};

mod utils_gclient;
use utils_gclient::*;

#[tokio::test]
async fn gclient_success_verify() -> Result<()> {
    let api = GearApi::dev_from_path("../target/tmp/gear").await?;
    println!("start");
    //let api = GearApi::dev().await?;

    let mut listener = api.subscribe().await?;
    assert!(listener.blocks_running().await?);

    let (start_vk, start_proof, start_public) = get_start_vk_proof_public();
    let (move_vk, move_proof, move_public) = get_move_vk_proof_public();
    println!("init");
    // init
    let (message_id, program_id) = init(&api, start_vk, move_vk).await;
    assert!(listener.message_processed(message_id).await?.succeed());
    println!("start game");
    // start
    let message_id = send_request!(api: &api, program_id: program_id, service_name: "Single", action: "StartSingleGame", payload: (start_proof, start_public, None::<ActorId>));
    assert!(listener.message_processed(message_id).await?.succeed());
    println!("state");
    let state = get_state_games(&api, program_id, &mut listener).await;
    assert!(!state.is_empty());
    println!("move");
    // make move
    let message_id = send_request!(api: &api, program_id: program_id, service_name: "Single", action: "MakeMove", payload: (7_u8, None::<ActorId>));
    assert!(listener.message_processed(message_id).await?.succeed());

    let state = get_state_games(&api, program_id, &mut listener).await;
    assert_eq!(state[0].1.total_shots, 1);
    assert!(matches!(
        state[0].1.status,
        Status::PendingVerificationOfTheMove(_)
    ));

    // verify move
    println!("verify");
    let message_id = send_request!(api: &api, program_id: program_id, service_name: "Single", action: "VerifyMove", payload: (move_proof, move_public, None::<ActorId>));
    assert!(listener.message_processed(message_id).await?.succeed());

    let state = get_state_games(&api, program_id, &mut listener).await;
    assert!(matches!(state[0].1.player_board[1], Entity::BoomShip));

    Ok(())
}
