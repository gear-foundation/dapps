// use car_races_io::*;
// use futures::executor::block_on;
// use gclient::{EventProcessor, GearApi, Result, WSAddress};
// use gear_core::ids::ProgramId;
// use gstd::{prelude::*, ActorId};
// use std::fs::read_to_string;
// use std::mem::size_of;
// pub const CAR_RACES_ID: &str = "d121314671eadf8168b761cce1228e75b3aacb06875f08a343c414dcbf91f9ab";

// pub trait ApiUtils {
//     fn get_actor_id(&self) -> ActorId;
//     fn get_specific_actor_id(&self, value: impl AsRef<str>) -> ActorId;
// }

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
// async fn start_racing_games() -> Result<()> {
//     let pid = hex::decode(CAR_RACES_ID).unwrap();
//     let pid = ProgramId::decode(&mut pid.as_slice()).unwrap();

//     let players = read_lines("./accounts_pr_keys_1.txt");
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

// async fn play_game(pid: ProgramId, accounts: &[String]) -> Result<()> {
//     let mut api = GearApi::init(WSAddress::new("wss://vit.vara-network.io", 443)).await?;

//     let accounts = accounts.to_vec();
//     let mut listener = api.subscribe().await?;

//     for (i, account) in accounts.iter().enumerate() {
//         println!("Number {:?}", i);
//         api = api
//             .clone()
//             .with(account)
//             .expect("Unable to log with indicated account");
//         let account_id = api.get_specific_actor_id(account);

//         let mut state: ContractState = Default::default();
//         let mut game;

//         state = api
//             .read_state(pid)
//             .await
//             .expect("Unable to decode Races state");
//         let (mut game_is_started, mut game_is_over) = (false, false);

//         let games_len = state.games.clone().len();
//         (game_is_started, game_is_over) =
//             if let Some((_, game)) = state.games.into_iter().find(|(id, game)| id == &account_id) {
//                 if game.state == GameState::Finished {
//                     (true, true)
//                 } else {
//                     (true, false)
//                 }
//             } else {
//                 (false, false)
//             };

//         if !game_is_started {
//             println!("SENDING MSG START GAME");
//             let (message_id, _) = api
//                 .send_message(pid, GameAction::StartGame, 100_000_000_000, 0)
//                 .await?;

//             let (_, raw_reply, returned_value) = listener.reply_bytes_on(message_id).await?;

//             match raw_reply {
//                 Ok(raw_reply) => {
//                     let decoded = GameReply::decode(&mut raw_reply.as_slice());
//                     match decoded {
//                         Ok(reply) => {
//                             if reply == GameReply::GameStarted {
//                                 println!("Game is started for {:?}", account);
//                             }
//                         }
//                         Err(error) => {
//                             println!("ERROR {:?}", error);
//                         }
//                     }
//                 }
//                 Err(error) => {
//                     println!("ERROR: {:?}", error);
//                 }
//             };

//             game_is_started = true;
//         }

//         if !game_is_over && game_is_started {
//             while true {
//                 state = api
//                     .read_state(pid)
//                     .await
//                     .expect("Unable to decode TTT state");
//                 (_, game) = state
//                     .games
//                     .into_iter()
//                     .find(|(id, game)| id == &account_id)
//                     .expect("No game for this account");

//                 if game.state == GameState::Finished {
//                     println!("Game is overwith result: {:?}", game.result);
//                     break;
//                 }

//                 println!("SENDING MSG PLAY");
//                 let gas = api
//                     .calculate_handle_gas(
//                         None,
//                         pid,
//                         GameAction::PlayerMove {
//                             strategy_action: StrategyAction::BuyAcceleration,
//                         }
//                         .encode(),
//                         0,
//                         true,
//                     )
//                     .await;
//                 match gas {
//                     Ok(gas) => {
//                         println!("Gas {:?} ", gas.burned);
//                         let (message_id, _) = api
//                             .send_message(
//                                 pid,
//                                 GameAction::PlayerMove {
//                                     strategy_action: StrategyAction::BuyAcceleration,
//                                 },
//                                 200_000_000_000,
//                                 0,
//                             )
//                             .await?;
//                     }
//                     Err(error) => {
//                         println!("Error in gas calculation {:?}", error);
//                     }
//                 }
//             }
//         }

//         println!("===============");
//         println!("NUMBER OF GAMES {:?}", games_len);
//         println!("===============");
//     }

//     Ok(())
// }
