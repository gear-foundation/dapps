use super::session::Storage as SessionStorage;
use crate::services;
use gstd::msg;
use sails_rs::{collections::HashMap, gstd::service, prelude::*};
mod funcs;
pub mod utils;
use utils::*;

#[derive(Default)]
pub struct Storage {
    admins: Vec<ActorId>,
    current_games: HashMap<ActorId, GameInstance>,
    config: Config,
    messages_allowed: bool,
}

impl Storage {
    pub fn get_config() -> &'static Config {
        unsafe { &STORAGE.as_ref().expect("Storage is not initialized").config }
    }
}

static mut STORAGE: Option<Storage> = None;

#[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum Event {
    GameFinished {
        game: GameInstance,
        player_address: ActorId,
    },
    GameStarted {
        game: GameInstance,
    },
    MoveMade {
        game: GameInstance,
    },
    GameInstanceRemoved,
    ConfigUpdated,
    AdminRemoved,
    AdminAdded,
    StatusMessagesUpdated,
}

#[derive(Clone)]
pub struct GameService(());

impl GameService {
    pub fn init(config: Config) -> Self {
        unsafe {
            STORAGE = Some(Storage {
                admins: vec![msg::source()],
                current_games: HashMap::with_capacity(10_000),
                config,
                messages_allowed: true,
            });
        }
        Self(())
    }
    pub fn get_mut(&mut self) -> &'static mut Storage {
        unsafe { STORAGE.as_mut().expect("Storage is not initialized") }
    }
    pub fn get(&self) -> &'static Storage {
        unsafe { STORAGE.as_ref().expect("Storage is not initialized") }
    }
}

#[service(events = Event)]
impl GameService {
    pub fn new() -> Self {
        Self(())
    }
    pub fn start_game(&mut self, session_for_account: Option<ActorId>) {
        let storage = self.get_mut();
        let sessions = SessionStorage::get_session_map();
        let event = services::utils::panicking(|| {
            funcs::start_game(storage, sessions, msg::source(), session_for_account)
        });
        self.notify_on(event.clone()).expect("Notification Error");
    }
    pub fn turn(&mut self, step: u8, session_for_account: Option<ActorId>) {
        let storage = self.get_mut();
        let sessions = SessionStorage::get_session_map();
        let event = services::utils::panicking(|| {
            funcs::turn(storage, sessions, msg::source(), step, session_for_account)
        });
        self.notify_on(event.clone()).expect("Notification Error");
    }
    pub fn skip(&mut self, session_for_account: Option<ActorId>) {
        let storage = self.get_mut();
        let sessions = SessionStorage::get_session_map();
        let event = services::utils::panicking(|| {
            funcs::skip(storage, sessions, msg::source(), session_for_account)
        });
        self.notify_on(event.clone()).expect("Notification Error");
    }
    pub fn remove_game_instance(&mut self, account: ActorId) {
        let storage = self.get_mut();
        let event = services::utils::panicking(|| {
            funcs::remove_game_instance(storage, msg::source(), account)
        });
        self.notify_on(event.clone()).expect("Notification Error");
    }
    pub fn remove_game_instances(&mut self, accounts: Option<Vec<ActorId>>) {
        let storage = self.get_mut();
        let event = services::utils::panicking(|| {
            funcs::remove_game_instances(storage, msg::source(), accounts)
        });
        self.notify_on(event.clone()).expect("Notification Error");
    }
    pub fn add_admin(&mut self, admin: ActorId) {
        let storage = self.get_mut();
        let event = services::utils::panicking(|| funcs::add_admin(storage, msg::source(), admin));
        self.notify_on(event.clone()).expect("Notification Error");
    }
    pub fn remove_admin(&mut self, admin: ActorId) {
        let storage = self.get_mut();
        let event =
            services::utils::panicking(|| funcs::remove_admin(storage, msg::source(), admin));
        self.notify_on(event.clone()).expect("Notification Error");
    }
    pub fn update_config(
        &mut self,
        s_per_block: Option<u64>,
        gas_to_remove_game: Option<u64>,
        time_interval: Option<u32>,
        turn_deadline_ms: Option<u64>,
        gas_to_delete_session: Option<u64>,
    ) {
        let storage = self.get_mut();
        let event = services::utils::panicking(|| {
            funcs::update_config(
                storage,
                msg::source(),
                s_per_block,
                gas_to_remove_game,
                time_interval,
                turn_deadline_ms,
                gas_to_delete_session,
            )
        });
        self.notify_on(event.clone()).expect("Notification Error");
    }
    pub fn allow_messages(&mut self, messages_allowed: bool) {
        let storage = self.get_mut();
        let event = services::utils::panicking(|| {
            funcs::allow_messages(storage, msg::source(), messages_allowed)
        });
        self.notify_on(event.clone()).expect("Notification Error");
    }

    pub fn admins(&self) -> &'static Vec<ActorId> {
        &self.get().admins
    }
    pub fn game(&self, player_id: ActorId) -> Option<GameInstance> {
        self.get().current_games.get(&player_id).cloned()
    }
    pub fn all_games(&self) -> Vec<(ActorId, GameInstance)> {
        self.get().current_games.clone().into_iter().collect()
    }
    pub fn config(&self) -> &'static Config {
        &self.get().config
    }
    pub fn messages_allowed(&self) -> &'static bool {
        &self.get().messages_allowed
    }
}
