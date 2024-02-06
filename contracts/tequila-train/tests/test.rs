use gstd::{ActorId, Encode};
use gtest::{Program, System};
use tequila_train_io::*;

const ADMIN: u64 = 100;
pub const PLAYERS: [u64; 3] = [10, 11, 12];

pub trait TestFunc {
    fn create_game(&self, from: u64, players_limit: u64, bid: u128, error: Option<Error>);
    fn register(&self, from: u64, creator: ActorId, name: String, error: Option<Error>);
    fn start_game(&self, from: u64, error: Option<Error>);
    fn restart_game(&self, from: u64, error: Option<Error>);
    fn add_admin(&self, from: u64, admin: ActorId, error: Option<Error>);
    fn delete_admin(&self, from: u64, admin: ActorId, error: Option<Error>);
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
    fn create_game(&self, from: u64, players_limit: u64, bid: u128, error: Option<Error>) {
        let result = self.send(
            from,
            Command::CtreateGame { players_limit, bid},
        );
        assert!(!result.main_failed());
        let reply = if let Some(error) = error {
            Err(error)
        } else {
            Ok(Event::GameCreated)
        };
        assert!(result.contains(&(from, reply.encode())));
    }
    fn register(&self, from: u64, creator: ActorId, name: String, error: Option<Error>) {
        let result = self.send(
            from,
            Command::Register {
                creator,
                name: name.clone(),
            },
        );
        assert!(!result.main_failed());
        let reply = if let Some(error) = error {
            Err(error)
        } else {
            Ok(Event::Registered { player: from.into(), name })
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
    fn restart_game(&self, from: u64, error: Option<Error>) {
        let result = self.send(from, Command::RestartGame);
        assert!(!result.main_failed());
        let reply = if let Some(error) = error {
            Err(error)
        } else {
            Ok(Event::GameRestarted)
        };
        assert!(result.contains(&(from, reply.encode())));
    }
    fn add_admin(&self, from: u64, admin: ActorId, error: Option<Error>) {
        let result = self.send(from, Command::AddAdmin(admin));
        assert!(!result.main_failed());
        let reply = if let Some(error) = error {
            Err(error)
        } else {
            Ok(Event::AdminAdded(admin))
        };
        assert!(result.contains(&(from, reply.encode())));
    }
    fn delete_admin(&self, from: u64, admin: ActorId, error: Option<Error>) {
        let result = self.send(from, Command::DeleteAdmin(admin));
        assert!(!result.main_failed());
        let reply = if let Some(error) = error {
            Err(error)
        } else {
            Ok(Event::AdminDeleted(admin))
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

    program.create_game(ADMIN, 6, 0, None);

    program.register(PLAYERS[0], ADMIN.into(), "A".to_owned(), None);
    program.register(PLAYERS[1], ADMIN.into(), "B".to_owned(), None);
    program.start_game(ADMIN, None);

    let state: GameLauncherState = program
        .read_state(0)
        .expect("Unexpected invalid game state.");
    println!("STATE: {:?}", state);

    program.skip(PLAYERS[0], ADMIN.into(), None);
    let state: GameLauncherState = program
        .read_state(0)
        .expect("Unexpected invalid game state.");

    let start_tile = state.games[0].1.game_state.clone().unwrap().start_tile;
    let game_state = state.games[0].1.game_state.clone().unwrap();
    let tile = game_state.tiles.get(start_tile as usize).unwrap();
    println!("tile: {:?}", tile);
    let tile = game_state.tiles.get(10 as usize).unwrap();
    println!("tile: {:?}", tile);
    system.spend_blocks(30);
    program.skip(PLAYERS[1], ADMIN.into(), None);

    

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

// #[test]
// fn add_admin_test() {
//     let system = System::new();

//     system.init_logger();

//     let program = Program::current_opt(&system);

//     let result = program.send(2, 0);
//     assert!(!result.main_failed());

//     program.register(2, 0.into(), "A".to_owned(), None);
//     program.register(2, 1.into(), "B".to_owned(), None);
//     program.start_game(3, Some(Error::NotAdmin));
//     program.add_admin(4, 3.into(), Some(Error::NotAdmin));
//     program.add_admin(2, 3.into(), None);
//     program.start_game(3, None);
//     program.restart_game(3, None);
//     program.delete_admin(3, 2.into(), None);
//     program.start_game(2, Some(Error::NotAdmin));
// }
