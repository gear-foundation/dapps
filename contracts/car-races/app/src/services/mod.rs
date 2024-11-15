#![allow(clippy::new_without_default)]
use collections::HashMap;
use sails_rs::prelude::*;
use session::Storage as SessionStorage;
use session::{ActionsForSession, SessionData};
pub mod error;
pub mod game;
pub mod session;
pub mod utils;
use crate::services::utils::{panicking, panic};
use error::Error;
use game::*;
pub struct CarRacesService;

use gstd::{exec, msg};
static mut DATA: Option<ContractData> = None;
static mut CONFIG: Option<Config> = None;

#[derive(Debug, Default)]
pub struct ContractData {
    admins: Vec<ActorId>,
    strategy_ids: Vec<ActorId>,
    games: HashMap<ActorId, Game>,
    messages_allowed: bool,
    dns_info: Option<(ActorId, String)>,
}

#[derive(Debug, Decode, Encode, TypeInfo)]
pub struct InitConfig {
    pub config: Config,
}

#[derive(Debug, Decode, Encode, TypeInfo, Clone)]
pub struct Config {
    pub gas_to_remove_game: u64,
    pub initial_speed: u32,
    pub min_speed: u32,
    pub max_speed: u32,
    pub gas_for_round: u64,
    pub time_interval: u32,
    pub max_distance: u32,
    pub time: u32,
    pub time_for_game_storage: u64,
    pub block_duration_ms: u64,
    pub gas_for_reply_deposit: u64,
}

#[derive(Debug, Decode, Encode, TypeInfo)]
pub enum Event {
    RoundInfo(RoundInfo),
    GameFinished{player: ActorId},
    Killed { inheritor: ActorId },
}
#[service(events = Event)]
impl CarRacesService {
    pub fn allow_messages(&mut self, messages_allowed: bool) {
        let msg_src = msg::source();
        assert!(self.data().admins.contains(&msg_src), "Not admin");
        self.data_mut().messages_allowed = messages_allowed;
    }

    pub async fn kill(&mut self, inheritor: ActorId) {
        let msg_src = msg::source();
        assert!(self.data().admins.contains(&msg_src), "Not admin");

        if let Some((id, _name)) = &self.data().dns_info {
            let request = ["Dns".encode(), "DeleteMe".to_string().encode(), ().encode()].concat();

            msg::send_bytes_with_gas_for_reply(*id, request, 5_000_000_000, 0, 0)
                .expect("Error in sending message")
                .await
                .expect("Error in `AddNewProgram`");
        }

        self.notify_on(Event::Killed { inheritor })
            .expect("Notification Error");
        exec::exit(inheritor);
    }

    pub fn add_strategy_ids(&mut self, car_ids: Vec<ActorId>) {
        let msg_src = msg::source();
        assert!(self.data().messages_allowed, "Message processing suspended");
        assert!(self.data().admins.contains(&msg_src), "Not admin");
        assert_eq!(car_ids.len(), 2, "Must be two strategies");
        self.data_mut().strategy_ids = car_ids;
    }

    pub fn start_game(&mut self, session_for_account: Option<ActorId>) {
        assert!(self.data().messages_allowed, "Message processing suspended");
        let msg_src = msg::source();
        let sessions = SessionStorage::get_session_map();
        let player = get_player(
            sessions,
            &msg_src,
            &session_for_account,
            ActionsForSession::StartGame,
        );
        let last_time_step = exec::block_timestamp();
        let strategy_ids = self.data().strategy_ids.clone();

        let game = if let Some(game) = self.data_mut().games.get_mut(&player) {
            assert!(game.state == GameState::Finished, "Game already started");
            game.current_round = 0;
            game.result = None;
            game.last_time_step = last_time_step;
            game
        } else {
            self.data_mut().games.entry(player).or_insert_with(|| Game {
                last_time_step,
                ..Default::default()
            })
        };

        game.car_ids = vec![player, strategy_ids[0], strategy_ids[1]];
        let initial_state = Car {
            position: 0,
            speed: config().initial_speed,
            car_actions: Vec::new(),
            round_result: None,
        };

        game.cars.insert(player, initial_state.clone());
        game.cars.insert(strategy_ids[0], initial_state.clone());
        game.cars.insert(strategy_ids[1], initial_state);

        game.state = GameState::PlayerAction;
    }

    pub async fn player_move(
        &mut self,
        strategy_move: StrategyAction,
        session_for_account: Option<ActorId>,
    ) {
        assert!(self.data().messages_allowed, "Message processing suspended");
        let msg_src = msg::source();
        let sessions = SessionStorage::get_session_map();

        let player = get_player(
            sessions,
            &msg_src,
            &session_for_account,
            ActionsForSession::Move,
        );
        let game_instance = self.get_game(&player);


        panicking(game_instance.verify_game_state());

        game_instance.apply_strategy_move(strategy_move);

        game_instance.state = GameState::Race;
        game_instance.last_time_step = exec::block_timestamp();

        let num_of_cars = game_instance.car_ids.len() as u8;

        game_instance.current_turn = (game_instance.current_turn + 1) % num_of_cars;

        let mut round_info: Option<RoundInfo> = None;
        let mut game_finished = false;
        while !game_instance.is_player_action_or_finished() {
            panicking(game_instance.process_car_turn().await);
            if game_instance.current_turn == 0 {
                game_instance.state = GameState::PlayerAction;
                game_instance.current_round = game_instance.current_round.saturating_add(1);

                game_instance.update_positions();

                round_info = Some(create_round_info(game_instance));

                if game_instance.state == GameState::Finished {
                    send_msg_to_remove_game_instance(player);
                    game_finished = true;
                }
            }
        }

        match round_info {
            Some(info) => {
                self.notify_on(Event::RoundInfo(info)).expect("Notification Error");
                if game_finished {
                    self.notify_on(Event::GameFinished{player: msg_src}).expect("Notification Error");
                }
            },
            None => {
                panic(Error::UnexpectedState);
            },
        }
    }

    pub fn remove_game_instance(&mut self, account: ActorId) {
        assert_eq!(msg::source(), exec::program_id(), "Not program");

        let game = self
            .data()
            .games
            .get(&account)
            .expect("Unexpected: the game does not exist");

        if game.state == GameState::Finished {
            self.data_mut().games.remove(&account);
        };
    }

    pub fn remove_instances(&mut self, player_ids: Option<Vec<ActorId>>) {
        let msg_src = msg::source();
        assert!(self.data().admins.contains(&msg_src), "Not admin");
        match player_ids {
            Some(player_ids) => {
                for player_id in player_ids {
                    self.data_mut().games.remove(&player_id);
                }
            }
            None => {
                self.data_mut().games.retain(|_, game| {
                    (exec::block_timestamp() - game.last_time_step) < config().time_for_game_storage
                });
            }
        }
    }

    pub fn add_admin(&mut self, admin: ActorId) {
        let msg_src = msg::source();
        assert!(self.data().admins.contains(&msg_src), "Not admin");
        self.data_mut().admins.push(admin);
    }

    pub fn remove_admin(&mut self, admin: ActorId) {
        let msg_src = msg::source();
        assert!(self.data().admins.contains(&msg_src), "Not admin");
        self.data_mut().admins.retain(|id| *id != admin);
    }

    pub fn update_config(&mut self, config: Config) {
        let msg_src = msg::source();
        assert!(self.data().admins.contains(&msg_src), "Not admin");

        unsafe {
            CONFIG = Some(config);
        }
    }

    pub fn admins(&self) -> Vec<ActorId> {
        self.data().admins.clone()
    }

    pub fn strategy_ids(&self) -> Vec<ActorId> {
        self.data().strategy_ids.clone()
    }

    pub fn game(&self, account_id: ActorId) -> Option<Game> {
        self.data().games.get(&account_id).cloned()
    }

    pub fn all_games(&self) -> Vec<(ActorId, Game)> {
        self.data().games.clone().into_iter().collect()
    }

    pub fn config_state(&self) -> Config {
        config().clone()
    }

    pub fn messages_allowed(&self) -> bool {
        self.data().messages_allowed
    }

    fn get_game(&mut self, account: &ActorId) -> &mut Game {
        self.data_mut()
            .games
            .get_mut(account)
            .expect("Game does not exist")
    }

    pub fn dns_info(&self) -> Option<(ActorId, String)> {
        self.data().dns_info.clone()
    }
}

impl CarRacesService {
    pub async fn init(config: InitConfig, dns_id_and_name: Option<(ActorId, String)>) {
        unsafe {
            DATA = Some(ContractData {
                admins: vec![msg::source()],
                games: HashMap::with_capacity(20_000),
                dns_info: dns_id_and_name.clone(),
                ..Default::default()
            });
            CONFIG = Some(config.config);
        }
        if let Some((id, name)) = dns_id_and_name {
            let request = [
                "Dns".encode(),
                "AddNewProgram".to_string().encode(),
                (name, exec::program_id()).encode(),
            ]
            .concat();

            msg::send_bytes_with_gas_for_reply(id, request, 5_000_000_000, 0, 0)
                .expect("Error in sending message")
                .await
                .expect("Error in `AddNewProgram`");
        }
    }
    pub fn new() -> Self {
        Self
    }

    fn data(&self) -> &ContractData {
        unsafe {
            DATA.as_ref()
                .expect("CarRacesService::seed() should be called")
        }
    }

    fn data_mut(&mut self) -> &mut ContractData {
        unsafe {
            DATA.as_mut()
                .expect("CarRacesService::seed() should be called")
        }
    }
}

fn create_round_info(game: &Game) -> RoundInfo {
    let mut cars = Vec::new();
    for (car_id, info) in game.cars.clone().iter() {
        cars.push((*car_id, info.position, info.round_result.clone()));
    }
    RoundInfo {
        cars,
        result: game.result.clone(),
    }
}

fn send_msg_to_remove_game_instance(player: ActorId) {
    let payload_bytes = [
        "CarRacesService".encode(),
        "RemoveGameInstance".encode(),
        player.encode(),
    ]
    .concat();

    msg::send_bytes_with_gas_delayed(
        exec::program_id(),
        payload_bytes,
        config().gas_to_remove_game,
        0,
        config().time_interval,
    )
    .expect("Error in sending message");
}

fn get_player(
    session_map: &HashMap<ActorId, SessionData>,
    msg_source: &ActorId,
    session_for_account: &Option<ActorId>,
    actions_for_session: ActionsForSession,
) -> ActorId {
    let player = match session_for_account {
        Some(account) => {
            let session = session_map
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

pub fn config() -> &'static Config {
    unsafe {
        CONFIG
            .as_ref()
            .expect("CarRacesService::seed() should be called")
    }
}
