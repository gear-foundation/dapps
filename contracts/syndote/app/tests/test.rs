use sails_rs::calls::*;
use sails_rs::{
    gtest::{calls::*, Program, System},
    Encode, MessageId,
};
use syndote::{
    traits::{Syndote, SyndoteFactory},
    Syndote as SyndoteClient, SyndoteFactory as Factory, Config, GameStatus
};

pub const ADMIN_ID: u64 = 10;
pub const USER_1: u64 = 11;
pub const USER_2: u64 = 12;
pub const USER_3: u64 = 13;

#[tokio::test]
async fn test_play_game() {
    let system = System::new();
    system.init_logger();
    system.mint_to(ADMIN_ID, 100_000_000_000_000);
    system.mint_to(USER_1, 100_000_000_000_000);
    system.mint_to(USER_2, 100_000_000_000_000);
    system.mint_to(USER_3, 100_000_000_000_000);
    let program_space = GTestRemoting::new(system, ADMIN_ID.into());
    let code_id = program_space
        .system()
        .submit_code_file("../../target/wasm32-unknown-unknown/release/syndote.opt.wasm");

    let syndote_factory = Factory::new(program_space.clone());
    let config = Config {
        reservation_amount: 700_000_000_000,
        reservation_duration_in_block: 1_000,
        time_for_step: 10,
        min_gas_limit: 5_000_000_000,
        gas_refill_timeout: 30,
        gas_for_step: 10_000_000_000,
    };

    let syndote_id = syndote_factory
        .new(config, None)
        .send_recv(code_id, "123")
        .await
        .unwrap();

    let mut client = SyndoteClient::new(program_space.clone());

    // upload player program
    let admin_player = Program::from_file(
        program_space.system(),
        "../../target/wasm32-unknown-unknown/release/syndote_player.opt.wasm",
    );
    let request = ["New".encode(), ().encode()].concat();
    check_send(
        program_space.system(),
        admin_player.send_bytes(ADMIN_ID, request.clone()),
    );
    client
        .create_game_session(None, "ADMIN".to_string(), admin_player.id())
        .send_recv(syndote_id)
        .await
        .unwrap();

    let player_1 = Program::from_file(
        program_space.system(),
        "../../target/wasm32-unknown-unknown/release/syndote_player.opt.wasm",
    );
    let player_2 = Program::from_file(
        program_space.system(),
        "../../target/wasm32-unknown-unknown/release/syndote_player.opt.wasm",
    );
    let player_3 = Program::from_file(
        program_space.system(),
        "../../target/wasm32-unknown-unknown/release/syndote_player.opt.wasm",
    );

    check_send(
        program_space.system(),
        player_1.send_bytes(USER_1, request.clone()),
    );
    check_send(
        program_space.system(),
        player_2.send_bytes(USER_2, request.clone()),
    );
    check_send(
        program_space.system(),
        player_3.send_bytes(USER_3, request),
    );

    let state = client.get_game_session(ADMIN_ID.into()).recv(syndote_id).await.unwrap().unwrap();
    assert_eq!(state.game_status, GameStatus::Registration);

    // registration
    client
        .register(ADMIN_ID.into(), player_1.id(), "player_1".to_string())
        .with_args(GTestArgs::new(USER_1.into()))
        .send_recv(syndote_id)
        .await
        .unwrap();
    client
        .register(ADMIN_ID.into(), player_2.id(), "player_2".to_string())
        .with_args(GTestArgs::new(USER_2.into()))
        .send_recv(syndote_id)
        .await
        .unwrap();
    client
        .register(ADMIN_ID.into(), player_3.id(), "player_3".to_string())
        .with_args(GTestArgs::new(USER_3.into()))
        .send_recv(syndote_id)
        .await
        .unwrap();

    client
        .make_reservation(ADMIN_ID.into())
        .send_recv(syndote_id)
        .await
        .unwrap();

    client
        .play(ADMIN_ID.into())
        .send_recv(syndote_id)
        .await
        .unwrap();

    // check state
    let state = client.get_game_session(ADMIN_ID.into()).recv(syndote_id).await.unwrap();
    println!("STATE {:?}", state);
    // assert_eq!(state.game_status, GameStatus::Play);
    // assert_eq!(state.round, 0);
    // assert_eq!(state.winner, 0.into());

    // // start game
    // client.play().send_recv(syndote_id).await.unwrap();

    // // check state
    // let state = client.get_storage().recv(syndote_id).await.unwrap();
    // assert_eq!(state.game_status, GameStatus::Finished);
    // assert_ne!(state.round, 0);
    // assert_ne!(state.winner, 0.into());
}

fn check_send(system: &System, mid: MessageId) {
    let res = system.run_next_block();
    assert!(res.succeed.contains(&mid));
}
