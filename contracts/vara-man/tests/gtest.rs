use extended_vft_client::vft::io as vft_io;
use gtest::{Log, Program};
use sails_rs::client::*;
use sails_rs::gtest::constants::DEFAULT_USERS_INITIAL_BALANCE;
use sails_rs::{gtest::System, ActorId, Encode, U256};

use vara_man_client::vara_man::VaraMan;
use vara_man_client::VaraMan as ClientVaraMan;
use vara_man_client::VaraManCtors;
use vara_man_client::{Config, Level, Status};

pub const ADMIN_ID: u64 = 10;
pub const USER_ID: u64 = 11;

fn init_fungible_token(sys: &System, vara_man_id: ActorId) -> (ActorId, Program<'_>) {
    let vft = Program::from_file(sys, "../target/wasm32-gear/release/extended_vft.opt.wasm");

    // ctor
    let payload = ("Name".to_string(), "Symbol".to_string(), 10_u8);
    let encoded = ["New".encode(), payload.encode()].concat();
    let mid = vft.send_bytes(ADMIN_ID, encoded);
    let res = sys.run_next_block();
    assert!(res.succeed.contains(&mid));

    // grant minter to VaraMan
    let encoded = vft_io::GrantMinterRole::encode_params_with_prefix("Vft", vara_man_id);
    let mid = vft.send_bytes(ADMIN_ID, encoded);
    let res = sys.run_next_block();
    assert!(res.succeed.contains(&mid));

    (vft.id(), vft)
}

fn ft_balance_of(program: Program<'_>, sys: &System, account: ActorId) -> U256 {
    let encoded = vft_io::BalanceOf::encode_params_with_prefix("Vft", account);
    let mid = program.send_bytes(ADMIN_ID, encoded);
    let res = sys.run_next_block();
    assert!(res.succeed.contains(&mid));
    vft_io::BalanceOf::decode_reply_with_prefix("Vft", res.log[0].payload()).unwrap()
}

// TODO: Remove `ignore` after adding it to the release tag https://github.com/gear-tech/gear/pull/4270
#[ignore]
#[tokio::test]
async fn test_play_game() {
    let system = System::new();
    system.init_logger();

    system.mint_to(ADMIN_ID, DEFAULT_USERS_INITIAL_BALANCE);

    let env = GtestEnv::new(system, ADMIN_ID.into());

    let vara_man_code_id = env
        .system()
        .submit_code_file("../target/wasm32-gear/release/vara_man.opt.wasm");

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

    let vara_man_prog = env
        .deploy::<vara_man_client::VaraManProgram>(vara_man_code_id, b"salt".to_vec())
        .new(config, None)
        .await
        .unwrap();

    let vara_man_id = vara_man_prog.id();

    // fund contract so it can pay out
    env.system()
        .transfer(ADMIN_ID, vara_man_id, 100_000_000_000_000, true);

    let mut vm = vara_man_prog.vara_man();

    vm.change_status(Status::StartedWithNativeToken)
        .await
        .unwrap();

    let status = vm.status().await.unwrap();
    assert_eq!(status, Status::StartedWithNativeToken);

    let old_balance = env.system().balance_of(ADMIN_ID);

    vm.finish_single_game(1, 5, Level::Easy, None)
        .await
        .unwrap();

    // payout should appear in ADMIN mailbox
    let mailbox = env.system().get_mailbox(ADMIN_ID);
    let log = Log::builder().dest(ADMIN_ID);
    assert!(mailbox.contains(&log));
    assert!(mailbox.claim_value(log).is_ok());

    let new_balance = env.system().balance_of(ADMIN_ID);
    assert_eq!(new_balance - old_balance, 100_000_000_000_000);
}

#[tokio::test]
async fn test_play_game_with_fungible_token() {
    let system = System::new();
    system.init_logger();

    system.mint_to(ADMIN_ID, DEFAULT_USERS_INITIAL_BALANCE);

    let env = GtestEnv::new(system, ADMIN_ID.into());

    let vara_man_code_id = env
        .system()
        .submit_code_file("../target/wasm32-gear/release/vara_man.opt.wasm");

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

    let vara_man_prog = env
        .deploy::<vara_man_client::VaraManProgram>(vara_man_code_id, b"salt".to_vec())
        .new(config, None)
        .await
        .unwrap();

    let vara_man_id = vara_man_prog.id();

    // fund contract if it needs it for internal ops
    env.system()
        .transfer(ADMIN_ID, vara_man_id, 100_000_000_000_000, true);

    // init FT and grant minter role to VaraMan
    let (ft_address, ft_program) = init_fungible_token(env.system(), vara_man_id);

    let mut vm = vara_man_prog.vara_man();

    vm.change_status(Status::StartedWithFungibleToken { ft_address })
        .await
        .unwrap();

    let status = vm.status().await.unwrap();
    assert_eq!(status, Status::StartedWithFungibleToken { ft_address });

    vm.finish_single_game(1, 5, Level::Easy, None)
        .await
        .unwrap();

    // player should receive minted FT (assuming payouts go to msg::source == ADMIN in this test)
    let bal = ft_balance_of(ft_program, env.system(), ADMIN_ID.into());
    assert!(bal > 0.into());
}

// TODO: Remove `ignore` after adding it to the release tag https://github.com/gear-tech/gear/pull/4270
#[ignore]
#[tokio::test]
async fn test_play_tournament() {
    let system = System::new();
    system.init_logger();

    system.mint_to(ADMIN_ID, DEFAULT_USERS_INITIAL_BALANCE);
    system.mint_to(USER_ID, 100_000_000_000_000);

    let env = GtestEnv::new(system, ADMIN_ID.into());

    let vara_man_code_id = env
        .system()
        .submit_code_file("../target/wasm32-gear/release/vara_man.opt.wasm");

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

    let vara_man_prog = env
        .deploy::<vara_man_client::VaraManProgram>(vara_man_code_id, b"salt".to_vec())
        .new(config, None)
        .await
        .unwrap();
    let vara_man_id = vara_man_prog.id();

    env.system()
        .transfer(ADMIN_ID, vara_man_id, 100_000_000_000_000, true);

    let mut vm = vara_man_prog.vara_man();

    vm.change_status(Status::StartedWithNativeToken)
        .await
        .unwrap();

    let status = vm.status().await.unwrap();
    assert_eq!(status, Status::StartedWithNativeToken);

    vm.create_new_tournament(
        "TOURNAMENT".to_string(),
        "Admin tournament".to_string(),
        Level::Easy,
        180_000,
        None,
    )
    .with_value(10_000_000_000_000)
    .await
    .unwrap();

    let state = vm.all().await.unwrap();
    assert_eq!(state.tournaments.len(), 1);
    assert_eq!(state.players_to_game_id.len(), 1);

    vm.register_for_tournament(ADMIN_ID.into(), "player #1".to_string(), None)
        .with_value(10_000_000_000_000)
        .with_actor_id(USER_ID.into())
        .await
        .unwrap();

    let state = vm.all().await.unwrap();
    assert_eq!(state.tournaments[0].1.participants.len(), 2);

    let old_balance = env.system().balance_of(USER_ID);

    vm.cancel_register(None)
        .with_actor_id(USER_ID.into())
        .await
        .unwrap();

    let mailbox = env.system().get_mailbox::<ActorId>(USER_ID.into());
    let log = Log::builder().dest(USER_ID);
    assert!(mailbox.contains(&log));
    assert!(mailbox.claim_value(log).is_ok());

    let new_balance = env.system().balance_of(USER_ID);
    assert_eq!(new_balance - old_balance, 10_000_000_000_000);

    vm.register_for_tournament(ADMIN_ID.into(), "player #1".to_string(), None)
        .with_value(10_000_000_000_000)
        .with_actor_id(USER_ID.into())
        .await
        .unwrap();

    vm.start_tournament(None).await.unwrap();

    vm.record_tournament_result(1_000, 1, 5, None)
        .await
        .unwrap();

    vm.record_tournament_result(1_000, 1, 5, None)
        .with_actor_id(USER_ID.into())
        .await
        .unwrap();

    let state = vm.all().await.unwrap();
    assert_eq!(state.tournaments[0].1.participants[1].1.points, 10);
}
