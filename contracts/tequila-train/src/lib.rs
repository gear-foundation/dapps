#![no_std]

use gstd::{collections::HashMap, msg, prelude::*, ActorId, exec, debug};
use tequila_train_io::*;

#[derive(Debug, Default)]
pub struct GameLauncher {
    pub games: HashMap<ActorId, Game>,
    pub players_to_game_id: HashMap<ActorId, ActorId>,
    pub config: Config,
}

/// All game initializing logic is inside `GameState` constructor.
static mut GAME_LAUNCHER: Option<GameLauncher> = None;

impl GameLauncher {
    pub fn create_game(&mut self, msg_source: ActorId, msg_value: u128) -> Result<Event, Error> {
        if self.players_to_game_id.contains_key(&msg_source) {
            return Err(Error::SeveralGames);
        }

        let mut game = Game {
            admin: msg_source,
            bid: msg_value,
            ..Default::default()
        };
        game.initial_players.push(msg_source);
        self.games.insert(msg_source, game);
        self.players_to_game_id
            .insert(msg_source, msg_source);
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
        if game.initial_players.len() < 2 {
            return Err(Error::NotEnoughPlayers);
        }

        game.is_started = true;
        game.game_state = GameState::new(game.initial_players.clone(), self.config.time_to_move);
        game.state = State::Playing;

        msg::send_with_gas_delayed(
            exec::program_id(),
            Command::CheckGame {
                game_id: msg_src,
                last_activity_time: game.game_state.clone().unwrap().last_activity_time,
            },
            self.config.gas_to_check_game,
            0,
            self.config.time_to_move/3000,
        )
        .expect("Error in sending delayed message");
        Ok(Event::GameStarted)
    }

    pub fn check_game(
        &mut self,
        game_id: ActorId,
        last_activity_time: u64,
    ) -> Result<Event, Error> {

        debug!("CHECK GAME");
        let program_id = exec::program_id();
        if msg::source() != program_id {
            return Err(Error::OnlyProgramCanSend)
        }
        let game = self
            .games
            .get_mut(&game_id)
            .ok_or(Error::GameDoesNotExist)?;

        let game_state = game.game_state.as_mut().ok_or(Error::GameHasNotStartedYet)?;

        if game_state.last_activity_time == last_activity_time {
            debug!("CHECK GAME 2 {:?}", last_activity_time);
            let current_player = game_state.current_player;
            game_state.players[current_player as usize].lose = true;
            if let Some(next_player) = game_state.next_player(current_player) {
                game_state.current_player = next_player;
                game_state.last_activity_time = exec::block_timestamp();
                msg::send_delayed(
                    program_id,
                    Command::CheckGame {
                        game_id,
                        last_activity_time: game_state.last_activity_time
                    },
                    0,
                    self.config.time_to_move/3000,
                )
                .expect("Error in sending delayed message");
            } else {

                let winner = game_state.players[current_player as usize].id;
                let prize = game.bid;
                if game.bid != 0 {
                    send_value(winner, prize * game.initial_players.len() as u128);
                }
                
                game.state = State::Winners(vec![winner]);
            }

        }
        Ok(Event::Checked)

    }

    pub fn register(
        &mut self,
        msg_source: ActorId,
        msg_value: u128,
        creator: ActorId,
    ) -> Result<Event, Error> {

        if self.players_to_game_id.contains_key(&msg_source) {
            return Err(Error::SeveralGames);  
        }
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

        if game.initial_players.len() >= 8 {
            return Err(Error::LimitHasBeenReached);
        }

        game.initial_players.push(msg_source);
        self.players_to_game_id
            .insert(msg_source, creator);
        Ok(Event::Registered { player: msg_source })
    }

    pub fn cancel_register(&mut self, creator: ActorId) -> Result<Event, Error> {
        let game = self
            .games
            .get_mut(&creator)
            .ok_or(Error::GameDoesNotExist)?;

        let msg_src = msg::source();

        if msg_src == game.admin {
            return Err(Error::YouAreAdmin);
        }

        if game.is_started {
            return Err(Error::GameHasAlreadyStarted);
        }
        if !game.initial_players.contains(&msg_src) {
            return Err(Error::NoSuchPlayer);
        }

        send_value(msg_src, game.bid);
        let index_to_remove = game.initial_players.iter().position(|x| x == &msg_src).expect("Critical Error");
        game.initial_players.remove(index_to_remove);
        self.players_to_game_id.remove(&msg_src);

        Ok(Event::RegistrationCanceled)
    }

    pub fn delete_player(&mut self, player_id: ActorId) -> Result<Event, Error> {
        let msg_src = msg::source();

        let game = self
            .games
            .get_mut(&msg_src)
            .ok_or(Error::GameDoesNotExist)?;

        if msg_src == player_id {
            return Err(Error::YouAreAdmin);
        }

        if game.is_started {
            return Err(Error::GameHasAlreadyStarted);
        }

        if !game.initial_players.contains(&player_id) {
            return Err(Error::NoSuchPlayer);
        }

        send_value(player_id, game.bid);
        let index_to_remove = game.initial_players.iter().position(|x| x == &player_id).expect("Critical Error");
        game.initial_players.remove(index_to_remove);
        self.players_to_game_id.remove(&player_id);

        Ok(Event::PlayerDeleted { player_id })
    }
    pub fn cancel_game(&mut self) -> Result<Event, Error> {
        let msg_src = msg::source();

        let game = self
            .games
            .get_mut(&msg_src)
            .ok_or(Error::GameDoesNotExist)?;

        if game.state == State::Playing || game.state == State::Registration {
            game.initial_players.iter().for_each(|id| {
                if game.bid != 0 {
                    send_value(*id, game.bid);
                }
                self.players_to_game_id.remove(id);
            });
        } else {
            game.initial_players.iter().for_each(|id| {
                self.players_to_game_id.remove(id);
            });
        }

        self.games.remove(&msg_src);

        Ok(Event::GameCanceled)
    }
    pub fn leave_game(&mut self) -> Result<Event, Error> {
        self.players_to_game_id.remove(&msg::source());
        Ok(Event::GameLeft)
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
        Command::CreateGame => {
            let msg_source = msg::source();
            let msg_value = msg::value();
            let reply = game_launcher.create_game(msg_source, msg_value);
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
                return Err(Error::StateIsNotPlaying);
            }
            let player = msg::source();
            // a non-registered player cannot make a move
            if !game.initial_players.contains(&player) {
                return Err(Error::NotRegistered);
            }
            if let Some(game_state) = &mut game.game_state {
                let result = game_state.skip_turn(player, game.bid);
                match result {
                    Ok(Event::GameFinished { ref winners }) => {
                        game.state = State::Winners(winners.clone());
                    }
                    Ok(Event::Skipped) => {
                        debug!("SEND CHECK GAME");
                        msg::send_with_gas_delayed(
                            exec::program_id(),
                            Command::CheckGame {
                                game_id: creator,
                                last_activity_time: game.game_state.clone().unwrap().last_activity_time,
                            },
                            game_launcher.config.gas_to_check_game,
                            0,
                            game_launcher.config.time_to_move/3000,
                        )
                        .expect("Error in sending delayed message");
                    }
                    _ => (),
                }

                result
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
                    Ok(Event::GameFinished { ref winners }) => {
                        game.state = State::Winners(winners.clone());
                    }
                    Ok(Event::Placed { .. }) => {
                        msg::send_with_gas_delayed(
                            exec::program_id(),
                            Command::CheckGame {
                                game_id: creator,
                                last_activity_time: game.game_state.clone().unwrap().last_activity_time,
                            },
                            game_launcher.config.gas_to_check_game,
                            0,
                            game_launcher.config.time_to_move/3000,
                        )
                        .expect("Error in sending delayed message");
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
        Command::CheckGame { game_id, last_activity_time } => game_launcher.check_game(game_id, last_activity_time),
        Command::StartGame => game_launcher.start(),
        Command::CancelGame => game_launcher.cancel_game(),
        Command::LeaveGame => game_launcher.leave_game(),
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
        StateQuery::GetGame { player_id } => {
            if let Some(creator_id) = game_launcher.players_to_game_id.get(&player_id){
                let game_reply = game_launcher.games.get(creator_id)
                    .map(|game| {
                        let last_activity_time_diff = game.game_state.as_ref().map(|state| game_launcher.config.time_to_move as u64 - (exec::block_timestamp() - state.last_activity_time));
                        (game.clone(), last_activity_time_diff)
                    })
                    .map(Some)
                    .unwrap_or(None);


                StateReply::Game(game_reply)
            } else {
                StateReply::Game(None)
            }
            

        }
    };
    msg::reply(reply, 0).expect("Failed to encode or reply with the game state");
}

impl From<GameLauncher> for GameLauncherState {
    fn from(
        GameLauncher {
            games,
            config,
            players_to_game_id,
        }: GameLauncher,
    ) -> Self {
        let games = games.into_iter().collect();
        let players_to_game_id = players_to_game_id.into_iter().collect();
        Self {
            games,
            config,
            players_to_game_id,
        }
    }
}
