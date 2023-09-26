// TODO: fix tests.

// mod utils;
// use crate::utils::ADMIN;
// use gstd::ActorId;
// use gtest::{Program, System};
// use utils::VaraMan;
// use vara_man_io::{Player, Status};

// #[test]
// fn success() {
//     let system = System::new();
//     system.init_logger();

//     let vara_man = Program::vara_man(&system);
//     vara_man.change_status(ADMIN, Status::Started);

//     let state = vara_man.get_state();
//     assert!(state.players.is_empty());

//     let player_0_id: ActorId = utils::PLAYERS[0].into();
//     let player_1_id: ActorId = utils::PLAYERS[1].into();
//     let player_2_id: ActorId = utils::PLAYERS[2].into();

//     vara_man.register_player(utils::PLAYERS[0], "John", false);
//     vara_man.register_player(utils::PLAYERS[1], "Jack", false);
//     vara_man.register_player(utils::PLAYERS[2], "James", false);

//     let state = vara_man.get_state();

//     assert_eq!(state.players.len(), 3);
//     assert!(state.players.contains(&(
//         player_0_id,
//         Player {
//             name: "John".to_owned(),
//             retries: 0,
//             claimed_gold_coins: 0,
//             claimed_silver_coins: 0,
//         }
//     )));
//     assert!(state.players.contains(&(
//         player_1_id,
//         Player {
//             name: "Jack".to_owned(),
//             retries: 0,
//             claimed_gold_coins: 0,
//             claimed_silver_coins: 0,
//         }
//     )));
//     assert!(state.players.contains(&(
//         player_2_id,
//         Player {
//             name: "James".to_owned(),
//             retries: 0,
//             claimed_gold_coins: 0,
//             claimed_silver_coins: 0,
//         }
//     )));
// }

// #[test]
// fn fail_player_already_registered() {
//     let system = System::new();
//     system.init_logger();

//     let vara_man = Program::vara_man(&system);
//     vara_man.change_status(ADMIN, Status::Started);

//     let state = vara_man.get_state();
//     assert!(state.players.is_empty());

//     vara_man.register_player(utils::PLAYERS[0], "John", false);
//     vara_man.register_player(utils::PLAYERS[0], "John", true);

//     let state = vara_man.get_state();
//     assert_eq!(state.players.len(), 1);
// }
