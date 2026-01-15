use sails_rs::client::*;
use sails_rs::gtest::constants::DEFAULT_USERS_INITIAL_BALANCE;
use sails_rs::{gtest::Program, gtest::System, ActorId, Encode};

use battle_client::battle::Battle;
use battle_client::Battle as ClientBatle;
use battle_client::BattleCtors;
use battle_client::{Appearance, Config, Move, SessionConfig, State};

use gstd::errors::{ErrorReplyReason, SimpleExecutionError};

use ::battle::WASM_BINARY;

const USER_1: u64 = 100;
const USER_2: u64 = 101;
const USER_3: u64 = 102;

fn init_warrior(sys: &System, user: u64) -> ActorId {
    let warrior = Program::from_file(sys, "../target/wasm32-gear/release/warrior.opt.wasm");
    let req = ["New".encode(), "link".to_string().encode()].concat();
    let mid = warrior.send_bytes(user, req);
    let res = sys.run_next_block();
    assert!(res.succeed.contains(&mid));
    warrior.id()
}

fn utils_cfg() -> Config {
    Config {
        health: 100,
        max_participants: 10,
        attack_range: (10, 20),
        defence_range: (0, 10),
        dodge_range: (0, 10),
        available_points: 20,
        time_for_move_in_blocks: 20,
        block_duration_ms: 3_000,
        gas_for_create_warrior: 10_000_000_000,
        gas_to_cancel_the_battle: 10_000_000_000,
        reservation_amount: 500_000_000_000,
        reservation_time: 1_000,
        time_to_cancel_the_battle: 10_000,
    }
}

fn session_cfg() -> SessionConfig {
    SessionConfig {
        gas_to_delete_session: 10_000_000_000,
        minimum_session_duration_ms: 180_000,
        ms_per_block: 3_000,
    }
}

fn appearance() -> Appearance {
    Appearance {
        head_index: 1,
        hat_index: 2,
        body_index: 3,
        accessory_index: 4,
        body_color: "#008000".to_string(),
        back_color: "#0000FF".to_string(),
    }
}

/// IMPORTANT:
/// deploy() returns Actor<BattleProgram, GtestEnv>, not BattleProgram
async fn deploy_battle(env: &GtestEnv) -> Actor<battle_client::BattleProgram, GtestEnv> {
    let code_id = env.system().submit_code(WASM_BINARY);

    env.deploy::<battle_client::BattleProgram>(code_id, b"salt-battle".to_vec())
        .new(utils_cfg(), session_cfg())
        .await
        .unwrap()
}

/// IMPORTANT:
/// battle_client::battle::Battle<...> is a trait, so accept `impl Trait`.
async fn battle_state(
    battle: &mut impl battle_client::battle::Battle<Env = GtestEnv>,
    game_id: ActorId,
) -> battle_client::BattleState {
    battle.get_battle(game_id).await.unwrap().unwrap()
}

async fn make_move_as(
    battle: &mut impl battle_client::battle::Battle<Env = GtestEnv>,
    user: u64,
    mv: Move,
) -> Result<(), GtestError> {
    battle
        .make_move(mv, None)
        .with_actor_id(ActorId::from(user))
        .await
}

#[tokio::test]
async fn test() {
    let system = System::new();
    system.init_logger_with_default_filter("gwasm=debug,gtest=info,sails_rs=debug");

    for u in [USER_1, USER_2, USER_3] {
        system.mint_to(u, DEFAULT_USERS_INITIAL_BALANCE);
    }

    let env = GtestEnv::new(system, USER_1.into());

    let battle_program = deploy_battle(&env).await;

    // warrior deployment happens via raw Program
    let warrior_id = init_warrior(env.system(), USER_1);

    // IMPORTANT: battle() exists on Actor<...>, not on BattleProgram
    let mut battle = battle_program.battle();

    battle
        .create_new_battle(
            "Battle".to_string(),
            "Warrior_1".to_string(),
            Some(warrior_id),
            None,
            15,
            10,
            5,
            None,
        )
        .with_value(10_000_000_000)
        .await
        .unwrap();

    let st = battle_state(&mut battle, env.actor_id()).await;
    assert_eq!(st.participants.len(), 1);
    assert_eq!(st.state, State::Registration);

    battle
        .register(
            env.actor_id(),
            None,
            Some(appearance()),
            "Warrior_2".to_string(),
            15,
            10,
            5,
            None,
        )
        .with_value(10_000_000_000)
        .with_actor_id(USER_2.into())
        .await
        .unwrap();

    let st = battle_state(&mut battle, env.actor_id()).await;
    assert_eq!(st.participants.len(), 2);

    battle.start_battle(None).await.unwrap();

    let st = battle_state(&mut battle, env.actor_id()).await;
    assert_eq!(st.state, State::Started);

    make_move_as(&mut battle, USER_2, Move::Attack)
        .await
        .unwrap();

    env.system().run_to_block(env.system().block_height() + 18);

    let st = battle_state(&mut battle, env.actor_id()).await;
    assert_eq!(st.pairs[0].1.round, 2);

    env.system().run_to_block(env.system().block_height() + 150);

    let st = battle_state(&mut battle, env.actor_id()).await;
    assert!(!st.defeated_participants.is_empty());
}

#[tokio::test]
async fn test_error() {
    let system = System::new();
    system.init_logger_with_default_filter("gwasm=debug,gtest=info,sails_rs=debug");

    for u in [USER_1, USER_2, USER_3] {
        system.mint_to(u, DEFAULT_USERS_INITIAL_BALANCE);
    }

    let env = GtestEnv::new(system, USER_1.into());
    let battle_program = deploy_battle(&env).await;

    let warrior_id = init_warrior(env.system(), USER_1);

    let mut battle = battle_program.battle();

    battle
        .create_new_battle(
            "Battle".to_string(),
            "Warrior_1".to_string(),
            Some(warrior_id),
            None,
            15,
            10,
            5,
            None,
        )
        .with_value(10_000_000_000)
        .await
        .unwrap();

    battle
        .register(
            env.actor_id(),
            None,
            Some(appearance()),
            "Warrior_2".to_string(),
            15,
            10,
            5,
            None,
        )
        .with_value(10_000_000_000)
        .with_actor_id(USER_2.into())
        .await
        .unwrap();

    battle.start_battle(None).await.unwrap();

    make_move_as(&mut battle, USER_1, Move::Ultimate)
        .await
        .unwrap();
    make_move_as(&mut battle, USER_2, Move::Reflect)
        .await
        .unwrap();

    let res = make_move_as(&mut battle, USER_1, Move::Ultimate).await;
    check_result(res, "UltimateReload".as_bytes());

    let res = make_move_as(&mut battle, USER_2, Move::Reflect).await;
    check_result(res, "ReflectReload".as_bytes());
}

fn check_result(res: Result<(), GtestError>, error: &[u8]) {
    assert!(matches!(
        res,
        Err(GtestError::ReplyHasError(
            ErrorReplyReason::Execution(SimpleExecutionError::UserspacePanic),
            message
        )) if message == error
    ));
}
