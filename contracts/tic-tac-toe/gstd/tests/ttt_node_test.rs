// use futures::executor::block_on;
// use gclient::{EventProcessor, GearApi, Result, WSAddress};
// use gear_core::ids::ProgramId;
// use gstd::{prelude::*, ActorId};
// use std::fs::read_to_string;
// use std::mem::size_of;
// use tic_tac_toe_io::*;
// pub const TTT_ID: &str = "ab4a777ce2394b3a8700bf5f61bf0a7d8ab7c7e62f8a446594b58b72fd886ac3";
// // pub const MARKETPLACE_ID: &str = "9a8d7a221dcfbe21d140a43f4c7f4c87ee54f331992effdd9d8ab9bd54951c3d";
// // pub const LEADEARBOARD_ID: &str =
// //     "76dfd330630c1427025aa76b098dc4ae7c40b6e0701da301aac7a4bf7b06b5a4";
// // pub const FT_ID: &str = "9a8d7a221dcfbe21d140a43f4c7f4c87ee54f331992effdd9d8ab9bd54951c3d";

// // pub trait ApiUtils {
// //     fn get_actor_id(&self) -> ActorId;
// //     fn get_specific_actor_id(&self, value: impl AsRef<str>) -> ActorId;
// // }

// impl ApiUtils for GearApi {
//     fn get_actor_id(&self) -> ActorId {
//         ActorId::new(
//             self.account_id()
//                 .encode()
//                 .try_into()
//                 .expect("Unexpected invalid account id length."),
//         )
//     }

//     fn get_specific_actor_id(&self, value: impl AsRef<str>) -> ActorId {
//         let api_temp = self
//             .clone()
//             .with(value)
//             .expect("Unable to build `GearApi` instance with provided signer.");
//         api_temp.get_actor_id()
//     }
// }

// fn read_lines(filename: &str) -> Vec<String> {
//     let mut result = Vec::new();

//     for line in read_to_string(filename).unwrap().lines() {
//         result.push(line.to_string())
//     }

//     result
// }

// #[tokio::test]
// async fn start_ttt_games() -> Result<()> {
//     let pid = hex::decode(TTT_ID).unwrap();
//     let pid = ProgramId::decode(&mut pid.as_slice()).unwrap();

//     let players = read_lines("./accounts_10k.txt");
//     let n = 400;
//     let res = futures::join!(
//         play_game(pid, &players[0..n]),
//         play_game(pid, &players[n..2 * n]),
//         play_game(pid, &players[2 * n..3 * n]),
//         play_game(pid, &players[3 * n..4 * n]),
//         play_game(pid, &players[4 * n..5 * n]),
//         play_game(pid, &players[5 * n..6 * n]),
//         play_game(pid, &players[6 * n..7 * n]),
//         play_game(pid, &players[7 * n..8 * n]),
//         play_game(pid, &players[8 * n..9 * n]),
//         play_game(pid, &players[9 * n..10 * n]),
//         play_game(pid, &players[10 * n..11 * n]),
//         play_game(pid, &players[11 * n..12 * n]),
//         play_game(pid, &players[12 * n..13 * n]),
//         play_game(pid, &players[13 * n..14 * n]),
//         play_game(pid, &players[14 * n..15 * n]),
//         play_game(pid, &players[15 * n..16 * n]),
//         play_game(pid, &players[16 * n..17 * n]),
//         play_game(pid, &players[17 * n..18 * n]),
//         play_game(pid, &players[18 * n..19 * n]),
//         play_game(pid, &players[19 * n..20 * n]),
//         play_game(pid, &players[20 * n..21 * n]),
//         play_game(pid, &players[21 * n..22 * n]),
//         play_game(pid, &players[22 * n..23 * n]),
//         play_game(pid, &players[23 * n..24 * n]),
//         play_game(pid, &players[24 * n..25 * n]),
//     );

//     if let Err(error) = res.0 {
//         println!("{:?}", error);
//     }

//     Ok(())
// }

// async fn play_game(ttt_pid: ProgramId, accounts: &[String]) -> Result<()> {
//     let mut api = GearApi::init(WSAddress::new("wss://vit.vara-network.io", 443)).await?;

//     let accounts = accounts.to_vec();
//     let mut listener = api.subscribe().await?;

//     for (i, account) in accounts.iter().enumerate() {
//         println!("Number {:?}", i);
//         api = api
//             .clone()
//             .with(account)
//             .expect("Unable to log with indicated account");
//         let player_id = api.get_actor_id();

//         let mut game;

//         let reply: StateReply = api
//             .read_state(ttt_pid, StateQuery::Game { player_id }.encode())
//             .await
//             .expect("Unable to decode TTT state");
//         let (mut game_is_started, mut game_is_over) = (false, false);

//         if let StateReply::Game(game) = reply {
//             match game {
//                 Some(game) => {
//                     println!("{} SENDING MSG START GAME", i);
//                     let (message_id, _) = api
//                         .send_message(ttt_pid, GameAction::StartGame, 100_000_000_000, 0, false)
//                         .await?;

//                     let (_, raw_reply, returned_value) =
//                         listener.reply_bytes_on(message_id).await?;

//                     match raw_reply {
//                         Ok(raw_reply) => {
//                             let decoded = GameReply::decode(&mut raw_reply.as_slice());
//                             match decoded {
//                                 Ok(reply) => {
//                                     if reply == GameReply::GameStarted {
//                                         println!("Game is started for {}", i);
//                                     }
//                                 }
//                                 Err(error) => {
//                                     println!("ERROR {:?}", error);
//                                 }
//                             }
//                         }
//                         Err(error) => {
//                             println!("ERROR: {:?}", error);
//                         }
//                     };

//                     game_is_started = true;
//                 }
//                 None => {}
//             }
//         } else {
//             println!("Unexpected: received wrong state reply");
//         }

//         if !game_is_started && !in_leaderboad {}

//         if !game_is_over && game_is_started {
//             while true {
//                 state = api
//                     .read_state(ttt_pid)
//                     .await
//                     .expect("Unable to decode TTT state");
//                 (_, game) = state
//                     .current_games
//                     .into_iter()
//                     .find(|(id, game)| id == &account_id)
//                     .expect("No game for this account");

//                 if game.game_over {
//                     println!("Game is overwith result: {:?}", game.game_result);
//                     break;
//                 }
//                 let mut step = None;
//                 for (i, cell) in game.board.clone().iter().enumerate() {
//                     if cell.is_none() {
//                         step = Some(i as u8);
//                         break;
//                     }
//                 }

//                 if let Some(step) = step {
//                     println!("SENDING MSG PLAY");
//                     println!("BOARD {:?} ", game.board);
//                     println!("Step {:?} ", step);
//                     let gas = api
//                         .calculate_handle_gas(
//                             None,
//                             ttt_pid,
//                             GameAction::Turn { step }.encode(),
//                             0,
//                             true,
//                         )
//                         .await;
//                     match gas {
//                         Ok(gas) => {
//                             println!("Gas {:?} ", gas.burned);
//                             let (message_id, _) = api
//                                 .send_message(
//                                     ttt_pid,
//                                     GameAction::Turn { step },
//                                     100_000_000_000,
//                                     0,
//                                 )
//                                 .await?;
//                         }
//                         Err(_) => {
//                             let (message_id, _) = api
//                                 .send_message(ttt_pid, GameAction::Skip, 100_000_000_000, 0)
//                                 .await?;
//                         }
//                     }
//                 };
//             }
//         }

//         println!("===============");
//         println!(
//             "NUMBER OF GAMES {:?} AND NUMBER OF USERS {:?}",
//             games_len, users_len
//         );
//     }

//     Ok(())
// }

// // #[tokio::test]
// // async fn account_to_hex() -> Result<()> {
// //     let api: GearApi = GearApi::init(WSAddress::new("wss://vit.vara-network.io", 443)).await?;

// //     let minters = read_lines("./participants_10k_1");

// //     for minter in minters.iter() {
// //         //  println!("{:?}", minter);
// //         let minter_id = api.get_specific_actor_id(minter);

// //         println!("{:?}", hex::encode(minter_id));
// //     }

// //     Ok(())
// // }

// // #[tokio::test]
// // async fn calculate_storage_key() -> Result<()> {
// //     let api = GearApi::init(WSAddress::new("wss://vit.vara-network.io", 443)).await?;
// //     //let api = GearApi::dev().await?;

// //     // let address = "544704d31bcf9beec7751bb4ecc73f147d79292387eaeb6202b647cf9f4d8411";
// //     // let pid = hex::decode(address).unwrap();
// //     // let address = ActorId::decode(&mut pid.as_slice()).unwrap();
// //     // let address_bytes: [u8; 32] = address.into();

// //     let address = api.get_actor_id();
// //     let address_bytes: [u8; 32] = address.into();
// //     let i = u16::from_be_bytes(address_bytes[0..2].try_into().unwrap()) % 60;
// //     println!(
// //         "{:?}",
// //         u16::from_be_bytes(address_bytes[0..2].try_into().unwrap())
// //     );
// //     println!("{:?}", i);
// //     Ok(())
// // }

// // #[tokio::test]
// // async fn add_ttt_users() -> Result<()> {
// //     let api = GearApi::init(WSAddress::new("wss://vit.vara-network.io", 443)).await?;
// //     //let mut api = GearApi::dev().await?;

// //     let pid = hex::decode(TTT_ID).unwrap();
// //     let pid = ProgramId::decode(&mut pid.as_slice()).unwrap();

// //     let users = read_lines("./participants_10k_3");

// //     let mut messages = Vec::new();

// //     let num_of_users_per_msg = 25 as usize;
// //     let num = 400 as usize;

// //     // let state: GameState = api
// //     // .read_state(pid)
// //     // .await
// //     // .expect("Unable to decode TTT state");

// //     // println!("{:?}", state);

// //     for i in 0..num {
// //         let mut users_per_msg = Vec::new();
// //         for j in 0..num_of_users_per_msg {
// //             let actor_id = api.get_specific_actor_id(&users[i * num_of_users_per_msg + j]);
// //             let user = User {
// //                 name: String::from("aaaaaaaaaaaaaa"),
// //                 total_wins: 100,
// //                 total_games: 10000,
// //                 points: 20000,
// //                 authorised: Some(10),
// //             };
// //             users_per_msg.push((actor_id, user));
// //         }
// //         let payload = GameAction::AddTestUsers(users_per_msg).encode();
// //         let message: (ProgramId, Vec<u8>, u64, u128) = (pid, payload, 240_000_000_000, 0);
// //         messages.push(message);
// //     }

// //     let msg_list = messages.chunks(40);
// //     let rpc_nonce = api.rpc_nonce().await?;
// //     //    api.set_nonce(rpc_nonce + 1000);
// //     for (i, msgs) in msg_list.enumerate() {
// //         println!("Sending messages {}", i);
// //         api.send_message_bytes_batch(msgs.to_vec()).await?;
// //     }

// //     Ok(())
// // }
