#![no_std]

use sails_rs::{
    gstd::{exec, msg},
    prelude::*,
};

pub const COST_FOR_UPGRADE: u32 = 500;
pub const FINE: u32 = 1_000;
pub type Price = u32;
pub type Rent = u32;
pub type Gears = Vec<Gear>;

#[derive(Default, Debug, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct PlayerInfo {
    pub position: u8,
    pub balance: u32,
    pub debt: u32,
    pub in_jail: bool,
    pub round: u128,
    pub cells: Vec<u8>,
    pub penalty: u8,
    pub lost: bool,
}

#[derive(Debug, PartialEq, Eq, Encode, Decode, Clone, TypeInfo, Copy)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum Gear {
    Bronze,
    Silver,
    Gold,
}

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

struct PlayerService(());

impl PlayerService {
    pub fn new() -> Self {
        Self(())
    }
    pub fn init() -> Self {
        Self(())
    }
}

#[sails_rs::service]
impl PlayerService {
    #[export]
    pub async fn your_turn(
        &self,
        players: Vec<(ActorId, PlayerInfo)>,
        properties: Vec<Option<(ActorId, Gears, Price, Rent)>>,
    ) -> bool {
        let monopoly_id = msg::source();
        let (_, mut player_info) = players
            .iter()
            .find(|(player, _player_info)| player == &exec::program_id())
            .expect("Can't find my address")
            .clone();

        if player_info.in_jail {
            if player_info.balance <= FINE {
                let request = [
                    "Syndote".encode(),
                    "ThrowRoll".to_string().encode(),
                    (false, None::<Vec<u8>>).encode(),
                ]
                .concat();

                let reply: Event = msg::send_bytes_for_reply_as(monopoly_id, request, 0, 0)
                    .expect("Error in sending a message `ThrowRoll`")
                    .await
                    .expect("Unable to decode `Event`");

                if let Event::Jail { in_jail, position } = reply {
                    if !in_jail {
                        player_info.position = position;
                    } else {
                        return true;
                    }
                }
            } else {
                let request = [
                    "Syndote".encode(),
                    "ThrowRoll".to_string().encode(),
                    (true, None::<Vec<u8>>).encode(),
                ]
                .concat();

                msg::send_bytes_for_reply(monopoly_id, request, 0, 0)
                    .expect("Error in sending a message `ThrowRoll`")
                    .await
                    .expect("Unable to decode `Event`");

                return true;
            }
        }

        let position = player_info.position;

        let (my_cell, free_cell, gears) =
            if let Some((account, gears, _, _)) = &properties[position as usize] {
                let my_cell = account == &exec::program_id();
                let free_cell = account == &ActorId::zero();
                (my_cell, free_cell, gears)
            } else {
                return true;
            };

        if my_cell {
            if gears.len() < 3 {
                let request = [
                    "Syndote".encode(),
                    "AddGear".to_string().encode(),
                    (None::<Vec<u8>>).encode(),
                ]
                .concat();

                msg::send_bytes_for_reply(monopoly_id, request, 0, 0)
                    .expect("Error in sending a message `ThrowRoll`")
                    .await
                    .expect("Unable to decode `Event`");

                return true;
            } else {
                let request = [
                    "Syndote".encode(),
                    "Upgrade".to_string().encode(),
                    (None::<Vec<u8>>).encode(),
                ]
                .concat();

                msg::send_bytes_for_reply(monopoly_id, request, 0, 0)
                    .expect("Error in sending a message `ThrowRoll`")
                    .await
                    .expect("Unable to decode `Event`");

                return true;
            }
        }
        if free_cell {
            //debug!("BUY CELL");

            let request = [
                "Syndote".encode(),
                "BuyCell".to_string().encode(),
                (None::<Vec<u8>>).encode(),
            ]
            .concat();

            msg::send_bytes_for_reply(monopoly_id, request, 0, 0)
                .expect("Error in sending a message `ThrowRoll`")
                .await
                .expect("Unable to decode `Event`");
        } else if !my_cell {
            //debug!("PAY RENT");
            let request = [
                "Syndote".encode(),
                "PayRent".to_string().encode(),
                (None::<Vec<u8>>).encode(),
            ]
            .concat();

            msg::send_bytes_for_reply(monopoly_id, request, 0, 0)
                .expect("Error in sending a message `ThrowRoll`")
                .await
                .expect("Unable to decode `Event`");
        }
        true
    }
}

pub struct SyndotePlayerProgram(());

#[sails_rs::program]
impl SyndotePlayerProgram {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        PlayerService::init();
        Self(())
    }

    pub fn player(&self) -> PlayerService {
        PlayerService::new()
    }
}
