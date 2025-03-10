#![allow(static_mut_refs)]
use crate::services::game::game_actions::GameSessionActions;
use gstd::{exec, msg, ReservationId};
use sails_rs::{
    collections::{HashMap, HashSet},
    gstd::service,
    prelude::*,
};
mod funcs;
mod game_actions;
pub mod utils;
use crate::services;
pub use utils::*;

#[derive(Clone, Default)]
pub struct Storage {
    game_sessions: HashMap<AdminId, Game>,
    config: Config,
    players_to_sessions: HashMap<ActorId, AdminId>,
    dns_info: Option<(ActorId, String)>,
    admin: ActorId,
}

#[derive(Default, Clone, Debug)]
pub struct Game {
    admin_id: AdminId,
    properties_in_bank: HashSet<u8>,
    round: u128,
    // strategy ID to PlayerInfo
    players: HashMap<ActorId, PlayerInfo>,
    owners_to_strategy_ids: HashMap<ActorId, ActorId>,
    players_queue: Vec<ActorId>,
    current_turn: u8,
    current_player: ActorId,
    current_step: u64,
    // mapping from cells to built properties,
    properties: Vec<Option<(ActorId, Gears, Price, Rent)>>,
    // mapping from cells to accounts who have properties on it
    ownership: Vec<ActorId>,
    game_status: GameStatus,
    winner: ActorId,
    current_msg_id: MessageId,
    reservations: Vec<ReservationId>,
    entry_fee: Option<u128>,
    prize_pool: u128,
}

static mut STORAGE: Option<Storage> = None;

#[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum Event {
    /// Reply on `CreateGameSession` message
    GameSessionCreated {
        admin_id: AdminId,
    },

    /// Reply on `MakeReservation` message
    ReservationMade,

    /// Reply on `Register` message
    StrategyRegistered,

    /// Reply on `Play` message
    /// in case of successful completion of the game
    GameFinished {
        admin_id: AdminId,
        winner: ActorId,
        participants: Vec<ActorId>,
    },

    /// Reply on `AddGasToPlayerStrategy`
    GasForPlayerStrategyAdded,

    /// Reply on `CancelGame`
    GameWasCancelled,

    /// Reply on `ExitGame`
    PlayerLeftGame,

    /// Event for the front-end app
    Step {
        players: Vec<(ActorId, PlayerInfoState)>,
        properties: Vec<Option<(ActorId, Gears, Price, Rent)>>,
        current_player: ActorId,
        ownership: Vec<ActorId>,
        current_step: u64,
    },

    /// Reply on `Play`` message, in case when the current gas runs out,
    /// the next reservation is taken and the next game cycle is started from the new reservation
    NextRoundFromReservation,

    /// Reply on `DeleteGame` message
    GameDeleted,

    /// Reply on `DeletePlayer` message
    PlayerDeleted,

    StrategicSuccess,

    Killed {
        inheritor: ActorId,
    },

    WaitingForGasForGameContract
}

#[derive(Clone)]
pub struct GameService(());

impl GameService {
    pub async fn init(config: Config, dns_id_and_name: Option<(ActorId, String)>) -> Self {
        unsafe {
            let storage = Storage {
                dns_info: dns_id_and_name.clone(),
                config,
                admin: msg::source(),
                ..Default::default()
            };
            STORAGE = Some(storage);
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

    pub fn create_game_session(
        &mut self,
        entry_fee: Option<u128>,
        name: String,
        strategy_id: ActorId,
    ) {
        let storage = self.get_mut();
        let event = services::utils::panicking(|| {
            funcs::create_game_session(storage, entry_fee, &name, &strategy_id)
        });
        self.notify_on(event.clone()).expect("Notification Error");
    }

    pub fn make_reservation(&mut self, admin_id: ActorId) {
        let storage = self.get_mut();
        let event = services::utils::panicking(|| funcs::make_reservation(storage, admin_id));
        self.notify_on(event.clone()).expect("Notification Error");
    }

    pub fn register(&mut self, admin_id: AdminId, strategy_id: ActorId, name: String) {
        let storage = self.get_mut();
        let event =
            services::utils::panicking(|| funcs::register(storage, admin_id, strategy_id, name));
        self.notify_on(event.clone()).expect("Notification Error");
    }
    pub fn play(&mut self, admin_id: AdminId) {
        let storage = self.get_mut();
        let event = services::utils::panicking(|| funcs::play(storage, admin_id));
        self.notify_on(event.clone()).expect("Notification Error");
    }
    pub fn add_gas_to_player_strategy(&mut self, admin_id: AdminId) {
        let storage = self.get_mut();
        let event =
            services::utils::panicking(|| funcs::add_gas_to_player_strategy(storage, admin_id));
        self.notify_on(event.clone()).expect("Notification Error");
    }
    pub fn cancel_game_session(&mut self, admin_id: AdminId) {
        let storage = self.get_mut();
        let event = services::utils::panicking(|| funcs::cancel_game_session(storage, admin_id));
        self.notify_on(event.clone()).expect("Notification Error");
    }

    pub fn exit_game(&mut self, admin_id: AdminId) {
        let storage = self.get_mut();
        let event = services::utils::panicking(|| funcs::exit_game(storage, admin_id));
        self.notify_on(event.clone()).expect("Notification Error");
    }

    pub fn delete_game(&mut self, admin_id: AdminId) {
        let storage = self.get_mut();
        let event = services::utils::panicking(|| funcs::delete_game(storage, admin_id));
        self.notify_on(event.clone()).expect("Notification Error");
    }

    pub fn delete_player(&mut self, player_id: AdminId) {
        let storage = self.get_mut();
        let event = services::utils::panicking(|| funcs::delete_player(storage, player_id));
        self.notify_on(event.clone()).expect("Notification Error");
    }

    pub fn throw_roll(
        &mut self,
        admin_id: AdminId,
        pay_fine: bool,
        properties_for_sale: Option<Vec<u8>>,
    ) {
        let storage = self.get_mut();
        let game_instance = storage
            .game_sessions
            .get_mut(&admin_id)
            .expect("Game does not exist");

        funcs::throw_roll(game_instance, pay_fine, properties_for_sale);

        let event = game_instance.finalize_turn_outcome(
            storage.config.gas_for_step,
            storage.config.min_gas_limit,
            storage.config.reservation_duration_in_block,
        );
        self.notify_on(event.clone()).expect("Notification Error");
    }

    pub fn add_gear(&mut self, admin_id: AdminId, properties_for_sale: Option<Vec<u8>>) {
        let storage = self.get_mut();
        let game_instance = storage
            .game_sessions
            .get_mut(&admin_id)
            .expect("Game does not exist");

        funcs::add_gear(game_instance, properties_for_sale);

        let event = game_instance.finalize_turn_outcome(
            storage.config.gas_for_step,
            storage.config.min_gas_limit,
            storage.config.reservation_duration_in_block,
        );
        self.notify_on(event.clone()).expect("Notification Error");
    }

    pub fn upgrade(&mut self, admin_id: AdminId, properties_for_sale: Option<Vec<u8>>) {
        let storage = self.get_mut();
        let game_instance = storage
            .game_sessions
            .get_mut(&admin_id)
            .expect("Game does not exist");

        funcs::upgrade(game_instance, properties_for_sale);

        let event = game_instance.finalize_turn_outcome(
            storage.config.gas_for_step,
            storage.config.min_gas_limit,
            storage.config.reservation_duration_in_block,
        );
        self.notify_on(event.clone()).expect("Notification Error");
    }

    pub fn buy_cell(&mut self, admin_id: AdminId, properties_for_sale: Option<Vec<u8>>) {
        let storage = self.get_mut();
        let game_instance = storage
            .game_sessions
            .get_mut(&admin_id)
            .expect("Game does not exist");

        funcs::buy_cell(game_instance, properties_for_sale);

        let event = game_instance.finalize_turn_outcome(
            storage.config.gas_for_step,
            storage.config.min_gas_limit,
            storage.config.reservation_duration_in_block,
        );
        self.notify_on(event.clone()).expect("Notification Error");
    }

    pub fn pay_rent(&mut self, admin_id: AdminId, properties_for_sale: Option<Vec<u8>>) {
        let storage = self.get_mut();
        let game_instance = storage
            .game_sessions
            .get_mut(&admin_id)
            .expect("Game does not exist");

        funcs::pay_rent(game_instance, properties_for_sale);
        let event = game_instance.finalize_turn_outcome(
            storage.config.gas_for_step,
            storage.config.min_gas_limit,
            storage.config.reservation_duration_in_block,
        );
        self.notify_on(event.clone()).expect("Notification Error");
    }
    pub fn skip(&mut self, admin_id: AdminId) {
        let storage = self.get_mut();
        let game_instance = storage
            .game_sessions
            .get_mut(&admin_id)
            .expect("Game does not exist");

        let event = game_instance.finalize_turn_outcome(
            storage.config.gas_for_step,
            storage.config.min_gas_limit,
            storage.config.reservation_duration_in_block,
        );
        self.notify_on(event.clone()).expect("Notification Error");
    }

    pub fn change_admin(&mut self, admin: ActorId) {
        let storage = self.get_mut();
        if storage.admin != msg::source() {
            services::utils::panic(GameError::AccessDenied);
        }
        storage.admin = admin;
    }

    pub async fn kill(&mut self, inheritor: ActorId) {
        let storage = self.get();
        if storage.admin != msg::source() {
            services::utils::panic(GameError::AccessDenied);
        }
        if let Some((id, _name)) = &storage.dns_info {
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

    pub fn get_players_to_sessions(&self) -> Vec<(ActorId, AdminId)> {
        self.get().players_to_sessions.clone().into_iter().collect()
    }

    pub fn get_config(&self) -> Config {
        self.get().config.clone()
    }
    pub fn get_owner_id(&self, admin_id: ActorId, strategy_id: ActorId) -> Option<ActorId> {
        if let Some(game_session) = self.get().game_sessions.get(&admin_id) {
            game_session
                .players
                .get(&strategy_id)
                .map(|player_info| player_info.owner_id)
        } else {
            None
        }
    }
    pub fn get_player_info(&self, account_id: ActorId) -> Option<PlayerInfoState> {
        let storage = self.get();
        if let Some(admin_id) = storage.players_to_sessions.get(&account_id) {
            if let Some(game_session) = storage.game_sessions.get(admin_id) {
                if let Some(strategy_id) = game_session.owners_to_strategy_ids.get(&account_id) {
                    if let Some(player_info) = game_session.players.get(strategy_id) {
                        return Some(player_info.clone().into());
                    }
                }
            }
        }
        None
    }

    pub fn get_game_session(&self, account_id: ActorId) -> Option<GameState> {
        let storage = self.get();
        if let Some(admin_id) = storage.players_to_sessions.get(&account_id) {
            if let Some(game_session) = storage.game_sessions.get(admin_id) {
                let game_session: GameState = game_session.clone().into();
                Some(game_session)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn dns_info(&self) -> Option<(ActorId, String)> {
        self.get().dns_info.clone()
    }
}
