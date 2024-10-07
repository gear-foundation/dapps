// use utils::prelude::*;

// mod utils;

// #[test]
// fn test() {
//     let system = utils::initialize_system();
//     let mut rockets = GalEx::initialize(&system, ADMIN);

//     let bid = 11_000_000_000_000;
//     system.mint_to(ADMIN, bid);
//     rockets
//         .create_new_session(ADMIN, "admin".to_string(), bid)
//         .succeed(0, 0);

//     for player_id in PLAYERS {
//         let player = Participant {
//             id: player_id.into(),
//             name: "player".to_string(),
//             fuel_amount: 42,
//             payload_amount: 20,
//         };
//         system.mint_to(player_id, bid);
//         rockets
//             .register(player_id, ADMIN.into(), player.clone(), bid)
//             .succeed((player_id, player), 0);
//     }

//     let state = rockets.state().expect("Unexpected invalid state.");

//     if let StageState::Registration(participants) = &state.games[0].1.stage {
//         assert_eq!(participants.len(), 3);
//     }

//     rockets
//         .start_game(ADMIN, 42, 20)
//         .succeed(PLAYERS.into_iter().chain(iter::once(ADMIN)).collect(), 3); // 3 since three players win and msg::send_with_gas is sent to them

//     let state = rockets.state().expect("Unexpected invalid state.");

//     if let StageState::Results(results) = &state.games[0].1.stage {
//         assert_eq!(results.rankings.len(), 4);
//     }
// }

// #[test]
// fn cancel_register_and_delete_player() {
//     let system = utils::initialize_system();
//     let mut rockets = GalEx::initialize(&system, ADMIN);

//     let bid = 11_000_000_000_000;
//     system.mint_to(ADMIN, bid);
//     rockets
//         .create_new_session(ADMIN, "admin".to_string(), bid)
//         .succeed(0_u128, 0);

//     for player_id in PLAYERS {
//         let player = Participant {
//             id: player_id.into(),
//             name: "player".to_string(),
//             fuel_amount: 42,
//             payload_amount: 20,
//         };
//         system.mint_to(player_id, bid);
//         rockets
//             .register(player_id, ADMIN.into(), player.clone(), bid)
//             .succeed((player_id, player), 0);
//     }

//     let state = rockets.state().expect("Unexpected invalid state.");

//     if let StageState::Registration(participants) = &state.games[0].1.stage {
//         assert_eq!(participants.len(), 3);
//     }
//     assert_eq!(state.player_to_game_id.len(), 4);

//     drop(rockets.cancel_register(PLAYERS[0]));

//     let state = rockets.state().expect("Unexpected invalid state.");

//     if let StageState::Registration(participants) = &state.games[0].1.stage {
//         assert_eq!(participants.len(), 2);
//     }
//     assert_eq!(state.player_to_game_id.len(), 3);

//     drop(rockets.delete_player(ADMIN, PLAYERS[1].into()));

//     let state = rockets.state().expect("Unexpected invalid state.");

//     if let StageState::Registration(participants) = &state.games[0].1.stage {
//         assert_eq!(participants.len(), 1);
//     }
//     assert_eq!(state.player_to_game_id.len(), 2);
// }

// #[test]
// fn errors() {
//     let system = utils::initialize_system();

//     let mut rockets = GalEx::initialize(&system, ADMIN);

//     rockets
//         .register(PLAYERS[0], ADMIN.into(), Default::default(), 0)
//         .failed(Error::NoSuchGame, 0);

//     rockets
//         .create_new_session(ADMIN, "admin".to_string(), 0)
//         .succeed(0, 0);

//     rockets
//         .register(ADMIN, ADMIN.into(), Default::default(), 0)
//         .failed(Error::SeveralRegistrations, 0);

//     rockets
//         .start_game(PLAYERS[0], 42, 20)
//         .failed(Error::NoSuchGame, 0);

//     rockets
//         .start_game(ADMIN, 42, 20)
//         .failed(Error::NotEnoughParticipants, 0);

//     for player in PLAYERS {
//         rockets
//             .register(player, ADMIN.into(), Default::default(), 0)
//             .succeed((player, Default::default()), 0);
//     }

//     rockets
//         .start_game(ADMIN, 101, 100)
//         .failed(Error::FuelOrPayloadOverload, 0);

//     rockets
//         .start_game(ADMIN, 100, 101)
//         .failed(Error::FuelOrPayloadOverload, 0);
//     rockets
//         .start_game(ADMIN, 101, 101)
//         .failed(Error::FuelOrPayloadOverload, 0);

//     rockets
//         .register(FOREIGN_USER, ADMIN.into(), Default::default(), 0)
//         .failed(Error::SessionFull, 0);
// }
