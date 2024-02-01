use battleship_io::{
    BattleshipAction, BattleshipError, BattleshipInit, BattleshipReply, Entity, Ships, StateQuery,
    StateReply,
};
use gstd::prelude::*;
use gtest::{Program, System};

fn init_battleship(sys: &System) {
    let battleship = Program::current(sys);
    let bot = Program::from_file(
        sys,
        "../target/wasm32-unknown-unknown/debug/battleship_bot.opt.wasm",
    );
    let bot_init_result = bot.send_bytes(3, []);
    assert!(!bot_init_result.main_failed());

    let res = battleship.send(
        3,
        BattleshipInit {
            bot_address: 2.into(),
        },
    );
    assert!(!res.main_failed());
}

#[test]
fn failures_location_ships() {
    let system = System::new();
    system.init_logger();
    init_battleship(&system);
    let battleship = system.get_program(1);
    // outfield
    let ships = Ships {
        ship_1: vec![27],
        ship_2: vec![0, 1, 2],
        ship_3: vec![4, 9],
        ship_4: vec![16, 21],
    };
    let res = battleship.send(3, BattleshipAction::StartGame { ships });
    assert!(!res.main_failed());
    assert!(res.contains(&(
        3,
        Err::<BattleshipReply, BattleshipError>(BattleshipError::OutOfBounds).encode()
    )));

    // wrong ship size
    let ships = Ships {
        ship_1: vec![19],
        ship_2: vec![0, 1, 2, 3],
        ship_3: vec![4, 9],
        ship_4: vec![16, 21],
    };
    let res = battleship.send(3, BattleshipAction::StartGame { ships });
    assert!(!res.main_failed());
    assert!(res.contains(&(
        3,
        Err::<BattleshipReply, BattleshipError>(BattleshipError::WrongLength).encode()
    )));
    // ship crossing
    let ships = Ships {
        ship_1: vec![1],
        ship_2: vec![0, 1, 2],
        ship_3: vec![4, 9],
        ship_4: vec![16, 21],
    };
    let res = battleship.send(3, BattleshipAction::StartGame { ships });
    assert!(!res.main_failed());
    assert!(res.contains(&(
        3,
        Err::<BattleshipReply, BattleshipError>(BattleshipError::IncorrectLocationShips).encode()
    )));
    // the ship isn't solid
    let ships = Ships {
        ship_1: vec![19],
        ship_2: vec![0, 1, 3],
        ship_3: vec![4, 9],
        ship_4: vec![16, 21],
    };
    let res = battleship.send(3, BattleshipAction::StartGame { ships });
    assert!(!res.main_failed());
    assert!(res.contains(&(
        3,
        Err::<BattleshipReply, BattleshipError>(BattleshipError::IncorrectLocationShips).encode()
    )));
    // the distance between the ships is not maintained
    let ships = Ships {
        ship_1: vec![5],
        ship_2: vec![0, 1, 2],
        ship_3: vec![4, 9],
        ship_4: vec![16, 21],
    };
    let res = battleship.send(3, BattleshipAction::StartGame { ships });
    assert!(!res.main_failed());
    assert!(res.contains(&(
        3,
        Err::<BattleshipReply, BattleshipError>(BattleshipError::IncorrectLocationShips).encode()
    )));
}

#[test]
fn failures_test() {
    let system = System::new();
    system.init_logger();
    init_battleship(&system);
    let battleship = system.get_program(1);

    // the game hasn't started
    let res = battleship.send(3, BattleshipAction::Turn { step: 10 });
    assert!(!res.main_failed());
    assert!(res.contains(&(
        3,
        Err::<BattleshipReply, BattleshipError>(BattleshipError::GameIsNotStarted).encode()
    )));

    let ships = Ships {
        ship_1: vec![19],
        ship_2: vec![0, 1, 2],
        ship_3: vec![4, 9],
        ship_4: vec![16, 21],
    };
    let res = battleship.send(3, BattleshipAction::StartGame { ships });
    assert!(!res.main_failed());
    // you cannot start a new game until the previous one is finished
    let ships = Ships {
        ship_1: vec![19],
        ship_2: vec![0, 1, 2],
        ship_3: vec![4, 9],
        ship_4: vec![16, 21],
    };
    let res = battleship.send(3, BattleshipAction::StartGame { ships });
    assert!(!res.main_failed());
    assert!(res.contains(&(
        3,
        Err::<BattleshipReply, BattleshipError>(BattleshipError::GameIsAlreadyStarted).encode()
    )));

    // outfield
    let res = battleship.send(3, BattleshipAction::Turn { step: 25 });
    assert!(!res.main_failed());
    assert!(res.contains(&(
        3,
        Err::<BattleshipReply, BattleshipError>(BattleshipError::OutOfBounds).encode()
    )));

    // only the admin can change the bot's contract address
    let res = battleship.send(4, BattleshipAction::ChangeBot { bot: 8.into() });
    assert!(!res.main_failed());
    assert!(res.contains(&(
        4,
        Err::<BattleshipReply, BattleshipError>(BattleshipError::NotAdmin).encode()
    )));

    let steps: Vec<u8> = (0..25).collect();
    for step in steps {
        let reply = battleship
            .read_state(StateQuery::All)
            .expect("Unexpected invalid state.");
        if let StateReply::All(state) = reply {
            if state.games[0].1.bot_board[step as usize] == Entity::Empty
                || state.games[0].1.bot_board[step as usize] == Entity::Ship
            {
                if !state.games[0].1.game_over {
                    let res = battleship.send(3, BattleshipAction::Turn { step });
                    assert!(!res.main_failed());
                } else {
                    // game is over
                    let res = battleship.send(3, BattleshipAction::Turn { step: 25 });
                    assert!(!res.main_failed());
                    assert!(res.contains(&(
                        3,
                        Err::<BattleshipReply, BattleshipError>(BattleshipError::GameIsAlreadyOver)
                            .encode()
                    )));
                }
            }
        }
    }
}

#[test]
fn success_test() {
    let system = System::new();
    system.init_logger();
    init_battleship(&system);
    let battleship = system.get_program(1);
    let ships = Ships {
        ship_1: vec![19],
        ship_2: vec![0, 1, 2],
        ship_3: vec![4, 9],
        ship_4: vec![16, 21],
    };
    let res = battleship.send(3, BattleshipAction::StartGame { ships });
    assert!(!res.main_failed());

    let steps: Vec<u8> = (0..25).collect();
    for step in steps {
        let reply = battleship
            .read_state(StateQuery::All)
            .expect("Unexpected invalid state.");
        if let StateReply::All(state) = reply {
            if (state.games[0].1.bot_board[step as usize] == Entity::Empty
                || state.games[0].1.bot_board[step as usize] == Entity::Ship)
                && !state.games[0].1.game_over
            {
                let res = battleship.send(3, BattleshipAction::Turn { step });
                assert!(!res.main_failed());
            }
        }
    }
    let res = battleship.send(3, BattleshipAction::ChangeBot { bot: 5.into() });
    assert!(!res.main_failed());
}
