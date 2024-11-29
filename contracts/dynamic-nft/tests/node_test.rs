// use gclient::{EventProcessor, GearApi, Result};
// use sails_rs::{ActorId, Decode, Encode, U256};
// mod utils_gclient;
// use extended_vnft_client::TokenMetadata;
// use utils_gclient::*;

// #[tokio::test]
// #[ignore]
// async fn test_basic_function() -> Result<()> {
//     let api = GearApi::dev_from_path("../target/tmp/gear").await?;
//     let john_api = get_new_client(&api, USERS_STR[0]).await;

//     let mut listener = api.subscribe().await?;
//     assert!(listener.blocks_running().await?);

//     // Init
//     let (message_id, program_id) = init(&api).await;
//     assert!(listener.message_processed(message_id).await?.succeed());
//     // Mint
//     let actor = api.get_actor_id();
//     let metadata = TokenMetadata {
//         name: "token_name".to_string(),
//         description: "token_description".to_string(),
//         media: "token_media".to_string(),
//         reference: "token_reference".to_string(),
//     };
//     let message_id = send_request!(api: &api, program_id: program_id, service_name: "Vnft", action: "Mint", payload: (actor, metadata));
//     assert!(listener.message_processed(message_id).await?.succeed());
//     // Check Balance
//     let balance_value = get_state!(api: &api, listener: listener, program_id: program_id, service_name: "Vnft", action: "BalanceOf", return_type: U256, payload: (actor));
//     assert_eq!(balance_value, 1.into());
//     // Check owner
//     let token_id: U256 = 0.into();
//     let owner = get_state!(api: &api, listener: listener, program_id: program_id, service_name: "Vnft", action: "OwnerOf", return_type: ActorId, payload: (token_id));
//     assert_eq!(actor, owner);

//     // Transfer
//     let john_actor_id = john_api.get_actor_id();
//     let message_id = send_request!(api: &api, program_id: program_id, service_name: "Vnft", action: "Transfer", payload: (john_actor_id, token_id));
//     assert!(listener.message_processed(message_id).await?.succeed());
//     // Check owner
//     let owner = get_state!(api: &api, listener: listener, program_id: program_id, service_name: "Vnft", action: "OwnerOf", return_type: ActorId, payload: (token_id));
//     assert_eq!(john_actor_id, owner);

//     // Approve
//     let message_id = send_request!(api: &john_api, program_id: program_id, service_name: "Vnft", action: "Approve", payload: (actor, token_id));
//     assert!(listener.message_processed(message_id).await?.succeed());

//     // TransferFrom
//     let message_id = send_request!(api: &api, program_id: program_id, service_name: "Vnft", action: "TransferFrom", payload: (john_actor_id, actor, token_id));
//     assert!(listener.message_processed(message_id).await?.succeed());
//     // Check owner
//     let owner = get_state!(api: &api, listener: listener, program_id: program_id, service_name: "Vnft", action: "OwnerOf", return_type: ActorId, payload: (token_id));
//     assert_eq!(actor, owner);

//     // Burn
//     let message_id = send_request!(api: &api, program_id: program_id, service_name: "Vnft", action: "Burn", payload: (actor, token_id));
//     assert!(listener.message_processed(message_id).await?.succeed());
//     // Check Balance
//     let balance_value = get_state!(api: &api, listener: listener, program_id: program_id, service_name: "Vnft", action: "BalanceOf", return_type: U256, payload: (actor));
//     assert_eq!(balance_value, 0.into());
//     // Check owner
//     let owner = get_state!(api: &api, listener: listener, program_id: program_id, service_name: "Vnft", action: "OwnerOf", return_type: ActorId, payload: (token_id));
//     assert_eq!(ActorId::zero(), owner);
//     Ok(())
// }
