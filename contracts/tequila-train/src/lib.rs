#![no_std]

use gstd::{msg, prelude::*, ActorId};
use tequila_train_io::*;

#[derive(Debug, Default)]
pub struct GameLauncher {
    pub game_state: Option<GameState>,
    pub players: Vec<(ActorId, String)>,
    pub is_started: bool,
    pub maybe_limit: Option<u64>,
}

/// All game initializing logic is inside `GameState` constructor.
static mut GAME_LAUNCHER: Option<GameLauncher> = None;

impl GameLauncher {
    fn check_limit_range(maybe_limit: Option<u64>) -> Result<(), Error> {
        if let Some(limit) = maybe_limit {
            if !(2..=8).contains(&limit) {
                return Err(Error::WrongPlayersCount);
            }
        }
        Ok(())
    }

    fn check_players_count(&self) -> Result<(), Error> {
        if !(2..=8).contains(&(self.players.len() as u32)) {
            return Err(Error::WrongPlayersCount);
        }
        Ok(())
    }

    pub fn new_with_limit(limit: u64) -> Self {
        Self::check_limit_range(Some(limit)).expect("The limit should lie in the range [2,8]");

        GameLauncher {
            maybe_limit: Some(limit),
            ..Default::default()
        }
    }

    pub fn start(&mut self) -> Result<Event, Error> {
        if self.is_started {
            return Err(Error::GameHasAlreadyStarted);
        }
        self.check_players_count()?;

        self.is_started = true;
        self.game_state = GameState::new(&Players {
            players: self.players.clone(),
        });

        assert!(self.game_state.is_some());
        Ok(Event::GameStarted)
    }

    pub fn restart(&mut self, maybe_limit: Option<u64>) -> Result<Event, Error> {
        if !self.is_started {
            return Err(Error::GameHasNotStartedYet);
        }
        Self::check_limit_range(maybe_limit)?;

        self.is_started = false;
        self.game_state = None;
        self.maybe_limit = maybe_limit;
        self.players.clear();
        Ok(Event::GameRestarted {
            players_limit: maybe_limit,
        })
    }

    pub fn register(&mut self, player: ActorId, name: String) -> Result<Event, Error> {
        if self.is_started {
            return Err(Error::GameHasAlreadyStarted);
        }

        if self.players.iter().any(|(p, n)| p == &player || n == &name) {
            return Err(Error::NameAlreadyExistsOrYouRegistered);
        }

        if let Some(limit) = self.maybe_limit {
            if (self.players.len() as u64) >= limit {
                return Err(Error::LimitHasBeenReached);
            }
        } else if self.players.len() >= 8 {
            return Err(Error::LimitHasBeenReached);
        }

        self.players.push((player, name.clone()));
        Ok(Event::Registered { player, name })
    }
}

#[no_mangle]
extern fn init() {
    let Init { players_limit } = msg::load().expect("Unexpected invalid payload.");

    unsafe {
        GAME_LAUNCHER = Some(if let Some(limit) = players_limit {
            GameLauncher::new_with_limit(limit)
        } else {
            GameLauncher::default()
        })
    }
}

#[no_mangle]
extern fn handle() {
    let reply = process_handle();
    msg::reply(reply, 0).expect("Failed to encode or reply with `Result<Event, Error>`.");
}

fn process_handle() -> Result<Event, Error> {
    let game_launcher = unsafe {
        GAME_LAUNCHER
            .as_mut()
            .expect("The contract is not initialized")
    };

    if let Some(game_state) = &game_launcher.game_state {
        match game_state.state() {
            State::Stalled => {
                return Err(Error::GameStalled);
            }
            State::Winner(_winner) => {
                return Err(Error::GameFinished);
            }
            _ => (),
        };
    }

    let command: Command = msg::load().expect("Unexpected invalid command payload.");
    let player = msg::source();

    match command {
        Command::Skip => {
            if let Some(game_state) = &mut game_launcher.game_state {
                game_state.skip_turn(player)
            } else {
                Err(Error::GameHasNotStartedYet)
            }
        }
        Command::Place {
            tile_id,
            track_id,
            remove_train,
        } => {
            if let Some(game_state) = &mut game_launcher.game_state {
                game_state.make_turn(player, tile_id, track_id, remove_train)
            } else {
                Err(Error::GameHasNotStartedYet)
            }
        }
        Command::Register { player, name } => game_launcher.register(player, name),
        Command::StartGame => game_launcher.start(),
        Command::RestartGame(maybe_limit) => game_launcher.restart(maybe_limit),
    }
}

#[no_mangle]
extern fn state() {
    let game_launcher = unsafe {
        GAME_LAUNCHER
            .take()
            .expect("Game launcher is not initialized")
    };

    msg::reply::<GameLauncherState>(game_launcher.into(), 0)
        .expect("Failed to encode or reply with the game state");
}

impl From<GameLauncher> for GameLauncherState {
    fn from(
        GameLauncher {
            game_state,
            players,
            is_started,
            maybe_limit,
        }: GameLauncher,
    ) -> Self {
        Self {
            game_state,
            players,
            is_started,
            maybe_limit,
        }
    }
}
