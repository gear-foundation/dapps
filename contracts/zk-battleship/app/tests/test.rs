// use battleship::services::single::{SingleGame, StepResult};
// use gstd::{prelude::*, ActorId};
// use gtest::{Log, Program, System};

// const USERS: &[u64] = &[3, 4, 5];

// #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Encode, Decode, TypeInfo)]
// #[codec(crate = sails_rs::scale_codec)]
// #[scale_info(crate = sails_rs::scale_info)]
// struct Test {
//     step_result: StepResult,
//     bot_step: u8,
// }

// #[test]
// fn start_single_game() {
//     let sys = System::new();
//     sys.init_logger();
//     let battleship = Program::from_file(
//         &sys,
//         "../target/wasm32-unknown-unknown/release/battleship_wasm.opt.wasm",
//     );
//     let encoded_request = ["New".encode(), ().encode()].concat();
//     battleship.send_bytes(USERS[0], encoded_request);

//     let encoded_request = [
//         "Single".encode(),
//         "StartSingleGame".encode(),
//         (None::<ActorId>).encode(),
//     ]
//     .concat();
//     let res = battleship.send_bytes(USERS[0], encoded_request);

//     // let result = &res.decoded_log::<(String, String, ())>();
//     // println!("res {:?}", result);
//     let encoded_payload = ["Single".encode(), "SingleGameStarted".encode(), "".encode()].concat();
//     let log = Log::builder().dest(0).payload(encoded_payload);
//     res.contains(&log);

//     // let mailbox = sys.get_mailbox(0);
//     // let payload: [u8; 0] = [];
//     // let encoded_payload = ["Single".encode(), "SingleGameStarted".encode(), payload.encode()].concat();
//     // let log = Log::builder().dest(0).payload(encoded_payload);
//     // assert!(mailbox.contains(&log));

//     let encoded_request = [
//         "Single".encode(),
//         "MakeMove".encode(),
//         (5, None::<ActorId>).encode(),
//     ]
//     .concat();
//     let res = battleship.send_bytes(USERS[0], encoded_request);

//     let result = &res.decoded_log::<(String, String, Test)>();
//     println!("res {:?}", result);

//     let user: ActorId = USERS[0].into();
//     let encoded_request = ["Single".encode(), "Game".encode(), (user).encode()].concat();
//     let state = battleship.send_bytes(USERS[0], encoded_request);
//     let state = &state.decoded_log::<(String, String, Option<SingleGame>)>();
//     println!("\nstate {:?}", state);
// }
