// use car_races_io::*;
// use gstd::{prelude::*, ActorId};
// use gtest::{Log, Program, System};
// use std::f64::consts::PI;
// use std::{thread, time::Duration};
// use wrecked::RectManager;
// const ADMIN: u64 = 100;
// #[test]
// fn run_game() {
//     let system = System::new();

//     system.init_logger();

//     let game = Program::current(&system);
//     let game_init_result = game.send(
//         ADMIN,
//         GameInit {
//             config: GameConfig {
//                 leaderboard_contract: None,
//                 ft_contract: None,
//                 nft_membership_guard: None,
//                 tokens_on_win: 100,
//                 tokens_on_draw: 200,
//                 tokens_on_lose: 300,
//             },
//         },
//     );
//     assert!(!game_init_result.main_failed());
//     let car_1 = Program::from_file(
//         &system,
//         "./target/wasm32-unknown-unknown/release/car_1.opt.wasm",
//     );
//     let car_init_result = car_1.send_bytes(ADMIN, []);
//     assert!(!car_init_result.main_failed());
//     let car_2 = Program::from_file(
//         &system,
//         "./target/wasm32-unknown-unknown/release/car_3.opt.wasm",
//     );
//     let car_init_result = car_2.send_bytes(ADMIN, []);
//     assert!(!car_init_result.main_failed());
//     let car_3 = Program::from_file(
//         &system,
//         "./target/wasm32-unknown-unknown/release/car_3.opt.wasm",
//     );
//     let car_init_result = car_3.send_bytes(ADMIN, []);
//     assert!(!car_init_result.main_failed());

//     // Add algorithm
//     let run_result = game.send(ADMIN, GameAction::AddStrategyId { car_id: 2.into() });
//     assert!(!run_result.main_failed());

//     // Add algorithm
//     let run_result = game.send(ADMIN, GameAction::AddStrategyId { car_id: 3.into() });
//     assert!(!run_result.main_failed());

//     let state: ContractState = game.read_state().expect("Unable to read state");

//     println!("state {:?}", state.games);

//     let run_result = game.send(ADMIN, GameAction::StartGame);

//     assert!(!run_result.main_failed());

//     for _i in 0..40 {
//         let run_result = game.send(
//             ADMIN,
//             GameAction::PlayerMove {
//                 strategy_action: StrategyAction::BuyShell,
//             },
//         );

//         assert!(!run_result.main_failed());

//         let state: ContractState = game.read_state().expect("Unable to read state");

//         println!("state {:?}", state.games);
//     }
// }
