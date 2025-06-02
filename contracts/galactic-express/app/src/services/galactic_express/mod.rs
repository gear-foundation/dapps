#![allow(static_mut_refs)]

use crate::services;
use gstd::{exec, msg};
use sails_rs::{collections::HashMap, gstd::service, prelude::*};
mod funcs;
pub mod utils;
use utils::*;

#[derive(Default)]
pub struct Storage {
    games: HashMap<ActorId, Game>,
    player_to_game_id: HashMap<ActorId, ActorId>,
    dns_info: Option<(ActorId, String)>,
    admin: ActorId,
}

#[derive(Default)]
pub struct Game {
    admin: ActorId,
    admin_name: String,
    bid: u128,
    altitude: u16,
    weather: Weather,
    reward: u128,
    stage: Stage,
}

static mut STORAGE: Option<Storage> = None;

#[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum Event {
    GameFinished(Results),
    NewSessionCreated {
        altitude: u16,
        weather: Weather,
        reward: u128,
        bid: u128,
    },
    Registered(ActorId, Participant),
    RegistrationCanceled,
    PlayerDeleted {
        player_id: ActorId,
    },
    GameCanceled,
    GameLeft,
    AdminChanged {
        new_admin: ActorId,
    },
    Killed {
        inheritor: ActorId,
    },
}

#[derive(Clone)]
pub struct GameService(());

impl GameService {
    pub async fn init(dns_id_and_name: Option<(ActorId, String)>) -> Self {
        unsafe {
            STORAGE = Some(Storage {
                dns_info: dns_id_and_name.clone(),
                admin: msg::source(),
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
impl GameService {
    pub fn new() -> Self {
        Self(())
    }
    pub fn create_new_session(&mut self, name: String) {
        let storage = self.get_mut();
        let event = services::utils::panicking(|| funcs::create_new_session(storage, name));
        self.emit_event(event.clone()).expect("Notification Error");
    }
    pub fn cancel_game(&mut self) {
        let storage = self.get_mut();
        let event = services::utils::panicking(|| funcs::cancel_game(storage));
        self.emit_event(event.clone()).expect("Notification Error");
    }
    pub fn leave_game(&mut self) {
        let storage = self.get_mut();
        let event = services::utils::panicking(|| funcs::leave_game(storage));
        self.emit_event(event.clone()).expect("Notification Error");
    }
    pub fn register(&mut self, creator: ActorId, participant: Participant) {
        let storage = self.get_mut();
        let event = services::utils::panicking(|| funcs::register(storage, creator, participant));
        self.emit_event(event.clone()).expect("Notification Error");
    }
    pub fn cancel_register(&mut self) {
        let storage = self.get_mut();
        let event = services::utils::panicking(|| funcs::cancel_register(storage));
        self.emit_event(event.clone()).expect("Notification Error");
    }
    pub fn delete_player(&mut self, player_id: ActorId) {
        let storage = self.get_mut();
        let event = services::utils::panicking(|| funcs::delete_player(storage, player_id));
        self.emit_event(event.clone()).expect("Notification Error");
    }
    pub fn start_game(&mut self, fuel_amount: u8, payload_amount: u8) {
        let storage = self.get_mut();
        let event =
            services::utils::panicking(|| funcs::start_game(storage, fuel_amount, payload_amount));
        self.emit_event(event.clone()).expect("Notification Error");
    }
    pub fn change_admin(&mut self, new_admin: ActorId) {
        let storage = self.get_mut();
        let msg_source = msg::source();
        if storage.admin != msg_source {
            services::utils::panic(GameError::DeniedAccess);
        }
        storage.admin = new_admin;
        self.emit_event(Event::AdminChanged { new_admin })
            .expect("Notification Error");
    }
    pub async fn kill(&mut self, inheritor: ActorId) {
        let storage = self.get();
        if storage.admin != msg::source() {
            services::utils::panic(GameError::DeniedAccess);
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
    pub fn get_game(&self, player_id: ActorId) -> Option<GameState> {
        let storage = self.get();
        storage
            .player_to_game_id
            .get(&player_id)
            .and_then(|creator_id| storage.games.get(creator_id))
            .map(|game| {
                let stage = match &game.stage {
                    Stage::Registration(participants_data) => {
                        StageState::Registration(participants_data.clone().into_iter().collect())
                    }
                    Stage::Results(results) => StageState::Results(results.clone()),
                };

                GameState {
                    admin: game.admin,
                    admin_name: game.admin_name.clone(),
                    altitude: game.altitude,
                    weather: game.weather,
                    reward: game.reward,
                    stage,
                    bid: game.bid,
                }
            })
    }
    pub fn all(&self) -> State {
        self.get().into()
    }
    pub fn admin(&self) -> &'static ActorId {
        &self.get().admin
    }
    pub fn dns_info(&self) -> &'static Option<(ActorId, String)> {
        &self.get().dns_info
    }
}
