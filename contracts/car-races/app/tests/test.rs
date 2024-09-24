use car_races_app::game::{
    game::{Game, GameState, StrategyAction},
    session::Config as SessionConfig,
    Config, InitConfig,
};
use gtest::{Program, System};
use sails_rs::{prelude::*, ActorId};
const PATH_TO_STRATEGIES: [&str; 2] = [
    "../../target/wasm32-unknown-unknown/release/car_strategy_1.opt.wasm",
    "../../target/wasm32-unknown-unknown/release/car_strategy_2.opt.wasm",
];

const PATH_TO_CAR_RACES: &str =
    "../../target/wasm32-unknown-unknown/release/car_races_wasm.opt.wasm";

#[test]
fn test_car_races_without_session() {
    let system = System::new();
    system.init_logger();

    // upload strategy 1
    let car_strategy_1 = Program::from_file(&system, PATH_TO_STRATEGIES[0]);
    let payload = ["New".encode()].concat();
    let result = car_strategy_1.send_bytes(10, payload);
    assert!(!result.main_failed());

    // upload strategy 2
    let car_strategy_2 = Program::from_file(&system, PATH_TO_STRATEGIES[1]);
    let payload = ["New".encode()].concat();
    let result = car_strategy_2.send_bytes(10, payload);
    assert!(!result.main_failed());

    // upload car races
    let car_races = Program::from_file(&system, PATH_TO_CAR_RACES);
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

    let payload = ["New".encode(), (init_config, session_config).encode()].concat();

    let result = car_races.send_bytes(10, payload);
    assert!(!result.main_failed());

    // allow messages
    let payload = [
        "CarRacesService".encode(),
        "AllowMessages".encode(),
        true.encode(),
    ]
    .concat();
    let result = car_races.send_bytes(10, payload);
    assert!(!result.main_failed());

    // add strategy ids
    let payload = [
        "CarRacesService".encode(),
        "AddStrategyIds".encode(),
        vec![
            ActorId::from(car_strategy_1.id()),
            ActorId::from(car_strategy_2.id()),
        ]
        .encode(),
    ]
    .concat();
    let result = car_races.send_bytes(10, payload);
    assert!(!result.main_failed());

    // start game
    let session_for_account: Option<ActorId> = None;
    let payload = [
        "CarRacesService".encode(),
        "StartGame".encode(),
        session_for_account.encode(),
    ]
    .concat();
    let result = car_races.send_bytes(10, payload);
    assert!(!result.main_failed());

    let mut game = if let Some(game) = get_game(&car_races, 10.into()) {
        game
    } else {
        std::panic!("Game does not exist")
    };

    while game.state != GameState::Finished {
        // make move (always accelerate)
        let session_for_account: Option<ActorId> = None;
        let payload = [
            "CarRacesService".encode(),
            "PlayerMove".encode(),
            (StrategyAction::BuyAcceleration, session_for_account).encode(),
        ]
        .concat();

        let result = car_races.send_bytes(10, payload);

        assert!(!result.main_failed());

        game = if let Some(game) = get_game(&car_races, 10.into()) {
            game
        } else {
            std::panic!("Game does not exist")
        };
    }

    // try to start game again
    let payload = [
        "CarRacesService".encode(),
        "StartGame".encode(),
        session_for_account.encode(),
    ]
    .concat();
    let result = car_races.send_bytes(10, payload);
    assert!(!result.main_failed());
}

fn get_game(car_races: &Program<'_>, account: ActorId) -> Option<Game> {
    let payload = [
        "CarRacesService".encode(),
        "Game".encode(),
        account.encode(),
    ]
    .concat();
    let result = car_races.send_bytes(10, payload);
    let log_entry = result
        .log()
        .iter()
        .find(|log_entry| log_entry.destination() == 10.into())
        .expect("Unable to get reply");

    let reply = <(String, String, Option<Game>)>::decode(&mut log_entry.payload())
        .expect("Unable to decode reply"); // Panic if decoding fails

    return reply.2;
}
