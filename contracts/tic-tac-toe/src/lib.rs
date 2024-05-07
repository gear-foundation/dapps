#![no_std]

use gstd::{collections::HashMap, exec, msg, prelude::*, ActorId};
use tic_tac_toe_io::{
    ActionsForSession, Config, GameAction, GameError, GameInit, GameInstance, GameReply,
    GameResult, Mark, Session, SignatureData, StateQuery, StateReply,
};

mod sr25519;
static mut GAME: Option<Game> = None;

// Minimum duration of session: 3 mins = 180_000 ms = 60 blocks
pub const MINIMUM_SESSION_SURATION_MS: u64 = 180_000;

#[derive(Debug, Default)]
struct Game {
    pub admins: Vec<ActorId>,
    pub current_games: HashMap<ActorId, GameInstance>,
    pub config: Config,
    pub messages_allowed: bool,
    pub sessions: HashMap<ActorId, Session>,
}

pub const VICTORIES: [[usize; 3]; 8] = [
    [0, 1, 2],
    [3, 4, 5],
    [6, 7, 8],
    [0, 3, 6],
    [1, 4, 7],
    [2, 5, 8],
    [0, 4, 8],
    [2, 4, 6],
];

#[no_mangle]
extern fn init() {
    let init_msg: GameInit = msg::load().expect("Unable to load the message");

    unsafe {
        GAME = Some(Game {
            admins: vec![msg::source()],
            current_games: HashMap::with_capacity(10_000),
            config: init_msg.config,
            messages_allowed: true,
            sessions: HashMap::new(),
        });
    }
}

impl Game {
    fn create_session(
        &mut self,
        key: &ActorId,
        duration: u64,
        allowed_actions: Vec<ActionsForSession>,
        signature: Option<Vec<u8>>,
    ) -> Result<GameReply, GameError> {
        assert!(
            duration >= MINIMUM_SESSION_SURATION_MS,
            "Duration is too small"
        );

        let msg_source = msg::source();
        let block_timestamp = exec::block_timestamp();
        let block_height = exec::block_height();

        let expires = block_timestamp + duration;

        let number_of_blocks = u32::try_from(duration.div_ceil(self.config.block_duration_ms))
            .expect("Duration is too large");

        assert!(
            !allowed_actions.is_empty(),
            "No messages for approval were passed."
        );

        let account = match signature {
            Some(sig_bytes) => {
                self.check_if_session_exists(key);
                let pub_key: [u8; 32] = (*key).into();
                let mut prefix = b"<Bytes>".to_vec();
                let mut message = SignatureData {
                    key: msg_source,
                    duration,
                    allowed_actions: allowed_actions.clone(),
                }
                .encode();
                let mut postfix = b"</Bytes>".to_vec();
                prefix.append(&mut message);
                prefix.append(&mut postfix);

                if crate::sr25519::verify(&sig_bytes, prefix, pub_key).is_err() {
                    panic!("Failed sign verification");
                }
                self.sessions.entry(*key).insert(Session {
                    key: msg_source,
                    expires,
                    allowed_actions,
                    expires_at_block: block_height + number_of_blocks,
                });
                *key
            }
            None => {
                self.check_if_session_exists(&msg_source);

                self.sessions.entry(msg_source).insert(Session {
                    key: *key,
                    expires,
                    allowed_actions,
                    expires_at_block: block_height + number_of_blocks,
                });
                msg_source
            }
        };

        msg::send_with_gas_delayed(
            exec::program_id(),
            GameAction::DeleteSessionFromProgram {
                account,
            },
            self.config.gas_to_delete_session,
            0,
            number_of_blocks,
        )
        .expect("Error in sending a delayed msg");

        Ok(GameReply::SessionCreated)
    }

    fn check_if_session_exists(&self, account: &ActorId) {
        if let Some(Session {
            key: _,
            expires: _,
            allowed_actions: _,
            expires_at_block,
        }) = self.sessions.get(account)
        {
            if *expires_at_block > exec::block_height() {
                panic!("You already have an active session. If you want to create a new one, please delete this one.")
            }
        }
    }
    
    fn delete_session_from_program(&mut self, session_for_account: &ActorId) -> Result<GameReply, GameError> {
        assert_eq!(
            exec::program_id(),
            msg::source(),
            "The msg source must be the program"
        );

        if let Some(session) = self.sessions.remove(session_for_account) {
            assert!(
                session.expires_at_block <= exec::block_height(),
                "Too early to delete session"
            );
        }
        Ok(GameReply::SessionDeleted)
    }

    fn delete_session_from_account(&mut self) -> Result<GameReply, GameError> {
        assert!(self.sessions.remove(&msg::source()).is_some(), "No session");
        Ok(GameReply::SessionDeleted)
    }

    fn start_game(
        &mut self,
        msg_source: &ActorId,
        session_for_account: Option<ActorId>,
    ) -> Result<GameReply, GameError> {
        let player = self.get_player(msg_source, &session_for_account, ActionsForSession::StartGame);

        if let Some(current_game) = self.current_games.get(&player) {
            if !current_game.game_over {
                return Err(GameError::GameIsAlreadyStarted);
            }
        }

        let turn = turn();

        let (player_mark, bot_mark) = if turn == 0 {
            (Mark::O, Mark::X)
        } else {
            (Mark::X, Mark::O)
        };
        let mut game_instance = GameInstance {
            board: vec![None; 9],
            player_mark,
            bot_mark,
            last_time: exec::block_timestamp(),
            game_result: None,
            game_over: false,
        };

        if bot_mark == Mark::X {
            game_instance.board[4] = Some(Mark::X);
        }

        self.current_games
            .insert(player, game_instance.clone());

        Ok(GameReply::GameStarted {
            game: game_instance,
        })
    }

    fn player_move(
        &mut self,
        msg_source: &ActorId,
        step: u8,
        session_for_account: Option<ActorId>,
    ) -> Result<GameReply, GameError> {
        let player = self.get_player(msg_source, &session_for_account, ActionsForSession::Move);

        let game_instance = self
            .current_games
            .get_mut(&player)
            .expect("The player has no game, please start the game");
        if game_instance.board[step as usize].is_some() {
            return Err(GameError::CellIsAlreadyOccupied);
        }
        if game_instance.game_over {
            return Err(GameError::GameIsAlreadyOver);
        }

        if game_instance.last_time + self.config.turn_deadline_ms < exec::block_timestamp() {
            return Err(GameError::MissedYourTurn);
        }
        game_instance.board[step as usize] = Some(game_instance.player_mark);

        game_instance.last_time = exec::block_timestamp();

        if let Some(mark) = get_result(&game_instance.board.clone()) {
            game_instance.game_over = true;
            if mark == game_instance.player_mark {
                game_instance.game_result = Some(GameResult::Player);
            } else {
                game_instance.game_result = Some(GameResult::Bot);
            }
            send_messages(&player, &self.config);
            return Ok(GameReply::GameFinished {
                game: game_instance.clone(),
                player_address: player,
            });
        }

        let bot_step = make_move(game_instance);

        if let Some(step_num) = bot_step {
            game_instance.board[step_num] = Some(game_instance.bot_mark);
        }

        let win = get_result(&game_instance.board.clone());

        if let Some(mark) = win {
            game_instance.game_over = true;
            if mark == game_instance.player_mark {
                game_instance.game_result = Some(GameResult::Player);
                send_messages(msg_source, &self.config);
            } else {
                game_instance.game_result = Some(GameResult::Bot);
                send_messages(msg_source, &self.config);
            }
            return Ok(GameReply::GameFinished {
                game: game_instance.clone(),
                player_address: player,
            });
        } else if !game_instance.board.contains(&None) || bot_step.is_none() {
            game_instance.game_over = true;
            game_instance.game_result = Some(GameResult::Draw);
            send_messages(msg_source, &self.config);
            return Ok(GameReply::GameFinished {
                game: game_instance.clone(),
                player_address: player,
            });
        }

        Ok(GameReply::MoveMade {
            game: game_instance.clone(),
        })
    }

    fn skip(
        &mut self,
        msg_source: &ActorId,
        session_for_account: Option<ActorId>,
    ) -> Result<GameReply, GameError> {
        let player = self.get_player(msg_source, &session_for_account, ActionsForSession::Skip);

        let Some(game_instance) = self.current_games.get_mut(msg_source) else {
            return Err(GameError::GameIsNotStarted);
        };

        if game_instance.game_over {
            return Err(GameError::GameIsAlreadyOver);
        }

        if game_instance.last_time + self.config.turn_deadline_ms >= exec::block_timestamp() {
            return Err(GameError::NotMissedTurnMakeMove);
        }

        let bot_step = make_move(game_instance);
        game_instance.last_time = exec::block_timestamp();

        match bot_step {
            Some(step_num) => {
                game_instance.board[step_num] = Some(game_instance.bot_mark);
                let win = get_result(&game_instance.board.clone());
                if let Some(mark) = win {
                    game_instance.game_over = true;
                    if mark == game_instance.player_mark {
                        game_instance.game_result = Some(GameResult::Player);
                    } else {
                        game_instance.game_result = Some(GameResult::Bot);
                    }
                    send_messages(&player, &self.config);
                    return Ok(GameReply::GameFinished {
                        game: game_instance.clone(),
                        player_address: player,
                    });
                } else if !game_instance.board.contains(&None) {
                    game_instance.game_over = true;
                    game_instance.game_result = Some(GameResult::Draw);
                    send_messages(&player, &self.config);
                    return Ok(GameReply::GameFinished {
                        game: game_instance.clone(),
                        player_address: player,
                    });
                }
            }
            None => {
                game_instance.game_over = true;
                game_instance.game_result = Some(GameResult::Draw);
                send_messages(&player, &self.config);
                return Ok(GameReply::GameFinished {
                    game: game_instance.clone(),
                    player_address: player,
                });
            }
        }

        Ok(GameReply::MoveMade {
            game: game_instance.clone(),
        })
    }

    fn remove_game_instance(
        &mut self,
        msg_source: &ActorId,
        account: &ActorId,
    ) -> Result<GameReply, GameError> {
        if *msg_source != exec::program_id() {
            return Err(GameError::MessageOnlyForProgram);
        }

        let game_instance = self
            .current_games
            .get(account)
            .expect("Unexpected: the game does not exist");

        if game_instance.game_over {
            self.current_games.remove(account);
        }
        Ok(GameReply::GameInstanceRemoved)
    }

    fn remove_instances(
        &mut self,
        msg_source: &ActorId,
        accounts: Option<Vec<ActorId>>,
    ) -> Result<GameReply, GameError> {
        if !self.admins.contains(msg_source) {
            return Err(GameError::NotAdmin);
        }
        match accounts {
            Some(accounts) => {
                for account in accounts {
                    self.current_games.remove(&account);
                }
            }
            None => {
                self.current_games.retain(|_, game_instance| {
                    exec::block_timestamp() - game_instance.last_time
                        < self.config.time_interval as u64 * self.config.s_per_block
                });
            }
        }
        Ok(GameReply::GameInstanceRemoved)
    }
    fn add_admin(&mut self, msg_source: &ActorId, admin: ActorId) -> Result<GameReply, GameError> {
        if !self.admins.contains(msg_source) {
            return Err(GameError::NotAdmin);
        }
        self.admins.push(admin);
        Ok(GameReply::AdminAdded)
    }
    fn remove_admin(
        &mut self,
        msg_source: &ActorId,
        admin: ActorId,
    ) -> Result<GameReply, GameError> {
        if !self.admins.contains(msg_source) {
            return Err(GameError::NotAdmin);
        }
        self.admins.retain(|id| *id != admin);
        Ok(GameReply::AdminRemoved)
    }

    fn get_player(
        &self,
        msg_source: &ActorId,
        session_for_account: &Option<ActorId>,
        actions_for_session: ActionsForSession,
    ) -> ActorId {
        let player = match session_for_account {
            Some(account) => {
                let session = self
                    .sessions
                    .get(account)
                    .expect("This account has no valid session");
                assert!(
                    session.expires > exec::block_timestamp(),
                    "The session has already expired"
                );
                assert!(
                    session.allowed_actions.contains(&actions_for_session),
                    "This message is not allowed"
                );
                assert_eq!(
                    session.key, *msg_source,
                    "The account is not approved for this session"
                );
                *account
            }
            None => *msg_source,
        };
        player
    }

    fn update_config(
        &mut self,
        msg_source: &ActorId,
        s_per_block: Option<u64>,
        gas_to_remove_game: Option<u64>,
        time_interval: Option<u32>,
        turn_deadline_ms: Option<u64>,
        block_duration_ms: Option<u64>,
        gas_to_delete_session: Option<u64>
    ) -> Result<GameReply, GameError> {
        if !self.admins.contains(msg_source) {
            return Err(GameError::NotAdmin);
        }

        if let Some(s_per_block) = s_per_block {
            self.config.s_per_block = s_per_block;
        }
        if let Some(gas_to_remove_game) = gas_to_remove_game {
            self.config.gas_to_remove_game = gas_to_remove_game;
        }
        if let Some(time_interval) = time_interval {
            self.config.time_interval = time_interval;
        }
        if let Some(turn_deadline_ms) = turn_deadline_ms {
            self.config.turn_deadline_ms = turn_deadline_ms;
        }

        if let Some(block_duration_ms) = block_duration_ms {
            self.config.block_duration_ms = block_duration_ms;
        }

        if let Some(gas_to_delete_session) = gas_to_delete_session {
            self.config.gas_to_delete_session = gas_to_delete_session;
        }
        Ok(GameReply::ConfigUpdated)
    }
    fn allow_messages(
        &mut self,
        msg_source: &ActorId,
        messages_allowed: bool,
    ) -> Result<GameReply, GameError> {
        if !self.admins.contains(msg_source) {
            return Err(GameError::NotAdmin);
        }
        self.messages_allowed = messages_allowed;
        Ok(GameReply::StatusMessagesUpdated)
    }
}

#[gstd::async_main]
async fn main() {
    let game = unsafe { GAME.as_mut().expect("`Game` is not initialized.") };
    let action: GameAction = msg::load().expect("Failed to decode `GameAction` message.");
    let msg_src = msg::source();
    if !game.messages_allowed && !game.admins.contains(&msg_src) {
        msg::reply(
            Err::<GameReply, GameError>(GameError::MessageProcessingSuspended),
            0,
        )
        .expect("Failed to encode or reply with `Result<GameReply, GameError>`.");
        return;
    }
    let reply = match action {
        GameAction::CreateSession {
            key,
            duration,
            allowed_actions,
            signature,
        } => game.create_session(&key, duration, allowed_actions, signature),
        GameAction::DeleteSessionFromAccount => game.delete_session_from_account(),
        GameAction::DeleteSessionFromProgram { account } => {
            game.delete_session_from_program(&account)
        }
        GameAction::StartGame {
            session_for_account,
        } => game.start_game(&msg_src, session_for_account),
        GameAction::Turn {
            step,
            session_for_account,
        } => game.player_move(&msg_src, step, session_for_account),
        GameAction::Skip {
            session_for_account,
        } => game.skip(&msg_src, session_for_account),
        GameAction::RemoveGameInstance { account_id } => {
            game.remove_game_instance(&msg_src, &account_id)
        }
        GameAction::RemoveGameInstances { accounts } => game.remove_instances(&msg_src, accounts),
        GameAction::AddAdmin(admin) => game.add_admin(&msg_src, admin),
        GameAction::RemoveAdmin(admin) => game.remove_admin(&msg_src, admin),
        GameAction::UpdateConfig {
            s_per_block,
            gas_to_remove_game,
            time_interval,
            turn_deadline_ms,
            block_duration_ms,
            gas_to_delete_session,
        } => game.update_config(
            &msg_src,
            s_per_block,
            gas_to_remove_game,
            time_interval,
            turn_deadline_ms,
            block_duration_ms,
            gas_to_delete_session
        ),
        GameAction::AllowMessages(messages_allowed) => {
            game.allow_messages(&msg_src, messages_allowed)
        }
    };
    msg::reply(reply, 0).expect("Failed to encode or reply with `Result<GameReply, GameError>`.");
}

fn turn() -> u8 {
    let random_input: [u8; 32] = msg::source().into();
    let (random, _) = exec::random(random_input).expect("Error in getting random number");
    random[0] % 2
}

fn make_move(game: &GameInstance) -> Option<usize> {
    match game.bot_mark {
        Mark::O => {
            // if on any of the winning lines there are 2 own pieces and 0 strangers
            // make move
            let step = check_line(&game.board, 2, 0);
            if let Some(step_num) = step {
                return Some(step_num);
            }

            // if on any of the winning lines there are 2 stranger pieces and 0 own
            // make move
            let step = check_line(&game.board, 0, 2);
            if let Some(step_num) = step {
                return Some(step_num);
            }
            // if on any of the winning lines there are 1 own pieces and 0 strangers
            // make move
            let step = check_line(&game.board, 1, 0);
            if let Some(step_num) = step {
                return Some(step_num);
            }
            // if the center is empty, then we occupy the center
            if game.board[4] != Some(Mark::O) && game.board[4] != Some(Mark::X) {
                return Some(4);
            }
            // occupy the first cell
            if game.board[0] != Some(Mark::O) && game.board[0] != Some(Mark::X) {
                return Some(0);
            }
        }
        Mark::X => {
            // if on any of the winning lines there are 2 own pieces and 0 strangers
            // make move
            let step = check_line(&game.board, 0, 2);

            if let Some(step_num) = step {
                return Some(step_num);
            }
            // if on any of the winning lines there are 2 stranger pieces and 0 own
            // make move
            let step = check_line(&game.board, 2, 0);
            if let Some(step_num) = step {
                return Some(step_num);
            }
            // if on any of the winning lines there are 1 own pieces and 0 strangers
            // make move
            let step = check_line(&game.board, 0, 1);

            if let Some(step_num) = step {
                return Some(step_num);
            }
            // if the center is empty, then we occupy the center
            if game.board[4] != Some(Mark::O) && game.board[4] != Some(Mark::X) {
                return Some(4);
            }
            // occupy the first cell
            if game.board[0] != Some(Mark::O) && game.board[0] != Some(Mark::X) {
                return Some(0);
            }
        }
    }
    None
}

fn check_line(map: &[Option<Mark>], sum_o: u8, sum_x: u8) -> Option<usize> {
    for line in VICTORIES.iter() {
        let mut o = 0;
        let mut x = 0;
        for i in 0..3 {
            if map[line[i]] == Some(Mark::O) {
                o += 1;
            }
            if map[line[i]] == Some(Mark::X) {
                x += 1;
            }
        }

        if sum_o == o && sum_x == x {
            for i in 0..3 {
                if map[line[i]] != Some(Mark::O) && map[line[i]] != Some(Mark::X) {
                    return Some(line[i]);
                }
            }
        }
    }
    None
}

fn get_result(map: &[Option<Mark>]) -> Option<Mark> {
    for i in VICTORIES.iter() {
        if map[i[0]] == Some(Mark::X) && map[i[1]] == Some(Mark::X) && map[i[2]] == Some(Mark::X) {
            return Some(Mark::X);
        }

        if map[i[0]] == Some(Mark::O) && map[i[1]] == Some(Mark::O) && map[i[2]] == Some(Mark::O) {
            return Some(Mark::O);
        }
    }
    None
}

#[no_mangle]
extern fn state() {
    let Game {
        admins,
        current_games,
        config,
        messages_allowed,
        sessions,
    } = unsafe { GAME.take().expect("Failed to get state") };
    let query: StateQuery = msg::load().expect("Unable to load the state query");

    match query {
        StateQuery::Admins => {
            msg::reply(StateReply::Admins(admins), 0).expect("Unable to share the state");
        }
        StateQuery::Game { player_id } => {
            let game: Option<GameInstance> = current_games.get(&player_id).cloned();
            msg::reply(StateReply::Game(game), 0).expect("Unable to share the state");
        }
        StateQuery::AllGames => {
            msg::reply(StateReply::AllGames(current_games.into_iter().collect()), 0)
                .expect("Unable to share the state");
        }
        StateQuery::Config => {
            msg::reply(StateReply::Config(config), 0).expect("Unable to share the state");
        }
        StateQuery::MessagesAllowed => {
            msg::reply(StateReply::MessagesAllowed(messages_allowed), 0)
                .expect("Unable to share the state");
        }
        StateQuery::SessionForTheAccount(account) => {
            msg::reply(
                StateReply::SessionForTheAccount(sessions.get(&account).cloned()),
                0,
            )
            .expect("Unable to share the state");
        }
    }
}

fn send_messages(account: &ActorId, config: &Config) {
    msg::send_with_gas_delayed(
        exec::program_id(),
        GameAction::RemoveGameInstance {
            account_id: *account,
        },
        config.gas_to_remove_game,
        0,
        config.time_interval,
    )
    .expect("Error in sending message");
}
