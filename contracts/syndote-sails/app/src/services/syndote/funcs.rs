use crate::services::syndote::*;
use gstd::{errors::Error, ReservationId, ReservationIdExt};
use sails_rs::ActorId;

pub fn register(storage: &mut Storage, player: &ActorId) -> Result<Event, GameError> {
    storage.check_status(GameStatus::Registration)?;
    if storage.players.contains_key(player) {
        return Err(GameError::AlreadyRegistered);
    }
    storage.players.insert(
        *player,
        PlayerInfo {
            balance: INITIAL_BALANCE,
            ..Default::default()
        },
    );
    storage.players_queue.push(*player);
    if storage.players_queue.len() == NUMBER_OF_PLAYERS as usize {
        storage.game_status = GameStatus::Play;
    }
    Ok(Event::Registered)
}

pub fn reserve_gas(storage: &mut Storage) -> Result<Event, GameError> {
    let reservation_id =
        ReservationId::reserve(RESERVATION_AMOUNT, 600).expect("reservation across executions");

    storage.reservations.push(reservation_id);
    Ok(Event::GasReserved)
}

pub fn start_registration(storage: &mut Storage) -> Result<Event, GameError> {
    storage.check_status(GameStatus::Finished)?;
    storage.only_admin()?;
    let mut game_storage = Storage {
        admin: storage.admin,
        ..Default::default()
    };
    init_properties(&mut game_storage.properties, &mut game_storage.ownership);
    *storage = game_storage;
    Ok(Event::StartRegistration)
}

pub async fn play(storage: &mut Storage) -> Result<Event, GameError> {
    //self.check_status(GameStatus::Play);
    let msg_src = msg::source();
    if msg_src != storage.admin && msg_src != exec::program_id() {
        return Err(GameError::AccessDenied);
    }
    while storage.game_status == GameStatus::Play {
        if storage.players_queue.len() <= 1 {
            storage.winner = storage.players_queue[0];
            storage.game_status = GameStatus::Finished;

            return Ok(Event::GameFinished {
                winner: storage.winner,
            });
        }
        if exec::gas_available() <= GAS_FOR_ROUND {
            if let Some(id) = storage.reservations.pop() {
                let request =
                    ["Syndote".encode(), "Play".to_string().encode(), ().encode()].concat();

                msg::send_bytes_from_reservation(id, exec::program_id(), request, 0)
                    .expect("Error in sending message");

                return Ok(Event::NextRoundFromReservation);
            } else {
                panic!("GIVE ME MORE GAS");
            };
        }
        // // check penalty and debt of the players for the previous round
        // // if penalty is equal to 5 points we remove the player from the game
        // // if a player has a debt and he has not enough balance to pay it
        // // he is also removed from the game
        // bankrupt_and_penalty(
        //     &self.admin,
        //     &mut self.players,
        //     &mut self.players_queue,
        //     &mut self.properties,
        //     &mut self.properties_in_bank,
        //     &mut self.ownership,
        // );

        // if self.players_queue.len() <= 1 {
        //     self.winner = self.players_queue[0];
        //     self.game_status = GameStatus::Finished;
        //     msg::reply(
        //         GameEvent::GameFinished {
        //             winner: self.winner,
        //         },
        //         0,
        //     )
        //     .expect("Error in sending a reply `GameEvent::GameFinished`");
        //     break;
        // }
        storage.round = storage.round.wrapping_add(1);
        for player in storage.players_queue.clone() {
            storage.current_player = player;
            storage.current_step += 1;
            // we save the state before the player's step in case
            // the player's contract does not reply or is executed with a panic.
            // Then we roll back all the changes that the player could have made.
            let mut state = storage.clone();
            let player_info = storage
                .players
                .get_mut(&player)
                .expect("Cant be None: Get Player");

            // if a player is in jail we don't throw rolls for him
            let position = if player_info.in_jail {
                player_info.position
            } else {
                let (r1, r2) = get_rolls();
                //     debug!("ROOLS {:?} {:?}", r1, r2);
                let roll_sum = r1 + r2;
                (player_info.position + roll_sum) % NUMBER_OF_CELLS
            };
            // If a player is on a cell that belongs to another player
            // we write down a debt on him in the amount of the rent.
            // This is done in order to penalize the participant's contract
            // if he misses the rent
            let account = storage.ownership[position as usize];

            if account != player && account != ActorId::zero() {
                if let Some((_, _, _, rent)) = storage.properties[position as usize] {
                    player_info.debt = rent;
                }
            }
            player_info.position = position;
            player_info.in_jail = position == JAIL_POSITION;
            state.players.insert(player, player_info.clone());
            match position {
                0 => {
                    player_info.balance += NEW_CIRCLE;
                    player_info.round = storage.round;
                }
                // free cells (it can be lottery or penalty): TODO as a task on hackathon
                2 | 4 | 7 | 16 | 20 | 30 | 33 | 36 | 38 => {
                    player_info.round = storage.round;
                }
                _ => {
                    let reply = take_your_turn(&player, &state).await;

                    if reply.is_err() {
                        player_info.penalty = PENALTY;
                    }
                }
            }
            // check penalty and debt of the players for the previous round
            // if penalty is equal to 5 points we remove the player from the game
            // if a player has a debt and he has not enough balance to pay it
            // he is also removed from the game
            bankrupt_and_penalty(
                &storage.admin,
                &mut storage.players,
                &mut storage.players_queue,
                &storage.properties,
                &mut storage.properties_in_bank,
                &mut storage.ownership,
            );

            msg::send(
                storage.admin,
                Event::Step {
                    players: storage
                        .players
                        .iter()
                        .map(|(key, value)| (*key, value.clone()))
                        .collect(),
                    properties: storage.properties.clone(),
                    current_player: storage.current_player,
                    current_step: storage.current_step,
                    ownership: storage.ownership.clone(),
                },
                0,
            )
            .expect("Error in sending a message `GameEvent::Step`");
        }
    }
    Ok(Event::Played)
}

async fn take_your_turn(player: &ActorId, storage: &Storage) -> Result<Vec<u8>, Error> {
    let players: Vec<_> = storage.players.clone().into_iter().collect();

    let request = [
        "Player".encode(),
        "YourTurn".to_string().encode(),
        (players, storage.properties.clone()).encode(),
    ]
    .concat();

    msg::send_bytes_for_reply(*player, request, 0, 0)
        .expect("Error on sending `YourTurn` message")
        .up_to(Some(WAIT_DURATION))
        .expect("Invalid wait duration.")
        .await
}

pub fn throw_roll(
    storage: &mut Storage,
    pay_fine: bool,
    properties_for_sale: Option<Vec<u8>>,
) -> Result<Event, GameError> {
    storage.only_player()?;
    let player_info =
        match get_player_info(&storage.current_player, &mut storage.players, storage.round) {
            Ok(player_info) => player_info,
            Err(_) => {
                return Ok(Event::StrategicError);
            }
        };

    // If a player is not in the jail
    if !player_info.in_jail {
        //     debug!("PENALTY: PLAYER IS NOT IN JAIL");
        player_info.penalty += 1;
        return Ok(Event::StrategicError);
    }

    if let Some(properties) = properties_for_sale {
        if sell_property(
            &storage.admin,
            &mut storage.ownership,
            &properties,
            &mut storage.properties_in_bank,
            &storage.properties,
            player_info,
        )
        .is_err()
        {
            return Ok(Event::StrategicError);
        };
    }

    let (r1, r2) = get_rolls();
    if r1 == r2 {
        player_info.in_jail = false;
        player_info.position = r1 + r2;
    } else if pay_fine {
        if player_info.balance < FINE {
            player_info.penalty += 1;
            return Ok(Event::StrategicError);
        }
        player_info.balance -= FINE;
        player_info.in_jail = false;
    }
    player_info.round = storage.round;
    Ok(Event::Jail {
        in_jail: player_info.in_jail,
        position: player_info.position,
    })
}

pub fn add_gear(
    storage: &mut Storage,
    properties_for_sale: Option<Vec<u8>>,
) -> Result<Event, GameError> {
    storage.only_player()?;
    let player_info =
        match get_player_info(&storage.current_player, &mut storage.players, storage.round) {
            Ok(player_info) => player_info,
            Err(_) => {
                return Ok(Event::StrategicError);
            }
        };

    if let Some(properties) = properties_for_sale {
        if sell_property(
            &storage.admin,
            &mut storage.ownership,
            &properties,
            &mut storage.properties_in_bank,
            &storage.properties,
            player_info,
        )
        .is_err()
        {
            return Ok(Event::StrategicError);
        };
    }

    // if player did not check his balance itself
    if player_info.balance < COST_FOR_UPGRADE {
        //  debug!("PENALTY: NOT ENOUGH BALANCE FOR UPGRADE");
        player_info.penalty += 1;
        return Ok(Event::StrategicError);
    }

    let position = player_info.position;

    let gears = if let Some((account, gears, _, _)) = &mut storage.properties[position as usize] {
        if account != &msg::source() {
            //       debug!("PENALTY: TRY TO UPGRADE NOT OWN CELL");
            player_info.penalty += 1;
            return Ok(Event::StrategicError);
        }
        gears
    } else {
        player_info.penalty += 1;
        return Ok(Event::StrategicError);
    };

    // maximum amount of gear is reached
    if gears.len() == 3 {
        //        debug!("PENALTY: MAXIMUM AMOUNT OF GEARS ON CELL");
        player_info.penalty += 1;
        return Ok(Event::StrategicError);
    }

    gears.push(Gear::Bronze);
    player_info.balance -= COST_FOR_UPGRADE;
    player_info.round = storage.round;
    Ok(Event::StrategicSuccess)
}

pub fn upgrade(
    storage: &mut Storage,
    properties_for_sale: Option<Vec<u8>>,
) -> Result<Event, GameError> {
    storage.only_player()?;
    let player_info =
        match get_player_info(&storage.current_player, &mut storage.players, storage.round) {
            Ok(player_info) => player_info,
            Err(_) => {
                return Ok(Event::StrategicError);
            }
        };

    if let Some(properties) = properties_for_sale {
        if sell_property(
            &storage.admin,
            &mut storage.ownership,
            &properties,
            &mut storage.properties_in_bank,
            &storage.properties,
            player_info,
        )
        .is_err()
        {
            return Ok(Event::StrategicError);
        };
    }

    // if player did not check his balance itself
    if player_info.balance < COST_FOR_UPGRADE {
        //       debug!("PENALTY: NOT ENOUGH BALANCE FOR UPGRADE");
        player_info.penalty += 1;
        return Ok(Event::StrategicError);
    }

    let position = player_info.position;

    if let Some((account, gears, price, rent)) = &mut storage.properties[position as usize] {
        if account != &msg::source() {
            player_info.penalty += 1;
            return Ok(Event::StrategicError);
        }
        // if nothing to upgrade
        if gears.is_empty() {
            //        debug!("PENALTY: NOTHING TO UPGRADE");
            player_info.penalty += 1;
            return Ok(Event::StrategicError);
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
        return Ok(Event::StrategicError);
    };

    player_info.balance -= COST_FOR_UPGRADE;
    player_info.round = storage.round;
    Ok(Event::StrategicSuccess)
}

pub fn buy_cell(
    storage: &mut Storage,
    properties_for_sale: Option<Vec<u8>>,
) -> Result<Event, GameError> {
    storage.only_player()?;
    let player_info =
        match get_player_info(&storage.current_player, &mut storage.players, storage.round) {
            Ok(player_info) => player_info,
            Err(_) => {
                return Ok(Event::StrategicError);
            }
        };
    let position = player_info.position;

    if let Some(properties) = properties_for_sale {
        if sell_property(
            &storage.admin,
            &mut storage.ownership,
            &properties,
            &mut storage.properties_in_bank,
            &storage.properties,
            player_info,
        )
        .is_err()
        {
            return Ok(Event::StrategicError);
        };
    }

    // if a player on the field that can't be sold (for example, jail)
    if let Some((account, _, price, _)) = storage.properties[position as usize].as_mut() {
        if account != &mut ActorId::zero() {
            //       debug!("PENALTY: THAT CELL IS ALREDY BOUGHT");
            player_info.penalty += 1;
            return Ok(Event::StrategicError);
        }
        // if a player has not enough balance
        if player_info.balance < *price {
            player_info.penalty += 1;
            //      debug!("PENALTY: NOT ENOUGH BALANCE FOR BUYING");
            return Ok(Event::StrategicError);
        }
        player_info.balance -= *price;
        *account = msg::source();
    } else {
        player_info.penalty += 1;
        //       debug!("PENALTY: THAT FIELD CAN'T BE SOLD");
        return Ok(Event::StrategicError);
    };
    player_info.cells.push(position);
    storage.ownership[position as usize] = msg::source();
    player_info.round = storage.round;
    Ok(Event::StrategicSuccess)
}

pub fn pay_rent(
    storage: &mut Storage,
    properties_for_sale: Option<Vec<u8>>,
) -> Result<Event, GameError> {
    storage.only_player()?;
    let player_info =
        match get_player_info(&storage.current_player, &mut storage.players, storage.round) {
            Ok(player_info) => player_info,
            Err(_) => {
                return Ok(Event::StrategicError);
            }
        };
    if let Some(properties) = properties_for_sale {
        if sell_property(
            &storage.admin,
            &mut storage.ownership,
            &properties,
            &mut storage.properties_in_bank,
            &storage.properties,
            player_info,
        )
        .is_err()
        {
            return Ok(Event::StrategicError);
        };
    }

    let position = player_info.position;
    let account = storage.ownership[position as usize];

    if account == msg::source() {
        player_info.penalty += 1;
        //   debug!("PENALTY: PAY RENT TO HIMSELF");
        return Ok(Event::StrategicError);
    }

    let (_, _, _, rent) = storage.properties[position as usize]
        .clone()
        .unwrap_or_default();
    if rent == 0 {
        //    debug!("PENALTY: CELL WITH NO PROPERTIES");
        player_info.penalty += 1;
        return Ok(Event::StrategicError);
    };

    if player_info.balance < rent {
        //    debug!("PENALTY: NOT ENOUGH BALANCE TO PAY RENT");
        player_info.penalty += 1;
        return Ok(Event::StrategicError);
    }
    player_info.balance -= rent;
    player_info.debt = 0;
    player_info.round = storage.round;
    storage
        .players
        .entry(account)
        .and_modify(|player_info| player_info.balance = player_info.balance.saturating_add(rent));
    Ok(Event::StrategicSuccess)
}

pub fn change_admin(storage: &mut Storage, admin: ActorId) -> Result<Event, GameError> {
    storage.only_admin()?;
    storage.admin = admin;
    Ok(Event::AdminChanged)
}
