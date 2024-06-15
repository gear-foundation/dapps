// use battleship::services::multiple::{ParticipantInfo, Status as MultipleStatus};
// use battleship::services::single::{Entity, Status};
// use battleship::services::verify::VerifyingKeyBytes;
// use gclient::{EventProcessor, GearApi, Result, WSAddress};
// use gstd::{ActorId, Encode};

// mod utils_gclient;
// use utils_gclient::*;

// #[tokio::test]
// async fn gclient_success_verify() -> Result<()> {
//     let api = GearApi::dev_from_path("../target/tmp/gear").await?;
//     println!("start");
//     // let api = GearApi::dev().await?;

//     let mut listener = api.subscribe().await?;
//     assert!(listener.blocks_running().await?);
//     // let (start_vk, start_proof, start_public) = get_test_move_vk_proof_public();
//     // println!("start_vk {:?}", start_vk);

//     let (start_vk, start_proof, start_public) = get_start_vk_proof_public();
//     let (move_vk, move_proof, move_public) = get_move_vk_proof_public();
//     println!("init");
//     // init
//     let (message_id, program_id) = init(&api, start_vk, move_vk).await;
//     assert!(listener.message_processed(message_id).await?.succeed());
//     println!("start game");
//     // start
//     let message_id = send_request!(api: &api, program_id: program_id, service_name: "Single", action: "StartSingleGame", payload: (start_proof, start_public, None::<ActorId>));
//     assert!(listener.message_processed(message_id).await?.succeed());
//     println!("state");
//     let state = get_state_single_games(&api, program_id, &mut listener).await;
//     assert!(!state.is_empty());
//     println!("move");
//     // make move
//     let message_id = send_request!(api: &api, program_id: program_id, service_name: "Single", action: "MakeMove", payload: (7_u8, None::<ActorId>));
//     assert!(listener.message_processed(message_id).await?.succeed());

//     let state = get_state_single_games(&api, program_id, &mut listener).await;
//     assert_eq!(state[0].1.total_shots, 1);
//     assert!(matches!(
//         state[0].1.status,
//         Status::PendingVerificationOfTheMove(_)
//     ));

//     // verify move
//     println!("verify");
//     let message_id = send_request!(api: &api, program_id: program_id, service_name: "Single", action: "VerifyMove", payload: (move_proof, move_public, None::<ActorId>));
//     assert!(listener.message_processed(message_id).await?.succeed());

//     let state = get_state_single_games(&api, program_id, &mut listener).await;
//     assert!(matches!(state[0].1.player_board[1], Entity::BoomShip));

//     Ok(())
// }

// #[tokio::test]
// async fn gclient_betting_check() -> Result<()> {
//     let api = GearApi::dev_from_path("../target/tmp/gear").await?;
//     println!("start");
//     //let api = GearApi::dev().await?;

//     let mut listener = api.subscribe().await?;
//     assert!(listener.blocks_running().await?);

//     let (start_vk, _start_proof, _start_public) = get_start_vk_proof_public();
//     let (move_vk, _move_proof, _move_public) = get_move_vk_proof_public();
//     println!("init");
//     // init
//     let (message_id, program_id) = init(&api, start_vk, move_vk).await;
//     assert!(listener.message_processed(message_id).await?.succeed());
//     println!("create game");
//     // create game
//     let message_id = send_request!(api: &api, program_id: program_id, service_name: "Multiple", action: "CreateGame", payload: (None::<ActorId>), value: 20_000_000_000_000);
//     assert!(listener.message_processed(message_id).await?.succeed());
//     println!("state");
//     let state = get_state_multiple_games(&api, program_id, &mut listener).await;
//     assert!(!state.is_empty());

//     // check wrong bid
//     let api_john = get_new_client(&api, USERS_STR[0]).await;
//     let initial_balance = api_john.total_balance(api_john.account_id()).await.unwrap();
//     let request = [
//         "Multiple".encode(),
//         "JoinGame".to_string().encode(),
//         (api.get_actor_id(), None::<ActorId>).encode(),
//     ]
//     .concat();

//     let (message_id, _) = api_john
//         .send_message_bytes(
//             program_id,
//             request.clone(),
//             250_000_000_000,
//             10_000_000_000_000,
//         )
//         .await?;
//     assert!(listener.message_processed(message_id).await?.failed());
//     assert!(
//         initial_balance - 10_000_000_000_000
//             < api_john.total_balance(api_john.account_id()).await.unwrap()
//     );

//     // success join to game
//     let message_id = send_request!(api: &api_john, program_id: program_id, service_name: "Multiple", action: "JoinGame", payload: (api.get_actor_id(), None::<ActorId>), value: 20_000_000_000_000);
//     assert!(listener.message_processed(message_id).await?.succeed());
//     assert!(
//         initial_balance - 20_000_000_000_000
//             > api_john.total_balance(api_john.account_id()).await.unwrap()
//     );

//     let state = get_state_multiple_games(&api, program_id, &mut listener).await;
//     assert_eq!(
//         state[0].1.status,
//         MultipleStatus::VerificationPlacement(None)
//     );
//     let info = ParticipantInfo {
//         board: vec![Entity::Unknown; 25],
//         ship_hash: Vec::new(),
//         total_shots: 0,
//         succesfull_shots: 0,
//     };
//     assert!(state[0]
//         .1
//         .participants_data
//         .contains(&(api_john.get_actor_id(), info.clone())));

//     // success leave game
//     let message_id = send_request!(api: &api_john, program_id: program_id, service_name: "Multiple", action: "LeaveGame", payload: (None::<ActorId>));
//     assert!(listener.message_processed(message_id).await?.succeed());
//     assert!(
//         initial_balance - 20_000_000_000_000
//             < api_john.total_balance(api_john.account_id()).await.unwrap()
//     );
//     let state = get_state_multiple_games(&api, program_id, &mut listener).await;
//     assert_eq!(state[0].1.status, MultipleStatus::Registration);
//     assert!(!state[0]
//         .1
//         .participants_data
//         .contains(&(api_john.get_actor_id(), info)));

//     Ok(())
// }

// #[tokio::test]
// async fn gclient_check_timing() -> Result<()> {
//     // let api = GearApi::dev_from_path("../target/tmp/gear").await?;
//     println!("start");
//     let api = GearApi::dev().await?;

//     let mut listener = api.subscribe().await?;
//     assert!(listener.blocks_running().await?);

//     let (start_vk, start_proof, start_public) = get_start_vk_proof_public();
//     let (move_vk, move_proof, move_public) = get_move_vk_proof_public();
//     println!("init");
//     // init
//     let (message_id, program_id) = init(&api, start_vk, move_vk).await;
//     assert!(listener.message_processed(message_id).await?.succeed());
//     println!("create game");
//     // create game
//     let message_id = send_request!(api: &api, program_id: program_id, service_name: "Multiple", action: "CreateGame", payload: (None::<ActorId>));
//     assert!(listener.message_processed(message_id).await?.succeed());
//     println!("state");
//     let state = get_state_multiple_games(&api, program_id, &mut listener).await;
//     assert!(!state.is_empty());

//     // success join to game
//     println!("join to game");
//     let api_john = get_new_client(&api, USERS_STR[0]).await;
//     let message_id = send_request!(api: &api_john, program_id: program_id, service_name: "Multiple", action: "JoinGame", payload: (api.get_actor_id(), None::<ActorId>));
//     assert!(listener.message_processed(message_id).await?.succeed());

//     let state = get_state_multiple_games(&api, program_id, &mut listener).await;
//     assert_eq!(
//         state[0].1.status,
//         MultipleStatus::VerificationPlacement(None)
//     );
//     let info = ParticipantInfo {
//         board: vec![Entity::Unknown; 25],
//         ship_hash: Vec::new(),
//         total_shots: 0,
//         succesfull_shots: 0,
//     };
//     assert!(state[0]
//         .1
//         .participants_data
//         .contains(&(api_john.get_actor_id(), info)));

//     // Verify Placement
//     println!("Verify Placement");
//     let message_id = send_request!(api: &api, program_id: program_id, service_name: "Multiple", action: "VerifyPlacement", payload: (start_proof.clone(), start_public.clone(), None::<ActorId>, api.get_actor_id()));
//     assert!(listener.message_processed(message_id).await?.succeed());
//     println!("Verify Placement");
//     let message_id = send_request!(api: &api_john, program_id: program_id, service_name: "Multiple", action: "VerifyPlacement", payload: (start_proof, start_public, None::<ActorId>, api.get_actor_id()));
//     assert!(listener.message_processed(message_id).await?.succeed());

//     let state = get_state_multiple_games(&api, program_id, &mut listener).await;
//     println!("state 1 {:?}", state);

//     std::thread::sleep(std::time::Duration::from_secs(5));

//     let state = get_state_multiple_games(&api, program_id, &mut listener).await;
//     println!("state 2 {:?}", state);

//     std::thread::sleep(std::time::Duration::from_secs(10));

//     let state = get_state_multiple_games(&api, program_id, &mut listener).await;
//     println!("state 3 {:?}", state);

//     Ok(())
// }

// #[tokio::test]
// async fn gclient_cleaning_state() -> Result<()> {
//     // let api = GearApi::dev_from_path("../target/tmp/gear").await?;
//     println!("start");
//     let api = GearApi::dev().await?;

//     let mut listener = api.subscribe().await?;
//     assert!(listener.blocks_running().await?);

//     let (start_vk, start_proof, start_public) = get_start_vk_proof_public();
//     let (move_vk, move_proof, move_public) = get_move_vk_proof_public();
//     println!("init");
//     // init
//     let (message_id, program_id) = init(&api, start_vk, move_vk).await;
//     assert!(listener.message_processed(message_id).await?.succeed());
//     println!("create game");
//     // create game
//     let message_id = send_request!(api: &api, program_id: program_id, service_name: "Multiple", action: "CreateGame", payload: (None::<ActorId>));
//     assert!(listener.message_processed(message_id).await?.succeed());
//     println!("state");
//     let state = get_state_multiple_games(&api, program_id, &mut listener).await;
//     assert!(!state.is_empty());

//     // success join to game
//     println!("join to game");
//     let api_john = get_new_client(&api, USERS_STR[0]).await;
//     let message_id = send_request!(api: &api_john, program_id: program_id, service_name: "Multiple", action: "JoinGame", payload: (api.get_actor_id(), None::<ActorId>));
//     assert!(listener.message_processed(message_id).await?.succeed());

//     let state = get_state_multiple_games(&api, program_id, &mut listener).await;
//     assert_eq!(
//         state[0].1.status,
//         MultipleStatus::VerificationPlacement(None)
//     );
//     assert_eq!(state[0].1.participants.1, api_john.get_actor_id());

//     // Verify Placement
//     println!("Verify Placement");
//     let message_id = send_request!(api: &api, program_id: program_id, service_name: "Multiple", action: "VerifyPlacement", payload: (start_proof.clone(), start_public.clone(), None::<ActorId>, api.get_actor_id()));
//     assert!(listener.message_processed(message_id).await?.succeed());
//     println!("Verify Placement");
//     let message_id = send_request!(api: &api_john, program_id: program_id, service_name: "Multiple", action: "VerifyPlacement", payload: (start_proof, start_public, None::<ActorId>, api.get_actor_id()));
//     assert!(listener.message_processed(message_id).await?.succeed());

//     let state = get_state_multiple_games(&api, program_id, &mut listener).await;
//     println!("state {:?}", state);

//     println!("MakeMove");
//     let message_id = send_request!(api: &api, program_id: program_id, service_name: "Multiple", action: "MakeMove", payload: (api.get_actor_id(), 8_u8, None::<ActorId>));
//     assert!(listener.message_processed(message_id).await?.succeed());

//     println!("state");
//     let state = get_state_multiple_games(&api, program_id, &mut listener).await;
//     println!("state {:?}", state);

//     std::thread::sleep(std::time::Duration::from_secs(30));

//     let state = get_state_multiple_games(&api, program_id, &mut listener).await;
//     println!("state {:?}", state);

//     Ok(())
// }

// #[tokio::test]
// async fn gclient_upload_program() -> Result<()> {
//     let mut api = GearApi::init(WSAddress::new("wss://testnet.vara.network", 443)).await?;
//     println!("start");
//     //let api = GearApi::dev().await?;

//     let mut listener = api.subscribe().await?;
//     assert!(listener.blocks_running().await?);

//     let move_vk = VerifyingKeyBytes {
//         alpha_g1_beta_g2: vec![
//             254, 15, 193, 53, 191, 149, 16, 203, 221, 95, 240, 199, 183, 229, 50, 103, 105, 101,
//             244, 237, 135, 16, 101, 138, 88, 68, 35, 85, 186, 0, 68, 61, 119, 131, 22, 123, 51,
//             202, 49, 103, 200, 167, 23, 104, 67, 23, 92, 1, 7, 9, 181, 215, 122, 217, 174, 207,
//             173, 180, 16, 206, 0, 28, 101, 190, 25, 226, 233, 121, 156, 59, 60, 50, 109, 231, 151,
//             111, 159, 74, 173, 23, 35, 49, 6, 17, 223, 236, 243, 74, 230, 197, 175, 213, 200, 230,
//             189, 19, 212, 223, 192, 131, 100, 186, 115, 144, 167, 30, 87, 99, 78, 172, 195, 188,
//             69, 230, 62, 240, 180, 130, 186, 79, 100, 212, 65, 57, 132, 224, 84, 150, 110, 139,
//             100, 186, 72, 50, 160, 250, 80, 63, 47, 81, 197, 1, 222, 0, 84, 72, 240, 98, 65, 132,
//             40, 244, 146, 145, 14, 185, 41, 87, 187, 179, 111, 200, 22, 220, 75, 123, 30, 183, 236,
//             192, 34, 156, 208, 42, 251, 173, 116, 121, 96, 86, 142, 210, 239, 195, 246, 69, 8, 240,
//             80, 6, 85, 9, 196, 218, 122, 242, 141, 231, 55, 182, 191, 184, 255, 201, 59, 86, 235,
//             16, 1, 14, 134, 168, 69, 222, 252, 34, 133, 242, 8, 46, 211, 112, 71, 123, 184, 230,
//             247, 155, 76, 212, 224, 10, 64, 172, 18, 176, 162, 44, 229, 19, 96, 44, 32, 233, 169,
//             140, 200, 188, 214, 103, 57, 14, 64, 119, 82, 171, 110, 213, 43, 198, 204, 244, 48,
//             225, 25, 202, 169, 181, 11, 86, 59, 236, 114, 142, 251, 34, 35, 232, 188, 247, 151, 66,
//             67, 82, 18, 81, 137, 3, 60, 108, 152, 155, 39, 16, 225, 45, 234, 20, 147, 165, 183,
//             179, 29, 169, 67, 52, 225, 182, 57, 190, 0, 172, 75, 155, 70, 63, 73, 214, 119, 93, 15,
//             190, 206, 182, 106, 198, 78, 74, 115, 208, 31, 48, 208, 207, 164, 17, 182, 216, 164,
//             95, 233, 59, 35, 202, 170, 233, 77, 134, 13, 21, 215, 117, 233, 137, 42, 66, 234, 3,
//             30, 112, 171, 147, 170, 50, 46, 40, 46, 28, 151, 128, 117, 251, 82, 131, 11, 140, 70,
//             38, 121, 195, 133, 204, 239, 13, 59, 101, 211, 58, 47, 65, 40, 229, 43, 38, 129, 110,
//             1, 177, 124, 147, 228, 198, 173, 157, 134, 35, 116, 23, 114, 158, 48, 177, 201, 191,
//             130, 163, 28, 55, 52, 54, 172, 232, 131, 86, 140, 84, 95, 114, 110, 93, 233, 7, 239,
//             126, 51, 175, 31, 84, 60, 105, 7, 106, 104, 250, 168, 44, 197, 222, 42, 212, 201, 157,
//             218, 165, 49, 104, 31, 162, 126, 55, 124, 66, 40, 147, 25, 46, 37, 154, 79, 244, 178,
//             41, 172, 59, 44, 81, 124, 205, 49, 10, 221, 60, 243, 46, 49, 253, 109, 15, 251, 186,
//             183, 185, 239, 28, 216, 249, 132, 193, 163, 254, 44, 163, 51, 61, 225, 137, 126, 173,
//             178, 14, 79, 1, 157, 176, 163, 57, 6, 79, 254, 41, 15, 34, 221, 45, 197, 207, 152, 15,
//             147, 47, 108, 179, 101, 223, 17, 54, 120, 135, 4, 52, 142, 6, 26, 54, 176, 24, 54, 44,
//             17, 139, 138, 213, 212, 21, 94, 178, 84, 186, 240, 26, 232, 121, 244, 50, 208, 59, 228,
//             190, 228, 95, 54, 52, 21, 244, 172, 6,
//         ],
//         gamma_g2_neg_pc: vec![
//             19, 224, 43, 96, 82, 113, 159, 96, 125, 172, 211, 160, 136, 39, 79, 101, 89, 107, 208,
//             208, 153, 32, 182, 26, 181, 218, 97, 187, 220, 127, 80, 73, 51, 76, 241, 18, 19, 148,
//             93, 87, 229, 172, 125, 5, 93, 4, 43, 126, 2, 74, 162, 178, 240, 143, 10, 145, 38, 8, 5,
//             39, 45, 197, 16, 81, 198, 228, 122, 212, 250, 64, 59, 2, 180, 81, 11, 100, 122, 227,
//             209, 119, 11, 172, 3, 38, 168, 5, 187, 239, 212, 128, 86, 200, 193, 33, 189, 184, 19,
//             250, 77, 74, 10, 216, 177, 206, 24, 110, 213, 6, 23, 137, 33, 61, 153, 57, 35, 6, 109,
//             221, 175, 16, 64, 188, 63, 245, 159, 130, 92, 120, 223, 116, 242, 215, 84, 103, 226,
//             94, 15, 85, 248, 160, 15, 160, 48, 237, 13, 27, 60, 194, 199, 2, 120, 136, 190, 81,
//             217, 239, 105, 29, 119, 188, 182, 121, 175, 218, 102, 199, 63, 23, 249, 238, 56, 55,
//             165, 80, 36, 247, 140, 113, 54, 50, 117, 167, 93, 117, 216, 107, 171, 121, 247, 71,
//             130, 170,
//         ],
//         delta_g2_neg_pc: vec![
//             10, 178, 105, 65, 233, 141, 105, 29, 113, 136, 244, 128, 137, 152, 104, 137, 154, 1,
//             235, 194, 104, 67, 90, 228, 210, 30, 252, 75, 135, 143, 190, 129, 247, 46, 249, 7, 88,
//             56, 111, 177, 141, 255, 254, 152, 153, 236, 164, 105, 17, 22, 125, 104, 84, 5, 34, 143,
//             211, 185, 170, 127, 127, 205, 189, 79, 18, 209, 140, 14, 102, 66, 20, 214, 60, 5, 163,
//             98, 121, 5, 169, 32, 174, 154, 22, 221, 234, 187, 116, 235, 12, 154, 21, 226, 187, 233,
//             216, 179, 24, 244, 231, 179, 7, 85, 217, 246, 215, 181, 26, 17, 45, 16, 63, 239, 99,
//             224, 122, 29, 240, 62, 156, 64, 179, 16, 226, 93, 92, 45, 85, 250, 2, 43, 89, 50, 76,
//             112, 19, 45, 241, 223, 218, 194, 64, 237, 61, 106, 3, 58, 58, 139, 10, 73, 250, 72,
//             148, 130, 109, 212, 155, 208, 75, 189, 254, 48, 33, 155, 187, 109, 35, 100, 65, 205,
//             46, 47, 52, 85, 47, 86, 108, 94, 236, 56, 205, 58, 20, 17, 36, 42, 75, 67, 112, 30, 87,
//             105,
//         ],
//         ic: vec![
//             vec![
//                 20, 246, 178, 74, 89, 126, 90, 110, 82, 159, 32, 251, 153, 141, 200, 251, 137, 112,
//                 195, 197, 96, 247, 105, 129, 202, 159, 73, 179, 9, 88, 11, 30, 146, 63, 60, 91,
//                 140, 5, 111, 21, 59, 250, 176, 198, 162, 89, 22, 21, 7, 136, 83, 145, 136, 58, 247,
//                 101, 214, 111, 90, 95, 161, 30, 203, 18, 44, 37, 212, 120, 107, 115, 109, 148, 111,
//                 36, 240, 2, 186, 151, 235, 215, 161, 184, 245, 228, 179, 128, 182, 133, 3, 62, 57,
//                 160, 85, 245, 198, 121,
//             ],
//             vec![
//                 13, 242, 157, 57, 82, 52, 175, 134, 238, 207, 217, 118, 250, 51, 83, 204, 86, 163,
//                 42, 231, 3, 199, 87, 48, 9, 100, 217, 130, 239, 254, 252, 178, 246, 96, 73, 153,
//                 236, 224, 182, 255, 10, 29, 59, 199, 59, 51, 214, 251, 22, 52, 221, 4, 99, 200, 93,
//                 172, 33, 104, 133, 127, 164, 66, 26, 230, 185, 5, 191, 222, 80, 86, 94, 10, 79, 69,
//                 7, 235, 165, 126, 88, 78, 214, 82, 145, 151, 235, 26, 166, 136, 110, 46, 36, 4,
//                 168, 59, 237, 117,
//             ],
//             vec![
//                 21, 78, 183, 127, 26, 178, 12, 103, 171, 73, 105, 140, 209, 174, 90, 18, 214, 248,
//                 208, 196, 205, 146, 69, 216, 42, 140, 152, 167, 135, 130, 56, 70, 65, 77, 20, 103,
//                 180, 97, 199, 10, 231, 118, 227, 254, 180, 23, 193, 148, 14, 134, 195, 185, 141,
//                 194, 93, 47, 169, 167, 62, 193, 165, 203, 235, 101, 166, 35, 9, 204, 234, 197, 78,
//                 50, 172, 186, 107, 22, 181, 255, 238, 203, 49, 199, 48, 18, 15, 176, 246, 242, 79,
//                 150, 112, 8, 54, 185, 202, 33,
//             ],
//             vec![
//                 5, 176, 25, 239, 192, 152, 102, 14, 106, 150, 46, 203, 46, 14, 43, 154, 234, 241,
//                 70, 163, 239, 2, 37, 148, 6, 246, 10, 2, 159, 226, 39, 108, 62, 48, 99, 97, 177, 6,
//                 209, 156, 99, 145, 39, 45, 231, 80, 166, 3, 13, 149, 35, 247, 116, 126, 137, 169,
//                 191, 121, 100, 240, 132, 12, 114, 172, 37, 207, 220, 72, 51, 40, 186, 130, 134, 8,
//                 169, 170, 56, 200, 249, 21, 151, 47, 128, 214, 111, 33, 132, 89, 199, 178, 192,
//                 205, 149, 167, 164, 87,
//             ],
//         ],
//     };

//     let start_vk = VerifyingKeyBytes {
//         alpha_g1_beta_g2: vec![
//             175, 8, 221, 58, 209, 112, 216, 114, 0, 173, 41, 45, 138, 92, 108, 27, 5, 113, 112, 24,
//             102, 208, 45, 87, 234, 203, 8, 164, 112, 66, 57, 140, 3, 120, 163, 211, 111, 79, 202,
//             160, 50, 198, 65, 174, 170, 112, 123, 9, 93, 68, 216, 23, 222, 233, 191, 213, 32, 18,
//             253, 63, 96, 5, 234, 126, 200, 20, 59, 139, 186, 216, 120, 64, 208, 53, 145, 183, 125,
//             90, 191, 46, 210, 75, 49, 193, 88, 86, 152, 226, 208, 231, 89, 7, 49, 50, 181, 6, 164,
//             60, 144, 117, 77, 103, 156, 159, 49, 41, 57, 112, 241, 140, 177, 168, 147, 134, 174,
//             92, 22, 58, 37, 72, 19, 225, 109, 164, 225, 78, 163, 180, 218, 90, 72, 48, 250, 143,
//             122, 16, 202, 253, 164, 107, 142, 227, 174, 14, 1, 89, 162, 228, 154, 74, 42, 1, 168,
//             252, 123, 159, 113, 12, 157, 222, 82, 102, 17, 0, 98, 49, 63, 137, 48, 231, 65, 103,
//             186, 96, 220, 194, 78, 168, 231, 225, 163, 138, 187, 226, 220, 56, 163, 242, 17, 82,
//             52, 16, 158, 137, 190, 22, 65, 87, 102, 125, 231, 82, 30, 115, 80, 125, 68, 208, 197,
//             20, 177, 185, 22, 146, 81, 154, 33, 113, 28, 149, 54, 166, 208, 144, 86, 142, 163, 50,
//             254, 74, 118, 243, 91, 17, 42, 37, 113, 206, 53, 3, 181, 162, 47, 70, 121, 41, 15, 229,
//             233, 227, 150, 215, 20, 38, 138, 158, 164, 164, 189, 133, 200, 217, 46, 160, 43, 131,
//             156, 59, 55, 70, 236, 164, 206, 0, 192, 249, 129, 231, 92, 60, 237, 168, 219, 180, 240,
//             131, 78, 3, 21, 83, 78, 192, 211, 43, 117, 97, 67, 97, 204, 232, 104, 241, 84, 230,
//             223, 48, 163, 168, 142, 189, 95, 161, 197, 210, 38, 47, 48, 69, 173, 102, 118, 99, 134,
//             146, 159, 36, 57, 210, 69, 162, 106, 135, 42, 148, 217, 15, 13, 197, 181, 225, 234,
//             142, 112, 139, 121, 144, 20, 90, 233, 1, 195, 98, 94, 73, 108, 255, 58, 89, 21, 252,
//             89, 154, 3, 48, 57, 76, 202, 129, 5, 230, 93, 215, 183, 27, 159, 215, 50, 142, 2, 138,
//             14, 83, 159, 20, 191, 161, 164, 151, 68, 166, 221, 13, 17, 149, 172, 253, 134, 37, 64,
//             250, 43, 243, 58, 127, 235, 206, 179, 188, 252, 89, 194, 199, 115, 66, 49, 70, 155, 85,
//             127, 30, 119, 42, 56, 57, 98, 85, 11, 247, 82, 11, 212, 1, 98, 8, 6, 158, 126, 83, 143,
//             60, 204, 184, 51, 128, 23, 90, 35, 132, 159, 41, 222, 225, 48, 191, 163, 163, 239, 161,
//             174, 4, 91, 186, 84, 253, 82, 219, 23, 136, 0, 66, 151, 113, 3, 199, 1, 135, 123, 163,
//             42, 13, 65, 52, 149, 104, 111, 112, 80, 119, 53, 245, 9, 60, 160, 149, 7, 18, 166, 100,
//             242, 78, 192, 233, 58, 235, 118, 162, 104, 96, 160, 103, 115, 65, 251, 99, 233, 14,
//             168, 161, 1, 127, 134, 235, 140, 215, 211, 37, 34, 18, 121, 164, 238, 60, 134, 82, 156,
//             232, 175, 101, 154, 110, 178, 13, 187, 91, 122, 7, 209, 148, 109, 251, 111, 162, 248,
//             190, 152, 231, 144, 101, 212, 182, 84, 142, 57, 248, 253, 183, 181, 208, 196, 24, 77,
//             213, 166, 67, 137, 24,
//         ],
//         gamma_g2_neg_pc: vec![
//             19, 224, 43, 96, 82, 113, 159, 96, 125, 172, 211, 160, 136, 39, 79, 101, 89, 107, 208,
//             208, 153, 32, 182, 26, 181, 218, 97, 187, 220, 127, 80, 73, 51, 76, 241, 18, 19, 148,
//             93, 87, 229, 172, 125, 5, 93, 4, 43, 126, 2, 74, 162, 178, 240, 143, 10, 145, 38, 8, 5,
//             39, 45, 197, 16, 81, 198, 228, 122, 212, 250, 64, 59, 2, 180, 81, 11, 100, 122, 227,
//             209, 119, 11, 172, 3, 38, 168, 5, 187, 239, 212, 128, 86, 200, 193, 33, 189, 184, 19,
//             250, 77, 74, 10, 216, 177, 206, 24, 110, 213, 6, 23, 137, 33, 61, 153, 57, 35, 6, 109,
//             221, 175, 16, 64, 188, 63, 245, 159, 130, 92, 120, 223, 116, 242, 215, 84, 103, 226,
//             94, 15, 85, 248, 160, 15, 160, 48, 237, 13, 27, 60, 194, 199, 2, 120, 136, 190, 81,
//             217, 239, 105, 29, 119, 188, 182, 121, 175, 218, 102, 199, 63, 23, 249, 238, 56, 55,
//             165, 80, 36, 247, 140, 113, 54, 50, 117, 167, 93, 117, 216, 107, 171, 121, 247, 71,
//             130, 170,
//         ],
//         delta_g2_neg_pc: vec![
//             2, 232, 212, 135, 176, 161, 49, 112, 69, 251, 124, 114, 89, 51, 90, 69, 43, 10, 35, 60,
//             51, 81, 193, 0, 245, 163, 83, 131, 110, 164, 1, 80, 65, 250, 183, 32, 72, 135, 24, 7,
//             172, 11, 47, 73, 46, 185, 10, 93, 5, 69, 93, 31, 11, 191, 118, 239, 192, 181, 112, 126,
//             101, 200, 64, 205, 239, 173, 20, 64, 170, 78, 118, 253, 27, 72, 106, 40, 161, 8, 115,
//             118, 188, 8, 1, 0, 171, 74, 207, 223, 17, 211, 15, 77, 152, 30, 201, 234, 14, 92, 219,
//             139, 180, 210, 192, 249, 201, 45, 160, 101, 83, 192, 201, 30, 76, 12, 237, 125, 240,
//             242, 228, 105, 152, 102, 5, 56, 93, 39, 217, 208, 223, 177, 245, 232, 4, 249, 119, 58,
//             107, 111, 235, 83, 183, 102, 180, 253, 7, 74, 68, 79, 129, 76, 142, 211, 132, 41, 169,
//             173, 159, 19, 169, 29, 199, 92, 220, 97, 23, 34, 242, 157, 126, 247, 111, 91, 27, 175,
//             107, 233, 51, 108, 43, 155, 170, 26, 53, 169, 231, 53, 165, 52, 48, 102, 150, 32,
//         ],
//         ic: vec![
//             vec![
//                 25, 201, 181, 56, 109, 217, 247, 208, 229, 102, 200, 85, 65, 15, 6, 5, 202, 210,
//                 106, 253, 15, 160, 199, 177, 15, 13, 93, 243, 231, 103, 76, 169, 242, 47, 222, 158,
//                 143, 73, 107, 13, 14, 75, 70, 126, 95, 66, 27, 3, 13, 186, 179, 216, 102, 124, 144,
//                 23, 74, 218, 126, 49, 9, 225, 181, 19, 224, 215, 58, 138, 62, 169, 91, 59, 221,
//                 126, 96, 196, 55, 136, 206, 166, 27, 31, 245, 39, 131, 29, 69, 51, 116, 152, 89,
//                 188, 82, 209, 180, 55,
//             ],
//             vec![
//                 4, 40, 219, 168, 16, 194, 150, 242, 26, 151, 39, 249, 1, 81, 108, 0, 16, 105, 45,
//                 222, 204, 50, 135, 189, 45, 19, 6, 11, 14, 229, 155, 165, 63, 67, 145, 2, 248, 17,
//                 142, 35, 201, 193, 117, 199, 85, 106, 233, 123, 20, 101, 13, 69, 140, 115, 206, 27,
//                 140, 155, 106, 12, 103, 95, 5, 232, 10, 142, 120, 62, 111, 213, 134, 105, 60, 71,
//                 238, 102, 148, 212, 81, 113, 207, 220, 102, 72, 185, 237, 240, 202, 78, 26, 185,
//                 43, 240, 44, 189, 116,
//             ],
//         ],
//     };

//     println!("init");
//     // init
//     let (message_id, program_id) = init(&api, start_vk, move_vk).await;
//     assert!(listener.message_processed(message_id).await?.succeed());
//     println!("create game");

//     Ok(())
// }
