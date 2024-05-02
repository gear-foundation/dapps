#![no_std]
mod sr25519;
use gstd::{
    collections::{BTreeMap, HashMap},
    exec, msg,
    prelude::*,
    ActorId, MessageId,
};
static mut CONTRACT: Option<Contract> = None;
// Minimum duration of session: 3 mins = 180_000 ms = 60 blocks
pub const MINIMUM_SESSION_SURATION_MS: u64 = 180_000;

use car_races_io::*;

#[derive(Default)]
pub struct Contract {
    pub admins: Vec<ActorId>,
    pub strategy_ids: Vec<ActorId>,
    pub games: HashMap<ActorId, Game>,
    pub msg_id_to_game_id: HashMap<MessageId, ActorId>,
    pub config: Config,
    pub messages_allowed: bool,
    pub sessions: HashMap<ActorId, Session>,
}

impl Contract {
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


    fn add_strategy_ids(
        &mut self,
        msg_src: &ActorId,
        car_ids: Vec<ActorId>,
    ) -> Result<GameReply, GameError> {
        if !self.admins.contains(msg_src) {
            return Err(GameError::NotAdmin);
        }
        if car_ids.len() != 2 {
            return Err(GameError::MustBeTwoStrategies);
        }
        self.strategy_ids = car_ids;
        Ok(GameReply::StrategyAdded)
    }

    fn start_game(
        &mut self,
        msg_src: &ActorId,
        session_for_account: Option<ActorId>,
    ) -> Result<GameReply, GameError> {
        let player = self.get_player(msg_src, &session_for_account, ActionsForSession::StartGame);
        let last_time_step = exec::block_timestamp();

        let game = if let Some(game) = self.games.get_mut(&player) {
            if game.state != GameState::Finished {
                return Err(GameError::GameAlreadyStarted);
            }
            game.current_round = 0;
            game.result = None;
            game.last_time_step = last_time_step;
            game
        } else {
            self.games.entry(player).or_insert_with(|| Game {
                last_time_step,
                ..Default::default()
            })
        };

        game.car_ids = vec![player, self.strategy_ids[0], self.strategy_ids[1]];
        let initial_state = Car {
            position: 0,
            speed: self.config.initial_speed,
            car_actions: Vec::new(),
            round_result: None,
        };

        game.cars.insert(player, initial_state.clone());
        game.cars
            .insert(self.strategy_ids[0], initial_state.clone());
        game.cars.insert(self.strategy_ids[1], initial_state);

        game.state = GameState::PlayerAction;
        Ok(GameReply::GameStarted)
    }

    fn player_move(
        &mut self,
        msg_src: &ActorId,
        strategy_move: StrategyAction,
        session_for_account: Option<ActorId>,
    ) -> Result<GameReply, GameError> {
        let player = self.get_player(msg_src, &session_for_account, ActionsForSession::PlayerMove);
        let game = self.get_game(&player);

        if game.state != GameState::PlayerAction {
            return Err(GameError::NotPlayerTurn);
        }
        match strategy_move {
            StrategyAction::BuyAcceleration => {
                game.buy_acceleration();
            }
            StrategyAction::BuyShell => {
                game.buy_shell();
            }
            StrategyAction::Skip => {}
        }

        game.state = GameState::Race;
        game.last_time_step = exec::block_timestamp();
        let num_of_cars = game.car_ids.len() as u8;

        game.current_turn = (game.current_turn + 1) % num_of_cars;

        let car_id = game.get_current_car_id();

        let msg_id = msg::send_with_gas(
            car_id,
            CarAction::YourTurn(game.cars.clone()),
            self.config.gas_for_round,
            0,
        )
        .expect("Error in sending a message");

        self.msg_id_to_game_id.insert(msg_id, player);
        Ok(GameReply::MoveMade)
    }

    fn play(&mut self, msg_src: &ActorId, account: &ActorId) -> Result<GameReply, GameError> {
        if *msg_src != exec::program_id() {
            return Err(GameError::NotProgram);
        }

        let game = self.get_game(account);

        if game.state == GameState::Finished {
            let result = game.result.clone();
            let cars = game.cars.clone();
            let car_ids = game.car_ids.clone();
            self.send_messages(account);
            send_message_round_info(&car_ids[0], &cars, &result);
            return Ok(GameReply::GameFinished);
        }
        if game.current_turn == 0 {
            game.state = GameState::PlayerAction;
            let result = game.result.clone();
            let cars = game.cars.clone();
            let car_ids = game.car_ids.clone();
            send_message_round_info(&car_ids[0], &cars, &result);
            return Ok(GameReply::MoveMade);
        }

        let car_id = game.get_current_car_id();

        let msg_id = msg::send(car_id, CarAction::YourTurn(game.cars.clone()), 0)
            .expect("Error in sending a message");

        self.msg_id_to_game_id.insert(msg_id, *account);
        Ok(GameReply::MoveMade)
    }

    fn get_game(&mut self, account: &ActorId) -> &mut Game {
        self.games.get_mut(account).expect("Game does not exist")
    }

    fn send_messages(&mut self, account: &ActorId) {
        msg::send_with_gas_delayed(
            exec::program_id(),
            GameAction::RemoveGameInstance {
                account_id: *account,
            },
            self.config.gas_to_remove_game,
            0,
            self.config.time_interval,
        )
        .expect("Error in sending message");
    }

    fn remove_game_instance(
        &mut self,
        msg_src: &ActorId,
        account: &ActorId,
    ) -> Result<GameReply, GameError> {
        if *msg_src != exec::program_id() {
            return Err(GameError::NotProgram);
        }
        let game = self
            .games
            .get(account)
            .expect("Unexpected: the game does not exist");

        if game.state == GameState::Finished {
            self.games.remove(account);
        }
        Ok(GameReply::GameInstanceRemoved)
    }

    fn remove_instances(
        &mut self,
        msg_src: &ActorId,
        player_ids: Option<Vec<ActorId>>,
    ) -> Result<GameReply, GameError> {
        if !self.admins.contains(msg_src) {
            return Err(GameError::NotAdmin);
        }
        match player_ids {
            Some(player_ids) => {
                for player_id in player_ids {
                    self.games.remove(&player_id);
                }
            }
            None => {
                self.games.retain(|_, game| {
                    (exec::block_timestamp() - game.last_time_step)
                        < self.config.time_for_game_storage
                });
            }
        }
        Ok(GameReply::InstancesRemoved)
    }
    fn add_admin(&mut self, msg_src: &ActorId, admin: ActorId) -> Result<GameReply, GameError> {
        if !self.admins.contains(msg_src) {
            return Err(GameError::NotAdmin);
        }
        self.admins.push(admin);
        Ok(GameReply::AdminAdded)
    }
    fn remove_admin(&mut self, msg_src: &ActorId, admin: ActorId) -> Result<GameReply, GameError> {
        if !self.admins.contains(msg_src) {
            return Err(GameError::NotAdmin);
        }
        self.admins.retain(|id| *id != admin);
        Ok(GameReply::AdminRemoved)
    }

    #[allow(clippy::too_many_arguments)]
    fn update_config(
        &mut self,
        msg_src: &ActorId,
        gas_to_remove_game: Option<u64>,
        initial_speed: Option<u32>,
        min_speed: Option<u32>,
        max_speed: Option<u32>,
        gas_for_round: Option<u64>,
        time_interval: Option<u32>,
        max_distance: Option<u32>,
        time: Option<u32>,
        time_for_game_storage: Option<u64>,
        block_duration_ms: Option<u64>,
        gas_to_delete_session: Option<u64>,
    ) -> Result<GameReply, GameError> {
        if !self.admins.contains(msg_src) {
            return Err(GameError::NotAdmin);
        }

        if let Some(gas_to_remove_game) = gas_to_remove_game {
            self.config.gas_to_remove_game = gas_to_remove_game;
        }
        if let Some(initial_speed) = initial_speed {
            self.config.initial_speed = initial_speed;
        }

        if let Some(min_speed) = min_speed {
            self.config.min_speed = min_speed;
        }

        if let Some(max_speed) = max_speed {
            self.config.max_speed = max_speed;
        }

        if let Some(gas_for_round) = gas_for_round {
            self.config.gas_for_round = gas_for_round;
        }
        if let Some(time_interval) = time_interval {
            self.config.time_interval = time_interval;
        }

        if let Some(max_distance) = max_distance {
            self.config.max_distance = max_distance;
        }

        if let Some(max_speed) = max_speed {
            self.config.max_speed = max_speed;
        }

        if let Some(time) = time {
            self.config.time = time;
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
        msg_src: &ActorId,
        messages_allowed: bool,
    ) -> Result<GameReply, GameError> {
        if !self.admins.contains(msg_src) {
            return Err(GameError::NotAdmin);
        }
        self.messages_allowed = messages_allowed;
        Ok(GameReply::StatusMessagesUpdated)
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
}

#[no_mangle]
extern fn handle() {
    let action: GameAction = msg::load().expect("Unable to decode the message");
    let contract = unsafe { CONTRACT.as_mut().expect("The game is not initialized") };
    let msg_src = msg::source();

    if !contract.messages_allowed && !contract.admins.contains(&msg_src) {
        msg::reply(
            Err::<GameReply, GameError>(GameError::MessageProcessingSuspended),
            0,
        )
        .expect("Failed to encode or reply with `Result<GameReply, GameError>`.");
        return;
    }

    let reply = match action {
        GameAction::AddStrategyIds { car_ids } => contract.add_strategy_ids(&msg_src, car_ids),
        GameAction::StartGame {
            session_for_account,
        } => contract.start_game(&msg_src, session_for_account),
        GameAction::Play { account } => contract.play(&msg_src, &account),
        GameAction::PlayerMove {
            strategy_action,
            session_for_account,
        } => contract.player_move(&msg_src, strategy_action, session_for_account),
        GameAction::RemoveGameInstance { account_id } => {
            contract.remove_game_instance(&msg_src, &account_id)
        }
        GameAction::RemoveGameInstances { players_ids } => {
            contract.remove_instances(&msg_src, players_ids)
        }
        GameAction::AddAdmin(admin) => contract.add_admin(&msg_src, admin),
        GameAction::RemoveAdmin(admin) => contract.remove_admin(&msg_src, admin),
        GameAction::UpdateConfig {
            gas_to_remove_game,
            initial_speed,
            min_speed,
            max_speed,
            gas_for_round,
            time_interval,
            max_distance,
            time,
            time_for_game_storage,
            block_duration_ms,
            gas_to_delete_session,
        } => contract.update_config(
            &msg_src,
            gas_to_remove_game,
            initial_speed,
            min_speed,
            max_speed,
            gas_for_round,
            time_interval,
            max_distance,
            time,
            time_for_game_storage,
            block_duration_ms,
            gas_to_delete_session,
        ),
        GameAction::AllowMessages(messages_allowed) => {
            contract.allow_messages(&msg_src, messages_allowed)
        }
        GameAction::CreateSession {
            key,
            duration,
            allowed_actions,
            signature,
        } => contract.create_session(&key, duration, allowed_actions, signature),
        GameAction::DeleteSessionFromAccount => contract.delete_session_from_account(),
        GameAction::DeleteSessionFromProgram { account } => {
            contract.delete_session_from_program(&account)
        }
    };
    msg::reply(reply, 0).expect("Failed to encode or reply with `Result<GameReply, GameError>`.");
}

#[no_mangle]
extern fn handle_reply() {
    let reply_to = msg::reply_to().expect("Unable to get the msg id");
    let contract = unsafe { CONTRACT.as_mut().expect("The game is not initialized") };

    let game_id = contract
        .msg_id_to_game_id
        .remove(&reply_to)
        .expect("Unexpected reply");

    let game = contract
        .games
        .get_mut(&game_id)
        .expect("Unexpected: Game does not exist");

    let bytes = msg::load_bytes().expect("Unable to load bytes");
    // car eliminated from race for wrong payload
    if let Ok(strategy) = StrategyAction::decode(&mut &bytes[..]) {
        match strategy {
            StrategyAction::BuyAcceleration => {
                game.buy_acceleration();
            }
            StrategyAction::BuyShell => {
                game.buy_shell();
            }
            StrategyAction::Skip => {}
        }
    } else {
        // car eliminated from race for wrong payload
        let current_car_id = game.get_current_car_id();
        game.car_ids.retain(|car_id| *car_id != current_car_id);
    }
    let num_of_cars = game.car_ids.len() as u8;

    game.current_turn = (game.current_turn + 1) % num_of_cars;

    // if one round is made, then we update the positions of the cars
    // and send a message about the new position of the fields
    if game.current_turn == 0 {
        game.current_round = game.current_round.saturating_add(1);
        game.update_positions(&contract.config);
    }

    msg::send(exec::program_id(), GameAction::Play { account: game_id }, 0)
        .expect("Error in sending a msg");
}

#[no_mangle]
extern fn init() {
    let init_msg: GameInit = msg::load().expect("Unable to load the message");

    unsafe {
        CONTRACT = Some(Contract {
            admins: vec![msg::source()],
            config: init_msg.config,
            games: HashMap::with_capacity(20_000),
            msg_id_to_game_id: HashMap::with_capacity(5_000),
            ..Default::default()
        });
    }
}

#[no_mangle]
extern fn state() {
    let Contract {
        admins,
        strategy_ids,
        games,
        msg_id_to_game_id,
        config,
        messages_allowed,
        sessions,
    } = unsafe { CONTRACT.take().expect("Failed to get state") };
    let query: StateQuery = msg::load().expect("Unable to load the state query");

    match query {
        StateQuery::Admins => {
            msg::reply(StateReply::Admins(admins), 0).expect("Unable to share the state");
        }
        StateQuery::StrategyIds => {
            msg::reply(StateReply::StrategyIds(strategy_ids), 0)
                .expect("Unable to share the state");
        }
        StateQuery::Game { account_id } => {
            let game = games.get(&account_id).cloned();
            msg::reply(StateReply::Game(game), 0).expect("Unable to share the state");
        }
        StateQuery::AllGames => {
            msg::reply(StateReply::AllGames(games.into_iter().collect()), 0)
                .expect("Unable to share the state");
        }
        StateQuery::MsgIdToGameId => {
            msg::reply(
                StateReply::MsgIdToGameId(msg_id_to_game_id.into_iter().collect()),
                0,
            )
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

fn send_message_round_info(
    account: &ActorId,
    cars_info: &BTreeMap<ActorId, Car>,
    result: &Option<GameResult>,
) {
    let mut cars = Vec::new();
    for (car_id, info) in cars_info.iter() {
        cars.push((*car_id, info.position, info.round_result.clone()));
    }
    msg::send(
        *account,
        RoundInfo {
            cars,
            result: result.clone(),
        },
        0,
    )
    .expect("Unable to send the message about round info");
}
