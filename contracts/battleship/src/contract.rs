use battleship_io::*;

use gstd::{
    collections::{BTreeMap, HashMap},
    exec, msg,
    prelude::*,
    ActorId, MessageId,
};

static mut BATTLESHIP: Option<Battleship> = None;

#[derive(Debug, Default)]
struct Battleship {
    pub games: HashMap<ActorId, Game>,
    pub msg_id_to_game_id: BTreeMap<MessageId, ActorId>,
    pub bot_address: ActorId,
    pub admin: ActorId,
    pub sessions: HashMap<ActorId, Session>,
    pub config: Config,
}

impl Battleship {
    fn create_session(
        &mut self,
        key: &ActorId,
        duration: u64,
        allowed_actions: Vec<ActionsForSession>,
    ) -> Result<BattleshipReply, BattleshipError> {
        if duration < MINIMUM_SESSION_SURATION_MS {
            return Err(BattleshipError::DurationIsSmall);
        }

        let msg_source = msg::source();
        let block_timestamp = exec::block_timestamp();
        if let Some(Session {
            key: _,
            expires,
            allowed_actions: _,
        }) = self.sessions.get(&msg_source)
        {
            if *expires > block_timestamp {
                return Err(BattleshipError::AlreadyHaveActiveSession);
            }
        }

        let expires = block_timestamp + duration;

        let number_of_blocks = u32::try_from(duration.div_ceil(self.config.block_duration_ms))
            .expect("Duration is too large");

        if allowed_actions.is_empty() {
            return Err(BattleshipError::NoMessagesForApprovalWerePassed);
        }

        self.sessions.entry(msg_source).insert(Session {
            key: *key,
            expires,
            allowed_actions,
        });

        msg::send_with_gas_delayed(
            exec::program_id(),
            BattleshipAction::DeleteSessionFromProgram {
                account: msg::source(),
            },
            self.config.gas_to_delete_session,
            0,
            number_of_blocks,
        )
        .expect("Error in sending a delayed msg");

        Ok(BattleshipReply::SessionCreated)
    }

    fn delete_session_from_program(
        &mut self,
        session_for_account: &ActorId,
    ) -> Result<BattleshipReply, BattleshipError> {
        if exec::program_id() != msg::source() {
            return Err(BattleshipError::AccessDenied);
        }

        if let Some(session) = self.sessions.remove(session_for_account) {
            if session.expires > exec::block_timestamp() {
                return Err(BattleshipError::AccessDenied);
            }
        }
        Ok(BattleshipReply::SessionDeleted)
    }

    fn delete_session_from_account(&mut self) -> Result<BattleshipReply, BattleshipError> {
        self.sessions.remove(&msg::source());
        Ok(BattleshipReply::SessionDeleted)
    }

    fn start_game(
        &mut self,
        mut ships: Ships,
        session_for_account: Option<ActorId>,
    ) -> Result<BattleshipReply, BattleshipError> {
        let player = self.get_player(&session_for_account, ActionsForSession::StartGame)?;

        if let Some(game) = self.games.get(&player) {
            if !game.game_over {
                return Err(BattleshipError::GameIsAlreadyStarted);
            }
        }

        ships.check_correct_location()?;
        let player_board = ships.get_field()?;

        ships.sort_by_length();
        let game_instance = Game {
            player_board,
            player_ships: ships,
            turn: Some(BattleshipParticipants::Player),
            start_time: exec::block_timestamp(),
            game_over: false,
            game_result: None,
            total_shots: 0,
            ..Default::default()
        };
        self.games.insert(player, game_instance);

        let msg_id = msg::send_with_gas(
            self.bot_address,
            BotBattleshipAction::Start,
            self.config.gas_for_start,
            0,
        )
        .expect("Error in sending a message");

        self.msg_id_to_game_id.insert(msg_id, player);
        Ok(BattleshipReply::MessageSentToBot)
    }

    fn player_move(
        &mut self,
        step: u8,
        session_for_account: Option<ActorId>,
    ) -> Result<BattleshipReply, BattleshipError> {
        let player = self.get_player(&session_for_account, ActionsForSession::Turn)?;
        if step > 24 {
            return Err(BattleshipError::OutOfBounds);
        }

        let Some(game) = self.games.get_mut(&player) else {
            return Err(BattleshipError::GameIsNotStarted);
        };
        if game.game_over {
            return Err(BattleshipError::GameIsAlreadyOver);
        }

        if game.bot_board.is_empty() {
            return Err(BattleshipError::BotDidNotInitializeBoard);
        }

        if game.turn != Some(BattleshipParticipants::Player) {
            return Err(BattleshipError::NotYourTurn);
        }

        if game.bot_board[step as usize] != Entity::Empty
            && game.bot_board[step as usize] != Entity::Ship
        {
            return Err(BattleshipError::ThisCellAlreadyKnown);
        }

        let res = game.bot_ships.bang(step);
        match res {
            Step::Missed => game.bot_board[step as usize] = Entity::Boom,
            Step::Injured => game.bot_board[step as usize] = Entity::BoomShip,
            Step::Killed => game.dead_ship(step, 1),
        }
        game.total_shots += 1;

        if game.bot_ships.check_end_game() {
            game.game_over = true;
            game.game_result = Some(BattleshipParticipants::Player);
            game.end_time = exec::block_timestamp();
            return Ok(BattleshipReply::GameFinished(
                BattleshipParticipants::Player,
            ));
        }
        game.turn = Some(BattleshipParticipants::Bot);

        let board = game.get_hidden_field();
        let msg_id = msg::send_with_gas(
            self.bot_address,
            BotBattleshipAction::Turn(board),
            self.config.gas_for_move,
            0,
        )
        .expect("Error in sending a message");

        self.msg_id_to_game_id.insert(msg_id, player);
        Ok(BattleshipReply::MessageSentToBot)
    }

    fn change_bot(&mut self, bot: ActorId) -> Result<BattleshipReply, BattleshipError> {
        if msg::source() != self.admin {
            return Err(BattleshipError::NotAdmin);
        }
        self.bot_address = bot;
        Ok(BattleshipReply::BotChanged(bot))
    }

    fn clear_state(
        &mut self,
        leave_active_games: bool,
    ) -> Result<BattleshipReply, BattleshipError> {
        if msg::source() != self.admin {
            return Err(BattleshipError::NotAdmin);
        }
        if leave_active_games {
            self.games.retain(|_actor_id, game| !game.game_over);
        } else {
            self.games.clear();
        }
        Ok(BattleshipReply::StateCleared)
    }

    fn delete_game(&mut self, player_address: ActorId) -> Result<BattleshipReply, BattleshipError> {
        if msg::source() != self.admin {
            return Err(BattleshipError::NotAdmin);
        }
        self.games.remove(&player_address);
        Ok(BattleshipReply::GameDeleted)
    }

    fn get_player(
        &self,
        session_for_account: &Option<ActorId>,
        actions_for_session: ActionsForSession,
    ) -> Result<ActorId, BattleshipError> {
        let msg_source = msg::source();
        let player = match session_for_account {
            Some(account) => {
                let session = self
                    .sessions
                    .get(account)
                    .ok_or(BattleshipError::HasNotValidSession)?;

                if session.expires <= exec::block_timestamp() {
                    return Err(BattleshipError::SessionHasAlreadyExpired);
                }
                if !session.allowed_actions.contains(&actions_for_session) {
                    return Err(BattleshipError::MessageIsNotAllowed);
                }
                if session.expires <= exec::block_timestamp() {
                    return Err(BattleshipError::SessionHasAlreadyExpired);
                }
                if session.key != msg_source {
                    return Err(BattleshipError::NotApproved);
                }
                *account
            }
            None => msg_source,
        };
        Ok(player)
    }

    fn update_config(
        &mut self,
        gas_for_start: Option<u64>,
        gas_for_move: Option<u64>,
        gas_to_delete_session: Option<u64>,
        block_duration_ms: Option<u64>,
    ) -> Result<BattleshipReply, BattleshipError> {
        if msg::source() != self.admin {
            return Err(BattleshipError::NotAdmin);
        }
        if let Some(gas_for_start) = gas_for_start {
            self.config.gas_for_start = gas_for_start;
        }

        if let Some(gas_for_move) = gas_for_move {
            self.config.gas_for_move = gas_for_move;
        }

        if let Some(gas_to_delete_session) = gas_to_delete_session {
            self.config.gas_to_delete_session = gas_to_delete_session;
        }

        if let Some(block_duration_ms) = block_duration_ms {
            self.config.block_duration_ms = block_duration_ms;
        }
        Ok(BattleshipReply::ConfigUpdated)
    }
}

#[no_mangle]
extern fn init() {
    let BattleshipInit {
        bot_address,
        config,
    } = msg::load().expect("Unable to decode BattleshipInit");
    unsafe {
        BATTLESHIP = Some(Battleship {
            bot_address,
            config,
            admin: msg::source(),
            ..Default::default()
        });
    }
}

#[no_mangle]
extern fn handle() {
    let battleship = unsafe {
        BATTLESHIP
            .as_mut()
            .expect("`Battleship` is not initialized.")
    };
    let action: BattleshipAction =
        msg::load().expect("Failed to decode `BattleshipAction` message.");
    let reply = match action {
        BattleshipAction::StartGame {
            ships,
            session_for_account,
        } => battleship.start_game(ships, session_for_account),
        BattleshipAction::Turn {
            step,
            session_for_account,
        } => battleship.player_move(step, session_for_account),
        BattleshipAction::ChangeBot { bot } => battleship.change_bot(bot),
        BattleshipAction::ClearState { leave_active_games } => {
            battleship.clear_state(leave_active_games)
        }
        BattleshipAction::DeleteGame { player_address } => battleship.delete_game(player_address),
        BattleshipAction::CreateSession {
            key,
            duration,
            allowed_actions,
        } => battleship.create_session(&key, duration, allowed_actions),
        BattleshipAction::DeleteSessionFromProgram { account } => {
            battleship.delete_session_from_program(&account)
        }
        BattleshipAction::DeleteSessionFromAccount => battleship.delete_session_from_account(),
        BattleshipAction::UpdateConfig {
            gas_for_start,
            gas_for_move,
            gas_to_delete_session,
            block_duration_ms,
        } => battleship.update_config(
            gas_for_start,
            gas_for_move,
            gas_to_delete_session,
            block_duration_ms,
        ),
    };
    msg::reply(reply, 0)
        .expect("Failed to encode or reply with `Result<BattleshipReply, BattleshipError>`.");
}

#[no_mangle]
extern fn handle_reply() {
    let reply_to = msg::reply_to().expect("Failed to query reply_to data");
    let battleship = unsafe { BATTLESHIP.as_mut().expect("The game is not initialized") };
    let game_id = battleship
        .msg_id_to_game_id
        .remove(&reply_to)
        .expect("Unexpected reply");

    let game = battleship
        .games
        .get_mut(&game_id)
        .expect("Unexpected: Game does not exist");

    let action: BattleshipAction =
        msg::load().expect("Failed to decode `BattleshipAction` message.");
    match action {
        BattleshipAction::StartGame {
            ships,
            session_for_account: _,
        } => game.start_bot(ships),
        BattleshipAction::Turn {
            step,
            session_for_account: _,
        } => {
            game.turn(step);
            game.turn = Some(BattleshipParticipants::Player);
            if game.player_ships.check_end_game() {
                game.game_over = true;
                game.game_result = Some(BattleshipParticipants::Bot);
                game.end_time = exec::block_timestamp();
                msg::send(
                    game_id,
                    BattleshipReply::GameFinished(BattleshipParticipants::Bot),
                    0,
                )
                .expect("Unable to send the message about game over");
            }
        }
        _ => (),
    }
}

#[no_mangle]
extern fn state() {
    let battleship = unsafe { BATTLESHIP.take().expect("Unexpected error in taking state") };
    let query: StateQuery = msg::load().expect("Unable to load the state query");
    match query {
        StateQuery::All => {
            msg::reply(StateReply::All(battleship.into()), 0).expect("Unable to share the state");
        }
        StateQuery::Game(player_id) => {
            let game_state = battleship.games.get(&player_id).map(|game| GameState {
                player_board: game.player_board.clone(),
                bot_board: game.bot_board.clone(),
                player_ships: game.player_ships.count_alive_ships(),
                bot_ships: game.bot_ships.count_alive_ships(),
                turn: game.turn.clone(),
                start_time: game.start_time,
                total_shots: game.total_shots,
                end_time: game.end_time,
                game_over: game.game_over,
                game_result: game.game_result.clone(),
            });

            msg::reply(StateReply::Game(game_state), 0).expect("Unable to share the state");
        }
        StateQuery::BotContractId => {
            msg::reply(StateReply::BotContractId(battleship.bot_address), 0)
                .expect("Unable to share the state");
        }
        StateQuery::SessionForTheAccount(account) => {
            msg::reply(
                StateReply::SessionForTheAccount(battleship.sessions.get(&account).cloned()),
                0,
            )
            .expect("Unable to share the state");
        }
    }
}

impl From<Battleship> for BattleshipState {
    fn from(value: Battleship) -> Self {
        let Battleship {
            games,
            bot_address,
            admin,
            ..
        } = value;

        let games: Vec<(ActorId, GameState)> = games
            .iter()
            .map(|(id, game)| {
                let game = GameState {
                    player_board: game.player_board.clone(),
                    bot_board: game.bot_board.clone(),
                    player_ships: game.player_ships.clone().count_alive_ships(),
                    bot_ships: game.bot_ships.clone().count_alive_ships(),
                    turn: game.turn.clone(),
                    start_time: game.start_time,
                    total_shots: game.total_shots,
                    end_time: game.end_time,
                    game_over: game.game_over,
                    game_result: game.game_result.clone(),
                };
                (*id, game)
            })
            .collect();

        Self {
            games,
            bot_address,
            admin,
        }
    }
}
