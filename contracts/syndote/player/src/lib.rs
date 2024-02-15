#![no_std]
use gstd::{debug, exec, msg, prelude::*, ActorId};
use syndote_io::*;
//static mut MONOPOLY: ActorId = ActorId::zero();
pub const COST_FOR_UPGRADE: u32 = 500;
pub const FINE: u32 = 1_000;

#[no_mangle]
extern fn handle() {
    debug!("Player");
    debug!("GAS IN STRATEGY {:?}", exec::gas_available());
    let message: YourTurn = msg::load().expect("Unable to decode struct`YourTurn`");

    let (_, player_info) = message
        .game_info
        .players
        .iter()
        .find(|(player, _player_info)| player == &exec::program_id())
        .expect("Can't find my address")
        .clone();

    if player_info.in_jail {
        if player_info.balance <= FINE {
            reply(StrategicAction::ThrowRoll {
                pay_fine: false,
                properties_for_sale: None,
            });
            return;
        } else {
            reply(StrategicAction::ThrowRoll {
                pay_fine: true,
                properties_for_sale: None,
            });
            return;
        }
    }

    let position = player_info.position;

    // debug!("BALANCE {:?}", my_player.balance);
    let (my_cell, free_cell, gears, price) = if let Some((account, gears, price, _)) =
        &message.game_info.properties[position as usize]
    {
        let my_cell = account == &exec::program_id();
        let free_cell = account == &ActorId::zero();
        (my_cell, free_cell, gears, price)
    } else {
        reply(StrategicAction::Skip);
        return;
    };
    debug!("my cell {:?}", my_cell);
    debug!("free cell {:?}", free_cell);
    if my_cell {
        if gears.len() < 3 {
            debug!("add gear ");
            reply(StrategicAction::AddGear {
                properties_for_sale: None,
            });
            return;
        } else {
            debug!("upgrade ");
            reply(StrategicAction::Upgrade {
                properties_for_sale: None,
            })
        }
    }
    if free_cell {
        if player_info.balance >= *price && player_info.balance >= 1_000 {
            debug!("buy cell ");
            reply(StrategicAction::BuyCell {
                properties_for_sale: None,
            });
        } else {
            debug!("skip");
            reply(StrategicAction::Skip);
        }
    } else if !my_cell {
        debug!("pay rent ");
        reply(StrategicAction::PayRent {
            properties_for_sale: None,
        });
    }
}

#[no_mangle]
unsafe extern fn init() {
    //   MONOPOLY = msg::load::<ActorId>().expect("Unable to decode ActorId");
}
fn reply(payload: StrategicAction) {
    msg::reply(payload, 0).expect("Error in sending a reply to monopoly contract");
}
