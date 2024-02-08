use gstd::{ActorId, Encode};
use gtest::{Program, System};
use tequila_train_io::*;

pub const PLAYERS: [u64; 3] = [10, 11, 12];

pub trait TestFunc {
    fn create_game(&self, from: u64, bid: u128, error: Option<Error>);
    fn register(&self, from: u64, bid: u128, creator: ActorId, error: Option<Error>);
    fn cancel_register(&self, from: u64, creator: ActorId, error: Option<Error>);
    fn delete_player(&self, from: u64, player_id: ActorId, error: Option<Error>);
    fn cancel_game(&self, from: u64, error: Option<Error>);
    fn start_game(&self, from: u64, error: Option<Error>);
    fn skip(&self, from: u64, creator: ActorId, error: Option<Error>);
    fn place(
        &self,
        from: u64,
        creator: ActorId,
        tile_id: u32,
        track_id: u32,
        remove_train: bool,
        error: Option<Error>,
    );
}

impl TestFunc for Program<'_> {
    fn create_game(&self, from: u64, bid: u128, error: Option<Error>) {
        let result = self.send_with_value(
            from,
            Command::CreateGame {bid},
            bid,
        );
        assert!(!result.main_failed());
        let reply = if let Some(error) = error {
            Err(error)
        } else {
            Ok(Event::GameCreated)
        };
        assert!(result.contains(&(from, reply.encode())));
    }
    fn register(&self, from: u64, bid: u128, creator: ActorId, error: Option<Error>) {
        let result = self.send_with_value(
            from,
            Command::Register {
                creator,
            },
            bid
        );
        assert!(!result.main_failed());
        let reply = if let Some(error) = error {
            Err(error)
        } else {
            Ok(Event::Registered { player: from.into() })
        };
        assert!(result.contains(&(from, reply.encode())));
    }
    fn cancel_register(&self, from: u64, creator: ActorId, error: Option<Error>) {
        let result = self.send(
            from,
            Command::CancelRegistration {
                creator,
            },
        );
        assert!(!result.main_failed());
        let reply = if let Some(error) = error {
            Err(error)
        } else {
            Ok(Event::RegistrationCanceled)
        };
        assert!(result.contains(&(from, reply.encode())));
    }
    fn delete_player(&self, from: u64, player_id: ActorId, error: Option<Error>) {
        let result = self.send(
            from,
            Command::DeletePlayer {
                player_id,
            },
        );
        assert!(!result.main_failed());
        let reply = if let Some(error) = error {
            Err(error)
        } else {
            Ok(Event::PlayerDeleted { player_id })
        };
        assert!(result.contains(&(from, reply.encode())));
    }
    fn cancel_game(&self, from: u64, error: Option<Error>) {
        let result = self.send(
            from,
            Command::CancelGame,
        );
        let res = &result.decoded_log::<Result<Event, Error>>();
        println!("RES: {:?}", res);
        assert!(!result.main_failed());
        let reply = if let Some(error) = error {
            Err(error)
        } else {
            Ok(Event::GameCanceled)
        };
        assert!(result.contains(&(from, reply.encode())));
    }
    fn start_game(&self, from: u64, error: Option<Error>) {
        let result = self.send(from, Command::StartGame);
        assert!(!result.main_failed());
        let res = &result.decoded_log::<Result<Event, Error>>();
        println!("RES: {:?}", res);
        let reply = if let Some(error) = error {
            Err(error)
        } else {
            Ok(Event::GameStarted)
        };
        assert!(result.contains(&(from, reply.encode())));
    }
    fn skip(&self, from: u64, creator: ActorId, error: Option<Error>) {
        let result = self.send(from, Command::Skip {creator});
        assert!(!result.main_failed());
        let res = &result.decoded_log::<Result<Event, Error>>();
        println!("RES: {:?}", res);
        let reply = if let Some(error) = error {
            Err(error)
        } else {
            Ok(Event::Skipped)
        };
        assert!(result.contains(&(from, reply.encode())));
    }
    fn place(
        &self,
        from: u64,
        creator: ActorId,
        tile_id: u32,
        track_id: u32,
        remove_train: bool,
        error: Option<Error>,
    ) {
        let result = self.send(
            from,
            Command::Place {
                creator,
                tile_id,
                track_id,
                remove_train,
            },
        );
        let res = &result.decoded_log::<Result<Event, Error>>();
        println!("RES: {:?}", res);
        assert!(!result.main_failed());
        let reply = if let Some(error) = error {
            Err(error)
        } else {
            Ok(Event::Placed {
                tile_id,
                track_id,
                remove_train,
            })
        };
        assert!(result.contains(&(from, reply.encode())));
    }
}
#[test]
fn success_test() {
    let system = System::new();

    system.init_logger();

    let program = Program::current_opt(&system);

    let config = Config{
        time_to_move: 30_000,
    };

    let result = program.send(2, config);
    assert!(!result.main_failed());

    program.create_game(PLAYERS[0], 0, None);

    program.register(PLAYERS[1], 0,  PLAYERS[0].into(), None);
    program.register(PLAYERS[2], 0, PLAYERS[0].into(), None);
    program.start_game(PLAYERS[0], None);

    let state: GameLauncherState = get_all_state(&program).expect("Unexpected invalid game state.");
    println!("STATE: {:?}", state);

    let game = state.games[0].1.game_state.clone().unwrap();
    let current_player = game.current_player;

    program.skip(PLAYERS[current_player as usize], PLAYERS[0].into(), None);

    let current_player = (current_player + 1) as usize % PLAYERS.len();
    program.skip(PLAYERS[current_player], PLAYERS[0].into(), None);

    system.spend_blocks(30);
    let current_player = (current_player + 1) as usize % PLAYERS.len();
    program.skip(PLAYERS[current_player], PLAYERS[0].into(), Some(Error::NotYourTurnOrYouLose));

    // program.place(PLAYERS[0], ADMIN.into(), 27, 0, false, None);

    // assert_eq!(
    //     state.games[0].1
    //         .game_state
    //         .expect("Invalid game state. Game is not initialized.")
    //         .players,
    //     vec![(0.into(), "A".to_owned()), (1.into(), "B".to_owned())]
    // );

    // program.restart_game(2, None);
    // program.register(2, 2.into(), "C".to_owned(), None);
    // program.register(2, 3.into(), "D".to_owned(), None);
    // program.start_game(2, None);

    // let state: GameLauncherState = program
    //     .read_state(0)
    //     .expect("Unexpected invalid game state.");

    // assert_eq!(
    //     state
    //         .game_state
    //         .expect("Invalid game state. Game is not initialized.")
    //         .players,
    //     vec![(2.into(), "C".to_owned()), (3.into(), "D".to_owned())]
    // );
    // program.place(2, 27, 0, false, None);
    // let state: GameLauncherState = program
    //     .read_state(0)
    //     .expect("Unexpected invalid game state.");
    // assert!(!state.game_state.unwrap().tracks[0].tiles.is_empty());
    // program.skip(2, None);
    // program.skip(3, None);
}
#[test]
fn cancel_register() {
    let system = System::new();

    system.init_logger();

    let program = Program::current_opt(&system);

    let config = Config{
        time_to_move: 30_000,
    };

    let result = program.send(2, config);
    assert!(!result.main_failed());

    let bid = 11_000_000_000_000;
    system.mint_to(PLAYERS[0], bid);
    program.create_game(PLAYERS[0], bid, None);

    system.mint_to(PLAYERS[1], bid);
    program.register(PLAYERS[1], bid, PLAYERS[0].into(), None);
    let balance = system.balance_of(PLAYERS[1]);
    assert_eq!(balance, 0);

    let state: GameLauncherState = get_all_state(&program).expect("Unexpected invalid game state.");
    assert_eq!(state.games[0].1.initial_players.len(), 2);


    program.cancel_register(PLAYERS[1], PLAYERS[0].into(), None);
    system.claim_value_from_mailbox(PLAYERS[1]);
    let balance = system.balance_of(PLAYERS[1]);
    assert_eq!(balance, bid);

    let state: GameLauncherState = get_all_state(&program).expect("Unexpected invalid game state.");
    assert_eq!(state.games[0].1.initial_players.len(), 1);

}

#[test]
fn delete_player() {
    let system = System::new();

    system.init_logger();

    let program = Program::current_opt(&system);

    let config = Config{
        time_to_move: 30_000,
    };

    let result = program.send(2, config);
    assert!(!result.main_failed());

    let bid = 11_000_000_000_000;
    system.mint_to(PLAYERS[0], bid);
    program.create_game(PLAYERS[0], bid, None);

    system.mint_to(PLAYERS[1], bid);
    program.register(PLAYERS[1], bid, PLAYERS[0].into(), None);
    let balance = system.balance_of(PLAYERS[1]);
    assert_eq!(balance, 0);

    let state: GameLauncherState = get_all_state(&program).expect("Unexpected invalid game state.");
    assert_eq!(state.games[0].1.initial_players.len(), 2);


    program.delete_player(PLAYERS[0], PLAYERS[1].into(), None);
    system.claim_value_from_mailbox(PLAYERS[1]);
    let balance = system.balance_of(PLAYERS[1]);
    assert_eq!(balance, bid);

    let state: GameLauncherState = get_all_state(&program).expect("Unexpected invalid game state.");
    assert_eq!(state.games[0].1.initial_players.len(), 1);

}

#[test]
fn cancel_game() {
    let system = System::new();

    system.init_logger();

    let program = Program::current_opt(&system);

    let config = Config{
        time_to_move: 30_000,
    };

    let result = program.send(2, config);
    assert!(!result.main_failed());

    let bid = 11_000_000_000_000;
    system.mint_to(PLAYERS[0], bid);
    program.create_game(PLAYERS[0], bid, None);

    system.mint_to(PLAYERS[1], bid);
    program.register(PLAYERS[1], bid, PLAYERS[0].into(), None);
    let balance = system.balance_of(PLAYERS[1]);
    assert_eq!(balance, 0);

    let state: GameLauncherState = get_all_state(&program).expect("Unexpected invalid game state.");
    assert!(!state.games.is_empty());

    program.cancel_game(PLAYERS[0], None);
    system.claim_value_from_mailbox(PLAYERS[1]);
    let balance = system.balance_of(PLAYERS[1]);
    assert_eq!(balance, bid);
    system.claim_value_from_mailbox(PLAYERS[0]);
    let balance = system.balance_of(PLAYERS[0]);
    assert_eq!(balance, bid);

    let state: GameLauncherState = get_all_state(&program).expect("Unexpected invalid game state.");
    assert!(state.games.is_empty());

}


// #[test]
// fn failures_test() {
//     let system = System::new();

//     system.init_logger();

//     let program = Program::current_opt(&system);

//     let result = program.send(2, Some(2_u64));
//     assert!(!result.main_failed());

//     program.start_game(2, Some(Error::WrongPlayersCount));
//     program.restart_game(2, Some(Error::GameHasNotStartedYet));
//     program.register(2, 0.into(), "A".to_owned(), None);
//     program.register(
//         2,
//         1.into(),
//         "A".to_owned(),
//         Some(Error::NameAlreadyExistsOrYouRegistered),
//     );
//     program.register(
//         2,
//         0.into(),
//         "B".to_owned(),
//         Some(Error::NameAlreadyExistsOrYouRegistered),
//     );
//     program.register(2, 1.into(), "B".to_owned(), None);
//     program.register(
//         2,
//         3.into(),
//         "C".to_owned(),
//         Some(Error::LimitHasBeenReached),
//     );

//     program.start_game(2, None);
//     program.register(
//         2,
//         3.into(),
//         "C".to_owned(),
//         Some(Error::GameHasAlreadyStarted),
//     );
//     program.start_game(2, Some(Error::GameHasAlreadyStarted));

//     let state: GameLauncherState = program
//         .read_state(0)
//         .expect("Unexpected invalid game state.");
//     assert_eq!(
//         state
//             .game_state
//             .expect("Invalid game state. Game is not initialized.")
//             .players,
//         vec![(0.into(), "A".to_owned()), (1.into(), "B".to_owned())]
//     );

//     program.place(3, 0, 0, false, Some(Error::NotYourTurn));
//     program.place(0, 3, 0, false, Some(Error::InvalidTileId));
//     program.place(0, 1, 1, false, Some(Error::InvalidTrack));
//     program.place(0, 1, 0, false, Some(Error::InvalidTile));
// }


