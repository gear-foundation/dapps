use car_races_io::*;
use gstd::{prelude::*, ActorId};
use gtest::{Program, System};
const ADMIN: u64 = 100;
pub const PLAYERS: [u64; 3] = [10, 11, 12];

#[test]
fn success_run_game() {
    let system = System::new();

    system.init_logger();

    let game = Program::current(&system);
    let game_init_result = game.send(
        ADMIN,
        GameInit {
            config: Config {
                gas_to_remove_game: 20_000_000_000,
                initial_speed: 100,
                min_speed: 10,
                max_speed: 2_000,
                gas_for_round: 100_000_000_000,
                time_interval: 20,
                max_distance: 3_242,
                time: 1,
                time_for_game_storage: 200,
            },
        },
    );
    assert!(!game_init_result.main_failed());
    let car_1 = Program::from_file(
        &system,
        "../target/wasm32-unknown-unknown/release/car_1.opt.wasm",
    );
    let car_init_result = car_1.send_bytes(ADMIN, []);
    assert!(!car_init_result.main_failed());
    let car_2 = Program::from_file(
        &system,
        "../target/wasm32-unknown-unknown/release/car_3.opt.wasm",
    );
    let car_init_result = car_2.send_bytes(ADMIN, []);
    assert!(!car_init_result.main_failed());
    let car_3 = Program::from_file(
        &system,
        "../target/wasm32-unknown-unknown/release/car_3.opt.wasm",
    );
    let car_init_result = car_3.send_bytes(ADMIN, []);
    assert!(!car_init_result.main_failed());

    let run_result = game.send(ADMIN, GameAction::AllowMessages(true));
    assert!(!run_result.main_failed());

    // Add algorithm
    let car_ids: Vec<ActorId> = vec![2.into(), 3.into()];
    let run_result = game.send(ADMIN, GameAction::AddStrategyIds { car_ids });
    assert!(!run_result.main_failed());

    let run_result = game.send(ADMIN, GameAction::StartGame);

    assert!(!run_result.main_failed());

    for _i in 0..40 {
        let run_result = game.send(
            ADMIN,
            GameAction::PlayerMove {
                strategy_action: StrategyAction::BuyShell,
            },
        );

        assert!(!run_result.main_failed());

        let reply = game
            .read_state(StateQuery::AllGames)
            .expect("Unexpected invalid state.");
        if let StateReply::AllGames(state) = reply {
            let (_id, game) = &state[0];
            if game.state == GameState::Finished {
                break;
            }
        }
    }
}

#[test]
fn success_add_admin() {
    let system = System::new();

    system.init_logger();

    let game = Program::current(&system);
    let game_init_result = game.send(
        ADMIN,
        GameInit {
            config: Config {
                gas_to_remove_game: 20_000_000_000,
                initial_speed: 100,
                min_speed: 10,
                max_speed: 2_000,
                gas_for_round: 100_000_000_000,
                time_interval: 20,
                max_distance: 3_242,
                time: 1,
                time_for_game_storage: 200,
            },
        },
    );
    assert!(!game_init_result.main_failed());

    let run_result = game.send(ADMIN, GameAction::AllowMessages(true));
    assert!(!run_result.main_failed());

    let run_result = game.send(ADMIN, GameAction::AddAdmin(1.into()));
    assert!(!run_result.main_failed());

    let reply = game
        .read_state(StateQuery::Admins)
        .expect("Unexpected invalid state.");
    if let StateReply::Admins(admins) = reply {
        let true_admins: Vec<ActorId> = vec![100.into(), 1.into()];
        assert_eq!(true_admins, admins, "Wrong admins");
    }
}

#[test]
fn failures_test() {
    let system = System::new();

    system.init_logger();

    let game = Program::current(&system);
    let game_init_result = game.send(
        ADMIN,
        GameInit {
            config: Config {
                gas_to_remove_game: 20_000_000_000,
                initial_speed: 100,
                min_speed: 10,
                max_speed: 2_000,
                gas_for_round: 100_000_000_000,
                time_interval: 20,
                max_distance: 3_242,
                time: 1,
                time_for_game_storage: 200,
            },
        },
    );
    assert!(!game_init_result.main_failed());

    // AllowMessages not true
    let run_result = game.send(PLAYERS[0], GameAction::StartGame);
    assert!(!run_result.main_failed());
    assert!(run_result.contains(&(
        PLAYERS[0],
        Err::<GameReply, GameError>(GameError::MessageProcessingSuspended).encode()
    )));

    let run_result = game.send(ADMIN, GameAction::AllowMessages(true));
    assert!(!run_result.main_failed());
    // not admin
    let run_result = game.send(PLAYERS[0], GameAction::AddAdmin(2.into()));
    assert!(!run_result.main_failed());
    assert!(run_result.contains(&(
        PLAYERS[0],
        Err::<GameReply, GameError>(GameError::NotAdmin).encode()
    )));

    let car_1 = Program::from_file(
        &system,
        "../target/wasm32-unknown-unknown/release/car_1.opt.wasm",
    );
    let car_init_result = car_1.send_bytes(ADMIN, []);
    assert!(!car_init_result.main_failed());
    let car_2 = Program::from_file(
        &system,
        "../target/wasm32-unknown-unknown/release/car_3.opt.wasm",
    );
    let car_init_result = car_2.send_bytes(ADMIN, []);
    assert!(!car_init_result.main_failed());
    let car_3 = Program::from_file(
        &system,
        "../target/wasm32-unknown-unknown/release/car_3.opt.wasm",
    );
    let car_init_result = car_3.send_bytes(ADMIN, []);
    assert!(!car_init_result.main_failed());

    // There must be 2 strategies of cars
    let car_ids: Vec<ActorId> = vec![1.into(), 2.into(), 3.into()];
    let run_result = game.send(ADMIN, GameAction::AddStrategyIds { car_ids });
    assert!(!run_result.main_failed());
    assert!(run_result.contains(&(
        ADMIN,
        Err::<GameReply, GameError>(GameError::MustBeTwoStrategies).encode()
    )));

    let car_ids: Vec<ActorId> = vec![2.into(), 3.into()];
    let run_result = game.send(ADMIN, GameAction::AddStrategyIds { car_ids });
    assert!(!run_result.main_failed());

    let run_result = game.send(ADMIN, GameAction::StartGame);
    assert!(!run_result.main_failed());
    // The game has already started
    let run_result = game.send(ADMIN, GameAction::StartGame);
    assert!(!run_result.main_failed());
    assert!(run_result.contains(&(
        ADMIN,
        Err::<GameReply, GameError>(GameError::GameAlreadyStarted).encode()
    )));

    for _i in 0..40 {
        let run_result = game.send(
            ADMIN,
            GameAction::PlayerMove {
                strategy_action: StrategyAction::BuyShell,
            },
        );

        assert!(!run_result.main_failed());

        let reply = game
            .read_state(StateQuery::AllGames)
            .expect("Unexpected invalid state.");
        if let StateReply::AllGames(state) = reply {
            let (_id, game) = &state[0];
            if game.state == GameState::Finished {
                break;
            }
        }
    }

    // The game's already over
    let run_result = game.send(
        ADMIN,
        GameAction::PlayerMove {
            strategy_action: StrategyAction::BuyShell,
        },
    );
    assert!(!run_result.main_failed());
    assert!(run_result.contains(&(
        ADMIN,
        Err::<GameReply, GameError>(GameError::NotPlayerTurn).encode()
    )));
}
