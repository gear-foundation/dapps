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
    fn add_strategy_ids(&mut self, car_ids: Vec<ActorId>) {
        assert!(self.admins.contains(&msg::source()), "You are not admin");

        assert!(car_ids.len() == 2, "There must be 2 strategies of cars");
        self.strategy_ids = car_ids;
    }

    fn start_game(&mut self) {
        let player = msg::source();

        let last_time_step = exec::block_timestamp();

        let game = if let Some(game) = self.games.get_mut(&player) {
            if game.state != GameState::Finished {
                panic!("Please complete the game");
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
        msg::reply(GameReply::GameStarted, 0).expect("Error during reply");
    }

    fn player_move(&mut self, strategy_move: StrategyAction) {
        let player = msg::source();
        let game = self.get_game(&player);

        assert_eq!(
            game.state,
            GameState::PlayerAction,
            "Not time for the player"
        );
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
    }

    fn play(&mut self, account: &ActorId) {
        assert_eq!(
            msg::source(),
            exec::program_id(),
            "Only program can send this message"
        );

        let game = self.get_game(account);

        if game.state == GameState::Finished {
            let result = game.result.clone();
            let cars = game.cars.clone();
            let car_ids = game.car_ids.clone();
            self.send_messages(account);
            send_message_round_info(&car_ids[0], &cars, &result);
            return;
        }
        if game.current_turn == 0 {
            game.state = GameState::PlayerAction;
            let result = game.result.clone();
            let cars = game.cars.clone();
            let car_ids = game.car_ids.clone();
            send_message_round_info(&car_ids[0], &cars, &result);
            return;
        }

        let car_id = game.get_current_car_id();

        let msg_id = msg::send(car_id, CarAction::YourTurn(game.cars.clone()), 0)
            .expect("Error in sending a message");

        self.msg_id_to_game_id.insert(msg_id, *account);
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

    fn remove_game_instance(&mut self, account: &ActorId) {
        assert_eq!(
            msg::source(),
            exec::program_id(),
            "This message can be sent only by the program"
        );
        let game = self
            .games
            .get(account)
            .expect("Unexpected: the game does not exist");

        if game.state == GameState::Finished {
            self.games.remove(account);
        }
    }

    fn remove_instances(&mut self, player_ids: Option<Vec<ActorId>>) {
        assert!(
            self.admins.contains(&msg::source()),
            "Only admin can send this message"
        );
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
    }
}

#[no_mangle]
extern fn handle() {
    let action: GameAction = msg::load().expect("Unable to decode the message");
    let contract = unsafe { CONTRACT.as_mut().expect("The game is not initialized") };

    if let GameAction::AllowMessages(messages_allowed) = action {
        assert!(
            contract.admins.contains(&msg::source()),
            "Only an admin can send this message"
        );
        contract.messages_allowed = messages_allowed;
        return;
    }

    assert!(
        contract.messages_allowed,
        "Message processing has been suspended for some time"
    );

    match action {
        GameAction::AddStrategyIds { car_ids } => contract.add_strategy_ids(car_ids),
        GameAction::StartGame => contract.start_game(),
        GameAction::Play { account } => contract.play(&account),
        GameAction::PlayerMove { strategy_action } => contract.player_move(strategy_action),
        GameAction::RemoveGameInstance { account_id } => contract.remove_game_instance(&account_id),
        GameAction::RemoveGameInstances { players_ids } => contract.remove_instances(players_ids),
        GameAction::AddAdmin(admin) => {
            assert!(
                contract.admins.contains(&msg::source()),
                "You are not admin"
            );
            contract.admins.push(admin);
        }
        GameAction::RemoveAdmin(admin) => {
            assert!(
                contract.admins.contains(&msg::source()),
                "You are not admin"
            );

            contract.admins.retain(|id| *id != admin);
        }
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
        } => {
            assert!(
                contract.admins.contains(&msg::source()),
                "You are not admin"
            );

            if let Some(gas_to_remove_game) = gas_to_remove_game {
                contract.config.gas_to_remove_game = gas_to_remove_game;
            }
            if let Some(initial_speed) = initial_speed {
                contract.config.initial_speed = initial_speed;
            }

            if let Some(min_speed) = min_speed {
                contract.config.min_speed = min_speed;
            }

            if let Some(max_speed) = max_speed {
                contract.config.max_speed = max_speed;
            }

            if let Some(gas_for_round) = gas_for_round {
                contract.config.gas_for_round = gas_for_round;
            }
            if let Some(time_interval) = time_interval {
                contract.config.time_interval = time_interval;
            }

            if let Some(max_distance) = max_distance {
                contract.config.max_distance = max_distance;
            }

            if let Some(max_speed) = max_speed {
                contract.config.max_speed = max_speed;
            }

            if let Some(time) = time {
                contract.config.time = time;
            }
            if let Some(time_for_game_storage) = time_for_game_storage {
                contract.config.time_for_game_storage = time_for_game_storage;
            }
        }
        _ => {}
    }
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
