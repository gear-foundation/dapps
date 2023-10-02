use battleship_bot::BotBattleshipAction;
use battleship_io::Entity;
use gstd::prelude::*;
use gtest::{Program, System};

#[test]
fn test() {
    let system = System::new();
    system.init_logger();

    let battleship = Program::from_file(
        &system,
        "../target/wasm32-unknown-unknown/debug/battleship_bot.opt.wasm",
    );

    let res = battleship.send(2, 0);
    assert!(!res.main_failed());
    let res = battleship.send(2, BotBattleshipAction::Start);
    assert!(!res.main_failed());

    let mut board = vec![Entity::Unknown; 25];
    board[12] = Entity::BoomShip;
    board[2] = Entity::DeadShip;
    board[3] = Entity::DeadShip;
    board[14] = Entity::DeadShip;
    board[17] = Entity::Boom;
    let res = battleship.send(2, BotBattleshipAction::Turn(board));
    assert!(!res.main_failed());
}
