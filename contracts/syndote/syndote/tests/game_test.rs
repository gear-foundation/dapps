use gstd::{prelude::*, ActorId};
use gtest::{Program, System};
use syndote_io::*;

#[test]
fn game() {
    let system = System::new();
    system.init_logger();
    let player_1 = Program::from_file(
        &system,
        "../target/wasm32-unknown-unknown/release/player.wasm",
    );
    let player_2 = Program::from_file(
        &system,
        "../target/wasm32-unknown-unknown/release/player.wasm",
    );
    let player_3 = Program::from_file(
        &system,
        "../target/wasm32-unknown-unknown/release/player.wasm",
    );
    let player_4 = Program::from_file(
        &system,
        "../target/wasm32-unknown-unknown/release/player.wasm",
    );
    let game = Program::current(&system);

    assert!(!player_1.send::<_, ActorId>(10, 5.into()).main_failed());
    assert!(!player_2.send::<_, ActorId>(10, 5.into()).main_failed());
    assert!(!player_3.send::<_, ActorId>(10, 5.into()).main_failed());
    assert!(!player_4.send::<_, ActorId>(10, 5.into()).main_failed());

    assert!(!game.send(10, 0x00).main_failed());

    assert!(!game
        .send(10, GameAction::Register { player: 1.into() })
        .main_failed());
    assert!(!game
        .send(10, GameAction::Register { player: 2.into() })
        .main_failed());
    assert!(!game
        .send(10, GameAction::Register { player: 3.into() })
        .main_failed());
    assert!(!game
        .send(10, GameAction::Register { player: 4.into() })
        .main_failed());

    game.send(10, GameAction::Play);
}
