#![allow(static_mut_refs)]
use super::session::Storage as SessionStorage;
use crate::services;
use sails_rs::{
    collections::HashMap,
    gstd::{exec, msg, service},
    prelude::*,
};
mod funcs;
pub mod utils;
use utils::*;

#[derive(Debug, Default, Clone)]
pub struct GameStorage {
    tournaments: HashMap<ActorId, Tournament>,
    players_to_game_id: HashMap<ActorId, ActorId>,
    status: Status,
    config: Config,
    admins: Vec<ActorId>,
    dns_info: Option<(ActorId, String)>,
}

impl GameStorage {
    pub fn get_config() -> &'static Config {
        unsafe {
            &STORAGE
                .as_ref()
                .expect("GameStorage is not initialized")
                .config
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct Tournament {
    tournament_name: String,
    admin: ActorId,
    level: Level,
    participants: HashMap<ActorId, Player>,
    bid: u128,
    stage: Stage,
    duration_ms: u32,
}

static mut STORAGE: Option<GameStorage> = None;

#[event]
#[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum Event {
    GameFinished {
        winners: Vec<ActorId>,
        participants: Vec<(ActorId, Player)>,
        prize: u128,
    },
    SingleGameFinished {
        gold_coins: u16,
        silver_coins: u16,
        points: u128,
        maximum_possible_points: u128,
        maximum_number_gold_coins: u16,
        maximum_number_silver_coins: u16,
        prize: u128,
        player_address: ActorId,
    },
    NewTournamentCreated {
        tournament_name: String,
        name: String,
        level: Level,
        bid: u128,
    },
    PlayerRegistered {
        admin_id: ActorId,
        name: String,
        bid: u128,
    },
    RegisterCanceled,
    TournamentCanceled {
        admin_id: ActorId,
    },
    PlayerDeleted {
        player_id: ActorId,
    },
    ResultTournamentRecorded {
        gold_coins: u16,
        silver_coins: u16,
        time: u128,
        points: u128,
        maximum_possible_points: u128,
        maximum_number_gold_coins: u16,
        maximum_number_silver_coins: u16,
        player_address: ActorId,
    },
    GameStarted,
    AdminAdded(ActorId),
    StatusChanged(Status),
    ConfigChanged(Config),
    LeftGame,
    Killed {
        inheritor: ActorId,
    },
}

#[derive(Clone)]
pub struct Service(());

impl Service {
    pub fn new() -> Self {
        Self(())
    }
    pub async fn init(config: Config, dns_id_and_name: Option<(ActorId, String)>) -> Self {
        unsafe {
            STORAGE = Some(GameStorage {
                config,
                admins: vec![msg::source()],
                dns_info: dns_id_and_name.clone(),
                ..Default::default()
            });
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

        Self(())
    }
    pub fn get_mut(&mut self) -> &'static mut GameStorage {
        unsafe { STORAGE.as_mut().expect("GameStorage is not initialized") }
    }
    pub fn get(&self) -> &'static GameStorage {
        unsafe { STORAGE.as_ref().expect("GameStorage is not initialized") }
    }
}

#[service(events = Event)]
impl Service {
    #[export]
    pub fn create_new_tournament(
        &mut self,
        tournament_name: String,
        name: String,
        level: Level,
        duration_ms: u32,
        session_for_account: Option<ActorId>,
    ) {
        let storage = self.get_mut();
        let sessions = SessionStorage::get_session_map();
        let event = services::utils::panicking(|| {
            funcs::create_new_tournament(
                storage,
                sessions,
                tournament_name,
                name,
                level,
                duration_ms,
                session_for_account,
            )
        });
        self.emit_event(event.clone()).expect("Notification Error");
    }

    #[export]
    pub fn register_for_tournament(
        &mut self,
        admin_id: ActorId,
        name: String,
        session_for_account: Option<ActorId>,
    ) {
        let storage = self.get_mut();
        let sessions = SessionStorage::get_session_map();
        let event = services::utils::panicking(|| {
            funcs::register_for_tournament(storage, sessions, admin_id, name, session_for_account)
        });
        self.emit_event(event.clone()).expect("Notification Error");
    }

    #[export]
    pub fn cancel_register(&mut self, session_for_account: Option<ActorId>) {
        let storage = self.get_mut();
        let sessions = SessionStorage::get_session_map();
        let event = services::utils::panicking(|| {
            funcs::cancel_register(storage, sessions, session_for_account)
        });
        self.emit_event(event.clone()).expect("Notification Error");
    }

    #[export]
    pub fn cancel_tournament(&mut self, session_for_account: Option<ActorId>) {
        let storage = self.get_mut();
        let sessions = SessionStorage::get_session_map();
        let event = services::utils::panicking(|| {
            funcs::cancel_tournament(storage, sessions, session_for_account)
        });
        self.emit_event(event.clone()).expect("Notification Error");
    }

    #[export]
    pub fn delete_player(&mut self, player_id: ActorId, session_for_account: Option<ActorId>) {
        let storage = self.get_mut();
        let sessions = SessionStorage::get_session_map();
        let event = services::utils::panicking(|| {
            funcs::delete_player(storage, sessions, player_id, session_for_account)
        });
        self.emit_event(event.clone()).expect("Notification Error");
    }

    #[export]
    pub async fn finish_single_game(
        &mut self,
        gold_coins: u16,
        silver_coins: u16,
        level: Level,
        session_for_account: Option<ActorId>,
    ) {
        let storage = self.get_mut();
        let sessions = SessionStorage::get_session_map();
        let res = funcs::finish_single_game(
            storage,
            sessions,
            gold_coins,
            silver_coins,
            level,
            session_for_account,
        )
        .await;
        let event = match res {
            Ok(v) => v,
            Err(e) => services::utils::panic(e),
        };

        self.emit_event(event.clone()).expect("Notification Error");
    }

    #[export]
    pub fn start_tournament(&mut self, session_for_account: Option<ActorId>) {
        let storage = self.get_mut();
        let sessions = SessionStorage::get_session_map();
        let event = services::utils::panicking(|| {
            funcs::start_tournament(storage, sessions, session_for_account)
        });
        self.emit_event(event.clone()).expect("Notification Error");
    }

    #[export]
    pub fn finish_tournament(&mut self, admin_id: ActorId, time_start: u64) {
        let storage = self.get_mut();
        let event =
            services::utils::panicking(|| funcs::finish_tournament(storage, admin_id, time_start));
        self.emit_event(event.clone()).expect("Notification Error");
    }

    #[export]
    pub fn record_tournament_result(
        &mut self,
        time: u128,
        gold_coins: u16,
        silver_coins: u16,
        session_for_account: Option<ActorId>,
    ) {
        let storage = self.get_mut();
        let sessions = SessionStorage::get_session_map();
        let event = services::utils::panicking(|| {
            funcs::record_tournament_result(
                storage,
                sessions,
                time,
                gold_coins,
                silver_coins,
                session_for_account,
            )
        });
        self.emit_event(event.clone()).expect("Notification Error");
    }

    #[export]
    pub fn leave_game(&mut self, session_for_account: Option<ActorId>) {
        let storage = self.get_mut();
        let sessions = SessionStorage::get_session_map();
        let event = services::utils::panicking(|| {
            funcs::leave_game(storage, sessions, session_for_account)
        });
        self.emit_event(event.clone()).expect("Notification Error");
    }

    #[export]
    pub fn change_status(&mut self, status: Status) {
        let storage = self.get_mut();
        let event = services::utils::panicking(|| funcs::change_status(storage, status));
        self.emit_event(event.clone()).expect("Notification Error");
    }

    #[export]
    pub fn change_config(&mut self, config: Config) {
        let storage = self.get_mut();
        let event = services::utils::panicking(|| funcs::change_config(storage, config));
        self.emit_event(event.clone()).expect("Notification Error");
    }

    #[export]
    pub fn add_admin(&mut self, new_admin_id: ActorId) {
        let storage = self.get_mut();
        let event = services::utils::panicking(|| funcs::add_admin(storage, new_admin_id));
        self.emit_event(event.clone()).expect("Notification Error");
    }

    #[export]
    pub async fn kill(&mut self, inheritor: ActorId) {
        let storage = self.get();
        if !storage.admins.contains(&msg::source()) {
            services::utils::panic(GameError::NotAdmin);
        }
        if let Some((id, _name)) = &storage.dns_info {
            let request = ["Dns".encode(), "DeleteMe".to_string().encode(), ().encode()].concat();

            msg::send_bytes_with_gas_for_reply(*id, request, 5_000_000_000, 0, 0)
                .expect("Error in sending message")
                .await
                .expect("Error in `AddNewProgram`");
        }

        self.emit_event(Event::Killed { inheritor })
            .expect("Notification Error");
        exec::exit(inheritor);
    }

    #[export]
    pub fn config(&self) -> &'static Config {
        &self.get().config
    }

    #[export]
    pub fn admins(&self) -> &'static Vec<ActorId> {
        &self.get().admins
    }

    #[export]
    pub fn status(&self) -> &'static Status {
        &self.get().status
    }

    #[export]
    pub fn get_tournament(&self, player_id: ActorId) -> Option<(TournamentState, Option<u64>)> {
        let storage = self.get();
        if let Some(admin_id) = storage.players_to_game_id.get(&player_id) {
            if let Some(tournament) = storage.tournaments.get(admin_id) {
                let tournament_state = TournamentState {
                    tournament_name: tournament.tournament_name.clone(),
                    admin: tournament.admin,
                    level: tournament.level,
                    participants: tournament.participants.clone().into_iter().collect(),
                    bid: tournament.bid,
                    stage: tournament.stage.clone(),
                    duration_ms: tournament.duration_ms,
                };
                let time = match tournament.stage {
                    Stage::Started(start_time) => Some(exec::block_timestamp() - start_time),
                    _ => None,
                };
                Some((tournament_state, time))
            } else {
                None
            }
        } else {
            None
        }
    }

    #[export]
    pub fn all(&self) -> VaraManState {
        (*self.get()).clone().into()
    }
}
