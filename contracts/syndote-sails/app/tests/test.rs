use sails_rs::calls::*;
use sails_rs::{
    gtest::{calls::*, Program, System},
    Encode, MessageId,
};
use syndote_wasm::{
    traits::{Syndote, SyndoteFactory},
    GameStatus, Syndote as SyndoteClient, SyndoteFactory as Factory,
};

pub const ADMIN_ID: u64 = 10;
pub const USER_ID: u64 = 11;

#[tokio::test]
async fn test_play_game() {
    let system = System::new();
    system.init_logger();
    system.mint_to(ADMIN_ID, 100_000_000_000_000);
    system.mint_to(USER_ID, 100_000_000_000_000);
    let program_space = GTestRemoting::new(system, ADMIN_ID.into());
    let code_id = program_space
        .system()
        .submit_code_file("../../target/wasm32-unknown-unknown/release/syndote_wasm.opt.wasm");

    let syndote_factory = Factory::new(program_space.clone());

    let syndote_id = syndote_factory
        .new(None)
        .send_recv(code_id, "123")
        .await
        .unwrap();

    let mut client = SyndoteClient::new(program_space.clone());

    // upload player program
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
    let player_4 = Program::from_file(
        program_space.system(),
        "../../target/wasm32-unknown-unknown/release/syndote_player.opt.wasm",
    );
    let request = ["New".encode(), ().encode()].concat();
    check_send(
        program_space.system(),
        player_1.send_bytes(USER_ID, request.clone()),
    );
    check_send(
        program_space.system(),
        player_2.send_bytes(USER_ID, request.clone()),
    );
    check_send(
        program_space.system(),
        player_3.send_bytes(USER_ID, request.clone()),
    );
    check_send(
        program_space.system(),
        player_4.send_bytes(USER_ID, request),
    );

    let state = client.get_storage().recv(syndote_id).await.unwrap();
    assert_eq!(state.game_status, GameStatus::Registration);

    // registration
    client
        .register(player_1.id())
        .send_recv(syndote_id)
        .await
        .unwrap();
    client
        .register(player_2.id())
        .send_recv(syndote_id)
        .await
        .unwrap();
    client
        .register(player_3.id())
        .send_recv(syndote_id)
        .await
        .unwrap();
    client
        .register(player_4.id())
        .send_recv(syndote_id)
        .await
        .unwrap();

    // check state
    let state = client.get_storage().recv(syndote_id).await.unwrap();
    assert_eq!(state.game_status, GameStatus::Play);
    assert_eq!(state.round, 0);
    assert_eq!(state.winner, 0.into());

    // start game
    client.play().send_recv(syndote_id).await.unwrap();

    // check state
    let state = client.get_storage().recv(syndote_id).await.unwrap();
    assert_eq!(state.game_status, GameStatus::Finished);
    assert_ne!(state.round, 0);
    assert_ne!(state.winner, 0.into());
}

fn check_send(system: &System, mid: MessageId) {
    let res = system.run_next_block();
    assert!(res.succeed.contains(&mid));
}
