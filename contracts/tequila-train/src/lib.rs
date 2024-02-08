#![no_std]

use gstd::{collections::HashMap, msg, prelude::*, ActorId};
use tequila_train_io::*;

#[derive(Debug, Default)]
pub struct GameLauncher {
    pub games: HashMap<ActorId, Game>,
    pub players_to_game_creator: HashMap<ActorId, ActorId>,
    pub config: Config,
}

/// All game initializing logic is inside `GameState` constructor.
static mut GAME_LAUNCHER: Option<GameLauncher> = None;

impl GameLauncher {
    pub fn create_game(
        &mut self,
        msg_source: ActorId,
        msg_value: u128,
        bid: u128,
    ) -> Result<Event, Error> {
        if bid < EXISTENTIAL_DEPOSIT && bid != 0 {
            return Err(Error::LessThanExistentialDeposit);
        }

        if bid != msg_value {
            return Err(Error::WrongBid);
        }

        if let Some(game) = self.games.get(&msg_source) {
            if matches!(game.state, State::Registration | State::Playing) {
                return Err(Error::AlreadyExists);
            }
        }

        let mut game = Game {
            bid,
            ..Default::default()
        };
        game.initial_players.insert(msg_source);

        self.games.insert(msg_source, game);

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
        game.state = State::Playing;
        Ok(Event::GameStarted)
    }

    pub fn register(
        &mut self,
        msg_source: ActorId,
        msg_value: u128,
        creator: ActorId,
    ) -> Result<Event, Error> {
        let game = self
            .games
            .get_mut(&creator)
            .ok_or(Error::GameDoesNotExist)?;

        if game.is_started {
            return Err(Error::GameHasAlreadyStarted);
        }

        if msg_value != game.bid {
            return Err(Error::WrongBid);
        }

        if game.initial_players.contains(&msg_source) {
            return Err(Error::YouAlreadyRegistered);
        }
        if self.players_to_game_creator.contains_key(&msg_source) {
            return Err(Error::RegisteredInAnotherGame);
        }
        if game.initial_players.len() >= 8 {
            return Err(Error::LimitHasBeenReached);
        }

        game.initial_players.insert(msg_source);
        self.players_to_game_creator.insert(msg_source, creator);
        Ok(Event::Registered { player: msg_source })
    }

    pub fn cancel_register(&mut self, creator: ActorId) -> Result<Event, Error> {
        let game = self
            .games
            .get_mut(&creator)
            .ok_or(Error::GameDoesNotExist)?;

        let msg_src = msg::source();

        if game.is_started {
            return Err(Error::GameHasAlreadyStarted);
        }
        if !game.initial_players.contains(&msg_src) {
            return Err(Error::NoSuchPlayer);
        }

        send_value(msg_src, game.bid);
        game.initial_players.remove(&msg_src);
        self.players_to_game_creator.remove(&msg_src);

        Ok(Event::RegistrationCanceled)
    }

    pub fn delete_player(&mut self, player_id: ActorId) -> Result<Event, Error> {
        let msg_src = msg::source();

        let game = self
            .games
            .get_mut(&msg_src)
            .ok_or(Error::GameDoesNotExist)?;

        if game.is_started {
            return Err(Error::GameHasAlreadyStarted);
        }

        if !game.initial_players.contains(&player_id) {
            return Err(Error::NoSuchPlayer);
        }

        send_value(player_id, game.bid);
        game.initial_players.remove(&player_id);
        self.players_to_game_creator.remove(&msg_src);

        Ok(Event::PlayerDeleted { player_id })
    }
    pub fn cancel_game(&mut self) -> Result<Event, Error> {
        let msg_src = msg::source();

        let game = self
            .games
            .get_mut(&msg_src)
            .ok_or(Error::GameDoesNotExist)?;

        if game.state != State::Playing && game.state != State::Registration {
            return Err(Error::GameFinished);
        }

        if game.bid != 0 {
            game.initial_players.iter().for_each(|id| {
                send_value(*id, game.bid);
                self.players_to_game_creator.remove(id);
            });
        }

        self.games.remove(&msg_src);

        Ok(Event::GameCanceled)
    }
}

fn send_value(destination: ActorId, value: u128) {
    if value != 0 {
        msg::send_with_gas(destination, "", 0, value).expect("Error in sending value");
    }
}

#[no_mangle]
extern fn init() {
    let config: Config = msg::load().expect("Unable to decode the initial msg");
    let game_launcher = GameLauncher {
        config,
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

    match command {
        Command::CreateGame { bid } => {
            let msg_source = msg::source();
            let msg_value = msg::value();
            let reply = game_launcher.create_game(msg_source, msg_value, bid);
            if reply.is_err() {
                send_value(msg_source, msg_value);
            }
            reply
        }
        Command::Skip { creator } => {
            let game = game_launcher
                .games
                .get_mut(&creator)
                .ok_or(Error::GameDoesNotExist)?;

            // a move can only be made with State::Playing
            if game.state != State::Playing {
                return Err(Error::GameHasNotStartedYet);
            }
            let player = msg::source();
            // a non-registered player cannot make a move
            if !game.initial_players.contains(&player) {
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

            // a move can only be made with State::Playing
            if game.state != State::Playing {
                return Err(Error::GameHasNotStartedYet);
            }

            let player = msg::source();
            // a non-registered player cannot make a move
            if !game.initial_players.contains(&player) {
                return Err(Error::NotRegistered);
            }
            if let Some(game_state) = &mut game.game_state {
                let result =
                    game_state.make_turn(player, tile_id, track_id, remove_train, game.bid);
                match result {
                    Ok(Event::GameFinished { winner }) => {
                        game.state = State::Winner(winner);
                        game.initial_players.iter().for_each(|id| {
                            game_launcher.players_to_game_creator.remove(id);
                        })
                    }
                    Ok(Event::GameStalled) => {
                        game.state = State::Stalled;
                        game.initial_players.iter().for_each(|id| {
                            game_launcher.players_to_game_creator.remove(id);
                        })
                    }
                    _ => (),
                }

                result
            } else {
                Err(Error::GameHasNotStartedYet)
            }
        }
        Command::Register { creator } => {
            let msg_source = msg::source();
            let msg_value = msg::value();
            let reply = game_launcher.register(msg_source, msg_value, creator);
            if reply.is_err() {
                send_value(msg_source, msg_value);
            }
            reply
        }
        Command::CancelRegistration { creator } => game_launcher.cancel_register(creator),
        Command::DeletePlayer { player_id } => game_launcher.delete_player(player_id),
        Command::StartGame => game_launcher.start(),
        Command::CancelGame => game_launcher.cancel_game(),
    }
}

#[no_mangle]
extern fn state() {
    let game_launcher = unsafe {
        GAME_LAUNCHER
            .take()
            .expect("Game launcher is not initialized")
    };
    let query: StateQuery = msg::load().expect("Unable to load the state query");
    let reply = match query {
        StateQuery::All => StateReply::All(game_launcher.into()),
        StateQuery::GetGame { creator } => {
            StateReply::Game(game_launcher.games.get(&creator).cloned())
        }
        StateQuery::GetGameId { player_id } => StateReply::GameId(
            game_launcher
                .players_to_game_creator
                .get(&player_id)
                .copied(),
        ),
    };
    msg::reply(reply, 0).expect("Failed to encode or reply with the game state");
}

impl From<GameLauncher> for GameLauncherState {
    fn from(
        GameLauncher {
            games,
            config,
            players_to_game_creator,
        }: GameLauncher,
    ) -> Self {
        let games = games.into_iter().collect();
        let players_to_game_creator = players_to_game_creator.into_iter().collect();
        Self {
            games,
            config,
            players_to_game_creator,
        }
    }
}
