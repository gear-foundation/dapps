// use car_races_app::services::{
//     game::{GameState, StrategyAction},
//     session::SessionConfig,
//     Config, InitConfig,
// };

use sails_rs::Encode;
use sails_rs::client::*;
use sails_rs::gtest::constants::DEFAULT_USERS_INITIAL_BALANCE;
use sails_rs::{ActorId, gtest::Program, gtest::System};

use car_races_client::CarRaces;
use car_races_client::CarRacesCtors;
use car_races_client::car_races_service::CarRacesService;
use car_races_client::{Config, GameState, InitConfig, SessionConfig, StrategyAction};

const PATH_TO_STRATEGIES: [&str; 2] = [
    "../target/wasm32-gear/release/car_strategy_1.opt.wasm",
    "../target/wasm32-gear/release/car_strategy_2.opt.wasm",
];

const PATH_TO_CAR_RACES: &str = "../target/wasm32-gear/release/car_races.opt.wasm";

const ADMIN: u64 = 10;

#[tokio::test]
async fn test_car_races_without_session() {
    let system = System::new();
    system.init_logger_with_default_filter("gwasm=debug,gtest=info,sails_rs=debug");
    system.mint_to(ADMIN, DEFAULT_USERS_INITIAL_BALANCE);

    let env = GtestEnv::new(system, ADMIN.into());

    let strategy_1 = deploy_strategy(env.system(), ADMIN, PATH_TO_STRATEGIES[0]);
    let strategy_2 = deploy_strategy(env.system(), ADMIN, PATH_TO_STRATEGIES[1]);

    let init_config = InitConfig {
        config: Config {
            gas_to_remove_game: 20_000_000_000,
            initial_speed: 100,
            min_speed: 10,
            max_speed: 2000,
            gas_for_round: 100_000_000_000,
            time_interval: 20,
            max_distance: 3242,
            time: 1,
            time_for_game_storage: 200,
            block_duration_ms: 3000,
            gas_for_reply_deposit: 15_000_000_000,
        },
    };
    let session_config = SessionConfig {
        gas_to_delete_session: 10_000_000_000,
        minimum_session_duration_ms: 180_000,
        ms_per_block: 3_000,
    };
    let dns_id_and_name: Option<(ActorId, String)> = None;

    let races_code_id = env.system().submit_code_file(PATH_TO_CAR_RACES);

    let races_program = env
        .deploy::<car_races_client::CarRacesProgram>(races_code_id, b"salt-car-races".to_vec())
        .new(init_config, session_config, dns_id_and_name)
        .await
        .unwrap();

    // If this method name does not compile in your generated client,
    // the accessor is the snake_case of the service name.
    // Most commonly: `car_races_service()`.
    let mut races = races_program.car_races_service();

    races.allow_messages(true).await.unwrap();
    races
        .add_strategy_ids(vec![strategy_1, strategy_2])
        .await
        .unwrap();

    let session_for_account: Option<ActorId> = None;
    races.start_game(session_for_account).await.unwrap();

    loop {
        let game = races.game(ActorId::from(ADMIN)).await.unwrap();
        let Some(game) = game else {
            panic!("Game does not exist");
        };

        if game.state == GameState::Finished {
            break;
        }

        races
            .player_move(StrategyAction::BuyAcceleration, session_for_account)
            .await
            .unwrap();
    }

    races.start_game(session_for_account).await.unwrap();
}

fn deploy_strategy(sys: &System, from: u64, path: &str) -> ActorId {
    let program = Program::from_file(sys, path);
    let mid = program.send_bytes(from, ["New".encode()].concat());
    let res = sys.run_next_block();
    assert!(res.succeed.contains(&mid));
    program.id()
}
