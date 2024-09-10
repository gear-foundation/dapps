use super::session::Storage as SessionStorage;
use crate::services::session::utils::{ActionsForSession, SessionData};
use collections::HashMap;
use sails_rs::{gstd::ExecContext, prelude::*};
pub mod error;
pub mod game;
pub mod utils;
use crate::event_or_panic_async;
use error::Error;
use game::*;
pub struct CarRacesService<ExecContext> {
    exec_context: ExecContext,
}

use gstd::{exec, msg};
static mut DATA: Option<ContractData> = None;
static mut CONFIG: Option<Config> = None;

#[derive(Debug, Default)]
pub struct ContractData {
    admins: Vec<ActorId>,
    strategy_ids: Vec<ActorId>,
    games: HashMap<ActorId, Game>,
    messages_allowed: bool,
}

#[derive(Debug, Decode, Encode, TypeInfo)]
pub struct InitConfig {
    pub config: Config,
}

#[derive(Debug, Decode, Encode, TypeInfo, Clone)]
pub struct Config {
    pub gas_to_remove_game: u64,
    pub gas_to_delete_session: u64,
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
    pub minimum_session_duration_ms: u64,
    pub s_per_block: u64,
}

#[derive(Debug, Decode, Encode, TypeInfo)]
pub enum Event {
    RoundInfo(RoundInfo),
}
#[service(events = Event)]
impl<T> CarRacesService<T>
where
    T: ExecContext,
{
    pub fn allow_messages(&mut self, messages_allowed: bool) {
        let msg_src = self.exec_context.actor_id();
        assert!(self.data().admins.contains(&msg_src), "Not admin");
        self.data_mut().messages_allowed = messages_allowed;
    }

    pub fn add_strategy_ids(&mut self, car_ids: Vec<ActorId>) {
        let msg_src = self.exec_context.actor_id();
        assert!(self.data().messages_allowed, "Message processing suspended");
        assert!(self.data().admins.contains(&msg_src), "Not admin");
        assert_eq!(car_ids.len(), 2, "Must be two strategies");
        self.data_mut().strategy_ids = car_ids;
    }

    pub fn start_game(&mut self, session_for_account: Option<ActorId>) {
        let msg_src = self.exec_context.actor_id();
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
            assert!(game.state != GameState::Finished, "Game already started");
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
        let msg_src = self.exec_context.actor_id();
        let sessions = SessionStorage::get_session_map();

        let player = get_player(
            sessions,
            &msg_src,
            &session_for_account,
            ActionsForSession::Move,
        );
        let game = self.get_game(&player);

        event_or_panic_async!(self, || async move {
            game.verify_game_state()?;

            game.apply_strategy_move(strategy_move);

            game.state = GameState::Race;
            game.last_time_step = exec::block_timestamp();

            let num_of_cars = game.car_ids.len() as u8;

            game.current_turn = (game.current_turn + 1) % num_of_cars;

            let mut round_info: Option<RoundInfo> = None;

            while !game.is_player_action_or_finished() {
                game.process_car_turn().await?;
                if game.current_turn == 0 {
                    game.state = GameState::PlayerAction;
                    game.current_round = game.current_round.saturating_add(1);

                    round_info = Some(create_round_info(&game));

                    game.update_positions();

                    if game.state == GameState::Finished {
                        send_msg_to_remove_game_instance(player);
                    }
                }
            }

            match round_info {
                Some(info) => Ok(Event::RoundInfo(info)),
                None => Err(Error::UnexpectedState),
            }
        })
    }

    pub fn remove_game_instance(&mut self, account: ActorId) {
        assert_eq!(
            self.exec_context.actor_id(),
            exec::program_id(),
            "Not program"
        );

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
        let msg_src = self.exec_context.actor_id();
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
        let msg_src = self.exec_context.actor_id();
        assert!(self.data().admins.contains(&msg_src), "Not admin");
        self.data_mut().admins.push(admin);
    }

    pub fn remove_admin(&mut self, admin: ActorId) {
        let msg_src = self.exec_context.actor_id();
        assert!(self.data().admins.contains(&msg_src), "Not admin");
        self.data_mut().admins.retain(|id| *id != admin);
    }

    pub fn update_config(&mut self, config: Config) {
        let msg_src = self.exec_context.actor_id();
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
}

impl<T> CarRacesService<T>
where
    T: ExecContext,
{
    pub fn seed(config: InitConfig, exec_context: T) {
        unsafe {
            DATA = Some(ContractData {
                admins: vec![exec_context.actor_id()],
                games: HashMap::with_capacity(20_000),
                ..Default::default()
            });
            CONFIG = Some(config.config);
        }
    }
    pub fn new(exec_context: T) -> Self {
        Self { exec_context }
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
