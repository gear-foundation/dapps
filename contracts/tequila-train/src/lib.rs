#![no_std]

use gstd::{collections::HashMap, msg, prelude::*, ActorId, debug};
use tequila_train_io::*;

#[derive(Debug, Default)]
pub struct GameLauncher {
    pub games: HashMap<ActorId, Game>,
    pub config: Config,
    pub admins: Vec<ActorId>,
}

/// All game initializing logic is inside `GameState` constructor.
static mut GAME_LAUNCHER: Option<GameLauncher> = None;

impl GameLauncher {
    // fn check_limit_range(maybe_limit: Option<u64>) -> Result<(), Error> {
    //     if let Some(limit) = maybe_limit {
    //         if !(2..=8).contains(&limit) {
    //             return Err(Error::WrongPlayersCount);
    //         }
    //     }
    //     Ok(())
    // }

    // fn check_players_count(&self) -> Result<(), Error> {
    //     if !(2..=8).contains(&(self.players.len() as u32)) {
    //         return Err(Error::WrongPlayersCount);
    //     }
    //     Ok(())
    // }

    pub fn create_game(&mut self, players_limit: u64, bid: u128) -> Result<Event, Error> {
        let msg_src = msg::source();
        if players_limit > 8 {
            return Err(Error::WrongPlayersCount);
        }
        if self.games.contains_key(&msg_src) {
            return Err(Error::AlreadyExists);
        }
        let game = Game {
            players_limit,
            bid,
            ..Default::default()
        };
        self.games.insert(msg_src, game);

        Ok(Event::GameCreated)
    }

    pub fn start(&mut self) -> Result<Event, Error> {
        let msg_src = msg::source();
        let game = self
            .games
            .get_mut(&msg_src)
            .ok_or(Error::GameDoesNotExist)?;

        if game.is_started {
            return Err(Error::GameHasAlreadyStarted);
        }

        game.is_started = true;
        game.game_state = GameState::new(
            &Players {
                players: game.initial_players.clone(),
            },
            self.config.time_to_move,
        );

        assert!(game.game_state.is_some());
        Ok(Event::GameStarted)
    }

    pub fn restart(&mut self) -> Result<Event, Error> {
        let msg_src = msg::source();
        let game = self
            .games
            .get_mut(&msg_src)
            .ok_or(Error::GameDoesNotExist)?;
        if !game.is_started {
            return Err(Error::GameHasNotStartedYet);
        }

        game.is_started = false;
        game.game_state = None;
        game.initial_players.clear();
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
        let index = self
            .admins
            .iter()
            .position(|value| value == admin)
            .ok_or(Error::AdminDoesNotExist)?;
        self.admins.remove(index);
        Ok(Event::AdminDeleted(*admin))
    }

    pub fn register(&mut self, creator: ActorId, name: String) -> Result<Event, Error> {
        let game = self
            .games
            .get_mut(&creator)
            .ok_or(Error::GameDoesNotExist)?;
        let msg_src = msg::source();
        let bid_value = msg::value();

        if game.is_started {
            return Err(Error::GameHasAlreadyStarted);
        }
        if bid_value != game.bid {
            msg::send_with_gas(msg_src, "", 0, bid_value)
                .expect("Error in sending value");
            return Err(Error::WrongBid);
        }

        if game
            .initial_players
            .iter()
            .any(|(p, n)| p == &msg_src || n == &name)
        {
            return Err(Error::NameAlreadyExistsOrYouRegistered);
        }

        if game.initial_players.len() >= 8 {
            return Err(Error::LimitHasBeenReached);
        }

        game.initial_players.push((msg_src, name.clone()));
        Ok(Event::Registered {
            player: msg_src,
            name,
        })
    }
}

#[no_mangle]
extern fn init() {
    let config: Config = msg::load().expect("Unable to decode the initial msg");
    let game_launcher = GameLauncher {
        config,
        admins: vec![msg::source()],
        ..Default::default()
    };

    unsafe { GAME_LAUNCHER = Some(game_launcher) };
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
    // if let Command::RestartGame{creator} !=  command {
    //     let game = game_launcher.games.get_mut(&creator).ok_or(Error::GameDoesNotExist)?;

    //     if let Some(game_state) = &game_launcher.game_state {
    //         match &game_state.state {
    //             State::Stalled => {
    //                 return Err(Error::GameStalled);
    //             }
    //             State::Winner(_) => {
    //                 return Err(Error::GameFinished);
    //             }
    //             _ => (),
    //         };
    //     }
    // }

    match command {
        Command::CtreateGame { players_limit, bid} => game_launcher.create_game(players_limit, bid),
        Command::Skip { creator } => {
            let game = game_launcher
                .games
                .get_mut(&creator)
                .ok_or(Error::GameDoesNotExist)?;
            let player = msg::source();
            if game.initial_players.iter().find(|(id, _)| *id == player).is_none(){
                return Err(Error::NotRegistered);
            }
            if let Some(game_state) = &mut game.game_state {
                game_state.skip_turn(player, game.bid)
            } else {
                Err(Error::GameHasNotStartedYet)
            }
        }
        Command::Place {
            creator,
            tile_id,
            track_id,
            remove_train,
        } => {
            let game = game_launcher
                .games
                .get_mut(&creator)
                .ok_or(Error::GameDoesNotExist)?;
            let player = msg::source();
            debug!("HERE");
            if game.initial_players.iter().find(|(id, _)| *id == player).is_none(){
                return Err(Error::NotRegistered);
            }
            debug!("HERE 2");
            if let Some(game_state) = &mut game.game_state {
                debug!("HERE 3");
                game_state.make_turn(player, tile_id, track_id, remove_train, game.bid)
            } else {
                Err(Error::GameHasNotStartedYet)
            }
        }
        Command::Register { creator, name } => game_launcher.register(creator, name),
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
            games,
            config,
            admins,
        }: GameLauncher,
    ) -> Self {
        let games = games.into_iter().map(|(id, game)| (id, game)).collect();
        Self {
            games,
            config,
            admins,
        }
    }
}
