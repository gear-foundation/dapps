#![no_std]

use sails_rs::gstd::{exec, msg};
use sails_rs::collections::BTreeSet;
use sails_rs::prelude::*;
use gstd::{ReservationId, debug};

pub const COST_FOR_UPGRADE: u32 = 500;
pub const FINE: u32 = 1_000;
pub type Price = u32;
pub type Rent = u32;
pub type Gears = Vec<Gear>;

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct GameInfo {
    pub admin_id: ActorId,
    pub properties_in_bank: Vec<u8>,
    pub players: Vec<(ActorId, PlayerInfo)>,
    pub players_queue: Vec<ActorId>,
    // mapping from cells to built properties,
    pub properties: Vec<Option<(ActorId, Gears, u32, u32)>>,
    // mapping from cells to accounts who have properties on it
    pub ownership: Vec<ActorId>,
}

#[derive(Default, Debug, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct PlayerInfo {
    pub owner_id: ActorId,
    pub name: String,
    pub position: u8,
    pub balance: u32,
    pub debt: u32,
    pub in_jail: bool,
    pub round: u128,
    pub cells: BTreeSet<u8>,
    pub penalty: u8,
    pub lost: bool,
    pub reservation_id: Option<ReservationId>,
}

#[derive(Debug, PartialEq, Eq, Encode, Decode, Clone, TypeInfo, Copy)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum Gear {
    Bronze,
    Silver,
    Gold,
}

struct PlayerService(());

impl PlayerService {
    pub fn init() -> Self {
        Self(())
    }
}

#[sails_rs::service]
impl PlayerService {
    pub fn new() -> Self {
        Self(())
    }
    pub fn your_turn(
        &self,
        game_info: GameInfo,
    ) {
        debug!("YOUR TURN! {:?}", exec::program_id());
        debug!("GAS IN STRATEGY {:?}", exec::gas_available());
        let monopoly_id = msg::source();

        let (_, player_info) = game_info
            .players
            .iter()
            .find(|(player, _player_info)| player == &exec::program_id())
            .expect("Can't find my address")
            .clone();

        if player_info.in_jail {
            if player_info.balance <= FINE {
                let request = [
                    "Syndote".encode(),
                    "ThrowRoll".to_string().encode(),
                    (game_info.admin_id, false, None::<Vec<u8>>).encode(),
                ]
                .concat();
            
                msg::send_bytes(monopoly_id, request, 0)
                    .expect("Error in sending a message `ThrowRoll`");
                debug!("ThrowRoll");
                debug!("I SEND");
                return;
            } else {
                let request = [
                    "Syndote".encode(),
                    "ThrowRoll".to_string().encode(),
                    (game_info.admin_id, true, None::<Vec<u8>>).encode(),
                ]
                .concat();
            
                msg::send_bytes(monopoly_id, request, 0)
                    .expect("Error in sending a message `ThrowRoll`");
                debug!("ThrowRoll");
                debug!("I SEND");
                return;
            }
        }

        let position = player_info.position;

        let (my_cell, free_cell, gears, price) = if let Some((account, gears, price, _)) =
            &game_info.properties[position as usize]
        {
            let my_cell = account == &exec::program_id();
            let free_cell = account == &ActorId::zero();
            (my_cell, free_cell, gears, price)
        } else {
            let request = [
                "Syndote".encode(),
                "Skip".to_string().encode(),
                (game_info.admin_id).encode(),
            ]
            .concat();
        
            msg::send_bytes(monopoly_id, request, 0)
                .expect("Error in sending a message `Skip`");
            debug!("Skip");
            debug!("I SEND");
            return;
        };
        if my_cell {
            if gears.len() < 3 {
                send_request(monopoly_id, "AddGear".to_string(), game_info.admin_id);
                debug!("AddGear");
                return;
            } else {
                send_request(monopoly_id, "Upgrade".to_string(), game_info.admin_id);
                debug!("Upgrade");
                return;
            }
        }
        if free_cell {
            if player_info.balance >= *price && player_info.balance >= 1_000 {
                send_request(monopoly_id, "BuyCell".to_string(), game_info.admin_id);
                debug!("BuyCell");
                return;
            } else {
                let request = [
                    "Syndote".encode(),
                    "Skip".to_string().encode(),
                    (game_info.admin_id).encode(),
                ]
                .concat();
            
                msg::send_bytes(monopoly_id, request, 0)
                    .expect("Error in sending a message `Skip`");
                debug!("Skip");
                debug!("I SEND");
                return;
            }
        } else if !my_cell {
            send_request(monopoly_id, "PayRent".to_string(), game_info.admin_id);
            debug!("PayRent");
            return;
        }
        debug!("END");
        let request = [
            "Syndote".encode(),
            "Skip".to_string().encode(),
            (game_info.admin_id).encode(),
        ]
        .concat();
    
        msg::send_bytes(monopoly_id, request, 0)
            .expect("Error in sending a message `Skip`");
        debug!("Skip");
        debug!("I SEND");
    }
}

fn send_request(program_id: ActorId, action: String, admin_id: ActorId) {
    let request = [
        "Syndote".encode(),
        action.encode(),
        (admin_id, None::<Vec<u8>>).encode(),
    ]
    .concat();

    msg::send_bytes(program_id, request, 0)
        .expect("Error in sending a message");
    debug!("I SEND");
}

pub struct PlayerProgram(());

#[sails_rs::program]
impl PlayerProgram {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        PlayerService::init();
        Self(())
    }

    pub fn player(&self) -> PlayerService {
        PlayerService::new()
    }
}
