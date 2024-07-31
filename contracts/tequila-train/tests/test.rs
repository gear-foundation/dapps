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
    fn get_all_state(&self) -> Option<GameLauncherState>;
    fn get_game_state(&self, creator_id: ActorId) -> Option<(Game, Option<u64>)>;
}

impl TestFunc for Program<'_> {
    fn create_game(&self, from: u64, bid: u128, error: Option<Error>) {
        let result = self.send_with_value(from, Command::CreateGame, bid);
        assert!(!result.main_failed());
        let reply = if let Some(error) = error {
            Err(error)
        } else {
            Ok(Event::GameCreated)
        };
        assert!(result.contains(&(from, reply.encode())));
    }
    fn register(&self, from: u64, bid: u128, creator: ActorId, error: Option<Error>) {
        let result = self.send_with_value(from, Command::Register { creator }, bid);
        assert!(!result.main_failed());
        let reply = if let Some(error) = error {
            Err(error)
        } else {
            Ok(Event::Registered {
                player: from.into(),
            })
        };
        assert!(result.contains(&(from, reply.encode())));
    }
    fn cancel_register(&self, from: u64, creator: ActorId, error: Option<Error>) {
        let result = self.send(from, Command::CancelRegistration { creator });
        assert!(!result.main_failed());
        let reply = if let Some(error) = error {
            Err(error)
        } else {
            Ok(Event::RegistrationCanceled)
        };
        assert!(result.contains(&(from, reply.encode())));
    }
    fn delete_player(&self, from: u64, player_id: ActorId, error: Option<Error>) {
        let result = self.send(from, Command::DeletePlayer { player_id });
        let res = &result.decoded_log::<Result<Event, Error>>();
        println!("RES: {:?}", res);
        assert!(!result.main_failed());
        let reply = if let Some(error) = error {
            Err(error)
        } else {
            Ok(Event::PlayerDeleted { player_id })
        };
        assert!(result.contains(&(from, reply.encode())));
    }
    fn cancel_game(&self, from: u64, error: Option<Error>) {
        let result = self.send(from, Command::CancelGame);
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
        let result = self.send(from, Command::Skip { creator });
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
    fn get_all_state(&self) -> Option<GameLauncherState> {
        let reply = self
            .read_state(StateQuery::All)
            .expect("Unexpected invalid state.");
        if let StateReply::All(state) = reply {
            Some(state)
        } else {
            None
        }
    }
    fn get_game_state(&self, player_id: ActorId) -> Option<(Game, Option<u64>)> {
        let reply = self
            .read_state(StateQuery::GetGame { player_id })
            .expect("Unexpected invalid state.");
        if let StateReply::Game(state) = reply {
            state
        } else {
            None
        }
    }
}
// TODO: fix test
#[test]
#[ignore]
fn success_test() {
    let system = System::new();

    system.init_logger();

    let program = Program::current_opt(&system);

    let config = Config {
        time_to_move: 30_000,
        gas_to_check_game: 200_000_000_000,
    };

    let result = program.send(2, config);
    assert!(!result.main_failed());

    program.create_game(PLAYERS[0], 0, None);

    program.register(PLAYERS[1], 0, PLAYERS[0].into(), None);
    program.register(PLAYERS[2], 0, PLAYERS[0].into(), None);
    program.start_game(PLAYERS[0], None);

    let state: GameLauncherState = program
        .get_all_state()
        .expect("Unexpected invalid game state.");
    println!("STATE: {:?}", state);

    let game = state.games[0].1.game_state.clone().unwrap();
    let current_player = game.current_player;

    program.skip(PLAYERS[current_player as usize], PLAYERS[0].into(), None);

    system.spend_blocks(3);
    let current_player = (current_player + 1) as usize % PLAYERS.len();
    program.skip(PLAYERS[current_player], PLAYERS[0].into(), None);

    system.spend_blocks(8);
    let state = program
        .get_game_state(PLAYERS[0].into())
        .expect("Unexpected invalid game state.");
    println!("STATE: {:?}", state);
    system.spend_blocks(2);
    let current_player = (current_player + 1) % PLAYERS.len();
    program.skip(
        PLAYERS[current_player],
        PLAYERS[0].into(),
        Some(Error::NotYourTurnOrYouLose),
    );
}
#[test]
fn cancel_register() {
    let system = System::new();

    system.init_logger();

    let program = Program::current_opt(&system);

    let config = Config {
        time_to_move: 30_000,
        gas_to_check_game: 200_000_000_000,
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

    let state: GameLauncherState = program
        .get_all_state()
        .expect("Unexpected invalid game state.");
    assert_eq!(state.games[0].1.initial_players.len(), 2);

    program.cancel_register(PLAYERS[1], PLAYERS[0].into(), None);
    system.claim_value_from_mailbox(PLAYERS[1]);
    let balance = system.balance_of(PLAYERS[1]);
    assert_eq!(balance, bid);

    let state: GameLauncherState = program
        .get_all_state()
        .expect("Unexpected invalid game state.");
    assert_eq!(state.games[0].1.initial_players.len(), 1);
    assert_eq!(state.players_to_game_id.len(), 1);
}

#[test]
fn delete_player() {
    let system = System::new();

    system.init_logger();

    let program = Program::current_opt(&system);

    let config = Config {
        time_to_move: 30_000,
        gas_to_check_game: 200_000_000_000,
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

    let state: GameLauncherState = program
        .get_all_state()
        .expect("Unexpected invalid game state.");
    assert_eq!(state.games[0].1.initial_players.len(), 2);

    program.delete_player(PLAYERS[0], PLAYERS[1].into(), None);
    system.claim_value_from_mailbox(PLAYERS[1]);
    let balance = system.balance_of(PLAYERS[1]);
    assert_eq!(balance, bid);

    let state: GameLauncherState = program
        .get_all_state()
        .expect("Unexpected invalid game state.");
    assert_eq!(state.games[0].1.initial_players.len(), 1);
    assert_eq!(state.players_to_game_id.len(), 1);
}

#[test]
fn cancel_game() {
    let system = System::new();

    system.init_logger();

    let program = Program::current_opt(&system);

    let config = Config {
        time_to_move: 30_000,
        gas_to_check_game: 200_000_000_000,
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

    let state: GameLauncherState = program
        .get_all_state()
        .expect("Unexpected invalid game state.");
    assert!(!state.games.is_empty());

    program.cancel_game(PLAYERS[0], None);
    system.claim_value_from_mailbox(PLAYERS[1]);
    let balance = system.balance_of(PLAYERS[1]);
    assert_eq!(balance, bid);
    system.claim_value_from_mailbox(PLAYERS[0]);
    let balance = system.balance_of(PLAYERS[0]);
    assert_eq!(balance, bid);

    let state: GameLauncherState = program
        .get_all_state()
        .expect("Unexpected invalid game state.");
    assert!(state.games.is_empty());
    assert_eq!(state.players_to_game_id.len(), 0);
}

#[test]
fn failures_test() {
    let system = System::new();

    system.init_logger();

    let program = Program::current_opt(&system);

    let config = Config {
        time_to_move: 30_000,
        gas_to_check_game: 200_000_000_000,
    };

    let result = program.send(2, config);
    assert!(!result.main_failed());

    // After each error, a balance check will be made to verify the balance return

    // Сan't create multiple games
    let bid = 11_000_000_000_000;
    system.mint_to(PLAYERS[0], 2 * bid);
    program.create_game(PLAYERS[0], bid, None);
    program.create_game(PLAYERS[0], bid, Some(Error::SeveralGames));
    system.claim_value_from_mailbox(PLAYERS[0]);
    assert_eq!(system.balance_of(PLAYERS[0]), bid);

    // You can't play one game and be an admin in another game
    system.mint_to(PLAYERS[1], 2 * bid);
    program.register(PLAYERS[1], bid, PLAYERS[0].into(), None);
    program.create_game(PLAYERS[1], bid, Some(Error::SeveralGames));
    system.claim_value_from_mailbox(PLAYERS[1]);
    assert_eq!(system.balance_of(PLAYERS[1]), bid);

    // A non-existent game id has been entered
    system.mint_to(PLAYERS[2], 2 * bid);
    program.register(
        PLAYERS[2],
        bid,
        PLAYERS[1].into(),
        Some(Error::GameDoesNotExist),
    );
    system.claim_value_from_mailbox(PLAYERS[2]);
    assert_eq!(system.balance_of(PLAYERS[2]), 2 * bid);
    // Wrong bid
    program.register(
        PLAYERS[2],
        bid - 1,
        PLAYERS[0].into(),
        Some(Error::WrongBid),
    );
    system.claim_value_from_mailbox(PLAYERS[2]);
    assert_eq!(system.balance_of(PLAYERS[2]), 2 * bid);
    // Already registered
    program.register(
        PLAYERS[1],
        bid,
        PLAYERS[0].into(),
        Some(Error::SeveralGames),
    );
    system.claim_value_from_mailbox(PLAYERS[1]);
    assert_eq!(system.balance_of(PLAYERS[1]), bid);
    // Registered In Another Game
    program.create_game(PLAYERS[2], bid, None);
    program.register(
        PLAYERS[1],
        bid,
        PLAYERS[2].into(),
        Some(Error::SeveralGames),
    );
    system.claim_value_from_mailbox(PLAYERS[1]);
    assert_eq!(system.balance_of(PLAYERS[1]), bid);

    // Admin try cancel register
    program.cancel_register(PLAYERS[0], PLAYERS[0].into(), Some(Error::YouAreAdmin));

    // No Such Player in registration list
    program.cancel_register(PLAYERS[2], PLAYERS[0].into(), Some(Error::NoSuchPlayer));

    // players less than 2
    program.start_game(PLAYERS[2], Some(Error::NotEnoughPlayers));

    // the game has already started
    program.start_game(PLAYERS[0], None);
    program.start_game(PLAYERS[0], Some(Error::GameHasAlreadyStarted));
}
