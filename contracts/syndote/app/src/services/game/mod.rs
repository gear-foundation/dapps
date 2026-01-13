#![allow(static_mut_refs)]
use gstd::{exec, msg, ReservationId};
use sails_rs::{
    collections::{HashMap, HashSet},
    gstd::service,
    prelude::*,
};
mod funcs;
pub mod utils;
use crate::services;
use utils::*;

const RESERVATION_AMOUNT: u64 = 245_000_000_000;
const GAS_FOR_ROUND: u64 = 60_000_000_000;

pub const NUMBER_OF_CELLS: u8 = 40;
pub const NUMBER_OF_PLAYERS: u8 = 4;
pub const JAIL_POSITION: u8 = 10;
pub const COST_FOR_UPGRADE: u32 = 500;
pub const FINE: u32 = 1_000;
pub const PENALTY: u8 = 5;
pub const INITIAL_BALANCE: u32 = 15_000;
pub const NEW_CIRCLE: u32 = 2_000;
pub const WAIT_DURATION: u32 = 5;

#[derive(Default, Clone)]
pub struct Storage {
    admin: ActorId,
    properties_in_bank: HashSet<u8>,
    round: u128,
    players: HashMap<ActorId, PlayerInfo>,
    players_queue: Vec<ActorId>,
    current_player: ActorId,
    current_step: u64,
    // mapping from cells to built properties,
    properties: Vec<Option<(ActorId, Gears, u32, u32)>>,
    // mapping from cells to accounts who have properties on it
    ownership: Vec<ActorId>,
    game_status: GameStatus,
    winner: ActorId,
    reservations: Vec<ReservationId>,
    dns_info: Option<(ActorId, String)>,
}

static mut STORAGE: Option<Storage> = None;

#[event]
#[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum Event {
    Registered,
    StartRegistration,
    Played,
    GameFinished {
        winner: ActorId,
    },
    StrategicError,
    StrategicSuccess,
    Step {
        players: Vec<(ActorId, PlayerInfo)>,
        properties: Vec<Option<(ActorId, Gears, Price, Rent)>>,
        current_player: ActorId,
        ownership: Vec<ActorId>,
        current_step: u64,
    },
    Jail {
        in_jail: bool,
        position: u8,
    },
    GasReserved,
    NextRoundFromReservation,
    AdminChanged,
    Killed {
        inheritor: ActorId,
    },
}

#[derive(Clone)]
pub struct GameService(());

impl GameService {
    pub fn new() -> Self {
        Self(())
    }
    pub async fn init(dns_id_and_name: Option<(ActorId, String)>) -> Self {
        unsafe {
            let mut storage = Storage {
                admin: msg::source(),
                dns_info: dns_id_and_name.clone(),
                ..Default::default()
            };
            init_properties(&mut storage.properties, &mut storage.ownership);
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
    #[export]
    pub fn register(&mut self, player: ActorId) {
        let storage = self.get_mut();
        let event = services::utils::panicking(|| funcs::register(storage, &player));
        self.emit_event(event.clone()).expect("Notification Error");
    }

    #[export]
    pub fn reserve_gas(&mut self) {
        let storage = self.get_mut();
        let event = services::utils::panicking(|| funcs::reserve_gas(storage));
        self.emit_event(event.clone()).expect("Notification Error");
    }

    #[export]
    pub fn start_registration(&mut self) {
        let storage = self.get_mut();
        let event = services::utils::panicking(|| funcs::start_registration(storage));
        self.emit_event(event.clone()).expect("Notification Error");
    }

    #[export]
    pub async fn play(&mut self) {
        let storage = self.get_mut();
        let res = funcs::play(storage).await;
        let event = match res {
            Ok(v) => v,
            Err(e) => services::utils::panic(e),
        };

        self.emit_event(event.clone()).expect("Notification Error");
    }

    #[export]
    pub fn throw_roll(&mut self, pay_fine: bool, properties_for_sale: Option<Vec<u8>>) {
        let storage = self.get_mut();
        let event = services::utils::panicking(|| {
            funcs::throw_roll(storage, pay_fine, properties_for_sale)
        });
        self.emit_event(event.clone()).expect("Notification Error");
    }

    #[export]
    pub fn add_gear(&mut self, properties_for_sale: Option<Vec<u8>>) {
        let storage = self.get_mut();
        let event = services::utils::panicking(|| funcs::add_gear(storage, properties_for_sale));
        self.emit_event(event.clone()).expect("Notification Error");
    }

    #[export]
    pub fn upgrade(&mut self, properties_for_sale: Option<Vec<u8>>) {
        let storage = self.get_mut();
        let event = services::utils::panicking(|| funcs::upgrade(storage, properties_for_sale));
        self.emit_event(event.clone()).expect("Notification Error");
    }

    #[export]
    pub fn buy_cell(&mut self, properties_for_sale: Option<Vec<u8>>) {
        let storage = self.get_mut();
        let event = services::utils::panicking(|| funcs::buy_cell(storage, properties_for_sale));
        self.emit_event(event.clone()).expect("Notification Error");
    }

    #[export]
    pub fn pay_rent(&mut self, properties_for_sale: Option<Vec<u8>>) {
        let storage = self.get_mut();
        let event = services::utils::panicking(|| funcs::pay_rent(storage, properties_for_sale));
        self.emit_event(event.clone()).expect("Notification Error");
    }

    #[export]
    pub fn change_admin(&mut self, admin: ActorId) {
        let storage = self.get_mut();
        let event = services::utils::panicking(|| funcs::change_admin(storage, admin));
        self.emit_event(event.clone()).expect("Notification Error");
    }

    #[export]
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

        self.emit_event(Event::Killed { inheritor })
            .expect("Notification Error");
        exec::exit(inheritor);
    }

    #[export]
    pub fn get_storage(&self) -> StorageState {
        self.get().clone().into()
    }

    #[export]
    pub fn dns_info(&self) -> Option<(ActorId, String)> {
        self.get().dns_info.clone()
    }
}
