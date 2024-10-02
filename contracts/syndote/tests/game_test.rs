use gstd::{prelude::*, ActorId, MessageId};
use gtest::{Program, System};
use syndote_io::*;

#[test]
fn game() {
    let system = System::new();
    system.init_logger();
    system.mint_to(10, 100_000_000_000_000);
    let player_1 = Program::from_file(
        &system,
        "../target/wasm32-unknown-unknown/release/syndote_player.opt.wasm",
    );
    let player_2 = Program::from_file(
        &system,
        "../target/wasm32-unknown-unknown/release/syndote_player.opt.wasm",
    );
    let player_3 = Program::from_file(
        &system,
        "../target/wasm32-unknown-unknown/release/syndote_player.opt.wasm",
    );
    let player_4 = Program::from_file(
        &system,
        "../target/wasm32-unknown-unknown/release/syndote_player.opt.wasm",
    );
    let game = Program::current_opt(&system);
    check_send(&system, player_1.send::<_, ActorId>(10, 5.into()));
    check_send(&system, player_2.send::<_, ActorId>(10, 5.into()));
    check_send(&system, player_3.send::<_, ActorId>(10, 5.into()));
    check_send(&system, player_4.send::<_, ActorId>(10, 5.into()));
    check_send(&system, game.send(10, 0x00));

    check_send(
        &system,
        game.send(10, GameAction::Register { player: 1.into() }),
    );
    check_send(
        &system,
        game.send(10, GameAction::Register { player: 2.into() }),
    );
    check_send(
        &system,
        game.send(10, GameAction::Register { player: 3.into() }),
    );
    check_send(
        &system,
        game.send(10, GameAction::Register { player: 4.into() }),
    );

    game.send(10, GameAction::Play);
}

fn check_send(system: &System, mid: MessageId) {
    let res = system.run_next_block();
    assert!(res.succeed.contains(&mid));
}
