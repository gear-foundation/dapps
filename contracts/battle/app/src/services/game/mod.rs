#![allow(clippy::too_many_arguments)]
#![allow(clippy::new_without_default)]
#![allow(static_mut_refs)]
use crate::services;
use sails_rs::{
    collections::{HashMap, HashSet},
    gstd::{msg, service},
    prelude::*,
};
mod funcs;
pub mod utils;
use utils::{Config, *};

#[derive(Debug, Default, Clone)]
struct Storage {
    battles: HashMap<ActorId, Battle>,
    players_to_battle_id: HashMap<ActorId, ActorId>,
    admins: HashSet<ActorId>,
    config: Config,
}

static mut STORAGE: Option<Storage> = None;

#[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum Event {
    NewBattleCreated {
        battle_id: ActorId,
        bid: u128,
    },
    PlayerRegistered {
        admin_id: ActorId,
        user_name: String,
        bid: u128,
    },
    RegisterCanceled {
        player_id: ActorId,
    },
    BattleCanceled {
        game_id: ActorId,
    },
    BattleStarted,
    MoveMade,
    BattleFinished {
        winner: ActorId,
    },
    PairChecked {
        game_id: ActorId,
        pair_id: u8,
        round: u8,
    },
    FirstRoundChecked {
        game_id: ActorId,
        wave: u8,
    },
    NextBattleStarted,
    EnemyWaiting,
    WarriorGenerated {
        address: ActorId,
    },
    AdminAdded {
        new_admin: ActorId,
    },
    ConfigChanged {
        config: Config,
    },
    GameLeft,
    RoundAction {
        round: u8,
        player_1: (ActorId, Move, u16),
        player_2: (ActorId, Move, u16),
    },
    AutomaticMoveMade,
}

#[derive(Clone)]
pub struct BattleService(());

impl BattleService {
    pub fn init(config: Config) -> Self {
        unsafe {
            STORAGE = Some(Storage {
                admins: HashSet::from([msg::source()]),
                config,
                ..Default::default()
            });
        }
        Self(())
    }
    fn get_mut(&mut self) -> &'static mut Storage {
        unsafe { STORAGE.as_mut().expect("Storage is not initialized") }
    }
    fn get(&self) -> &'static Storage {
        unsafe { STORAGE.as_ref().expect("Storage is not initialized") }
    }
}

#[service(events = Event)]
impl BattleService {
    pub fn new() -> Self {
        Self(())
    }

    pub async fn create_new_battle(
        &mut self,
        battle_name: String,
        user_name: String,
        warrior_id: Option<ActorId>,
        appearance: Option<Appearance>,
        attack: u16,
        defence: u16,
        dodge: u16,
    ) {
        let storage = self.get_mut();
        let res = funcs::create_new_battle(
            storage,
            warrior_id,
            appearance,
            battle_name,
            user_name,
            attack,
            defence,
            dodge,
        )
        .await;
        let event = match res {
            Ok(v) => v,
            Err(e) => services::utils::panic(e),
        };
        self.notify_on(event.clone()).expect("Notification Error");
    }
    pub async fn register(
        &mut self,
        game_id: ActorId,
        warrior_id: Option<ActorId>,
        appearance: Option<Appearance>,
        user_name: String,
        attack: u16,
        defence: u16,
        dodge: u16,
    ) {
        let storage = self.get_mut();
        let res = funcs::battle_registration(
            storage, game_id, warrior_id, appearance, user_name, attack, defence, dodge,
        )
        .await;
        let event = match res {
            Ok(v) => v,
            Err(e) => services::utils::panic(e),
        };
        self.notify_on(event.clone()).expect("Notification Error");
    }
    pub fn cancel_register(&mut self) {
        let storage = self.get_mut();
        let event = services::utils::panicking(|| funcs::cancel_register(storage));
        self.notify_on(event.clone()).expect("Notification Error");
    }
    pub fn delete_player(&mut self, player_id: ActorId) {
        let storage = self.get_mut();
        let event = services::utils::panicking(|| funcs::delete_player(storage, player_id));
        self.notify_on(event.clone()).expect("Notification Error");
    }
    pub fn cancel_tournament(&mut self) {
        let storage = self.get_mut();
        let event = services::utils::panicking(|| funcs::cancel_tournament(storage));
        self.notify_on(event.clone()).expect("Notification Error");
    }
    pub fn delayed_cancel_tournament(&mut self, game_id: ActorId, time_creation: u64) {
        let storage = self.get_mut();
        let event = services::utils::panicking(|| {
            funcs::delayed_cancel_tournament(storage, game_id, time_creation)
        });
        self.notify_on(event.clone()).expect("Notification Error");
    }
    pub fn start_battle(&mut self) {
        let storage = self.get_mut();
        let event = services::utils::panicking(|| funcs::start_battle(storage));
        self.notify_on(event.clone()).expect("Notification Error");
    }
    pub fn make_move(&mut self, warrior_move: Move) {
        let storage = self.get_mut();
        let event = services::utils::panicking(|| funcs::make_move(storage, warrior_move));
        self.notify_on(event.clone()).expect("Notification Error");
    }
    pub fn automatic_move(&mut self, player_id: ActorId, number_of_victories: u8, round: u8) {
        let storage = self.get_mut();
        let event = services::utils::panicking(|| {
            funcs::automatic_move(storage, player_id, number_of_victories, round)
        });
        self.notify_on(event.clone()).expect("Notification Error");
    }
    pub fn start_next_fight(&mut self) {
        let storage = self.get_mut();
        let event = services::utils::panicking(|| funcs::start_next_fight(storage));
        self.notify_on(event.clone()).expect("Notification Error");
    }
    pub fn exit_game(&mut self) {
        let storage = self.get_mut();
        let event = services::utils::panicking(|| funcs::exit_game(storage));
        self.notify_on(event.clone()).expect("Notification Error");
    }
    pub fn add_admin(&mut self, new_admin: ActorId) {
        let storage = self.get_mut();
        if !storage.admins.contains(&msg::source()) {
            services::utils::panic(BattleError::AccessDenied);
        }
        storage.admins.insert(new_admin);
        self.notify_on(Event::AdminAdded { new_admin })
            .expect("Notification Error");
    }
    pub fn change_config(&mut self, config: Config) {
        let storage = self.get_mut();
        if !storage.admins.contains(&msg::source()) {
            services::utils::panic(BattleError::AccessDenied);
        }
        storage.config = config.clone();
        self.notify_on(Event::ConfigChanged { config })
            .expect("Notification Error");
    }

    pub fn get_battle(&self, game_id: ActorId) -> Option<BattleState> {
        let storage = self.get();
        storage
            .battles
            .get(&game_id)
            .cloned()
            .map(|battle| battle.into())
    }
    pub fn get_my_battle(&self) -> Option<BattleState> {
        let storage = self.get();
        if let Some(game_id) = storage.players_to_battle_id.get(&msg::source()) {
            storage
                .battles
                .get(game_id)
                .cloned()
                .map(|battle| battle.into())
        } else {
            None
        }
    }
    pub fn admins(&self) -> Vec<ActorId> {
        let storage = self.get();
        storage.admins.clone().into_iter().collect()
    }
    pub fn config(&self) -> &'static Config {
        let storage = self.get();
        &storage.config
    }
}
