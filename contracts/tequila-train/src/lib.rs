#![no_std]

use gstd::{exec, msg, prelude::*, ActorId};
use tequila_train_io::*;

#[derive(Debug, Default)]
pub struct GameLauncher {
    pub game_state: Option<GameState>,
    pub players: Vec<(ActorId, String)>,
    pub is_started: bool,
    pub maybe_limit: Option<u64>,
    pub admins: Vec<ActorId>,
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
            admins: vec![msg::source()],
            ..Default::default()
        }
    }

    pub fn start(&mut self) -> Result<Event, Error> {
        if !self.admins.contains(&msg::source()) {
            return Err(Error::NotAdmin);
        }
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

    pub fn restart(&mut self) -> Result<Event, Error> {
        if !self.admins.contains(&msg::source()) {
            return Err(Error::NotAdmin);
        }
        if !self.is_started {
            return Err(Error::GameHasNotStartedYet);
        }
        exec::block_height();

        self.is_started = false;
        self.game_state = None;
        self.players.clear();
        Ok(Event::GameRestarted)
    }
    pub fn add_admin(&mut self, admin: &ActorId) -> Result<Event, Error> {
        if !self.admins.contains(&msg::source()) {
            return Err(Error::NotAdmin);
        }
        if self.admins.contains(admin) {
            return Err(Error::AlreadyExists);
        }
        self.admins.push(*admin);
        Ok(Event::AdminAdded(*admin))
    }

    pub fn delete_admin(&mut self, admin: &ActorId) -> Result<Event, Error> {
        if !self.admins.contains(&msg::source()) {
            return Err(Error::NotAdmin);
        }
        let index = self.admins.iter().position(|value| value == admin);
        if let Some(id) = index {
            self.admins.remove(id);
        } else {
            return Err(Error::AdminDoesNotExist);
        }
        Ok(Event::AdminDeleted(*admin))
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
            GameLauncher {
                admins: vec![msg::source()],
                ..Default::default()
            }
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

    let command: Command = msg::load().expect("Unexpected invalid command payload.");
    if command != Command::RestartGame {
        if let Some(game_state) = &game_launcher.game_state {
            match &game_state.state {
                State::Stalled => {
                    return Err(Error::GameStalled);
                }
                State::Winner(_winner) => {
                    return Err(Error::GameFinished);
                }
                _ => (),
            };
        }
    }

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
        Command::RestartGame => game_launcher.restart(),
        Command::AddAdmin(admin) => game_launcher.add_admin(&admin),
        Command::DeleteAdmin(admin) => game_launcher.delete_admin(&admin),
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
            admins,
        }: GameLauncher,
    ) -> Self {
        Self {
            game_state,
            players,
            is_started,
            maybe_limit,
            admins,
        }
    }
}
