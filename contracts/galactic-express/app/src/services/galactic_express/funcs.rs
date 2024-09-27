use crate::services::galactic_express::{
    Event, Game, GameError, HaltReason, Participant, Random, Results, Stage, Storage, Turn,
    Weather, MAX_FUEL, MAX_PARTICIPANTS, MAX_PAYLOAD, PENALTY_LEVEL, REWARD, TURNS, TURN_ALTITUDE,
};
use gstd::{collections::HashMap, msg};
use sails_rs::prelude::*;

pub fn create_new_session(storage: &mut Storage, name: String) -> Result<Event, GameError> {
    let msg_src = msg::source();
    let msg_value = msg::value();

    if storage.player_to_game_id.contains_key(&msg_src) {
        return Err(GameError::SeveralRegistrations);
    }

    let game = storage.games.entry(msg_src).or_insert_with(|| Game {
        admin: msg_src,
        admin_name: name,
        bid: msg_value,
        ..Default::default()
    });

    let stage = &mut game.stage;

    match stage {
        Stage::Registration(participants) => {
            participants.clear();
        }
        Stage::Results { .. } => *stage = Stage::Registration(HashMap::new()),
    }

    let mut random = Random::new()?;

    game.weather = match random.next() % (Weather::Tornado as u8 + 1) {
        0 => Weather::Clear,
        1 => Weather::Cloudy,
        2 => Weather::Rainy,
        3 => Weather::Stormy,
        4 => Weather::Thunder,
        5 => Weather::Tornado,
        _ => unreachable!(),
    };
    game.altitude = random.generate(TURN_ALTITUDE.0, TURN_ALTITUDE.1) * TURNS as u16;
    game.reward = random.generate(REWARD.0, REWARD.1);
    storage.player_to_game_id.insert(msg_src, msg_src);

    Ok(Event::NewSessionCreated {
        altitude: game.altitude,
        weather: game.weather,
        reward: game.reward,
        bid: msg_value,
    })
}

pub fn cancel_game(storage: &mut Storage) -> Result<Event, GameError> {
    let msg_src = msg::source();
    let game = storage.games.get(&msg_src).ok_or(GameError::NoSuchGame)?;

    match &game.stage {
        Stage::Registration(players) => {
            players.iter().for_each(|(id, _)| {
                send_value(*id, game.bid);
                storage.player_to_game_id.remove(id);
            });
        }
        Stage::Results(results) => {
            results.rankings.iter().for_each(|(id, _)| {
                storage.player_to_game_id.remove(id);
            });
        }
    }

    storage.player_to_game_id.remove(&msg_src);
    storage.games.remove(&msg_src);
    Ok(Event::GameCanceled)
}

pub fn leave_game(storage: &mut Storage) -> Result<Event, GameError> {
    let msg_src = msg::source();
    storage.player_to_game_id.remove(&msg_src);
    Ok(Event::GameLeft)
}

pub fn register(
    storage: &mut Storage,
    creator: ActorId,
    participant: Participant,
) -> Result<Event, GameError> {
    let msg_source = msg::source();
    let msg_value = msg::value();
    let reply = register_for_session(storage, creator, participant, msg_source, msg_value);
    if reply.is_err() {
        send_value(msg_source, msg_value);
    }
    reply
}

fn register_for_session(
    storage: &mut Storage,
    creator: ActorId,
    participant: Participant,
    msg_source: ActorId,
    msg_value: u128,
) -> Result<Event, GameError> {
    if storage.player_to_game_id.contains_key(&msg_source) {
        return Err(GameError::SeveralRegistrations);
    }

    if let Some(game) = storage.games.get_mut(&creator) {
        if msg_value != game.bid {
            return Err(GameError::WrongBid);
        }
        if let Stage::Results(_) = game.stage {
            return Err(GameError::SessionEnded);
        }

        let participants = game.stage.mut_participants()?;

        if participants.contains_key(&msg_source) {
            return Err(GameError::AlreadyRegistered);
        }

        if participants.len() >= MAX_PARTICIPANTS - 1 {
            return Err(GameError::SessionFull);
        }

        participant.check()?;
        participants.insert(msg_source, participant.clone());
        storage.player_to_game_id.insert(msg_source, creator);

        Ok(Event::Registered(msg_source, participant))
    } else {
        Err(GameError::NoSuchGame)
    }
}

pub fn cancel_register(storage: &mut Storage) -> Result<Event, GameError> {
    let msg_source = msg::source();

    let creator = storage
        .player_to_game_id
        .get(&msg_source)
        .ok_or(GameError::Unregistered)?;
    let game = storage
        .games
        .get_mut(creator)
        .ok_or(GameError::NoSuchGame)?;

    if msg_source != game.admin {
        let participants = game.stage.mut_participants()?;
        if participants.contains_key(&msg_source) {
            send_value(msg_source, game.bid);
            participants.remove(&msg_source).expect("Critical error");
            storage.player_to_game_id.remove(&msg_source);
        } else {
            return Err(GameError::NoSuchPlayer);
        }
        Ok(Event::RegistrationCanceled)
    } else {
        Err(GameError::NotForAdmin)
    }
}

pub fn delete_player(storage: &mut Storage, player_id: ActorId) -> Result<Event, GameError> {
    let msg_source = msg::source();

    if let Some(game) = storage.games.get_mut(&msg_source) {
        if let Stage::Results(_) = game.stage {
            return Err(GameError::SessionEnded);
        }

        let participants = game.stage.mut_participants()?;

        if participants.contains_key(&player_id) {
            send_value(player_id, game.bid);
            participants.remove(&player_id).expect("Critical error");
            storage.player_to_game_id.remove(&player_id);
        } else {
            return Err(GameError::NoSuchPlayer);
        }

        Ok(Event::PlayerDeleted { player_id })
    } else {
        Err(GameError::NoSuchGame)
    }
}

pub fn start_game(
    storage: &mut Storage,
    fuel_amount: u8,
    payload_amount: u8,
) -> Result<Event, GameError> {
    let msg_source = msg::source();

    let game = storage
        .games
        .get_mut(&msg_source)
        .ok_or(GameError::NoSuchGame)?;

    if fuel_amount > MAX_FUEL || payload_amount > MAX_PAYLOAD {
        return Err(GameError::FuelOrPayloadOverload);
    }
    let participant = Participant {
        id: msg_source,
        name: game.admin_name.clone(),
        fuel_amount,
        payload_amount,
    };

    let participants = game.stage.mut_participants()?;

    if participants.is_empty() {
        return Err(GameError::NotEnoughParticipants);
    }
    participants.insert(msg_source, participant);

    let mut random = Random::new()?;
    let mut turns = HashMap::new();

    for (actor, participant) in participants.into_iter() {
        let mut actor_turns = Vec::with_capacity(TURNS);
        let mut remaining_fuel = participant.fuel_amount;

        for turn_index in 0..TURNS {
            match turn(
                turn_index,
                remaining_fuel,
                &mut random,
                game.weather,
                participant.payload_amount,
            ) {
                Ok(fuel_left) => {
                    remaining_fuel = fuel_left;

                    actor_turns.push(Turn::Alive {
                        fuel_left,
                        payload_amount: participant.payload_amount,
                    });
                }
                Err(halt_reason) => {
                    actor_turns.push(Turn::Destroyed(halt_reason));

                    break;
                }
            }
        }

        turns.insert(*actor, actor_turns);
    }

    let mut scores: Vec<(ActorId, u128)> = turns
        .iter()
        .map(|(actor, turns)| {
            let last_turn = turns.last().expect("there must be at least 1 turn");

            (
                *actor,
                match last_turn {
                    Turn::Alive {
                        fuel_left,
                        payload_amount,
                    } => (*payload_amount as u128 + *fuel_left as u128) * game.altitude as u128,
                    Turn::Destroyed(_) => 0,
                },
            )
        })
        .collect();

    scores.sort_by(|(_, score_a), (_, score_b)| score_a.cmp(score_b));

    let mut io_turns: Vec<Vec<(ActorId, Turn)>> = vec![vec![]; TURNS];

    for (i, io_turn) in io_turns.iter_mut().enumerate().take(TURNS) {
        for (actor, actor_turns) in &turns {
            let turn = actor_turns
                .get(i)
                .unwrap_or_else(|| actor_turns.last().expect("There must be at least 1 turn"));
            io_turn.push((*actor, *turn));
        }
    }

    let max_value = scores.iter().map(|(_, value)| value).max().unwrap();
    let winners: Vec<_> = scores
        .iter()
        .filter_map(|(actor_id, value)| {
            if value == max_value {
                Some(*actor_id)
            } else {
                None
            }
        })
        .collect();
    let prize = game.bid * scores.len() as u128 / winners.len() as u128;

    if game.bid != 0 {
        winners.iter().for_each(|id| {
            send_value(*id, prize);
        });
    }
    let participants = participants
        .iter()
        .map(|(id, participant)| (*id, participant.clone()))
        .collect();

    let results = Results {
        turns: io_turns,
        rankings: scores.clone(),
        participants,
    };
    game.stage = Stage::Results(results.clone());

    Ok(Event::GameFinished(results))
}

fn turn(
    turn: usize,
    remaining_fuel: u8,
    random: &mut Random,
    weather: Weather,
    payload: u8,
) -> Result<u8, HaltReason> {
    let new_remaining_fuel =
        match remaining_fuel.checked_sub((payload + 2 * weather as u8) / TURNS as u8) {
            Some(actual_fuel) => actual_fuel,
            None => return Err(HaltReason::FuelShortage),
        };

    match turn {
        0 => {
            // values in "chance" are transmitted as percentages
            if random.chance(3) {
                return Err(HaltReason::EngineFailure);
            }
            // this trap for someone who specified a lot of fuel
            if remaining_fuel >= PENALTY_LEVEL - 2 * weather as u8 && random.chance(10) {
                return Err(HaltReason::FuelOverload);
            }
        }
        1 => {
            // this trap for someone who specified a lot of payload
            if payload >= PENALTY_LEVEL - 2 * weather as u8 && random.chance(10) {
                return Err(HaltReason::PayloadOverload);
            }

            if random.chance(5 + weather as u8) {
                return Err(HaltReason::SeparationFailure);
            }
        }
        2 => {
            if random.chance(10 + weather as u8) {
                return Err(HaltReason::AsteroidCollision);
            }
        }
        _ => unreachable!(),
    }

    Ok(new_remaining_fuel)
}

fn send_value(destination: ActorId, value: u128) {
    if value != 0 {
        msg::send_with_gas(destination, "", 0, value).expect("Error in sending value");
    }
}
