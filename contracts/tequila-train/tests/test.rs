use gstd::{ActorId, Encode};
use gtest::{Program, System};
use tequila_train_io::*;

pub trait TestFunc {
    fn register(&self, from: u64, player: ActorId, name: String, error: Option<Error>);
    fn start_game(&self, from: u64, error: Option<Error>);
    fn restart_game(&self, from: u64, players_limit: Option<u64>, error: Option<Error>);
    fn skip(&self, from: u64, error: Option<Error>);
    fn place(
        &self,
        from: u64,
        tile_id: u32,
        track_id: u32,
        remove_train: bool,
        error: Option<Error>,
    );
}

impl TestFunc for Program<'_> {
    fn register(&self, from: u64, player: ActorId, name: String, error: Option<Error>) {
        let result = self.send(
            from,
            Command::Register {
                player,
                name: name.clone(),
            },
        );
        assert!(!result.main_failed());
        let reply: Result<Event, Error>;
        if let Some(error) = error {
            reply = Err(error);
            assert!(result.contains(&(from, reply.encode())));
        } else {
            reply = Ok(Event::Registered { player, name });
            assert!(result.contains(&(from, reply.encode())));
        }
    }
    fn start_game(&self, from: u64, error: Option<Error>) {
        let result = self.send(from, Command::StartGame);
        assert!(!result.main_failed());
        let reply: Result<Event, Error>;
        if let Some(error) = error {
            reply = Err(error);
            assert!(result.contains(&(from, reply.encode())));
        } else {
            reply = Ok(Event::GameStarted);
            assert!(result.contains(&(from, reply.encode())));
        }
    }
    fn restart_game(&self, from: u64, players_limit: Option<u64>, error: Option<Error>) {
        let result = self.send(from, Command::RestartGame(players_limit));
        assert!(!result.main_failed());
        let reply: Result<Event, Error>;
        if let Some(error) = error {
            reply = Err(error);
            assert!(result.contains(&(from, reply.encode())));
        } else {
            reply = Ok(Event::GameRestarted(players_limit));
            assert!(result.contains(&(from, reply.encode())));
        }
    }
    fn skip(&self, from: u64, error: Option<Error>) {
        let result = self.send(from, Command::Skip);
        assert!(!result.main_failed());
        let reply: Result<Event, Error>;
        if let Some(error) = error {
            reply = Err(error);
            assert!(result.contains(&(from, reply.encode())));
        } else {
            reply = Ok(Event::Skipped);
            assert!(result.contains(&(from, reply.encode())));
        }
    }
    fn place(
        &self,
        from: u64,
        tile_id: u32,
        track_id: u32,
        remove_train: bool,
        error: Option<Error>,
    ) {
        let result = self.send(
            from,
            Command::Place {
                tile_id,
                track_id,
                remove_train,
            },
        );
        assert!(!result.main_failed());
        let reply: Result<Event, Error>;
        if let Some(error) = error {
            reply = Err(error);
            assert!(result.contains(&(from, reply.encode())));
        } else {
            reply = Ok(Event::Placed {
                tile_id,
                track_id,
                remove_train,
            });
            assert!(result.contains(&(from, reply.encode())));
        }
    }
}
#[test]
fn success_test() {
    let system = System::new();

    system.init_logger();

    let program = Program::current_opt(&system);

    let result = program.send(2, 0);
    assert!(!result.main_failed());

    program.register(2, 0.into(), "A".to_owned(), None);
    program.register(2, 1.into(), "B".to_owned(), None);
    program.start_game(2, None);

    let state: GameLauncher = program
        .read_state(0)
        .expect("Unexpected invalid game state.");
    assert_eq!(
        state
            .game_state
            .expect("Invalid game state. Game is not initialized.")
            .players,
        vec![(0.into(), "A".to_owned()), (1.into(), "B".to_owned())]
    );

    program.restart_game(2, None, None);
    program.register(2, 2.into(), "C".to_owned(), None);
    program.register(2, 3.into(), "D".to_owned(), None);
    program.start_game(2, None);

    let state: GameLauncher = program
        .read_state(0)
        .expect("Unexpected invalid game state.");

    assert_eq!(
        state
            .game_state
            .expect("Invalid game state. Game is not initialized.")
            .players,
        vec![(2.into(), "C".to_owned()), (3.into(), "D".to_owned())]
    );
    program.place(2, 27, 0, false, None);
    let state: GameLauncher = program
        .read_state(0)
        .expect("Unexpected invalid game state.");
    assert!(!state.game_state.unwrap().tracks[0].tiles.is_empty());
    program.skip(2, None);
    program.skip(3, None);
}

#[test]
fn failures_test() {
    let system = System::new();

    system.init_logger();

    let program = Program::current_opt(&system);

    let result = program.send(2, Some(2_u64));
    assert!(!result.main_failed());

    program.start_game(
        2,
        Some(Error("The number of players is incorrect".to_owned())),
    );
    program.restart_game(
        2,
        None,
        Some(Error("The game hasn't started yet".to_owned())),
    );
    program.register(2, 0.into(), "A".to_owned(), None);
    program.register(
        2,
        1.into(),
        "A".to_owned(),
        Some(Error(
            "This name already exists, or you have already registered".to_owned(),
        )),
    );
    program.register(
        2,
        0.into(),
        "B".to_owned(),
        Some(Error(
            "This name already exists, or you have already registered".to_owned(),
        )),
    );
    program.register(2, 1.into(), "B".to_owned(), None);
    program.register(
        2,
        3.into(),
        "C".to_owned(),
        Some(Error("The player limit has been reached".to_owned())),
    );

    program.start_game(2, None);
    program.register(
        2,
        3.into(),
        "C".to_owned(),
        Some(Error("The game has already started".to_owned())),
    );
    program.start_game(2, Some(Error("The game has already started".to_owned())));

    let state: GameLauncher = program
        .read_state(0)
        .expect("Unexpected invalid game state.");
    assert_eq!(
        state
            .game_state
            .expect("Invalid game state. Game is not initialized.")
            .players,
        vec![(0.into(), "A".to_owned()), (1.into(), "B".to_owned())]
    );

    program.restart_game(
        2,
        Some(10),
        Some(Error("The limit should lie in the range [2,8]".to_owned())),
    );
    program.place(
        3,
        0,
        0,
        false,
        Some(Error("It is not your turn".to_owned())),
    );
    program.place(0, 3, 0, false, Some(Error("Invalid tile id".to_owned())));
    program.place(0, 1, 1, false, Some(Error("Invalid track".to_owned())));
    program.place(0, 1, 0, false, Some(Error("Invalid tile".to_owned())));
}
