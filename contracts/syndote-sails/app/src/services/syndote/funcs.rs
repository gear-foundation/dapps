// use crate::services::syndote::{Storage, GameStatus, PlayerInfo, GameError, Event, init_properties, NUMBER_OF_PLAYERS, INITIAL_BALANCE, RESERVATION_AMOUNT, GAS_FOR_ROUND};
use crate::services::syndote::*;
use sails_rs::ActorId;
use gstd::{ReservationId, ReservationIdExt, errors::Error};

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
    let reservation_id = ReservationId::reserve(RESERVATION_AMOUNT, 600)
        .expect("reservation across executions");

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
                let request = [
                    "Syndote".encode(),
                    "Play".to_string().encode(),
                    ().encode(),
                ]
                .concat();
            
                msg::send_bytes_from_reservation(
                    id,
                    exec::program_id(),
                    request,
                    0,
                )
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
    let players: Vec<_> = storage
        .players
        .clone()
        .into_iter()
        .collect();

    let request = [
        "Player".encode(),
        "YourTurn".to_string().encode(),
        (players, storage.properties.clone()).encode(),
    ]
    .concat();

    msg::send_bytes_for_reply(
        *player,
        request,
        0,
        0,
    )
    .expect("Error on sending `YourTurn` message")
    .up_to(Some(WAIT_DURATION))
    .expect("Invalid wait duration.")
    .await
}
