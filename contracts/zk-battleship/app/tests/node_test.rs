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
//     let (start_vk, start_proof, start_public) = get_test_move_vk_proof_public();
//     println!("start_vk {:?}", start_vk);

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
//     let message_id = send_request!(api: &api, program_id: program_id, service_name: "Multiple", action: "CreateGame", payload: ("Name".to_string(), None::<ActorId>), value: 20_000_000_000_000);
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
//         (api.get_actor_id(), "Name".to_string(), None::<ActorId>).encode(),
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
//     let message_id = send_request!(api: &api_john, program_id: program_id, service_name: "Multiple", action: "JoinGame", payload: (api.get_actor_id(), "Name".to_string(), None::<ActorId>), value: 20_000_000_000_000);
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
//     let message_id = send_request!(api: &api, program_id: program_id, service_name: "Multiple", action: "CreateGame", payload: ("Name".to_string(), None::<ActorId>));
//     assert!(listener.message_processed(message_id).await?.succeed());
//     println!("state");
//     let state = get_state_multiple_games(&api, program_id, &mut listener).await;
//     assert!(!state.is_empty());

//     // success join to game
//     println!("join to game");
//     let api_john = get_new_client(&api, USERS_STR[0]).await;
//     let message_id = send_request!(api: &api_john, program_id: program_id, service_name: "Multiple", action: "JoinGame", payload: (api.get_actor_id(), "Name".to_string(), None::<ActorId>));
//     assert!(listener.message_processed(message_id).await?.succeed());

//     let state = get_state_multiple_games(&api, program_id, &mut listener).await;
//     assert_eq!(
//         state[0].1.status,
//         MultipleStatus::VerificationPlacement(None)
//     );
//     let info = ParticipantInfo {
//         name: "Name".to_string(),
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

//     // let state = get_state_multiple_games(&api, program_id, &mut listener).await;
//     // println!("state 1 {:?}", state);

//     // std::thread::sleep(std::time::Duration::from_secs(5));

//     // let state = get_state_multiple_games(&api, program_id, &mut listener).await;
//     // println!("state 2 {:?}", state);

//     std::thread::sleep(std::time::Duration::from_secs(120));

//     let state = get_state_multiple_games(&api, program_id, &mut listener).await;
//     println!("state 3 {:?}", state);

//     let state = get_state_games_pairs(&api, program_id, &mut listener).await;
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
//     let message_id = send_request!(api: &api, program_id: program_id, service_name: "Multiple", action: "CreateGame", payload: ("Name".to_string(), None::<ActorId>));
//     assert!(listener.message_processed(message_id).await?.succeed());
//     println!("state");
//     let state = get_state_multiple_games(&api, program_id, &mut listener).await;
//     assert!(!state.is_empty());

//     // success join to game
//     println!("join to game");
//     let api_john = get_new_client(&api, USERS_STR[0]).await;
//     let message_id = send_request!(api: &api_john, program_id: program_id, service_name: "Multiple", action: "JoinGame", payload: (api.get_actor_id(), "Name".to_string(), None::<ActorId>));
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
//             16, 144, 144, 7, 0, 101, 178, 153, 169, 227, 12, 161, 140, 43, 165, 39, 222, 22, 11,
//             237, 168, 195, 60, 142, 244, 228, 146, 18, 249, 159, 229, 199, 57, 248, 10, 6, 201,
//             136, 182, 62, 83, 152, 61, 248, 241, 221, 136, 10, 74, 92, 167, 16, 25, 161, 183, 160,
//             78, 201, 5, 176, 171, 33, 66, 86, 240, 170, 252, 55, 9, 85, 178, 175, 62, 213, 150, 50,
//             203, 78, 61, 234, 117, 187, 38, 107, 171, 55, 156, 112, 231, 57, 109, 40, 63, 177, 221,
//             16, 74, 203, 163, 250, 33, 148, 166, 75, 177, 222, 230, 119, 104, 200, 126, 195, 11,
//             67, 220, 172, 73, 140, 87, 121, 54, 8, 195, 34, 68, 164, 26, 184, 34, 52, 173, 135,
//             223, 205, 183, 180, 16, 107, 95, 45, 11, 163, 146, 20, 130, 8, 214, 182, 141, 83, 154,
//             22, 155, 217, 126, 113, 59, 84, 162, 57, 2, 89, 99, 92, 142, 199, 186, 212, 115, 219,
//             2, 68, 144, 64, 254, 121, 162, 59, 199, 125, 243, 122, 149, 89, 153, 137, 212, 127,
//             194, 207, 119, 19, 207, 195, 213, 194, 221, 182, 135, 191, 167, 165, 78, 80, 19, 20,
//             232, 206, 116, 126, 129, 141, 11, 214, 79, 70, 24, 170, 20, 195, 104, 148, 159, 198,
//             158, 31, 32, 84, 160, 11, 108, 33, 196, 234, 195, 181, 110, 25, 134, 16, 125, 115, 208,
//             103, 184, 33, 145, 251, 160, 243, 66, 81, 63, 134, 206, 160, 214, 215, 156, 36, 227,
//             112, 102, 79, 99, 29, 25, 232, 222, 221, 151, 10, 203, 233, 218, 28, 219, 102, 119,
//             190, 90, 37, 226, 97, 241, 235, 230, 20, 112, 229, 45, 27, 109, 29, 7, 76, 98, 6, 236,
//             252, 123, 81, 98, 8, 218, 17, 200, 92, 118, 68, 54, 191, 155, 100, 20, 69, 225, 169,
//             41, 219, 184, 198, 219, 181, 226, 137, 147, 205, 24, 66, 64, 93, 213, 102, 91, 12, 87,
//             253, 43, 174, 62, 64, 199, 164, 45, 145, 248, 70, 60, 37, 162, 96, 251, 159, 104, 87,
//             195, 37, 92, 35, 211, 129, 5, 226, 28, 69, 110, 238, 145, 241, 183, 195, 22, 195, 67,
//             238, 43, 176, 160, 39, 104, 50, 92, 20, 63, 38, 125, 150, 78, 231, 186, 102, 75, 215,
//             235, 60, 177, 64, 27, 214, 199, 185, 150, 196, 107, 56, 100, 128, 7, 129, 63, 207, 160,
//             127, 75, 148, 247, 61, 63, 255, 233, 139, 64, 57, 110, 137, 161, 89, 0, 151, 237, 24,
//             94, 204, 115, 162, 70, 13, 130, 88, 131, 81, 167, 250, 219, 156, 156, 71, 230, 120,
//             208, 60, 109, 107, 0, 202, 169, 231, 166, 219, 27, 246, 139, 218, 62, 118, 183, 176,
//             229, 121, 30, 194, 152, 46, 160, 101, 157, 206, 198, 25, 174, 4, 102, 150, 131, 68,
//             102, 186, 228, 26, 228, 151, 231, 236, 114, 7, 176, 225, 46, 247, 205, 107, 226, 16,
//             103, 34, 47, 237, 121, 6, 174, 89, 71, 250, 106, 77, 114, 189, 22, 155, 172, 54, 126,
//             64, 142, 15, 160, 16, 159, 119, 107, 133, 77, 202, 184, 220, 105, 88, 225, 17, 140, 21,
//             153, 149, 12, 6, 45, 218, 111, 23, 163, 177, 119, 221, 223, 125, 18, 211, 81, 206, 88,
//             247, 127, 1, 247, 4, 76, 255, 40, 58, 131, 205, 100, 192, 2, 14,
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
//             10, 209, 205, 221, 137, 128, 211, 49, 33, 72, 54, 182, 205, 239, 156, 241, 156, 10, 77,
//             20, 194, 15, 1, 43, 4, 145, 50, 211, 128, 24, 223, 156, 36, 27, 20, 20, 161, 31, 175,
//             101, 222, 10, 194, 210, 187, 80, 123, 62, 19, 200, 47, 147, 133, 220, 68, 31, 116, 170,
//             226, 133, 44, 2, 97, 63, 225, 205, 183, 250, 88, 98, 128, 92, 43, 80, 171, 19, 67, 100,
//             96, 155, 136, 129, 132, 232, 22, 203, 194, 50, 231, 27, 52, 211, 67, 181, 252, 202, 25,
//             21, 25, 220, 7, 35, 222, 6, 39, 200, 102, 49, 64, 124, 164, 180, 138, 72, 13, 174, 102,
//             184, 203, 126, 201, 237, 86, 64, 53, 132, 211, 145, 138, 244, 209, 229, 219, 57, 100,
//             214, 17, 66, 118, 30, 30, 173, 2, 122, 22, 170, 33, 40, 26, 15, 235, 3, 91, 78, 137, 4,
//             219, 169, 217, 117, 77, 205, 232, 219, 8, 155, 109, 95, 208, 83, 179, 128, 133, 184,
//             66, 212, 238, 18, 175, 111, 33, 191, 196, 101, 136, 134, 118, 222, 129, 6, 81, 77,
//         ],
//         ic: vec![
//             vec![
//                 19, 154, 246, 117, 4, 235, 132, 27, 60, 9, 188, 142, 3, 135, 198, 161, 38, 123, 19,
//                 172, 112, 142, 218, 219, 171, 183, 23, 64, 97, 95, 104, 109, 114, 253, 173, 241,
//                 147, 164, 206, 148, 206, 124, 230, 185, 251, 46, 142, 97, 13, 1, 37, 39, 180, 198,
//                 206, 238, 106, 31, 69, 143, 173, 87, 8, 69, 201, 159, 255, 241, 71, 119, 138, 227,
//                 125, 210, 43, 172, 230, 152, 245, 85, 201, 2, 23, 236, 27, 100, 177, 57, 154, 253,
//                 216, 159, 237, 47, 160, 182,
//             ],
//             vec![
//                 21, 2, 129, 214, 132, 103, 215, 26, 250, 181, 33, 178, 130, 229, 54, 78, 34, 4,
//                 170, 138, 72, 111, 195, 253, 97, 210, 255, 32, 209, 47, 67, 181, 29, 183, 229, 26,
//                 153, 213, 66, 56, 214, 233, 52, 57, 101, 189, 54, 100, 13, 95, 76, 173, 43, 220,
//                 174, 67, 255, 134, 223, 182, 23, 223, 254, 30, 211, 244, 188, 38, 33, 118, 90, 252,
//                 131, 202, 60, 19, 232, 95, 40, 68, 243, 192, 21, 34, 148, 132, 124, 253, 128, 139,
//                 146, 47, 92, 221, 57, 55,
//             ],
//             vec![
//                 1, 220, 106, 233, 82, 7, 6, 81, 60, 180, 23, 212, 68, 13, 95, 201, 249, 242, 217,
//                 26, 251, 47, 164, 190, 132, 37, 202, 196, 223, 219, 179, 64, 14, 118, 88, 107, 57,
//                 157, 136, 167, 52, 23, 143, 242, 5, 29, 125, 14, 9, 88, 229, 181, 14, 205, 91, 188,
//                 249, 87, 10, 108, 235, 32, 167, 203, 61, 243, 221, 143, 201, 15, 153, 49, 77, 213,
//                 138, 40, 53, 141, 22, 145, 227, 135, 10, 248, 45, 218, 124, 82, 13, 117, 28, 54,
//                 113, 3, 107, 200,
//             ],
//             vec![
//                 14, 229, 203, 38, 163, 118, 154, 195, 109, 163, 159, 35, 134, 155, 109, 199, 178,
//                 128, 49, 197, 187, 68, 65, 20, 211, 175, 39, 144, 65, 7, 168, 143, 247, 31, 13, 83,
//                 77, 219, 41, 29, 31, 206, 152, 78, 232, 117, 69, 213, 11, 17, 98, 18, 77, 121, 27,
//                 1, 56, 36, 187, 90, 5, 216, 134, 40, 89, 63, 164, 79, 148, 154, 202, 200, 243, 39,
//                 83, 225, 250, 213, 1, 107, 224, 137, 228, 65, 67, 91, 8, 178, 31, 176, 233, 55, 42,
//                 234, 161, 107,
//             ],
//         ],
//     };

//     let start_vk = VerifyingKeyBytes {
//         alpha_g1_beta_g2: vec![
//             127, 195, 127, 121, 155, 137, 34, 215, 138, 197, 232, 51, 200, 81, 154, 62, 43, 109,
//             64, 108, 185, 188, 150, 88, 72, 204, 174, 180, 160, 208, 183, 139, 143, 6, 146, 36,
//             223, 194, 92, 110, 49, 255, 127, 39, 186, 136, 159, 15, 224, 123, 102, 23, 176, 95, 24,
//             109, 117, 240, 208, 105, 153, 87, 43, 232, 124, 176, 86, 60, 139, 75, 213, 66, 148,
//             155, 104, 109, 127, 167, 49, 248, 187, 104, 22, 229, 136, 224, 251, 118, 151, 92, 122,
//             138, 30, 49, 171, 23, 238, 251, 234, 79, 136, 45, 132, 134, 116, 242, 30, 44, 129, 231,
//             107, 117, 193, 247, 17, 6, 36, 175, 87, 82, 25, 126, 91, 163, 227, 211, 229, 228, 142,
//             161, 172, 76, 6, 230, 50, 147, 83, 95, 221, 136, 169, 12, 151, 22, 223, 45, 167, 244,
//             247, 162, 65, 99, 60, 33, 89, 116, 104, 171, 55, 235, 215, 78, 253, 227, 255, 219, 58,
//             180, 60, 231, 19, 113, 35, 235, 185, 184, 12, 65, 215, 179, 200, 235, 161, 151, 224,
//             63, 194, 2, 249, 181, 139, 11, 154, 223, 17, 46, 87, 226, 26, 161, 177, 121, 86, 88,
//             237, 22, 29, 60, 130, 99, 51, 236, 159, 88, 88, 69, 83, 216, 17, 178, 78, 68, 63, 106,
//             24, 135, 223, 65, 20, 34, 69, 93, 217, 88, 55, 30, 247, 212, 158, 1, 142, 26, 101, 184,
//             112, 118, 93, 184, 118, 77, 155, 4, 78, 219, 125, 209, 134, 49, 78, 198, 65, 95, 184,
//             167, 240, 93, 86, 9, 213, 31, 96, 249, 129, 143, 235, 48, 224, 238, 53, 187, 111, 47,
//             111, 213, 248, 128, 31, 15, 110, 183, 37, 99, 180, 163, 47, 231, 38, 243, 248, 77, 142,
//             26, 47, 145, 79, 40, 254, 66, 171, 251, 190, 92, 220, 71, 223, 26, 99, 58, 91, 241,
//             118, 231, 219, 176, 11, 181, 20, 6, 100, 118, 136, 114, 251, 161, 207, 1, 17, 97, 252,
//             68, 35, 102, 218, 187, 103, 0, 208, 120, 211, 200, 105, 244, 100, 234, 242, 47, 192,
//             198, 201, 34, 37, 182, 177, 64, 233, 61, 89, 41, 7, 94, 145, 22, 201, 154, 229, 144,
//             57, 214, 77, 17, 167, 216, 211, 5, 7, 120, 244, 33, 255, 106, 201, 232, 83, 126, 69,
//             242, 225, 68, 37, 27, 164, 70, 181, 81, 31, 214, 120, 79, 57, 152, 147, 95, 9, 56, 106,
//             182, 127, 4, 185, 211, 42, 72, 157, 1, 209, 81, 187, 11, 222, 114, 38, 2, 95, 36, 109,
//             218, 177, 71, 73, 103, 155, 21, 118, 175, 208, 117, 236, 190, 108, 190, 108, 186, 34,
//             129, 206, 64, 202, 253, 87, 104, 97, 179, 187, 62, 196, 189, 55, 177, 222, 70, 105, 71,
//             224, 234, 46, 141, 73, 160, 0, 26, 167, 75, 3, 118, 98, 156, 65, 18, 169, 250, 47, 184,
//             140, 166, 215, 23, 183, 84, 37, 36, 222, 163, 48, 117, 71, 169, 56, 199, 242, 83, 154,
//             108, 3, 212, 176, 166, 149, 132, 87, 209, 182, 80, 177, 101, 247, 178, 207, 16, 128,
//             150, 22, 157, 151, 191, 192, 201, 238, 104, 107, 223, 99, 15, 88, 164, 61, 93, 22, 201,
//             68, 187, 79, 101, 168, 56, 46, 204, 153, 240, 157, 93, 41, 155, 85, 193, 69, 75, 67,
//             103, 33, 111, 238, 230, 135, 43, 39, 20,
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
//             20, 209, 76, 38, 165, 53, 219, 247, 74, 4, 222, 60, 143, 185, 208, 251, 244, 190, 237,
//             216, 141, 192, 219, 131, 108, 237, 179, 210, 196, 112, 40, 140, 206, 26, 216, 86, 45,
//             144, 211, 205, 147, 59, 103, 199, 93, 86, 141, 82, 22, 233, 153, 120, 211, 179, 250,
//             184, 45, 17, 204, 111, 253, 26, 224, 149, 52, 127, 1, 96, 156, 117, 102, 3, 65, 195,
//             149, 190, 106, 16, 15, 205, 40, 84, 77, 152, 43, 151, 163, 216, 115, 214, 184, 6, 167,
//             221, 254, 94, 21, 82, 88, 146, 27, 248, 0, 48, 213, 225, 94, 63, 235, 251, 102, 72, 20,
//             136, 76, 159, 242, 3, 219, 241, 75, 49, 135, 136, 88, 103, 15, 116, 186, 206, 226, 93,
//             126, 181, 192, 223, 202, 206, 116, 131, 115, 196, 253, 13, 21, 156, 9, 185, 192, 91,
//             34, 135, 78, 215, 5, 108, 44, 253, 52, 178, 29, 72, 232, 131, 14, 211, 180, 76, 243,
//             29, 7, 186, 223, 238, 245, 187, 49, 123, 230, 119, 52, 173, 247, 87, 24, 99, 80, 195,
//             98, 0, 193, 24,
//         ],
//         ic: vec![
//             vec![
//                 12, 145, 248, 226, 253, 142, 132, 49, 66, 68, 247, 180, 87, 254, 50, 200, 168, 18,
//                 160, 105, 189, 201, 170, 154, 101, 182, 173, 157, 0, 146, 97, 134, 47, 142, 74,
//                 146, 50, 164, 254, 167, 162, 157, 111, 149, 168, 187, 173, 208, 8, 199, 67, 229,
//                 179, 1, 96, 164, 105, 253, 30, 245, 255, 197, 252, 250, 246, 227, 141, 7, 231, 136,
//                 123, 197, 145, 237, 90, 49, 135, 148, 83, 87, 89, 176, 146, 221, 114, 242, 77, 42,
//                 31, 122, 215, 76, 95, 111, 86, 66,
//             ],
//             vec![
//                 6, 31, 176, 51, 11, 82, 63, 91, 101, 254, 27, 141, 154, 172, 183, 79, 216, 248, 86,
//                 196, 207, 172, 216, 92, 142, 69, 202, 205, 145, 46, 215, 4, 166, 248, 251, 233,
//                 133, 207, 89, 142, 16, 126, 220, 220, 11, 34, 144, 197, 7, 189, 179, 217, 237, 172,
//                 33, 105, 154, 20, 179, 71, 174, 124, 103, 91, 255, 205, 230, 253, 25, 203, 68, 84,
//                 123, 92, 210, 247, 65, 23, 228, 198, 6, 131, 72, 173, 227, 195, 142, 79, 101, 13,
//                 202, 201, 147, 101, 220, 236,
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
