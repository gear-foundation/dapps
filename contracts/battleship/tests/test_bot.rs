use battleship_bot::BotBattleshipAction;
use battleship_io::Entity;
use gstd::prelude::*;
use gtest::{Program, System};

const USER_ID: u64 = 3;

#[test]
fn test() {
    let system = System::new();
    system.init_logger();
    system.mint_to(USER_ID, 1_000_000_000_000_000);

    let battleship = Program::from_file(
        &system,
        "../target/wasm32-gear/release/battleship_bot.opt.wasm",
    );

    let mid = battleship.send(USER_ID, 0);
    let res = system.run_next_block();
    assert!(res.succeed.contains(&mid));
    let mid = battleship.send(USER_ID, BotBattleshipAction::Start);
    let res = system.run_next_block();
    assert!(res.succeed.contains(&mid));

    let mut board = vec![Entity::Unknown; 25];
    board[12] = Entity::BoomShip;
    board[2] = Entity::DeadShip;
    board[3] = Entity::DeadShip;
    board[14] = Entity::DeadShip;
    board[17] = Entity::Boom;
    let mid = battleship.send(USER_ID, BotBattleshipAction::Turn(board));
    let res = system.run_next_block();
    assert!(res.succeed.contains(&mid));
}
