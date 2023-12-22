use gclient::{EventProcessor, GearApi, Result};
use gstd::Encode;

use nft_io::*;
mod utils_node;
use utils_node::*;

const ALICE: [u8; 32] = [
    212, 53, 147, 199, 21, 253, 211, 28, 97, 20, 26, 189, 4, 169, 159, 214, 130, 44, 133, 88, 133,
    76, 205, 227, 154, 86, 132, 231, 165, 109, 162, 125,
];

#[tokio::test]
async fn gclient_mint_test() -> Result<()> {
    let api = GearApi::dev_from_path("../target/tmp/gear").await?;
    let mut listener = api.subscribe().await?; // Subscribing for events.

    // Checking that blocks still running.
    assert!(listener.blocks_running().await?);

    // Init
    let (message_id, program_id, _hash) = init(&api).await?;
    assert!(listener.message_processed(message_id).await?.succeed());

    // Mint
    let (message_id, _) = mint(&api, &program_id, ALICE.into()).await?;
    assert!(listener.message_processed(message_id).await?.succeed());

    // Check State
    let state = get_state(&api, &program_id)
        .await
        .expect("Unexpected invalid state.");
    assert_eq!(state.owner_by_id, [(0_u128, ALICE.into())]);
    assert_eq!(state.tokens_for_owner, [(ALICE.into(), vec![0])]);

    assert!(listener.blocks_running().await?);

    Ok(())
}

#[tokio::test]
async fn gclient_burn_test() -> Result<()> {
    let api = GearApi::dev_from_path("../target/tmp/gear").await?;
    let mut listener = api.subscribe().await?; // Subscribing for events.

    // Checking that blocks still running.
    assert!(listener.blocks_running().await?);

    // Init
    let (message_id, program_id, _hash) = init(&api).await?;
    assert!(listener.message_processed(message_id).await?.succeed());

    // Mint
    let (message_id, _) = mint(&api, &program_id, ALICE.into()).await?;
    assert!(listener.message_processed(message_id).await?.succeed());
    assert!(listener.blocks_running().await?);

    // Burn
    let (message_id, _) = burn(&api, &program_id, 0).await?;
    assert!(listener.message_processed(message_id).await?.succeed());

    // Check State
    let state = get_state(&api, &program_id)
        .await
        .expect("Unexpected invalid state.");
    assert!(state.owner_by_id.is_empty());
    assert!(state.tokens_for_owner.is_empty());

    assert!(listener.blocks_running().await?);

    Ok(())
}

#[tokio::test]
async fn gclient_transfer_test() -> Result<()> {
    let api = GearApi::dev_from_path("../target/tmp/gear").await?;
    let mut listener = api.subscribe().await?; // Subscribing for events.

    // Checking that blocks still running.
    assert!(listener.blocks_running().await?);

    // Init
    let (message_id, program_id, _hash) = init(&api).await?;
    assert!(listener.message_processed(message_id).await?.succeed());

    // Mint
    let (message_id, _) = mint(&api, &program_id, ALICE.into()).await?;
    assert!(listener.message_processed(message_id).await?.succeed());
    assert!(listener.blocks_running().await?);

    // Transfer
    let (message_id, _) = transfer(&api, &program_id, 4.into(), 0).await?;
    assert!(listener.message_processed(message_id).await?.succeed());
    assert!(listener.blocks_running().await?);

    // Check State
    let state = get_state(&api, &program_id)
        .await
        .expect("Unexpected invalid state.");
    assert_eq!(state.owner_by_id, [(0_u128, 4.into())]);
    assert_eq!(state.tokens_for_owner, [(4.into(), vec![0])]);

    assert!(listener.blocks_running().await?);

    Ok(())
}

#[tokio::test]
async fn gclient_approved() -> Result<()> {
    let api = GearApi::dev_from_path("../target/tmp/gear").await?;
    let mut listener = api.subscribe().await?; // Subscribing for events.

    // Checking that blocks still running.
    assert!(listener.blocks_running().await?);

    // Init
    let (message_id, program_id, _hash) = init(&api).await?;
    assert!(listener.message_processed(message_id).await?.succeed());

    // Mint
    let (message_id, _) = mint(&api, &program_id, ALICE.into()).await?;
    assert!(listener.message_processed(message_id).await?.succeed());
    assert!(listener.blocks_running().await?);

    // Approve
    let (message_id, _) = approve(&api, &program_id, 3.into(), 0).await?;
    assert!(listener.message_processed(message_id).await?.succeed());
    assert!(listener.blocks_running().await?);

    // Check state
    let state = get_state(&api, &program_id)
        .await
        .expect("Unexpected invalid state.");
    assert_eq!(state.token_approvals, [(0_u128, 3.into())]);

    // Transfer
    let (message_id, _) = transfer(&api, &program_id, 4.into(), 0).await?;
    assert!(listener.message_processed(message_id).await?.succeed());
    assert!(listener.blocks_running().await?);

    // Check state
    let state = get_state(&api, &program_id)
        .await
        .expect("Unexpected invalid state.");
    assert!(state.token_approvals.is_empty());

    assert!(listener.blocks_running().await?);

    Ok(())
}

#[tokio::test]
async fn gclient_owner_test() -> Result<()> {
    let api = GearApi::dev_from_path("../target/tmp/gear").await?;
    let mut listener = api.subscribe().await?; // Subscribing for events.

    // Checking that blocks still running.
    assert!(listener.blocks_running().await?);

    // Init
    let (message_id, program_id, _hash) = init(&api).await?;
    assert!(listener.message_processed(message_id).await?.succeed());

    // Mint
    let (message_id, _) = mint(&api, &program_id, ALICE.into()).await?;
    assert!(listener.message_processed(message_id).await?.succeed());
    assert!(listener.blocks_running().await?);

    let owner_payload = NftAction::GetOwner { token_id: 0 };

    let gas_info = api
        .calculate_handle_gas(None, program_id, owner_payload.encode(), 0, true)
        .await?;

    let (message_id, _) = api
        .send_message(program_id, owner_payload, gas_info.burned * 2, 0)
        .await?;

    assert!(listener.message_processed(message_id).await?.succeed());
    assert!(listener.blocks_running().await?);

    Ok(())
}

#[tokio::test]
async fn gclient_is_approved_test() -> Result<()> {
    let api = GearApi::dev_from_path("../target/tmp/gear").await?;
    let mut listener = api.subscribe().await?; // Subscribing for events.

    // Checking that blocks still running.
    assert!(listener.blocks_running().await?);

    // Init
    let (message_id, program_id, _hash) = init(&api).await?;
    assert!(listener.message_processed(message_id).await?.succeed());

    // Mint
    let (message_id, _) = mint(&api, &program_id, ALICE.into()).await?;
    assert!(listener.message_processed(message_id).await?.succeed());
    assert!(listener.blocks_running().await?);

    // Approve
    let (message_id, _) = approve(&api, &program_id, 3.into(), 0).await?;
    assert!(listener.message_processed(message_id).await?.succeed());
    assert!(listener.blocks_running().await?);

    let is_approved_payload = NftAction::CheckIfApproved {
        to: 3.into(),
        token_id: 0,
    };

    let gas_info = api
        .calculate_handle_gas(None, program_id, is_approved_payload.encode(), 0, true)
        .await?;

    let (message_id, _) = api
        .send_message(program_id, is_approved_payload, gas_info.burned * 2, 0)
        .await?;

    assert!(listener.message_processed(message_id).await?.succeed());
    assert!(listener.blocks_running().await?);

    Ok(())
}
