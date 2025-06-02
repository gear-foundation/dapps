use battleship_io::{
    ActionsForSession, BattleshipAction, BattleshipError, BattleshipInit, BattleshipParticipants,
    BattleshipReply, Config, Entity, GameState, Session, Ships, StateQuery, StateReply,
    MINIMUM_SESSION_DURATION_MS,
};
use gstd::prelude::*;
use gtest::{Program, System};

const BLOCK_DURATION_MS: u64 = 3_000;
const USER_ID: [u64; 2] = [3, 4];

fn init_battleship(sys: &System) {
    let battleship = Program::current(sys);
    let bot = Program::from_file(
        sys,
        "../target/wasm32-unknown-unknown/release/battleship_bot.opt.wasm",
    );
    let mid = bot.send_bytes(3, []);
    let res = sys.run_next_block();
    assert!(res.succeed.contains(&mid));

    let mid = battleship.send(
        USER_ID[0],
        BattleshipInit {
            bot_address: 2.into(),
            config: Config {
                gas_for_start: 5_000_000_000,
                gas_for_move: 5_000_000_000,
                gas_to_delete_session: 5_000_000_000,
                block_duration_ms: BLOCK_DURATION_MS,
            },
        },
    );
    let res = sys.run_next_block();
    assert!(res.succeed.contains(&mid));
}

#[test]
fn failures_location_ships() {
    let system = System::new();
    system.init_logger();
    system.mint_to(USER_ID[0], 100_000_000_000_000);
    init_battleship(&system);
    let battleship = system.get_program(1).unwrap();
    // outfield
    let ships = Ships {
        ship_1: vec![27],
        ship_2: vec![0, 1, 2],
        ship_3: vec![4, 9],
        ship_4: vec![16, 21],
    };
    battleship.send(
        USER_ID[0],
        BattleshipAction::StartGame {
            ships,
            session_for_account: None,
        },
    );
    let res = system.run_next_block();
    assert!(res.contains(&(
        USER_ID[0],
        Err::<BattleshipReply, BattleshipError>(BattleshipError::OutOfBounds).encode()
    )));
    // wrong ship size
    let ships = Ships {
        ship_1: vec![19],
        ship_2: vec![0, 1, 2, 3],
        ship_3: vec![4, 9],
        ship_4: vec![16, 21],
    };
    battleship.send(
        USER_ID[0],
        BattleshipAction::StartGame {
            ships,
            session_for_account: None,
        },
    );
    let res = system.run_next_block();
    assert!(res.contains(&(
        USER_ID[0],
        Err::<BattleshipReply, BattleshipError>(BattleshipError::WrongLength).encode()
    )));
    // ship crossing
    let ships = Ships {
        ship_1: vec![1],
        ship_2: vec![0, 1, 2],
        ship_3: vec![4, 9],
        ship_4: vec![16, 21],
    };
    battleship.send(
        USER_ID[0],
        BattleshipAction::StartGame {
            ships,
            session_for_account: None,
        },
    );
    let res = system.run_next_block();
    assert!(res.contains(&(
        USER_ID[0],
        Err::<BattleshipReply, BattleshipError>(BattleshipError::IncorrectLocationShips).encode()
    )));
    // the ship isn't solid
    let ships = Ships {
        ship_1: vec![19],
        ship_2: vec![0, 1, 3],
        ship_3: vec![4, 9],
        ship_4: vec![16, 21],
    };
    battleship.send(
        USER_ID[0],
        BattleshipAction::StartGame {
            ships,
            session_for_account: None,
        },
    );
    let res = system.run_next_block();
    assert!(res.contains(&(
        USER_ID[0],
        Err::<BattleshipReply, BattleshipError>(BattleshipError::IncorrectLocationShips).encode()
    )));
    // the distance between the ships is not maintained
    let ships = Ships {
        ship_1: vec![5],
        ship_2: vec![0, 1, 2],
        ship_3: vec![4, 9],
        ship_4: vec![16, 21],
    };
    battleship.send(
        USER_ID[0],
        BattleshipAction::StartGame {
            ships,
            session_for_account: None,
        },
    );
    let res = system.run_next_block();
    assert!(res.contains(&(
        USER_ID[0],
        Err::<BattleshipReply, BattleshipError>(BattleshipError::IncorrectLocationShips).encode()
    )));
}

#[test]
fn failures_test() {
    let system = System::new();
    system.init_logger();
    system.mint_to(USER_ID[0], 100_000_000_000_000);
    system.mint_to(USER_ID[1], 100_000_000_000_000);

    init_battleship(&system);
    let battleship = system.get_program(1).unwrap();

    // the game hasn't started
    battleship.send(
        USER_ID[0],
        BattleshipAction::Turn {
            step: 10,
            session_for_account: None,
        },
    );
    let res = system.run_next_block();
    assert!(res.contains(&(
        USER_ID[0],
        Err::<BattleshipReply, BattleshipError>(BattleshipError::GameIsNotStarted).encode()
    )));

    let ships = Ships {
        ship_1: vec![19],
        ship_2: vec![0, 1, 2],
        ship_3: vec![4, 9],
        ship_4: vec![16, 21],
    };
    battleship.send(
        USER_ID[0],
        BattleshipAction::StartGame {
            ships,
            session_for_account: None,
        },
    );
    system.run_next_block();
    // you cannot start a new game until the previous one is finished
    let ships = Ships {
        ship_1: vec![19],
        ship_2: vec![0, 1, 2],
        ship_3: vec![4, 9],
        ship_4: vec![16, 21],
    };
    battleship.send(
        USER_ID[0],
        BattleshipAction::StartGame {
            ships,
            session_for_account: None,
        },
    );
    let res = system.run_next_block();
    assert!(res.contains(&(
        USER_ID[0],
        Err::<BattleshipReply, BattleshipError>(BattleshipError::GameIsAlreadyStarted).encode()
    )));
    // outfield
    battleship.send(
        USER_ID[0],
        BattleshipAction::Turn {
            step: 25,
            session_for_account: None,
        },
    );
    let res = system.run_next_block();
    assert!(res.contains(&(
        USER_ID[0],
        Err::<BattleshipReply, BattleshipError>(BattleshipError::OutOfBounds).encode()
    )));

    // only the admin can change the bot's contract address
    battleship.send(USER_ID[1], BattleshipAction::ChangeBot { bot: 8.into() });
    let res = system.run_next_block();
    assert!(res.contains(&(
        USER_ID[1],
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
                    let mid = battleship.send(
                        USER_ID[0],
                        BattleshipAction::Turn {
                            step,
                            session_for_account: None,
                        },
                    );
                    let res = system.run_next_block();
                    assert!(res.succeed.contains(&mid));
                } else {
                    // game is over
                    battleship.send(
                        USER_ID[0],
                        BattleshipAction::Turn {
                            step: 24,
                            session_for_account: None,
                        },
                    );
                    let res = system.run_next_block();
                    assert!(res.contains(&(
                        USER_ID[0],
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
    system.mint_to(USER_ID[0], 100_000_000_000_000);
    init_battleship(&system);
    let battleship = system.get_program(1).unwrap();
    let ships = Ships {
        ship_1: vec![19],
        ship_2: vec![0, 1, 2],
        ship_3: vec![4, 9],
        ship_4: vec![16, 21],
    };
    let mid = battleship.send(
        USER_ID[0],
        BattleshipAction::StartGame {
            ships,
            session_for_account: None,
        },
    );
    let res = system.run_next_block();
    assert!(res.succeed.contains(&mid));

    let steps: Vec<u8> = (0..25).collect();
    for step in steps {
        let reply = battleship
            .read_state(StateQuery::All)
            .expect("Unexpected invalid state.");
        if let StateReply::All(state) = reply {
            if !(state.games[0].1.bot_board[step as usize] == Entity::Empty
                || state.games[0].1.bot_board[step as usize] == Entity::Ship
                || state.games[0].1.game_over)
            {
                let mid = battleship.send(
                    USER_ID[0],
                    BattleshipAction::Turn {
                        step,
                        session_for_account: None,
                    },
                );
                let res = system.run_next_block();
                assert!(res.succeed.contains(&mid));
            }
        }
    }
    let mid = battleship.send(USER_ID[0], BattleshipAction::ChangeBot { bot: 5.into() });
    let res = system.run_next_block();
    assert!(res.succeed.contains(&mid));
}

// successful session creation
#[test]
fn create_session_success() {
    let system = System::new();
    system.init_logger();
    system.mint_to(USER_ID[0], 100_000_000_000_000);
    init_battleship(&system);
    let battleship = system.get_program(1).unwrap();

    let duration = 180_000;
    let mut session = Session {
        key: USER_ID[1].into(),
        expires: system.block_timestamp() + duration,
        allowed_actions: vec![ActionsForSession::StartGame, ActionsForSession::Turn],
    };
    battleship.send(
        USER_ID[0],
        BattleshipAction::CreateSession {
            key: USER_ID[1].into(),
            duration,
            allowed_actions: session.allowed_actions.clone(),
        },
    );
    let res = system.run_next_block();
    assert!(res.contains(&(
        USER_ID[0],
        Ok::<BattleshipReply, BattleshipError>(BattleshipReply::SessionCreated).encode()
    )));
    session.expires += 3000;
    check_session_in_state(&battleship, USER_ID[0], Some(session));
}

// Failed session creation attempts:
// - If the session duration is too long: the number of blocks is greater than u32::MAX.
// - If the session duration is less minimum session duration (3 mins)
// - If there are no permitted actions (empty array of allowed_actions).
// - If the user already has a current active session.
#[test]
fn create_session_failures() {
    let system = System::new();
    system.init_logger();
    system.mint_to(USER_ID[0], 100_000_000_000_000);
    init_battleship(&system);
    let battleship = system.get_program(1).unwrap();

    // The session duration is too long: the number of blocks is greater than u32::MAX.
    let number_of_blocks = u32::MAX as u64 + 1;
    // Block duration: 3 sec = 3000 ms
    let duration = number_of_blocks * BLOCK_DURATION_MS;
    let allowed_actions = vec![ActionsForSession::StartGame, ActionsForSession::Turn];

    let mid = battleship.send(
        USER_ID[0],
        BattleshipAction::CreateSession {
            key: USER_ID[1].into(),
            duration,
            allowed_actions,
        },
    );

    let res = system.run_next_block();
    res.failed.contains(&mid);
    // The session duration is less than minimum session duration
    let duration = MINIMUM_SESSION_DURATION_MS - 1;
    let allowed_actions = vec![ActionsForSession::StartGame, ActionsForSession::Turn];

    battleship.send(
        USER_ID[0],
        BattleshipAction::CreateSession {
            key: USER_ID[1].into(),
            duration,
            allowed_actions,
        },
    );

    let res = system.run_next_block();
    assert!(res.contains(&(
        USER_ID[0],
        Err::<BattleshipReply, BattleshipError>(BattleshipError::DurationIsSmall).encode()
    )));

    // there are no allowed actions (empty array of allowed_actions).
    let duration = 180_000;
    let allowed_actions = vec![];

    battleship.send(
        USER_ID[0],
        BattleshipAction::CreateSession {
            key: USER_ID[1].into(),
            duration,
            allowed_actions,
        },
    );

    let res = system.run_next_block();
    assert!(res.contains(&(
        USER_ID[0],
        Err::<BattleshipReply, BattleshipError>(BattleshipError::NoMessagesForApprovalWerePassed)
            .encode()
    )));

    // The user already has a current active session.
    let allowed_actions = vec![ActionsForSession::StartGame, ActionsForSession::Turn];

    battleship.send(
        USER_ID[0],
        BattleshipAction::CreateSession {
            key: USER_ID[1].into(),
            duration,
            allowed_actions: allowed_actions.clone(),
        },
    );
    let res = system.run_next_block();
    assert!(res.contains(&(
        USER_ID[0],
        Ok::<BattleshipReply, BattleshipError>(BattleshipReply::SessionCreated).encode()
    )));
    battleship.send(
        USER_ID[0],
        BattleshipAction::CreateSession {
            key: USER_ID[1].into(),
            duration,
            allowed_actions,
        },
    );

    let res = system.run_next_block();
    assert!(res.contains(&(
        USER_ID[0],
        Err::<BattleshipReply, BattleshipError>(BattleshipError::AlreadyHaveActiveSession).encode()
    )));
}

// TODO: fix test
// This function tests the mechanism where, upon creating a session, a delayed message is sent.
// This message is responsible for removing the session after its duration has expired.
// successful session creation
#[test]
fn session_deletion_on_expiration() {
    let system = System::new();
    system.init_logger();
    system.mint_to(USER_ID[0], 100_000_000_000_000);
    init_battleship(&system);
    let battleship = system.get_program(1).unwrap();

    let duration = MINIMUM_SESSION_DURATION_MS;
    let number_of_blocks = duration / BLOCK_DURATION_MS;
    let session = Session {
        key: USER_ID[1].into(),
        expires: system.block_timestamp() + duration,
        allowed_actions: vec![ActionsForSession::StartGame, ActionsForSession::Turn],
    };
    battleship.send(
        USER_ID[0],
        BattleshipAction::CreateSession {
            key: USER_ID[1].into(),
            duration,
            allowed_actions: session.allowed_actions.clone(),
        },
    );
    let res = system.run_next_block();
    assert!(res.contains(&(
        USER_ID[0],
        Ok::<BattleshipReply, BattleshipError>(BattleshipReply::SessionCreated).encode()
    )));
    system.run_to_block(system.block_height() + (number_of_blocks as u32) + 1);
    check_session_in_state(&battleship, USER_ID[0], None);
}

// This test verifies that the contract does not allow the game to start
// if 'startGame' is not included in 'allowed_actions',
// and similarly, it prevents gameplay if 'Turn' is not specified in 'allowed_actions'."
#[test]
fn disallow_game_without_required_actions() {
    let system = System::new();
    system.init_logger();
    system.mint_to(USER_ID[0], 100_000_000_000_000);
    system.mint_to(USER_ID[1], 100_000_000_000_000);

    init_battleship(&system);
    let battleship = system.get_program(1).unwrap();

    let duration = MINIMUM_SESSION_DURATION_MS;
    let mut session = Session {
        key: USER_ID[1].into(),
        expires: system.block_timestamp() + duration,
        allowed_actions: vec![ActionsForSession::Turn],
    };

    battleship.send(
        USER_ID[0],
        BattleshipAction::CreateSession {
            key: USER_ID[1].into(),
            duration,
            allowed_actions: session.allowed_actions.clone(),
        },
    );
    let res = system.run_next_block();
    assert!(res.contains(&(
        USER_ID[0],
        Ok::<BattleshipReply, BattleshipError>(BattleshipReply::SessionCreated).encode()
    )));
    session.expires += 3000;
    check_session_in_state(&battleship, USER_ID[0], Some(session));

    let ships = Ships {
        ship_1: vec![19],
        ship_2: vec![0, 1, 2],
        ship_3: vec![4, 9],
        ship_4: vec![16, 21],
    };

    // must fail since `StartGame` wasn't indicated in the `allowed_actions`
    battleship.send(
        USER_ID[1],
        BattleshipAction::StartGame {
            ships: ships.clone(),
            session_for_account: Some(USER_ID[0].into()),
        },
    );

    let res = system.run_next_block();
    assert!(res.contains(&(
        USER_ID[1],
        Err::<BattleshipReply, BattleshipError>(BattleshipError::MessageIsNotAllowed).encode()
    )));

    // delete session and create a new one
    battleship.send(USER_ID[0], BattleshipAction::DeleteSessionFromAccount);
    let res = system.run_next_block();
    assert!(res.contains(&(
        USER_ID[0],
        Ok::<BattleshipReply, BattleshipError>(BattleshipReply::SessionDeleted).encode()
    )));

    check_session_in_state(&battleship, USER_ID[0], None);

    let mut session = Session {
        key: USER_ID[1].into(),
        expires: system.block_timestamp() + duration,
        allowed_actions: vec![ActionsForSession::StartGame],
    };

    battleship.send(
        USER_ID[0],
        BattleshipAction::CreateSession {
            key: USER_ID[1].into(),
            duration,
            allowed_actions: session.allowed_actions.clone(),
        },
    );
    let res = system.run_next_block();
    assert!(res.contains(&(
        USER_ID[0],
        Ok::<BattleshipReply, BattleshipError>(BattleshipReply::SessionCreated).encode()
    )));
    session.expires += 3000;
    check_session_in_state(&battleship, USER_ID[0], Some(session));

    // start game from USER_ID[1]
    battleship.send(
        USER_ID[1],
        BattleshipAction::StartGame {
            ships,
            session_for_account: Some(USER_ID[0].into()),
        },
    );
    let res = system.run_next_block();
    assert!(res.contains(&(
        USER_ID[1],
        Ok::<BattleshipReply, BattleshipError>(BattleshipReply::MessageSentToBot).encode()
    )));

    // must fail since `Turn` wasn't indicated in the `allowed_actions`
    let steps: Vec<u8> = (0..25).collect();
    for step in steps {
        let game = get_game(&battleship, USER_ID[0]);
        if (game.bot_board[step as usize] == Entity::Empty
            || game.bot_board[step as usize] == Entity::Ship)
            && !game.game_over
        {
            battleship.send(
                USER_ID[1],
                BattleshipAction::Turn {
                    step,
                    session_for_account: Some(USER_ID[0].into()),
                },
            );
            let res = system.run_next_block();
            assert!(res.contains(&(
                USER_ID[1],
                Err::<BattleshipReply, BattleshipError>(BattleshipError::MessageIsNotAllowed)
                    .encode()
            )));
        }
    }
}

// This test verifies the successful execution of a full game session, ensuring all gameplay mechanics and session lifecycle work as intended
#[test]
fn complete_session_game() {
    let system = System::new();
    system.init_logger();
    system.mint_to(USER_ID[0], 100_000_000_000_000);
    system.mint_to(USER_ID[1], 100_000_000_000_000);

    init_battleship(&system);
    let battleship = system.get_program(1).unwrap();

    let duration = MINIMUM_SESSION_DURATION_MS;
    let mut session = Session {
        key: USER_ID[1].into(),
        expires: system.block_timestamp() + duration,
        allowed_actions: vec![ActionsForSession::StartGame, ActionsForSession::Turn],
    };

    battleship.send(
        USER_ID[0],
        BattleshipAction::CreateSession {
            key: USER_ID[1].into(),
            duration,
            allowed_actions: session.allowed_actions.clone(),
        },
    );
    let res = system.run_next_block();
    assert!(res.contains(&(
        USER_ID[0],
        Ok::<BattleshipReply, BattleshipError>(BattleshipReply::SessionCreated).encode()
    )));
    session.expires += 3000;
    check_session_in_state(&battleship, USER_ID[0], Some(session));

    let ships = Ships {
        ship_1: vec![19],
        ship_2: vec![0, 1, 2],
        ship_3: vec![4, 9],
        ship_4: vec![16, 21],
    };

    // start game from USER_ID[1]
    battleship.send(
        USER_ID[1],
        BattleshipAction::StartGame {
            ships,
            session_for_account: Some(USER_ID[0].into()),
        },
    );
    let res = system.run_next_block();
    assert!(res.contains(&(
        USER_ID[1],
        Ok::<BattleshipReply, BattleshipError>(BattleshipReply::MessageSentToBot).encode()
    )));

    let steps: Vec<u8> = (0..25).collect();
    for step in steps {
        let game = get_game(&battleship, USER_ID[0]);
        if (game.bot_board[step as usize] == Entity::Empty
            || game.bot_board[step as usize] == Entity::Ship)
            && !game.game_over
        {
            battleship.send(
                USER_ID[1],
                BattleshipAction::Turn {
                    step,
                    session_for_account: Some(USER_ID[0].into()),
                },
            );

            let game = get_game(&battleship, USER_ID[0]);
            let res = system.run_next_block();
            if game.game_over {
                assert!(res.contains(&(
                    USER_ID[1],
                    Ok::<BattleshipReply, BattleshipError>(BattleshipReply::GameFinished(
                        BattleshipParticipants::Player
                    ))
                    .encode()
                )));
            } else {
                assert!(res.contains(&(
                    USER_ID[1],
                    Ok::<BattleshipReply, BattleshipError>(BattleshipReply::MessageSentToBot)
                        .encode()
                )));
            }
        }
    }
}

// Checks whether the session is correctly terminated when a user attempts to delete it prematurely,
// ensuring that the contract handles early termination scenarios appropriately.
#[test]
fn premature_session_deletion_by_user() {
    let system = System::new();
    system.init_logger();
    system.mint_to(USER_ID[0], 100_000_000_000_000);
    system.mint_to(USER_ID[1], 100_000_000_000_000);

    init_battleship(&system);
    let battleship = system.get_program(1).unwrap();

    let duration = MINIMUM_SESSION_DURATION_MS;
    let mut session = Session {
        key: USER_ID[1].into(),
        expires: system.block_timestamp() + duration,
        allowed_actions: vec![ActionsForSession::Turn],
    };

    battleship.send(
        USER_ID[0],
        BattleshipAction::CreateSession {
            key: USER_ID[1].into(),
            duration,
            allowed_actions: session.allowed_actions.clone(),
        },
    );
    let res = system.run_next_block();
    assert!(res.contains(&(
        USER_ID[0],
        Ok::<BattleshipReply, BattleshipError>(BattleshipReply::SessionCreated).encode()
    )));
    session.expires += 3000;
    check_session_in_state(&battleship, USER_ID[0], Some(session));

    // delete session
    battleship.send(USER_ID[0], BattleshipAction::DeleteSessionFromAccount);
    let res = system.run_next_block();
    assert!(res.contains(&(
        USER_ID[0],
        Ok::<BattleshipReply, BattleshipError>(BattleshipReply::SessionDeleted).encode()
    )));

    check_session_in_state(&battleship, USER_ID[0], None);
}

fn check_session_in_state(battleship: &Program<'_>, account: u64, session: Option<Session>) {
    let reply = battleship
        .read_state(StateQuery::SessionForTheAccount(account.into()))
        .expect("Error in reading the state");

    if let StateReply::SessionForTheAccount(session_from_state) = reply {
        assert_eq!(session, session_from_state, "Sessions do not match");
    } else {
        gstd::panic!("Wrong received state reply");
    }
}

fn get_game(battleship: &Program<'_>, player_id: u64) -> GameState {
    let reply = battleship
        .read_state(StateQuery::Game(player_id.into()))
        .expect("Error in reading the state");

    if let StateReply::Game(Some(game_state)) = reply {
        game_state
    } else {
        gstd::panic!("Wrong received state reply");
    }
}
