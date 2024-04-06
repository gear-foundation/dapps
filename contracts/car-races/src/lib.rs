#![no_std]

use gstd::{
    collections::{BTreeMap, HashMap},
    exec, msg,
    prelude::*,
    ActorId, MessageId,
};
static mut CONTRACT: Option<Contract> = None;

use car_races_io::*;

#[derive(Default)]
pub struct Contract {
    pub admins: Vec<ActorId>,
    pub strategy_ids: Vec<ActorId>,
    pub games: HashMap<ActorId, Game>,
    pub msg_id_to_game_id: HashMap<MessageId, ActorId>,
    pub config: Config,
    pub messages_allowed: bool,
}

impl Contract {
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

    fn start_game(&mut self, msg_src: &ActorId) -> Result<GameReply, GameError> {
        let last_time_step = exec::block_timestamp();

        let game = if let Some(game) = self.games.get_mut(msg_src) {
            if game.state != GameState::Finished {
                return Err(GameError::GameAlreadyStarted);
            }
            game.current_round = 0;
            game.result = None;
            game.last_time_step = last_time_step;
            game
        } else {
            self.games.entry(*msg_src).or_insert_with(|| Game {
                last_time_step,
                ..Default::default()
            })
        };

        game.car_ids = vec![*msg_src, self.strategy_ids[0], self.strategy_ids[1]];
        let initial_state = Car {
            position: 0,
            speed: self.config.initial_speed,
            car_actions: Vec::new(),
            round_result: None,
        };

        game.cars.insert(*msg_src, initial_state.clone());
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
    ) -> Result<GameReply, GameError> {
        let game = self.get_game(msg_src);

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

        self.msg_id_to_game_id.insert(msg_id, *msg_src);
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
        if let Some(time_for_game_storage) = time_for_game_storage {
            self.config.time_for_game_storage = time_for_game_storage;
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
        GameAction::StartGame => contract.start_game(&msg_src),
        GameAction::Play { account } => contract.play(&msg_src, &account),
        GameAction::PlayerMove { strategy_action } => {
            contract.player_move(&msg_src, strategy_action)
        }
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
        ),
        GameAction::AllowMessages(messages_allowed) => {
            contract.allow_messages(&msg_src, messages_allowed)
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
