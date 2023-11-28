use battleship_io::{
    ActionsForSession, BattleshipAction, BattleshipInit, BattleshipParticipants, BattleshipReply,
    BattleshipState, BotBattleshipAction, Config, Entity, Game, GameState, Session, Ships,
    StateQuery, StateReply, Step,
};
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
    ) {
        let msg_source = msg::source();
        let block_timestamp = exec::block_timestamp();
        if let Some(Session {
            key: _,
            expires,
            allowed_actions: _,
        }) = self.sessions.get(&msg_source)
        {
            if *expires > block_timestamp {
                panic!("You already have an active session. If you want to create a new one, please delete this one.")
            }
        }

        assert!(
            !allowed_actions.is_empty(),
            "No messages for approval were passed."
        );

        self.sessions.entry(msg_source).or_insert_with(|| Session {
            key: *key,
            expires: block_timestamp + duration,
            allowed_actions,
        });

        msg::reply(BattleshipReply::SessionCreated, 0).expect("Error in sending a reply");
    }

    fn delete_session(&mut self) {
        assert!(
            self.sessions.remove(&msg::source()).is_some(),
            "No active session"
        );

        msg::reply(BattleshipReply::SessionDeleted, 0).expect("Error in sending a reply");
    }
    fn start_game(&mut self, mut ships: Ships, session_for_account: Option<ActorId>) {
        let player = self.get_player(&session_for_account, ActionsForSession::StartGame);

        if let Some(game) = self.games.get(&player) {
            assert!(game.game_over, "Please finish the previous game");
        }

        assert!(
            ships.check_correct_location(),
            "Incorrect location of ships"
        );

        let player_board = ships.get_field();
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
        msg::reply(BattleshipReply::MessageSentToBot, 0).expect("Error in sending a reply");
    }

    fn player_move(&mut self, step: u8, session_for_account: Option<ActorId>) {
        let player = self.get_player(&session_for_account, ActionsForSession::Turn);
        assert!(step < 25, "Step must be less than 24");

        let game = self
            .games
            .get_mut(&player)
            .expect("The player has no game, please start the game");

        assert!(!game.game_over, "Game is already over");

        if game.bot_board.is_empty() {
            panic!("The bot did not initialize the board");
        }

        if game.turn != Some(BattleshipParticipants::Player) {
            panic!("Please wait your turn");
        }

        if game.bot_board[step as usize] != Entity::Empty
            && game.bot_board[step as usize] != Entity::Ship
        {
            panic!("The value of this cell is already known");
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
            msg::reply(BattleshipReply::EndGame(BattleshipParticipants::Player), 0)
                .expect("Error in sending a reply");
            return;
        }
        game.turn = Some(BattleshipParticipants::Bot);

        let board = game.get_hidden_field();
        let msg_id = msg::send_with_gas(
            self.bot_address,
            BotBattleshipAction::Turn(board),
            self.config.gas_for_turn,
            0,
        )
        .expect("Error in sending a message");

        self.msg_id_to_game_id.insert(msg_id, player);
        msg::reply(BattleshipReply::MessageSentToBot, 0).expect("Error in sending a reply");
    }

    fn change_bot(&mut self, bot: ActorId) {
        assert!(
            msg::source() == self.admin,
            "Only the admin can change the bot's contract address"
        );
        self.bot_address = bot;
        msg::reply(BattleshipReply::BotChanged(bot), 0).expect("Error in sending a reply");
    }

    fn change_config(&mut self, config: Config) {
        assert!(
            msg::source() == self.admin,
            "Only the admin can change the configuration"
        );
        self.config = config.clone();
        msg::reply(BattleshipReply::ConfigChanged(config), 0).expect("Error in sending a reply");
    }

    fn clear_state(&mut self, leave_active_games: bool) {
        assert!(
            msg::source() == self.admin,
            "Only the admin can change the contract state"
        );
        if leave_active_games {
            self.games.retain(|_actor_id, game| !game.game_over);
        } else {
            self.games.clear();
        }
    }

    fn delete_game(&mut self, player_address: ActorId) {
        assert!(
            msg::source() == self.admin,
            "Only the admin can change the contract state"
        );
        self.games.remove(&player_address);
    }

    fn get_player(
        &self,
        session_for_account: &Option<ActorId>,
        actions_for_session: ActionsForSession,
    ) -> ActorId {
        let msg_source = msg::source();
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
                    session.key, msg_source,
                    "The account is not approved for this session"
                );
                *account
            }
            None => msg_source,
        };
        player
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
            admin: msg::source(),
            config,
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
    match action {
        BattleshipAction::StartGame {
            ships,
            session_for_account,
        } => battleship.start_game(ships, session_for_account),
        BattleshipAction::Turn {
            step,
            session_for_account,
        } => battleship.player_move(step, session_for_account),
        BattleshipAction::ChangeBot { bot } => battleship.change_bot(bot),
        BattleshipAction::ChangeConfig { config } => battleship.change_config(config),
        BattleshipAction::ClearState { leave_active_games } => {
            battleship.clear_state(leave_active_games)
        }
        BattleshipAction::DeleteGame { player_address } => battleship.delete_game(player_address),
        BattleshipAction::CreateSession {
            key,
            duration,
            allowed_actions,
        } => battleship.create_session(&key, duration, allowed_actions),
        BattleshipAction::DeleteSession => battleship.delete_session(),
    }
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
        StateQuery::Config => {
            msg::reply(StateReply::Config(battleship.config), 0)
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
            config,
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
            config,
        }
    }
}
