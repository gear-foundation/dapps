use extended_vft_client::vft::io as vft_io;
use gtest::{Log, Program};
use sails_rs::calls::*;
use sails_rs::gtest::{calls::*, System};
use sails_rs::{ActorId, Encode, U256};
use vara_man::{
    traits::{VaraMan, VaraManFactory},
    Config, Level, Status, VaraMan as VaraManClient, VaraManFactory as Factory,
};

pub const ADMIN_ID: u64 = 10;
pub const USER_ID: u64 = 11;

fn init_fungible_token(sys: &System, vara_man_id: ActorId) -> (ActorId, Program<'_>) {
    let vft = Program::from_file(
        sys,
        "../../target/wasm32-unknown-unknown/release/extended_vft.opt.wasm",
    );
    let payload = ("Name".to_string(), "Symbol".to_string(), 10_u8);
    let encoded_request = ["New".encode(), payload.encode()].concat();
    let mid = vft.send_bytes(ADMIN_ID, encoded_request);
    let res = sys.run_next_block();
    assert!(res.succeed.contains(&mid));

    let encoded_request = vft_io::GrantMinterRole::encode_call(vara_man_id);
    let mid = vft.send_bytes(ADMIN_ID, encoded_request);
    let res = sys.run_next_block();
    assert!(res.succeed.contains(&mid));

    (vft.id(), vft)
}

fn ft_balance_of(program: Program<'_>, sys: &System, account: ActorId) {
    let encoded_request = vft_io::BalanceOf::encode_call(account);
    let mid = program.send_bytes(ADMIN_ID, encoded_request);
    let res = sys.run_next_block();
    assert!(res.succeed.contains(&mid));
    let state = &res.decoded_log::<(String, String, U256)>();
    println!("STATE {:?}", state)
}

// TODO: Remove `ignore` after adding it to the release tag https://github.com/gear-tech/gear/pull/4270
#[ignore]
#[tokio::test]
async fn test_play_game() {
    let system = System::new();
    system.init_logger();
    system.mint_to(ADMIN_ID, 1_000_000_000_000_000);
    let program_space = GTestRemoting::new(system, ADMIN_ID.into());

    let code_id = program_space
        .system()
        .submit_code_file("../../target/wasm32-unknown-unknown/release/vara_man.opt.wasm");

    let vara_man_factory = Factory::new(program_space.clone());
    let config = Config {
        one_point_in_value: 10_000_000_000_000,
        max_number_gold_coins: 2,
        max_number_silver_coins: 82,
        points_per_gold_coin_easy: 5,
        points_per_silver_coin_easy: 1,
        points_per_gold_coin_medium: 8,
        points_per_silver_coin_medium: 2,
        points_per_gold_coin_hard: 10,
        points_per_silver_coin_hard: 3,
        gas_for_finish_tournament: 10_000_000_000,
        gas_for_mint_fungible_token: 10_000_000_000,
        minimum_session_duration_ms: 180_000,
        gas_to_delete_session: 5_000_000_000,
        s_per_block: 3,
    };
    let vara_man_id = vara_man_factory
        .new(config, None)
        .send_recv(code_id, "123")
        .await
        .unwrap();

    program_space
        .system()
        .transfer(ADMIN_ID, vara_man_id, 100_000_000_000_000, true);

    let mut client = VaraManClient::new(program_space.clone());
    // change status
    client
        .change_status(Status::StartedWithNativeToken)
        .send_recv(vara_man_id)
        .await
        .unwrap();

    // check game status
    let status = client.status().recv(vara_man_id).await.unwrap();
    assert_eq!(status, Status::StartedWithNativeToken);

    let old_balance = program_space.system().balance_of(program_space.actor_id());
    client
        .finish_single_game(1, 5, Level::Easy, None)
        .send_recv(vara_man_id)
        .await
        .unwrap();

    let mailbox = program_space.system().get_mailbox(program_space.actor_id());

    let log = Log::builder().dest(program_space.actor_id());
    assert!(mailbox.contains(&log));
    assert!(mailbox.claim_value(log).is_ok());

    let new_balance = program_space.system().balance_of(program_space.actor_id());

    assert_eq!(new_balance - old_balance, 100_000_000_000_000);
}

#[tokio::test]
async fn test_play_game_with_fungible_token() {
    let system = System::new();
    system.init_logger();
    system.mint_to(ADMIN_ID, 1_000_000_000_000_000);
    let program_space = GTestRemoting::new(system, ADMIN_ID.into());

    let code_id = program_space
        .system()
        .submit_code_file("../../target/wasm32-unknown-unknown/release/vara_man.opt.wasm");

    let vara_man_factory = Factory::new(program_space.clone());
    let config = Config {
        one_point_in_value: 10_000_000_000_000,
        max_number_gold_coins: 2,
        max_number_silver_coins: 82,
        points_per_gold_coin_easy: 5,
        points_per_silver_coin_easy: 1,
        points_per_gold_coin_medium: 8,
        points_per_silver_coin_medium: 2,
        points_per_gold_coin_hard: 10,
        points_per_silver_coin_hard: 3,
        gas_for_finish_tournament: 10_000_000_000,
        gas_for_mint_fungible_token: 10_000_000_000,
        minimum_session_duration_ms: 180_000,
        gas_to_delete_session: 5_000_000_000,
        s_per_block: 3,
    };
    let vara_man_id = vara_man_factory
        .new(config, None)
        .send_recv(code_id, "123")
        .await
        .unwrap();

    program_space
        .system()
        .transfer(ADMIN_ID, vara_man_id, 100_000_000_000_000, true);
    let mut client = VaraManClient::new(program_space.clone());

    let (ft_address, ft_program) = init_fungible_token(program_space.system(), vara_man_id);
    // change status
    client
        .change_status(Status::StartedWithFungibleToken { ft_address })
        .send_recv(vara_man_id)
        .await
        .unwrap();

    // check game status
    let status = client.status().recv(vara_man_id).await.unwrap();
    assert_eq!(status, Status::StartedWithFungibleToken { ft_address });
    client
        .finish_single_game(1, 5, Level::Easy, None)
        .send_recv(vara_man_id)
        .await
        .unwrap();

    ft_balance_of(ft_program, program_space.system(), program_space.actor_id());
}

// TODO: Remove `ignore` after adding it to the release tag https://github.com/gear-tech/gear/pull/4270
#[ignore]
#[tokio::test]
async fn test_play_tournament() {
    let system = System::new();
    system.init_logger();
    system.mint_to(ADMIN_ID, 1_000_000_000_000_000);
    system.mint_to(USER_ID, 100_000_000_000_000);
    let program_space = GTestRemoting::new(system, ADMIN_ID.into());

    let code_id = program_space
        .system()
        .submit_code_file("../../target/wasm32-unknown-unknown/release/vara_man.opt.wasm");

    let vara_man_factory = Factory::new(program_space.clone());
    let config = Config {
        one_point_in_value: 10_000_000_000_000,
        max_number_gold_coins: 2,
        max_number_silver_coins: 82,
        points_per_gold_coin_easy: 5,
        points_per_silver_coin_easy: 1,
        points_per_gold_coin_medium: 8,
        points_per_silver_coin_medium: 2,
        points_per_gold_coin_hard: 10,
        points_per_silver_coin_hard: 3,
        gas_for_finish_tournament: 10_000_000_000,
        gas_for_mint_fungible_token: 10_000_000_000,
        minimum_session_duration_ms: 180_000,
        gas_to_delete_session: 5_000_000_000,
        s_per_block: 3,
    };
    let vara_man_id = vara_man_factory
        .new(config, None)
        .send_recv(code_id, "123")
        .await
        .unwrap();

    program_space
        .system()
        .transfer(ADMIN_ID, vara_man_id, 100_000_000_000_000, true);

    let mut client = VaraManClient::new(program_space.clone());
    // change status
    client
        .change_status(Status::StartedWithNativeToken)
        .send_recv(vara_man_id)
        .await
        .unwrap();

    // check game status
    let status = client.status().recv(vara_man_id).await.unwrap();
    assert_eq!(status, Status::StartedWithNativeToken);

    client
        .create_new_tournament(
            "TOURNAMENT".to_string(),
            "Admin tournament".to_string(),
            Level::Easy,
            180_000,
            None,
        )
        .with_value(10_000_000_000_000)
        .send_recv(vara_man_id)
        .await
        .unwrap();

    let state = client.all().recv(vara_man_id).await.unwrap();
    assert_eq!(state.tournaments.len(), 1);
    assert_eq!(state.players_to_game_id.len(), 1);

    client
        .register_for_tournament(program_space.actor_id(), "player #1".to_string(), None)
        .with_value(10_000_000_000_000)
        .with_args(GTestArgs::new(USER_ID.into()))
        .send_recv(vara_man_id)
        .await
        .unwrap();

    let state = client.all().recv(vara_man_id).await.unwrap();
    assert_eq!(state.tournaments[0].1.participants.len(), 2);

    let old_balance = program_space.system().balance_of(USER_ID);
    client
        .cancel_register(None)
        .with_args(GTestArgs::new(USER_ID.into()))
        .send_recv(vara_man_id)
        .await
        .unwrap();

    let mailbox = program_space
        .system()
        .get_mailbox::<ActorId>(USER_ID.into());

    let log = Log::builder().dest(USER_ID);
    assert!(mailbox.contains(&log));
    assert!(mailbox.claim_value(log).is_ok());

    let new_balance = program_space.system().balance_of(USER_ID);
    assert_eq!(new_balance - old_balance, 10_000_000_000_000);

    client
        .register_for_tournament(program_space.actor_id(), "player #1".to_string(), None)
        .with_value(10_000_000_000_000)
        .with_args(GTestArgs::new(USER_ID.into()))
        .send_recv(vara_man_id)
        .await
        .unwrap();

    client
        .start_tournament(None)
        .send_recv(vara_man_id)
        .await
        .unwrap();
    client
        .record_tournament_result(1_000, 1, 5, None)
        .send_recv(vara_man_id)
        .await
        .unwrap();
    client
        .record_tournament_result(1_000, 1, 5, None)
        .with_args(GTestArgs::new(USER_ID.into()))
        .send_recv(vara_man_id)
        .await
        .unwrap();

    let state = client.all().recv(vara_man_id).await.unwrap();
    assert_eq!(state.tournaments[0].1.participants[1].1.points, 10);

    // TODO: uncomment after fix gtest
    // system.spend_blocks(61);
    // let state = client.all().recv(vara_man_id).await.unwrap();
    // assert_eq!(
    //     state.tournaments[0].1.stage,
    //     Stage::Finished(vec![ADMIN_ID.into(), USER_ID.into()])
    // );
}
