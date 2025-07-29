use battle_client::{traits::*, Appearance, Move, SessionConfig, State, UtilsConfig};
use gstd::errors::{ErrorReplyReason, SimpleExecutionError};
use gtest::{Program, System};
use sails_rs::{
    calls::*,
    errors::{Error, RtlError},
    gtest::calls::*,
    ActorId, Encode,
};

const USER_1: u64 = 100;
const USER_2: u64 = 101;
const USER_3: u64 = 102;

fn init_warrior(system: &System, user: u64) -> ActorId {
    let warrior = Program::from_file(system, "../target/wasm32-gear/release/warrior.opt.wasm");
    let request = ["New".encode(), ("link".to_string()).encode()].concat();

    let mid = warrior.send_bytes(user, request);
    let res = system.run_next_block();
    assert!(res.succeed.contains(&mid));
    warrior.id()
}

#[tokio::test]
async fn test() {
    let system = System::new();
    system.init_logger();
    system.mint_to(USER_1, 1_000_000_000_000_000);
    system.mint_to(USER_2, 1_000_000_000_000_000);
    system.mint_to(USER_3, 1_000_000_000_000_000);

    let remoting = GTestRemoting::new(system, USER_1.into());

    // Submit program code into the system
    let program_code_id = remoting.system().submit_code(battle::WASM_BINARY);

    let program_factory = battle_client::BattleFactory::new(remoting.clone());
    let session_config = SessionConfig {
        gas_to_delete_session: 10_000_000_000,
        minimum_session_duration_ms: 180_000,
        ms_per_block: 3_000,
    };

    let program_id = program_factory
        .new(
            UtilsConfig {
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
            },
            session_config,
        )
        .send_recv(program_code_id, b"salt")
        .await
        .unwrap();

    let mut service_client = battle_client::Battle::new(remoting.clone());
    let warrior_id = init_warrior(remoting.system(), USER_1);

    service_client
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
        .send_recv(program_id)
        .await
        .unwrap();

    let state = get_battle(&service_client, remoting.actor_id(), program_id)
        .await
        .unwrap();
    assert_eq!(state.participants.len(), 1);
    assert_eq!(state.state, State::Registration);
    service_client
        .register(
            remoting.actor_id(),
            None,
            Some(Appearance {
                head_index: 1,
                hat_index: 2,
                body_index: 3,
                accessory_index: 4,
                body_color: "#008000".to_string(),
                back_color: "#0000FF".to_string(),
            }),
            "Warrior_2".to_string(),
            15,
            10,
            5,
            None,
        )
        .with_value(10_000_000_000)
        .with_args(|args| args.with_actor_id(USER_2.into()))
        .send_recv(program_id)
        .await
        .unwrap();

    let state = get_battle(&service_client, remoting.actor_id(), program_id)
        .await
        .unwrap();
    assert_eq!(state.participants.len(), 2);

    service_client
        .start_battle(None)
        .send_recv(program_id)
        .await
        .unwrap();

    let state = get_battle(&service_client, remoting.actor_id(), program_id)
        .await
        .unwrap();
    assert_eq!(state.state, State::Started);

    make_move(&mut service_client, Move::Attack, USER_2, program_id)
        .await
        .unwrap();

    remoting
        .system()
        .run_to_block(remoting.system().block_height() + 18);

    let state = get_battle(&service_client, remoting.actor_id(), program_id)
        .await
        .unwrap();
    assert_eq!(state.pairs[0].1.round, 2);

    remoting
        .system()
        .run_to_block(remoting.system().block_height() + 150);

    let state = get_battle(&service_client, remoting.actor_id(), program_id)
        .await
        .unwrap();
    assert!(!state.defeated_participants.is_empty());
}

#[tokio::test]
async fn test_both_made_move() {
    let system = System::new();
    system.init_logger();
    system.mint_to(USER_1, 1_000_000_000_000_000);
    system.mint_to(USER_2, 1_000_000_000_000_000);
    system.mint_to(USER_3, 1_000_000_000_000_000);

    let remoting = GTestRemoting::new(system, USER_1.into());

    // Submit program code into the system
    let program_code_id = remoting.system().submit_code(battle::WASM_BINARY);

    let program_factory = battle_client::BattleFactory::new(remoting.clone());
    let session_config = SessionConfig {
        gas_to_delete_session: 10_000_000_000,
        minimum_session_duration_ms: 180_000,
        ms_per_block: 3_000,
    };
    let program_id = program_factory
        .new(
            UtilsConfig {
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
            },
            session_config,
        )
        .send_recv(program_code_id, b"salt")
        .await
        .unwrap();

    let mut service_client = battle_client::Battle::new(remoting.clone());

    let warrior_id = init_warrior(remoting.system(), USER_1);
    service_client
        .create_new_battle(
            "Battle".to_string(),
            "Warrior_1".to_string(),
            Some(warrior_id),
            None,
            20,
            5,
            5,
            None,
        )
        .with_value(10_000_000_000)
        .send_recv(program_id)
        .await
        .unwrap();

    let state = get_battle(&service_client, remoting.actor_id(), program_id)
        .await
        .unwrap();
    assert_eq!(state.participants.len(), 1);
    assert_eq!(state.state, State::Registration);

    service_client
        .register(
            remoting.actor_id(),
            None,
            Some(Appearance {
                head_index: 1,
                hat_index: 2,
                body_index: 3,
                accessory_index: 4,
                body_color: "#008000".to_string(),
                back_color: "#0000FF".to_string(),
            }),
            "Warrior_2".to_string(),
            15,
            8,
            7,
            None,
        )
        .with_value(10_000_000_000)
        .with_args(|args| args.with_actor_id(USER_2.into()))
        .send_recv(program_id)
        .await
        .unwrap();

    let state = get_battle(&service_client, remoting.actor_id(), program_id)
        .await
        .unwrap();
    assert_eq!(state.participants.len(), 2);

    service_client
        .start_battle(None)
        .send_recv(program_id)
        .await
        .unwrap();

    let state = get_battle(&service_client, remoting.actor_id(), program_id)
        .await
        .unwrap();
    assert_eq!(state.state, State::Started);

    remoting
        .system()
        .run_to_block(remoting.system().block_height() + 5);

    make_move(&mut service_client, Move::Ultimate, USER_1, program_id)
        .await
        .unwrap();
    make_move(&mut service_client, Move::Reflect, USER_2, program_id)
        .await
        .unwrap();

    let state = get_battle(&service_client, remoting.actor_id(), program_id)
        .await
        .unwrap();
    assert_eq!(state.pairs[0].1.round, 2);

    remoting
        .system()
        .run_to_block(remoting.system().block_height() + 150);

    let state = get_battle(&service_client, remoting.actor_id(), program_id)
        .await
        .unwrap();
    assert!(!state.defeated_participants.is_empty());
}

#[tokio::test]
async fn test_three_player() {
    let system = System::new();
    system.init_logger();
    system.mint_to(USER_1, 1_000_000_000_000_000);
    system.mint_to(USER_2, 1_000_000_000_000_000);
    system.mint_to(USER_3, 1_000_000_000_000_000);

    let remoting = GTestRemoting::new(system, USER_1.into());

    // Submit program code into the system
    let program_code_id = remoting.system().submit_code(battle::WASM_BINARY);

    let program_factory = battle_client::BattleFactory::new(remoting.clone());
    let session_config = SessionConfig {
        gas_to_delete_session: 10_000_000_000,
        minimum_session_duration_ms: 180_000,
        ms_per_block: 3_000,
    };
    let program_id = program_factory
        .new(
            UtilsConfig {
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
            },
            session_config,
        )
        .send_recv(program_code_id, b"salt")
        .await
        .unwrap();

    let mut service_client = battle_client::Battle::new(remoting.clone());
    let warrior_id = init_warrior(remoting.system(), USER_1);

    service_client
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
        .send_recv(program_id)
        .await
        .unwrap();

    let state = get_battle(&service_client, remoting.actor_id(), program_id)
        .await
        .unwrap();
    assert_eq!(state.participants.len(), 1);
    assert_eq!(state.state, State::Registration);
    service_client
        .register(
            remoting.actor_id(),
            None,
            Some(Appearance {
                head_index: 1,
                hat_index: 2,
                body_index: 3,
                accessory_index: 4,
                body_color: "#008000".to_string(),
                back_color: "#0000FF".to_string(),
            }),
            "Warrior_2".to_string(),
            15,
            10,
            5,
            None,
        )
        .with_value(10_000_000_000)
        .with_args(|args| args.with_actor_id(USER_2.into()))
        .send_recv(program_id)
        .await
        .unwrap();

    let state = get_battle(&service_client, remoting.actor_id(), program_id)
        .await
        .unwrap();
    assert_eq!(state.participants.len(), 2);

    service_client
        .register(
            remoting.actor_id(),
            None,
            Some(Appearance {
                head_index: 1,
                hat_index: 2,
                body_index: 3,
                accessory_index: 4,
                body_color: "#008000".to_string(),
                back_color: "#0000FF".to_string(),
            }),
            "Warrior_2".to_string(),
            15,
            10,
            5,
            None,
        )
        .with_value(10_000_000_000)
        .with_args(|args| args.with_actor_id(USER_3.into()))
        .send_recv(program_id)
        .await
        .unwrap();

    let state = get_battle(&service_client, remoting.actor_id(), program_id)
        .await
        .unwrap();
    assert_eq!(state.participants.len(), 3);

    service_client
        .start_battle(None)
        .send_recv(program_id)
        .await
        .unwrap();

    let state = get_battle(&service_client, remoting.actor_id(), program_id)
        .await
        .unwrap();
    assert_eq!(state.state, State::Started);

    remoting
        .system()
        .run_to_block(remoting.system().block_height() + 300);

    let state = get_battle(&service_client, remoting.actor_id(), program_id)
        .await
        .unwrap();
    assert_eq!(state.defeated_participants.len(), 1);
    assert_eq!(state.pairs.len(), 1);
    assert_eq!(state.participants.len(), 2);
    let waiting_user = state.pairs[0].1.player_1;
    let users = state.participants;
    let mut user = ActorId::zero();
    for (id, _) in users.iter() {
        if *id != waiting_user {
            user = *id;
        }
    }

    let state = get_battle(&service_client, remoting.actor_id(), program_id)
        .await
        .unwrap();
    assert_eq!(state.pairs[0].1.round_start_time, 0);

    service_client
        .start_next_fight(None)
        .with_args(|args| args.with_actor_id(user))
        .send_recv(program_id)
        .await
        .unwrap();

    let state = get_battle(&service_client, remoting.actor_id(), program_id)
        .await
        .unwrap();
    assert_ne!(state.pairs[0].1.round_start_time, 0);
}

#[tokio::test]
async fn test_error() {
    let system = System::new();
    system.init_logger();
    system.mint_to(USER_1, 1_000_000_000_000_000);
    system.mint_to(USER_2, 1_000_000_000_000_000);
    system.mint_to(USER_3, 1_000_000_000_000_000);

    let remoting = GTestRemoting::new(system, USER_1.into());

    // Submit program code into the system
    let program_code_id = remoting.system().submit_code(battle::WASM_BINARY);

    let program_factory = battle_client::BattleFactory::new(remoting.clone());
    let session_config = SessionConfig {
        gas_to_delete_session: 10_000_000_000,
        minimum_session_duration_ms: 180_000,
        ms_per_block: 3_000,
    };
    let program_id = program_factory
        .new(
            UtilsConfig {
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
            },
            session_config,
        )
        .send_recv(program_code_id, b"salt")
        .await
        .unwrap();

    let mut service_client = battle_client::Battle::new(remoting.clone());

    let warrior_id = init_warrior(remoting.system(), USER_1);
    service_client
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
        .send_recv(program_id)
        .await
        .unwrap();

    service_client
        .register(
            remoting.actor_id(),
            None,
            Some(Appearance {
                head_index: 1,
                hat_index: 2,
                body_index: 3,
                accessory_index: 4,
                body_color: "#008000".to_string(),
                back_color: "#0000FF".to_string(),
            }),
            "Warrior_2".to_string(),
            15,
            10,
            5,
            None,
        )
        .with_value(10_000_000_000)
        .with_args(|args| args.with_actor_id(USER_2.into()))
        .send_recv(program_id)
        .await
        .unwrap();

    service_client
        .start_battle(None)
        .send_recv(program_id)
        .await
        .unwrap();

    make_move(&mut service_client, Move::Ultimate, USER_1, program_id)
        .await
        .unwrap();
    make_move(&mut service_client, Move::Reflect, USER_2, program_id)
        .await
        .unwrap();

    let res = make_move(&mut service_client, Move::Ultimate, USER_1, program_id).await;
    check_result(res, "UltimateReload".as_bytes());

    let res = make_move(&mut service_client, Move::Reflect, USER_2, program_id).await;
    check_result(res, "ReflectReload".as_bytes());
}

async fn make_move(
    service_client: &mut battle_client::Battle<GTestRemoting>,
    turn: battle_client::Move,
    user: u64,
    program_id: ActorId,
) -> Result<(), Error> {
    service_client
        .make_move(turn, None)
        .with_args(|args| args.with_actor_id(user.into()))
        .send_recv(program_id)
        .await
}

async fn get_battle(
    service_client: &battle_client::Battle<GTestRemoting>,
    game_id: ActorId,
    program_id: ActorId,
) -> Option<battle_client::BattleState> {
    service_client
        .get_battle(game_id)
        .recv(program_id)
        .await
        .unwrap()
}

fn check_result(result: Result<(), Error>, error: &[u8]) {
    assert!(matches!(
        result,
        Err(sails_rs::errors::Error::Rtl(RtlError::ReplyHasError(
            ErrorReplyReason::Execution(SimpleExecutionError::UserspacePanic),
            message
        ))) if message == *error
    ));
}
