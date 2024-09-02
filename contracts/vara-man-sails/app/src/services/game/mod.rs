use crate::services;
use gstd::{collections::HashMap, exec, msg, String};
use sails_rs::{gstd::service, prelude::*};
mod funcs;
pub mod utils;
use utils::*;

#[derive(Debug, Default, Clone)]
pub struct Storage {
    tournaments: HashMap<ActorId, Tournament>,
    players_to_game_id: HashMap<ActorId, ActorId>,
    status: Status,
    config: Config,
    admins: Vec<ActorId>,
    dns_info: Option<(ActorId, String)>,
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

static mut STORAGE: Option<Storage> = None;

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
    pub async fn init(config: Config, dns_id_and_name: Option<(ActorId, String)>) -> Self {
        unsafe {
            STORAGE = Some(Storage {
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
    pub fn get_mut(&mut self) -> &'static mut Storage {
        unsafe { STORAGE.as_mut().expect("Storage is not initialized") }
    }
    pub fn get(&self) -> &'static Storage {
        unsafe { STORAGE.as_ref().expect("Storage is not initialized") }
    }
}

#[service(events = Event)]
impl Service {
    pub fn new() -> Self {
        Self(())
    }

    pub fn create_new_tournament(
        &mut self,
        tournament_name: String,
        name: String,
        level: Level,
        duration_ms: u32,
    ) {
        let storage = self.get_mut();
        let event = services::utils::panicking(|| {
            funcs::create_new_tournament(storage, tournament_name, name, level, duration_ms)
        });
        self.notify_on(event.clone()).expect("Notification Error");
    }

    pub fn register_for_tournament(&mut self, admin_id: ActorId, name: String) {
        let storage = self.get_mut();
        let event =
            services::utils::panicking(|| funcs::register_for_tournament(storage, admin_id, name));
        self.notify_on(event.clone()).expect("Notification Error");
    }

    pub fn cancel_register(&mut self) {
        let storage = self.get_mut();
        let event = services::utils::panicking(|| funcs::cancel_register(storage));
        self.notify_on(event.clone()).expect("Notification Error");
    }

    pub fn cancel_tournament(&mut self) {
        let storage = self.get_mut();
        let event = services::utils::panicking(|| funcs::cancel_tournament(storage));
        self.notify_on(event.clone()).expect("Notification Error");
    }

    pub fn delete_player(&mut self, player_id: ActorId) {
        let storage = self.get_mut();
        let event = services::utils::panicking(|| funcs::delete_player(storage, player_id));
        self.notify_on(event.clone()).expect("Notification Error");
    }

    pub async fn finish_single_game(&mut self, gold_coins: u16, silver_coins: u16, level: Level) {
        let storage = self.get_mut();
        let res = funcs::finish_single_game(storage, gold_coins, silver_coins, level).await;
        let event = match res {
            Ok(v) => v,
            Err(e) => services::utils::panic(e),
        };

        self.notify_on(event.clone()).expect("Notification Error");
    }

    pub fn start_tournament(&mut self) {
        let storage = self.get_mut();
        let event = services::utils::panicking(|| funcs::start_tournament(storage));
        self.notify_on(event.clone()).expect("Notification Error");
    }

    pub fn finish_tournament(&mut self, admin_id: ActorId, time_start: u64) {
        let storage = self.get_mut();
        let event =
            services::utils::panicking(|| funcs::finish_tournament(storage, admin_id, time_start));
        self.notify_on(event.clone()).expect("Notification Error");
    }

    pub fn record_tournament_result(&mut self, time: u128, gold_coins: u16, silver_coins: u16) {
        let storage = self.get_mut();
        let event = services::utils::panicking(|| {
            funcs::record_tournament_result(storage, time, gold_coins, silver_coins)
        });
        self.notify_on(event.clone()).expect("Notification Error");
    }

    pub fn leave_game(&mut self) {
        let storage = self.get_mut();
        let event = services::utils::panicking(|| funcs::leave_game(storage));
        self.notify_on(event.clone()).expect("Notification Error");
    }

    pub fn change_status(&mut self, status: Status) {
        let storage = self.get_mut();
        let event = services::utils::panicking(|| funcs::change_status(storage, status));
        self.notify_on(event.clone()).expect("Notification Error");
    }

    pub fn change_config(&mut self, config: Config) {
        let storage = self.get_mut();
        let event = services::utils::panicking(|| funcs::change_config(storage, config));
        self.notify_on(event.clone()).expect("Notification Error");
    }

    pub fn add_admin(&mut self, new_admin_id: ActorId) {
        let storage = self.get_mut();
        let event = services::utils::panicking(|| funcs::add_admin(storage, new_admin_id));
        self.notify_on(event.clone()).expect("Notification Error");
    }

    pub async fn kill(&mut self, inheritor: ActorId) {
        let storage = self.get();
        if let Some((id, _name)) = &storage.dns_info {
            let request = ["Dns".encode(), "DeleteMe".to_string().encode(), ().encode()].concat();

            msg::send_bytes_with_gas_for_reply(*id, request, 5_000_000_000, 0, 0)
                .expect("Error in sending message")
                .await
                .expect("Error in `AddNewProgram`");
        }

        if !storage.admins.contains(&msg::source()) {
            services::utils::panic(GameError::NotAdmin);
        }

        self.notify_on(Event::Killed { inheritor })
            .expect("Notification Error");
        exec::exit(inheritor);
    }

    pub fn config(&self) -> &'static Config {
        &self.get().config
    }

    pub fn admins(&self) -> &'static Vec<ActorId> {
        &self.get().admins
    }

    pub fn status(&self) -> &'static Status {
        &self.get().status
    }

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

    pub fn all(&self) -> VaraManState {
        (*self.get()).clone().into()
    }
}
