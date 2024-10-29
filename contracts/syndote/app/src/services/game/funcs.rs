use crate::services::game::*;
use crate::services::game::game::{GameSessionActions, FINE, COST_FOR_UPGRADE};
use sails_rs::ActorId;

pub fn create_game_session(storage: &mut Storage, entry_fee: Option<u128>, name: &String, strategy_id: &ActorId) -> Result<Event, GameError> {
    if let Some(fee) = entry_fee {
        if fee < exec::env_vars().existential_deposit{
            return Err(GameError::FeeIsLessThanED);
        }
    }

    let admin_id = msg::source();
    if storage.game_sessions.contains_key(&admin_id) {
        return Err(GameError::GameSessionAlreadyExists);
    }

    let mut game = Game {
        admin_id,
        entry_fee,
        ..Default::default()
    };
    game.init_properties();
    game.register(
        &admin_id,
        strategy_id,
        name,
        storage.config.reservation_amount,
        storage.config.reservation_duration_in_block,
    )?;
    storage.game_sessions.insert(admin_id, game);
    storage.players_to_sessions.insert(admin_id, admin_id);
    Ok(Event::GameSessionCreated { admin_id })
}

pub fn make_reservation(storage: &mut Storage, admin_id: ActorId) -> Result<Event, GameError> {
    let game = storage
        .game_sessions
        .get_mut(&admin_id)
        .ok_or(GameError::GameDoesNotExist)?;
    game.make_reservation(
        storage.config.reservation_amount,
        storage.config.reservation_duration_in_block,
    )?;
    Ok(Event::ReservationMade)
}

pub fn register(storage: &mut Storage, admin_id: AdminId, strategy_id: ActorId, name: String) -> Result<Event, GameError> {
    let player_id = msg::source();

    if storage.game_sessions.contains_key(&player_id)
        || storage.players_to_sessions.contains_key(&player_id)
    {
        return Err(GameError::AccountAlreadyRegistered);
    }

    let game = storage
        .game_sessions
        .get_mut(&admin_id)
        .ok_or(GameError::GameDoesNotExist)?;

    game.register(
        &player_id,
        &strategy_id,
        &name,
        storage.config.reservation_amount,
        storage.config.reservation_duration_in_block,
    )?;
    storage.players_to_sessions.insert(player_id, admin_id);
    Ok(Event::StrategyRegistered)
}

pub fn play(storage: &mut Storage, admin_id: AdminId) -> Result<Event, GameError> {
    debug!("PLAY");
    let game = storage
        .game_sessions
        .get_mut(&admin_id)
        .ok_or(GameError::GameDoesNotExist)?;

    game.play(
        storage.config.min_gas_limit,
        storage.config.time_for_step,
        &mut storage.awaiting_reply_msg_id_to_session_id,
        storage.config.gas_refill_timeout,
    )
}

pub fn add_gas_to_player_strategy(storage: &mut Storage, admin_id: AdminId) -> Result<Event, GameError> {
    let game = storage
        .game_sessions
        .get_mut(&admin_id)
        .ok_or(GameError::GameDoesNotExist)?;

    game.add_gas_to_player_strategy(
        storage.config.reservation_amount,
        storage.config.reservation_duration_in_block,
    )?;
    game.game_status = GameStatus::Play;
    Ok(Event::GasForPlayerStrategyAdded)
}

pub fn cancel_game_session(storage: &mut Storage, admin_id: AdminId) -> Result<Event, GameError> {
    let game = storage
        .game_sessions
        .get_mut(&admin_id)
        .ok_or(GameError::GameDoesNotExist)?;
    game.cancel_game_session()?;
    for player_id in game.owners_to_strategy_ids.keys() {
        storage.players_to_sessions.remove(player_id);
    }
    storage.game_sessions.remove(&admin_id);
    Ok(Event::GameWasCancelled)
}

pub fn exit_game(storage: &mut Storage, admin_id: AdminId) -> Result<Event, GameError> {
    let game = storage
        .game_sessions
        .get_mut(&admin_id)
        .ok_or(GameError::GameDoesNotExist)?;
    game.exit_game()?;
    storage.players_to_sessions.remove(&msg::source());
    Ok(Event::PlayerLeftGame)
}

pub fn delete_game(storage: &mut Storage, admin_id: AdminId) -> Result<Event, GameError> {
    let game = storage
        .game_sessions
        .get(&admin_id)
        .ok_or(GameError::GameDoesNotExist)?;
    game.check_status(GameStatus::Finished)?;
    for player_id in game.owners_to_strategy_ids.keys() {
        storage.players_to_sessions.remove(player_id);
    }
    storage.game_sessions.remove(&admin_id);
    Ok(Event::GameDeleted)
}

pub fn delete_player(storage: &mut Storage, player_id: AdminId) -> Result<Event, GameError> {
    let admin_id = storage
        .players_to_sessions
        .get(&player_id)
        .ok_or(GameError::AccountAlreadyRegistered)?;

    if *admin_id != msg::source() {
        return Err(GameError::OnlyAdmin);
    }

    let game = storage
        .game_sessions
        .get_mut(admin_id)
        .ok_or(GameError::GameDoesNotExist)?;

    game.delete_player(&player_id)?;

    storage.players_to_sessions.remove(&player_id);

    Ok(Event::PlayerDeleted)

}
// pub fn reserve_gas(storage: &mut Storage) -> Result<Event, GameError> {
//     let reservation_id =
//         ReservationId::reserve(RESERVATION_AMOUNT, 600).expect("reservation across executions");

//     storage.reservations.push(reservation_id);
//     Ok(Event::GasReserved)
// }

// pub fn start_registration(storage: &mut Storage) -> Result<Event, GameError> {
//     storage.check_status(GameStatus::Finished)?;
//     storage.only_admin()?;
//     let mut game_storage = Storage {
//         admin: storage.admin,
//         ..Default::default()
//     };
//     init_properties(&mut game_storage.properties, &mut game_storage.ownership);
//     *storage = game_storage;
//     Ok(Event::StartRegistration)
// }

// pub async fn play(storage: &mut Storage) -> Result<Event, GameError> {
//     //self.check_status(GameStatus::Play);
//     let msg_src = msg::source();
//     if msg_src != storage.admin && msg_src != exec::program_id() {
//         return Err(GameError::AccessDenied);
//     }
//     while storage.game_status == GameStatus::Play {
//         if storage.players_queue.len() <= 1 {
//             storage.winner = storage.players_queue[0];
//             storage.game_status = GameStatus::Finished;

//             return Ok(Event::GameFinished {
//                 winner: storage.winner,
//             });
//         }
//         if exec::gas_available() <= GAS_FOR_ROUND {
//             if let Some(id) = storage.reservations.pop() {
//                 let request =
//                     ["Syndote".encode(), "Play".to_string().encode(), ().encode()].concat();

//                 msg::send_bytes_from_reservation(id, exec::program_id(), request, 0)
//                     .expect("Error in sending message");

//                 return Ok(Event::NextRoundFromReservation);
//             } else {
//                 panic!("GIVE ME MORE GAS");
//             };
//         }
//         // // check penalty and debt of the players for the previous round
//         // // if penalty is equal to 5 points we remove the player from the game
//         // // if a player has a debt and he has not enough balance to pay it
//         // // he is also removed from the game
//         // bankrupt_and_penalty(
//         //     &self.admin,
//         //     &mut self.players,
//         //     &mut self.players_queue,
//         //     &mut self.properties,
//         //     &mut self.properties_in_bank,
//         //     &mut self.ownership,
//         // );

//         // if self.players_queue.len() <= 1 {
//         //     self.winner = self.players_queue[0];
//         //     self.game_status = GameStatus::Finished;
//         //     msg::reply(
//         //         GameEvent::GameFinished {
//         //             winner: self.winner,
//         //         },
//         //         0,
//         //     )
//         //     .expect("Error in sending a reply `GameEvent::GameFinished`");
//         //     break;
//         // }
//         storage.round = storage.round.wrapping_add(1);
//         for player in storage.players_queue.clone() {
//             storage.current_player = player;
//             storage.current_step += 1;
//             // we save the state before the player's step in case
//             // the player's contract does not reply or is executed with a panic.
//             // Then we roll back all the changes that the player could have made.
//             let mut state = storage.clone();
//             let player_info = storage
//                 .players
//                 .get_mut(&player)
//                 .expect("Cant be None: Get Player");

//             // if a player is in jail we don't throw rolls for him
//             let position = if player_info.in_jail {
//                 player_info.position
//             } else {
//                 let (r1, r2) = get_rolls();
//                 //     debug!("ROOLS {:?} {:?}", r1, r2);
//                 let roll_sum = r1 + r2;
//                 (player_info.position + roll_sum) % NUMBER_OF_CELLS
//             };
//             // If a player is on a cell that belongs to another player
//             // we write down a debt on him in the amount of the rent.
//             // This is done in order to penalize the participant's contract
//             // if he misses the rent
//             let account = storage.ownership[position as usize];

//             if account != player && account != ActorId::zero() {
//                 if let Some((_, _, _, rent)) = storage.properties[position as usize] {
//                     player_info.debt = rent;
//                 }
//             }
//             player_info.position = position;
//             player_info.in_jail = position == JAIL_POSITION;
//             state.players.insert(player, player_info.clone());
//             match position {
//                 0 => {
//                     player_info.balance += NEW_CIRCLE;
//                     player_info.round = storage.round;
//                 }
//                 // free cells (it can be lottery or penalty): TODO as a task on hackathon
//                 2 | 4 | 7 | 16 | 20 | 30 | 33 | 36 | 38 => {
//                     player_info.round = storage.round;
//                 }
//                 _ => {
//                     let reply = take_your_turn(&player, &state).await;

//                     if reply.is_err() {
//                         player_info.penalty = PENALTY;
//                     }
//                 }
//             }
//             // check penalty and debt of the players for the previous round
//             // if penalty is equal to 5 points we remove the player from the game
//             // if a player has a debt and he has not enough balance to pay it
//             // he is also removed from the game
//             bankrupt_and_penalty(
//                 &storage.admin,
//                 &mut storage.players,
//                 &mut storage.players_queue,
//                 &storage.properties,
//                 &mut storage.properties_in_bank,
//                 &mut storage.ownership,
//             );

//             msg::send(
//                 storage.admin,
//                 Event::Step {
//                     players: storage
//                         .players
//                         .iter()
//                         .map(|(key, value)| (*key, value.clone()))
//                         .collect(),
//                     properties: storage.properties.clone(),
//                     current_player: storage.current_player,
//                     current_step: storage.current_step,
//                     ownership: storage.ownership.clone(),
//                 },
//                 0,
//             )
//             .expect("Error in sending a message `GameEvent::Step`");
//         }
//     }
//     Ok(Event::Played)
// }

// async fn take_your_turn(player: &ActorId, storage: &Storage) -> Result<Vec<u8>, Error> {
//     let players: Vec<_> = storage.players.clone().into_iter().collect();

//     let request = [
//         "Player".encode(),
//         "YourTurn".to_string().encode(),
//         (players, storage.properties.clone()).encode(),
//     ]
//     .concat();

//     msg::send_bytes_for_reply(*player, request, 0, 0)
//         .expect("Error on sending `YourTurn` message")
//         .up_to(Some(WAIT_DURATION))
//         .expect("Invalid wait duration.")
//         .await
// }

pub fn throw_roll(
    game: &mut Game,
    pay_fine: bool,
    properties_for_sale: Option<Vec<u8>>,
) {

    let player = game.current_player;
    let player_info = game.players.get_mut(&player).expect("Can't be None");

    // If a player is not in the jail
    if !player_info.in_jail {
        player_info.penalty += 1;
        return;
    }

    if let Some(properties) = properties_for_sale {
        if sell_property(
            &game.admin_id,
            &mut game.ownership,
            &properties,
            &mut game.properties_in_bank,
            &game.properties,
            player_info,
        )
        .is_err()
        {
            player_info.penalty += 1;
            return;
        };
    }

    let (r1, r2) = get_rolls();
    if r1 == r2 {
        player_info.in_jail = false;
        player_info.position = r1 + r2;
    } else if pay_fine {
        if player_info.balance < FINE {
            player_info.penalty += 1;
            return;
        }
        player_info.balance -= FINE;
        player_info.in_jail = false;
    }
    player_info.round = game.round;
}

pub fn add_gear(
    game: &mut Game,
    properties_for_sale: Option<Vec<u8>>,
) {
    let player = game.current_player;
    let player_info = game.players.get_mut(&player).expect("Can't be None");

    if let Some(properties) = properties_for_sale {
        if sell_property(
            &game.admin_id,
            &mut game.ownership,
            &properties,
            &mut game.properties_in_bank,
            &game.properties,
            player_info,
        )
        .is_err()
        {
            return;
        };
    }

    // if player did not check his balance itself
    if player_info.balance < COST_FOR_UPGRADE {
        player_info.penalty += 1;
        return;
    }

    let position = player_info.position;

    let gears = if let Some((account, gears, _, _)) = &mut game.properties[position as usize] {
        if account != &player {
            player_info.penalty += 1;
            return;
        }
        gears
    } else {
        player_info.penalty += 1;
        return;
    };

    // maximum amount of gear is reached
    if gears.len() == 3 {
        player_info.penalty += 1;
        return;
    }

    gears.push(Gear::Bronze);
    player_info.balance -= COST_FOR_UPGRADE;
    player_info.round = game.round;
}

pub fn upgrade(
    game: &mut Game,
    properties_for_sale: Option<Vec<u8>>,
) {
    let player = game.current_player;
    let player_info = game.players.get_mut(&player).expect("Can't be None");

    if let Some(properties) = properties_for_sale {
        if sell_property(
            &game.admin_id,
            &mut game.ownership,
            &properties,
            &mut game.properties_in_bank,
            &game.properties,
            player_info,
        )
        .is_err()
        {
            return;
        };
    }

    // if player did not check his balance itself
    if player_info.balance < COST_FOR_UPGRADE {
        player_info.penalty += 1;
        return;
    }

    let position = player_info.position;

    if let Some((account, gears, price, rent)) = &mut game.properties[position as usize] {
        if account != &player {
            player_info.penalty += 1;
            return;
        }
        // if nothing to upgrade
        if gears.is_empty() {
            player_info.penalty += 1;
            return;
        }
        for gear in gears {
            if *gear != Gear::Gold {
                *gear = gear.upgrade();
                // add 10 percent to the price of cell
                *price += *price / 10;
                // add 10 percent to the price of rent
                *rent += *rent / 10;
                break;
            }
        }
    } else {
        player_info.penalty += 1;
        return;
    };

    player_info.balance -= COST_FOR_UPGRADE;
    player_info.round = game.round;
}

pub fn buy_cell(
    game: &mut Game,
    properties_for_sale: Option<Vec<u8>>,
){
    let player = game.current_player;
    let player_info = game.players.get_mut(&player).expect("Can't be None");

    let position = player_info.position;

    if let Some(properties) = properties_for_sale {
        if sell_property(
            &game.admin_id,
            &mut game.ownership,
            &properties,
            &mut game.properties_in_bank,
            &game.properties,
            player_info,
        )
        .is_err()
        {
            player_info.penalty += 1;
            return;
        };
    }

    // if a player on the field that can't be sold (for example, jail)
    if let Some((account, _, price, _)) = game.properties[position as usize].as_mut() {
        if !account.is_zero() {
            player_info.penalty += 1;
            return;
        }
        // if a player has not enough balance
        if player_info.balance < *price {
            player_info.penalty += 1;
            return;
        }
        player_info.balance -= *price;
        *account = msg::source();
    } else {
        player_info.penalty += 1;
        return;
    };
    player_info.cells.insert(position);
    game.ownership[position as usize] = player;
    player_info.round = game.round;
}

pub fn pay_rent(
    game: &mut Game,
    properties_for_sale: Option<Vec<u8>>,
) {
    let player = game.current_player;
    let player_info = game.players.get_mut(&player).expect("Can't be None");
    if let Some(properties) = properties_for_sale {
        if sell_property(
            &game.admin_id,
            &mut game.ownership,
            &properties,
            &mut game.properties_in_bank,
            &game.properties,
            player_info,
        )
        .is_err()
        {
            return;
        };
    }

    let position = player_info.position;
    let account = game.ownership[position as usize];

    if account == player {
        player_info.penalty += 1;
        return;
    }

    let rent = if let Some((_, _, _, rent)) = game.properties[position as usize] {
        rent
    } else {
        0
    };
    if rent == 0 {
        player_info.penalty += 1;
        return;
    };

    if player_info.balance < rent {
        player_info.penalty += 1;
        return;
    }
    player_info.balance -= rent;
    player_info.debt = 0;
    player_info.round = game.round;
    game.players.entry(account).and_modify(|player_info| {
        player_info.balance = player_info.balance.saturating_add(rent)
    });
}
