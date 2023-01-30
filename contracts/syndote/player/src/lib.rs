#![no_std]
use gstd::{exec, msg, prelude::*, ActorId};
use syndote_io::*;
//static mut MONOPOLY: ActorId = ActorId::zero();
pub const COST_FOR_UPGRADE: u32 = 500;
pub const FINE: u32 = 1_000;

#[gstd::async_main]
async fn main() {
    //let monopoly_id = unsafe { MONOPOLY };
    let monopoly_id = msg::source();
    // assert_eq!(
    //     msg::source(),
    //     monopoly_id,
    //     "Only monopoly contract can call strategic contract"
    // );
    let message: YourTurn = msg::load().expect("Unable to decode struct`YourTurn`");
    let (_, mut player_info) = message
        .players
        .iter()
        .find(|(player, _player_info)| player == &exec::program_id())
        .expect("Can't find my address")
        .clone();
    if player_info.in_jail {
        if player_info.balance <= FINE {
            let reply: GameEvent = msg::send_for_reply_as(
                monopoly_id,
                GameAction::ThrowRoll {
                    pay_fine: false,
                    properties_for_sale: None,
                },
                0,
            )
            .expect("Error in sending a message `GameAction::ThrowRoll`")
            .await
            .expect("Unable to decode `GameEvent");

            if let GameEvent::Jail { in_jail, position } = reply {
                if !in_jail {
                    player_info.position = position;
                } else {
                    msg::reply("", 0).expect("Error in sending a reply to monopoly contract");
                    return;
                }
            }
        } else {
            msg::send_for_reply_as::<_, GameEvent>(
                monopoly_id,
                GameAction::ThrowRoll {
                    pay_fine: true,
                    properties_for_sale: None,
                },
                0,
            )
            .expect("Error in sending a message `GameAction::ThrowRoll`")
            .await
            .expect("Unable to decode `GameEvent");

            msg::reply("", 0).expect("Error in sending a reply to monopoly contract");
            return;
        }
    }

    let position = player_info.position;

    // debug!("BALANCE {:?}", my_player.balance);
    let (my_cell, free_cell, gears) =
        if let Some((account, gears, _, _)) = &message.properties[position as usize] {
            let my_cell = account == &exec::program_id();
            let free_cell = account == &ActorId::zero();
            (my_cell, free_cell, gears)
        } else {
            msg::reply("", 0).expect("Error in sending a reply to monopoly contract");
            return;
        };

    if my_cell {
        //debug!("ADD GEAR");
        if gears.len() < 3 {
            msg::send_for_reply_as::<_, GameEvent>(
                monopoly_id,
                GameAction::AddGear {
                    properties_for_sale: None,
                },
                0,
            )
            .expect("Error in sending a message `GameAction::AddGear`")
            .await
            .expect("Unable to decode `GameEvent");
            msg::reply("", 0).expect("Error in sending a reply to monopoly contract");
            return;
        } else {
            //debug!("UPGRADE");
            msg::send_for_reply_as::<_, GameEvent>(
                monopoly_id,
                GameAction::Upgrade {
                    properties_for_sale: None,
                },
                0,
            )
            .expect("Error in sending a message `GameAction::Upgrade`")
            .await
            .expect("Unable to decode `GameEvent");
            msg::reply("", 0).expect("Error in sending a reply to monopoly contract");
            return;
        }
    }
    if free_cell {
        //debug!("BUY CELL");
        msg::send_for_reply_as::<_, GameEvent>(
            monopoly_id,
            GameAction::BuyCell {
                properties_for_sale: None,
            },
            0,
        )
        .expect("Error in sending a message `GameAction::BuyCell`")
        .await
        .expect("Unable to decode `GameEvent");
    } else if !my_cell {
        //debug!("PAY RENT");
        msg::send_for_reply_as::<_, GameEvent>(
            monopoly_id,
            GameAction::PayRent {
                properties_for_sale: None,
            },
            0,
        )
        .expect("Error in sending a message `GameAction::PayRent`")
        .await
        .expect("Unable to decode `GameEvent");
    }
    msg::reply("", 0).expect("Error in sending a reply to monopoly contract");
}

#[no_mangle]
unsafe extern "C" fn init() {
    //   MONOPOLY = msg::load::<ActorId>().expect("Unable to decode ActorId");
}
