// use fmt::Debug;
// use gclient::{EventListener, EventProcessor, GearApi, Result};
// use gstd::{collections::BTreeMap, prelude::*, ActorId};
// use syndote::Game;
// use syndote_io::*;

// use gear_core::ids::{MessageId, ProgramId};

// const PATHS: [&str; 2] = [
//     "../target/wasm32-unknown-unknown/release/syndote_player.opt.wasm",
//     "../target/wasm32-unknown-unknown/release/syndote.opt.wasm",
// ];

// async fn upload_and_register_player(
//     client: &GearApi,
//     listener: &mut EventListener,
//     game_id: ProgramId,
// ) -> Result<()> {
//     let (message_id, program_id, _) = client
//         .upload_program_by_path(
//             PATHS[0],
//             gclient::now_micros().to_le_bytes(),
//             b"",
//             1_000_000_000,
//             0,
//         )
//         .await?;

//     assert!(listener
//         .message_processed(message_id.into())
//         .await?
//         .succeed());

//     let payload = GameAction::Register {
//         player: <[u8; 32]>::from(program_id).into(),
//     };
//     println!("Sending a payload: `{payload:?}`.");

//     let (message_id, _) = client
//         .send_message(game_id, payload, 730_000_000_000, 0)
//         .await?;

//     println!("Sending completed.");

//     let (_, raw_reply, _) = listener.reply_bytes_on(message_id).await?;

//     match raw_reply {
//         Ok(raw_reply) => {
//             let reply = <Result<GameReply, GameError>>::decode(&mut raw_reply.as_slice())?;
//             println!("Received reply {:?}", reply);
//         }
//         Err(error) => {
//             gstd::panic!("Reply received with error");
//         }
//     };
//     Ok(())
// }

// async fn make_reservation(
//     client: &GearApi,
//     listener: &mut EventListener,
//     game_id: ProgramId,
// ) -> Result<()> {
//     let (message_id, _) = client
//         .send_message(game_id, GameAction::MakeReservation, 740_000_000_000, 0)
//         .await?;

//     let (_, raw_reply, _) = listener.reply_bytes_on(message_id).await?;

//     match raw_reply {
//         Ok(raw_reply) => {
//             let reply = <Result<GameReply, GameError>>::decode(&mut raw_reply.as_slice())?;
//             println!("Received reply {:?}", reply);
//         }
//         Err(error) => {
//             gstd::panic!("Reply received with error");
//         }
//     };
//     Ok(())
// }
// #[tokio::test]
// async fn syndote() -> Result<()> {
//     let client = GearApi::dev().await?;
//     let mut listener = client.subscribe().await?;

//     let (message_id, program_id, _) = client
//         .upload_program_by_path(
//             PATHS[1],
//             gclient::now_micros().to_le_bytes(),
//             Config {
//                 reservation_amount: 700_000_000_000,
//                 reservation_duration: 1_000,
//                 time_for_step: 10,
//                 min_gas_limit: 10_000_000_000,
//             },
//             10_000_000_000,
//             0,
//         )
//         .await?;
//     assert!(listener
//         .message_processed(message_id.into())
//         .await?
//         .succeed());

//     // upload players and register them in game
//     for _i in 0..4 {
//         upload_and_register_player(&client, &mut listener, program_id).await?;
//     }

//     // make reservations
//     for _i in 0..6 {
//         make_reservation(&client, &mut listener, program_id).await?;
//     }

//     // start game
//     let (message_id, _) = client
//         .send_message(program_id, GameAction::Play, 730_000_000_000, 0)
//         .await?;

//     let (_, raw_reply, _) = listener.reply_bytes_on(message_id).await?;

//     match raw_reply {
//         Ok(raw_reply) => {
//             let reply = <Result<GameReply, GameError>>::decode(&mut raw_reply.as_slice())?;
//             println!("Received reply {:?}", reply);
//             match reply {
//                 Ok(reply) => {
//                     if reply == GameReply::NextRoundFromReservation {
//                         let state_reply: StateReply = client
//                             .read_state(program_id, StateQuery::MessageId.encode())
//                             .await
//                             .expect("Unable to read state");
//                         if let StateReply::MessageId(message_id) = state_reply {
//                             let (_, raw_reply, _) = listener
//                                 .reply_bytes_on(<[u8; 32]>::from(message_id).into())
//                                 .await?;
//                             match raw_reply {
//                                 Ok(raw_reply) => {
//                                     let reply = <Result<GameReply, GameError>>::decode(
//                                         &mut raw_reply.as_slice(),
//                                     )?;
//                                     println!("Received reply {:?}", reply);
//                                 }
//                                 Err(error) => {
//                                     gstd::panic!("Reply received with error");
//                                 }
//                             };
//                         }
//                     }
//                 }
//                 Err(_) => {}
//             }
//         }
//         Err(error) => {
//             gstd::panic!("Reply received with error");
//         }
//     };

//     Ok(())
// }
