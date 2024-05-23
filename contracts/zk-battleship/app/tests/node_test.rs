use gclient::{EventProcessor, GearApi, Result};
use gstd::{ActorId, Encode};
use zk_battleship::services::single::{Entity, Status};

mod utils_gclient;
use utils_gclient::*;

#[tokio::test]
async fn gclient_success_verify() -> Result<()> {
    let api = GearApi::dev_from_path("../../target/tmp/gear").await?;
    // let api = GearApi::dev().await?;

    let mut listener = api.subscribe().await?;
    assert!(listener.blocks_running().await?);

    // init
    let (message_id, program_id) = init(&api).await;
    assert!(listener.message_processed(message_id).await?.succeed());

    // start
    let message_id = send_request!(api: &api, program_id: program_id, service_name: "Single", action: "StartSingleGame", payload: (None::<ActorId>));
    assert!(listener.message_processed(message_id).await?.succeed());

    let state = get_state_games(&api, program_id, &mut listener).await;
    assert!(!state.is_empty());

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
    let (vk, proof, public, ic) = get_vk_proof_public_ic();

    let message_id = send_request!(api: &api, program_id: program_id, service_name: "Single", action: "VerifyMove", payload: (vk, proof, public, ic, None::<ActorId>));
    assert!(listener.message_processed(message_id).await?.succeed());

    let state = get_state_games(&api, program_id, &mut listener).await;
    assert!(matches!(state[0].1.player_board[1], Entity::BoomShip));

    Ok(())
}
