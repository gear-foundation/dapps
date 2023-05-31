// use gear_lib::non_fungible_token::io::*;
// use gear_lib::non_fungible_token::token::TokenId;
// use gstd::{ActorId, Encode};
// use gtest::System;
// mod utils;
// use auto_changed_nft_io::*;
// use utils::*;

// const USERS: &[u64] = &[3, 4, 5];
// const ZERO_ID: u64 = 0;

// #[test]
// fn mint_success() {
//     let sys = System::new();
//     init_nft(&sys);
//     let nft = sys.get_program(1);
//     let transaction_id: u64 = 0;
//     let res = mint(&nft, transaction_id, USERS[0]);
//     let message = NFTEvent::Transfer(NFTTransfer {
//         from: ZERO_ID.into(),
//         to: USERS[0].into(),
//         token_id: 0.into(),
//     })
//     .encode();
//     assert!(res.contains(&(USERS[0], message)));
// }

// #[test]
// fn burn_success() {
//     let sys = System::new();
//     init_nft(&sys);
//     let nft = sys.get_program(1);
//     let mut transaction_id: u64 = 0;
//     assert!(!mint(&nft, transaction_id, USERS[0]).main_failed());
//     transaction_id += 1;
//     let res = burn(&nft, transaction_id, USERS[0], 0);
//     let message = NFTEvent::Transfer(NFTTransfer {
//         from: USERS[0].into(),
//         to: ZERO_ID.into(),
//         token_id: 0.into(),
//     })
//     .encode();
//     assert!(res.contains(&(USERS[0], message)));
// }

// #[test]
// fn burn_failures() {
//     let sys = System::new();
//     init_nft(&sys);
//     let nft = sys.get_program(1);
//     let mut transaction_id: u64 = 0;
//     assert!(!mint(&nft, transaction_id, USERS[0]).main_failed());
//     // must fail since the token doesn't exist
//     transaction_id += 1;
//     assert!(burn(&nft, transaction_id, USERS[0], 1).main_failed());
//     // must fail since the caller is not the token owner
//     transaction_id += 1;
//     assert!(burn(&nft, transaction_id, USERS[1], 0).main_failed());
// }

// #[test]
// fn transfer_success() {
//     let sys = System::new();
//     init_nft(&sys);
//     let nft = sys.get_program(1);
//     let mut transaction_id: u64 = 0;
//     assert!(!mint(&nft, transaction_id, USERS[0]).main_failed());
//     transaction_id += 1;
//     let res = transfer(&nft, transaction_id, USERS[0], USERS[1], 0);
//     let message = NFTEvent::Transfer(NFTTransfer {
//         from: USERS[0].into(),
//         to: USERS[1].into(),
//         token_id: 0.into(),
//     })
//     .encode();
//     assert!(res.contains(&(USERS[0], message)));
// }

// #[test]
// fn transfer_failures() {
//     let sys = System::new();
//     init_nft(&sys);
//     let nft = sys.get_program(1);
//     let mut transaction_id: u64 = 0;
//     assert!(!mint(&nft, transaction_id, USERS[0]).main_failed());

//     // must fail since the token doesn't exist
//     transaction_id += 1;
//     assert!(transfer(&nft, transaction_id, USERS[0], USERS[1], 1).main_failed());
//     // must fail since the caller is not the token owner
//     transaction_id += 1;
//     assert!(transfer(&nft, transaction_id, USERS[1], USERS[0], 0).main_failed());
//     // must fail since transfer to the zero address
//     transaction_id += 1;
//     assert!(transfer(&nft, transaction_id, USERS[1], ZERO_ID, 0).main_failed());
// }

// #[test]
// fn owner_success() {
//     let sys = System::new();
//     init_nft(&sys);
//     let nft = sys.get_program(1);
//     let mut transaction_id: u64 = 0;
//     assert!(!mint(&nft, transaction_id, USERS[0]).main_failed());
//     transaction_id += 1;
//     assert!(!approve(&nft, transaction_id, USERS[0], USERS[1], 0).main_failed());
//     let res = owner_of(&nft, USERS[1], 0);
//     println!("{:?}", res.decoded_log::<NFTEvent>());
//     let message = NFTEvent::Owner {
//         token_id: 0.into(),
//         owner: ActorId::from(USERS[0]),
//     }
//     .encode();
//     assert!(res.contains(&(USERS[1], message)));
// }

// #[test]
// fn is_approved_to_success() {
//     let sys = System::new();
//     init_nft(&sys);
//     let nft = sys.get_program(1);
//     let mut transaction_id: u64 = 0;
//     assert!(!mint(&nft, transaction_id, USERS[0]).main_failed());
//     transaction_id += 1;
//     assert!(!approve(&nft, transaction_id, USERS[0], USERS[1], 0).main_failed());

//     let res = is_approved_to(&nft, USERS[1], 0, USERS[1]);
//     println!("{:?}", res.decoded_log::<NFTEvent>());
//     let message = NFTEvent::IsApproved {
//         to: USERS[1].into(),
//         token_id: 0.into(),
//         approved: true,
//     }
//     .encode();
//     assert!(res.contains(&(USERS[1], message)));

//     let res = is_approved_to(&nft, USERS[1], 0, USERS[0]);
//     println!("{:?}", res.decoded_log::<NFTEvent>());
//     let message = NFTEvent::IsApproved {
//         to: USERS[0].into(),
//         token_id: 0.into(),
//         approved: false,
//     }
//     .encode();
//     assert!(res.contains(&(USERS[1], message)));
// }

// #[test]
// fn is_approved_to_failure() {
//     let sys = System::new();
//     init_nft(&sys);
//     let nft = sys.get_program(1);
//     let mut transaction_id: u64 = 0;
//     assert!(!mint(&nft, transaction_id, USERS[0]).main_failed());
//     transaction_id += 1;
//     assert!(!approve(&nft, transaction_id, USERS[0], USERS[1], 0).main_failed());
//     let res = is_approved_to(&nft, USERS[1], 1, USERS[1]);
//     println!("{:?}", res.decoded_log::<NFTEvent>());
//     assert!(res.main_failed());
// }

// #[test]
// fn approve_success() {
//     let sys = System::new();
//     init_nft(&sys);
//     let nft = sys.get_program(1);
//     let mut transaction_id: u64 = 0;
//     assert!(!mint(&nft, transaction_id, USERS[0]).main_failed());
//     transaction_id += 1;
//     let res = approve(&nft, transaction_id, USERS[0], USERS[1], 0);
//     let message = NFTEvent::Approval(NFTApproval {
//         owner: USERS[0].into(),
//         approved_account: USERS[1].into(),
//         token_id: 0.into(),
//     })
//     .encode();
//     assert!(res.contains(&(USERS[0], message)));
//     transaction_id += 1;
//     assert!(!transfer(&nft, transaction_id, USERS[1], USERS[2], 0).main_failed());
// }

// #[test]
// fn auto_change_success() {
//     let sys = System::new();
//     init_nft(&sys);
//     let nft = sys.get_program(1);
//     let transaction_id: u64 = 0;
//     assert!(!mint(&nft, transaction_id, USERS[0]).main_failed());

//     let link1 = "link 1";
//     let link2 = "link 2";
//     let link3 = "link 3";
//     let link4 = "link 4";

//     let token_id = TokenId::default();
//     assert!(!add_url(&nft, token_id, link1, USERS[0]).main_failed());
//     assert!(!add_url(&nft, token_id, link2, USERS[0]).main_failed());
//     assert!(!add_url(&nft, token_id, link3, USERS[0]).main_failed());
//     assert!(!add_url(&nft, token_id, link4, USERS[0]).main_failed());

//     let updates_count = 8;
//     let updates_period = 5;
//     assert!(!start_auto_changing(
//         &nft,
//         vec![token_id],
//         updates_count,
//         updates_period,
//         USERS[0]
//     )
//     .main_failed());

//     // Start update
//     assert_eq!(current_media(&nft, token_id), link1);

//     sys.spend_blocks(updates_period);
//     assert_eq!(current_media(&nft, token_id), link4);

//     sys.spend_blocks(updates_period);
//     assert_eq!(current_media(&nft, token_id), link3);

//     sys.spend_blocks(updates_period);
//     assert_eq!(current_media(&nft, token_id), link2);

//     // Media rotation happens
//     sys.spend_blocks(updates_period);
//     assert_eq!(current_media(&nft, token_id), link1);

//     sys.spend_blocks(updates_period);
//     assert_eq!(current_media(&nft, token_id), link4);

//     sys.spend_blocks(updates_period);
//     assert_eq!(current_media(&nft, token_id), link3);

//     sys.spend_blocks(updates_period);
//     assert_eq!(current_media(&nft, token_id), link2);
// }

// #[test]
// fn approve_failures() {
//     let sys = System::new();
//     init_nft(&sys);
//     let nft = sys.get_program(1);
//     let mut transaction_id: u64 = 0;
//     assert!(!mint(&nft, transaction_id, USERS[0]).main_failed());
//     transaction_id += 1;
//     // must fail since the token doesn't exist
//     assert!(approve(&nft, transaction_id, USERS[0], USERS[1], 1).main_failed());
//     transaction_id += 1;
//     // must fail since the caller is not the token owner
//     assert!(approve(&nft, transaction_id, USERS[1], USERS[0], 0).main_failed());
//     transaction_id += 1;
//     // must fail since approval to the zero address
//     assert!(approve(&nft, transaction_id, USERS[1], ZERO_ID, 0).main_failed());

//     //approve
//     transaction_id += 1;
//     assert!(!approve(&nft, transaction_id, USERS[0], USERS[1], 0).main_failed());
//     //transfer
//     transaction_id += 1;
//     assert!(!transfer(&nft, transaction_id, USERS[1], USERS[2], 0).main_failed());
//     //must fail since approval was removed after transferring
//     transaction_id += 1;
//     assert!(transfer(&nft, transaction_id, USERS[1], USERS[0], 0).main_failed());
// }
