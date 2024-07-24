use battleship_io::{
    ActionsForSession, BattleshipAction, BattleshipError, BattleshipInit, BattleshipParticipants,
    BattleshipReply, Config, Entity, GameState, Session, Ships, StateQuery, StateReply,
    MINIMUM_SESSION_SURATION_MS,
};
use gstd::prelude::*;
use gtest::{Program, System};

const BLOCK_DURATION_MS: u64 = 1_000;

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
            config: Config {
                gas_for_start: 5_000_000_000,
                gas_for_move: 5_000_000_000,
                gas_to_delete_session: 5_000_000_000,
                block_duration_ms: BLOCK_DURATION_MS,
            },
        },
    );
    assert!(!res.main_failed());
}

#[test]
fn failures_location_ships() {
    let system = System::new();
    system.init_logger();
    init_battleship(&system);
    let battleship = system.get_program(1).unwrap();
    // outfield
    let ships = Ships {
        ship_1: vec![27],
        ship_2: vec![0, 1, 2],
        ship_3: vec![4, 9],
        ship_4: vec![16, 21],
    };
    let res = battleship.send(
        3,
        BattleshipAction::StartGame {
            ships,
            session_for_account: None,
        },
    );
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
    let res = battleship.send(
        3,
        BattleshipAction::StartGame {
            ships,
            session_for_account: None,
        },
    );
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
    let res = battleship.send(
        3,
        BattleshipAction::StartGame {
            ships,
            session_for_account: None,
        },
    );
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
    let res = battleship.send(
        3,
        BattleshipAction::StartGame {
            ships,
            session_for_account: None,
        },
    );
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
    let res = battleship.send(
        3,
        BattleshipAction::StartGame {
            ships,
            session_for_account: None,
        },
    );
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
    let battleship = system.get_program(1).unwrap();

    // the game hasn't started
    let res = battleship.send(
        3,
        BattleshipAction::Turn {
            step: 10,
            session_for_account: None,
        },
    );
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
    let res = battleship.send(
        3,
        BattleshipAction::StartGame {
            ships,
            session_for_account: None,
        },
    );
    assert!(!res.main_failed());
    // you cannot start a new game until the previous one is finished
    let ships = Ships {
        ship_1: vec![19],
        ship_2: vec![0, 1, 2],
        ship_3: vec![4, 9],
        ship_4: vec![16, 21],
    };
    let res = battleship.send(
        3,
        BattleshipAction::StartGame {
            ships,
            session_for_account: None,
        },
    );
    assert!(!res.main_failed());
    assert!(res.contains(&(
        3,
        Err::<BattleshipReply, BattleshipError>(BattleshipError::GameIsAlreadyStarted).encode()
    )));
    // outfield
    let res = battleship.send(
        3,
        BattleshipAction::Turn {
            step: 25,
            session_for_account: None,
        },
    );
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
                    let res = battleship.send(
                        3,
                        BattleshipAction::Turn {
                            step,
                            session_for_account: None,
                        },
                    );
                    assert!(!res.main_failed());
                } else {
                    // game is over
                    let res = battleship.send(
                        3,
                        BattleshipAction::Turn {
                            step: 25,
                            session_for_account: None,
                        },
                    );
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
    let battleship = system.get_program(1).unwrap();
    let ships = Ships {
        ship_1: vec![19],
        ship_2: vec![0, 1, 2],
        ship_3: vec![4, 9],
        ship_4: vec![16, 21],
    };
    let res = battleship.send(
        3,
        BattleshipAction::StartGame {
            ships,
            session_for_account: None,
        },
    );
    assert!(!res.main_failed());

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
                let res = battleship.send(
                    3,
                    BattleshipAction::Turn {
                        step,
                        session_for_account: None,
                    },
                );
                assert!(!res.main_failed());
            }
        }
    }
    let res = battleship.send(3, BattleshipAction::ChangeBot { bot: 5.into() });
    assert!(!res.main_failed());
}

// successful session creation
#[test]
fn create_session_success() {
    let system = System::new();
    system.init_logger();
    init_battleship(&system);
    let battleship = system.get_program(1).unwrap();

    let main_account = 3;
    let proxy_account = 10;

    let duration = MINIMUM_SESSION_SURATION_MS;
    let session = Session {
        key: proxy_account.into(),
        expires: system.block_timestamp() + duration,
        allowed_actions: vec![ActionsForSession::StartGame, ActionsForSession::Turn],
    };
    let res = battleship.send(
        main_account,
        BattleshipAction::CreateSession {
            key: proxy_account.into(),
            duration,
            allowed_actions: session.allowed_actions.clone(),
        },
    );

    assert!(res.contains(&(
        main_account,
        Ok::<BattleshipReply, BattleshipError>(BattleshipReply::SessionCreated).encode()
    )));

    check_session_in_state(&battleship, main_account, Some(session));
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
    init_battleship(&system);
    let battleship = system.get_program(1).unwrap();

    // The session duration is too long: the number of blocks is greater than u32::MAX.
    let number_of_blocks = u32::MAX as u64 + 1;
    // Block duration: 3 sec = 3000 ms
    let duration = number_of_blocks * BLOCK_DURATION_MS;
    let allowed_actions = vec![ActionsForSession::StartGame, ActionsForSession::Turn];
    let main_account = 3;
    let proxy_account = 10;

    let res = battleship.send(
        main_account,
        BattleshipAction::CreateSession {
            key: proxy_account.into(),
            duration,
            allowed_actions,
        },
    );

    assert!(res.main_failed());

    // The session duration is less than minimum session duration
    let duration = MINIMUM_SESSION_SURATION_MS - 1;
    let allowed_actions = vec![ActionsForSession::StartGame, ActionsForSession::Turn];

    let res = battleship.send(
        main_account,
        BattleshipAction::CreateSession {
            key: proxy_account.into(),
            duration,
            allowed_actions,
        },
    );

    assert!(!res.main_failed());
    assert!(res.contains(&(
        main_account,
        Err::<BattleshipReply, BattleshipError>(BattleshipError::DurationIsSmall).encode()
    )));

    // there are no allowed actions (empty array of allowed_actions).
    let duration = MINIMUM_SESSION_SURATION_MS;
    let allowed_actions = vec![];

    let res = battleship.send(
        main_account,
        BattleshipAction::CreateSession {
            key: proxy_account.into(),
            duration,
            allowed_actions,
        },
    );

    assert!(!res.main_failed());
    assert!(res.contains(&(
        main_account,
        Err::<BattleshipReply, BattleshipError>(BattleshipError::NoMessagesForApprovalWerePassed)
            .encode()
    )));

    // The user already has a current active session.
    let allowed_actions = vec![ActionsForSession::StartGame, ActionsForSession::Turn];

    let res = battleship.send(
        main_account,
        BattleshipAction::CreateSession {
            key: proxy_account.into(),
            duration,
            allowed_actions: allowed_actions.clone(),
        },
    );

    assert!(res.contains(&(
        main_account,
        Ok::<BattleshipReply, BattleshipError>(BattleshipReply::SessionCreated).encode()
    )));
    let res = battleship.send(
        main_account,
        BattleshipAction::CreateSession {
            key: proxy_account.into(),
            duration,
            allowed_actions,
        },
    );

    assert!(!res.main_failed());
    assert!(res.contains(&(
        main_account,
        Err::<BattleshipReply, BattleshipError>(BattleshipError::AlreadyHaveActiveSession).encode()
    )));
}

// TODO: fix test
// This function tests the mechanism where, upon creating a session, a delayed message is sent.
// This message is responsible for removing the session after its duration has expired.
// successful session creation
#[test]
#[ignore]
fn session_deletion_on_expiration() {
    let system = System::new();
    system.init_logger();
    init_battleship(&system);
    let battleship = system.get_program(1).unwrap();

    let main_account = 3;
    let proxy_account = 10;

    let duration = MINIMUM_SESSION_SURATION_MS + 1;
    let number_of_blocks = duration / BLOCK_DURATION_MS;
    let session = Session {
        key: proxy_account.into(),
        expires: system.block_timestamp() + duration,
        allowed_actions: vec![ActionsForSession::StartGame, ActionsForSession::Turn],
    };
    let res = battleship.send(
        main_account,
        BattleshipAction::CreateSession {
            key: proxy_account.into(),
            duration,
            allowed_actions: session.allowed_actions.clone(),
        },
    );

    assert!(res.contains(&(
        main_account,
        Ok::<BattleshipReply, BattleshipError>(BattleshipReply::SessionCreated).encode()
    )));

    system.spend_blocks((number_of_blocks as u32) + 1);

    check_session_in_state(&battleship, main_account, None);
}

// This test verifies that the contract does not allow the game to start
// if 'startGame' is not included in 'allowed_actions',
// and similarly, it prevents gameplay if 'Turn' is not specified in 'allowed_actions'."
#[test]
fn disallow_game_without_required_actions() {
    let system = System::new();
    system.init_logger();

    let main_account = 3;
    let proxy_account = 10;

    init_battleship(&system);
    let battleship = system.get_program(1).unwrap();

    let duration = MINIMUM_SESSION_SURATION_MS;
    let session = Session {
        key: proxy_account.into(),
        expires: system.block_timestamp() + duration,
        allowed_actions: vec![ActionsForSession::Turn],
    };

    let res = battleship.send(
        main_account,
        BattleshipAction::CreateSession {
            key: proxy_account.into(),
            duration,
            allowed_actions: session.allowed_actions.clone(),
        },
    );

    assert!(res.contains(&(
        main_account,
        Ok::<BattleshipReply, BattleshipError>(BattleshipReply::SessionCreated).encode()
    )));

    check_session_in_state(&battleship, main_account, Some(session));

    let ships = Ships {
        ship_1: vec![19],
        ship_2: vec![0, 1, 2],
        ship_3: vec![4, 9],
        ship_4: vec![16, 21],
    };

    // must fail since `StartGame` wasn't indicated in the `allowed_actions`
    let res = battleship.send(
        proxy_account,
        BattleshipAction::StartGame {
            ships: ships.clone(),
            session_for_account: Some(main_account.into()),
        },
    );

    assert!(!res.main_failed());
    assert!(res.contains(&(
        proxy_account,
        Err::<BattleshipReply, BattleshipError>(BattleshipError::MessageIsNotAllowed).encode()
    )));

    // delete session and create a new one
    let res = battleship.send(main_account, BattleshipAction::DeleteSessionFromAccount);
    assert!(res.contains(&(
        main_account,
        Ok::<BattleshipReply, BattleshipError>(BattleshipReply::SessionDeleted).encode()
    )));

    check_session_in_state(&battleship, main_account, None);

    let session = Session {
        key: proxy_account.into(),
        expires: system.block_timestamp() + duration,
        allowed_actions: vec![ActionsForSession::StartGame],
    };

    let res = battleship.send(
        main_account,
        BattleshipAction::CreateSession {
            key: proxy_account.into(),
            duration,
            allowed_actions: session.allowed_actions.clone(),
        },
    );

    assert!(res.contains(&(
        main_account,
        Ok::<BattleshipReply, BattleshipError>(BattleshipReply::SessionCreated).encode()
    )));

    check_session_in_state(&battleship, main_account, Some(session));

    // start game from proxy_account
    let res = battleship.send(
        proxy_account,
        BattleshipAction::StartGame {
            ships,
            session_for_account: Some(main_account.into()),
        },
    );

    assert!(res.contains(&(
        proxy_account,
        Ok::<BattleshipReply, BattleshipError>(BattleshipReply::MessageSentToBot).encode()
    )));

    // must fail since `Turn` wasn't indicated in the `allowed_actions`
    let steps: Vec<u8> = (0..25).collect();
    for step in steps {
        let game = get_game(&battleship, main_account);
        if (game.bot_board[step as usize] == Entity::Empty
            || game.bot_board[step as usize] == Entity::Ship)
            && !game.game_over
        {
            let res = battleship.send(
                proxy_account,
                BattleshipAction::Turn {
                    step,
                    session_for_account: Some(main_account.into()),
                },
            );
            assert!(!res.main_failed());
            assert!(res.contains(&(
                proxy_account,
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

    let main_account = 3;
    let proxy_account = 10;

    init_battleship(&system);
    let battleship = system.get_program(1).unwrap();

    let duration = MINIMUM_SESSION_SURATION_MS;
    let session = Session {
        key: proxy_account.into(),
        expires: system.block_timestamp() + duration,
        allowed_actions: vec![ActionsForSession::StartGame, ActionsForSession::Turn],
    };

    let res = battleship.send(
        main_account,
        BattleshipAction::CreateSession {
            key: proxy_account.into(),
            duration,
            allowed_actions: session.allowed_actions.clone(),
        },
    );

    assert!(res.contains(&(
        main_account,
        Ok::<BattleshipReply, BattleshipError>(BattleshipReply::SessionCreated).encode()
    )));

    check_session_in_state(&battleship, main_account, Some(session));

    let ships = Ships {
        ship_1: vec![19],
        ship_2: vec![0, 1, 2],
        ship_3: vec![4, 9],
        ship_4: vec![16, 21],
    };

    // start game from proxy_account
    let res = battleship.send(
        proxy_account,
        BattleshipAction::StartGame {
            ships,
            session_for_account: Some(main_account.into()),
        },
    );

    assert!(res.contains(&(
        proxy_account,
        Ok::<BattleshipReply, BattleshipError>(BattleshipReply::MessageSentToBot).encode()
    )));

    let steps: Vec<u8> = (0..25).collect();
    for step in steps {
        let game = get_game(&battleship, main_account);
        if (game.bot_board[step as usize] == Entity::Empty
            || game.bot_board[step as usize] == Entity::Ship)
            && !game.game_over
        {
            let res = battleship.send(
                proxy_account,
                BattleshipAction::Turn {
                    step,
                    session_for_account: Some(main_account.into()),
                },
            );
            let game = get_game(&battleship, main_account);
            if game.game_over {
                assert!(res.contains(&(
                    proxy_account,
                    Ok::<BattleshipReply, BattleshipError>(BattleshipReply::GameFinished(
                        BattleshipParticipants::Player
                    ))
                    .encode()
                )));
            } else {
                assert!(res.contains(&(
                    proxy_account,
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

    let main_account = 3;
    let proxy_account = 10;

    init_battleship(&system);
    let battleship = system.get_program(1).unwrap();

    let duration = MINIMUM_SESSION_SURATION_MS;
    let session = Session {
        key: proxy_account.into(),
        expires: system.block_timestamp() + duration,
        allowed_actions: vec![ActionsForSession::Turn],
    };

    let res = battleship.send(
        main_account,
        BattleshipAction::CreateSession {
            key: proxy_account.into(),
            duration,
            allowed_actions: session.allowed_actions.clone(),
        },
    );

    assert!(res.contains(&(
        main_account,
        Ok::<BattleshipReply, BattleshipError>(BattleshipReply::SessionCreated).encode()
    )));

    check_session_in_state(&battleship, main_account, Some(session));

    // delete session
    let res = battleship.send(main_account, BattleshipAction::DeleteSessionFromAccount);
    assert!(res.contains(&(
        main_account,
        Ok::<BattleshipReply, BattleshipError>(BattleshipReply::SessionDeleted).encode()
    )));

    check_session_in_state(&battleship, main_account, None);
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
