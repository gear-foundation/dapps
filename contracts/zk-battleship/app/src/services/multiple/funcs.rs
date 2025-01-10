use super::{
    utils::{Result, Status, *},
    Event,
};
use crate::{
    admin::storage::configuration::Configuration,
    services::verify::{VerificationResult, VerificationVariables},
    single::Entity,
};
use gstd::{collections::HashMap, exec, msg, prelude::*, ActorId};

/// Creates a new multiplayer game and schedules its potential deletion.
///
/// # Arguments
///
/// * `games` - A mutable reference to the map storing multiple games.
/// * `game_pair` - A mutable reference to the map storing game pairs.
/// * `config` - The configuration settings for the game.
/// * `player` - The `ActorId` representing the player creating the game.
/// * `bid` - A 128-bit unsigned integer representing the bid amount for the game.
///
/// # Returns
///
/// * `Result<ActorId>` - Returns the `ActorId` of the player if the game is successfully created, or an error if the player is already in a game.
///
/// # Errors
///
/// * `Error::SeveralGames` - If the player is already involved in another game.
pub fn create_game(
    games: &mut MultipleGamesMap,
    game_pair: &mut GamePairsMap,
    config: Configuration,
    player: ActorId,
    name: String,
    bid: u128,
) -> Result<ActorId> {
    if game_pair.contains_key(&player) {
        return Err(Error::SeveralGames);
    }
    let create_time = exec::block_timestamp();
    let mut participants_data = HashMap::with_capacity(2);
    participants_data.insert(
        player,
        ParticipantInfo {
            name,
            board: vec![Entity::Unknown; 25],
            ship_hash: Vec::new(),
            total_shots: 0,
            succesfull_shots: 0,
        },
    );
    let game_instance = MultipleGame {
        admin: player,
        participants_data,
        create_time,
        start_time: None,
        status: Status::Registration,
        last_move_time: 0,
        bid,
    };
    games.insert(player, game_instance);
    game_pair.insert(player, player);

    let request = [
        "Multiple".encode(),
        "DeleteGame".to_string().encode(),
        (player, create_time).encode(),
    ]
    .concat();

    msg::send_bytes_with_gas_delayed(
        exec::program_id(),
        request,
        config.gas_for_delete_multiple_game,
        0,
        config.delay_for_delete_multiple_game,
    )
    .expect("Error in sending message");

    Ok(player)
}

/// Cancels a multiplayer game, returning bids to participants and removing game data.
///
/// # Arguments
///
/// * `games` - A mutable reference to the map storing multiple games.
/// * `game_pair` - A mutable reference to the map storing game pairs.
/// * `player` - The `ActorId` representing the player canceling the game.
///
pub fn cancel_game(
    games: &mut MultipleGamesMap,
    game_pair: &mut GamePairsMap,
    player: ActorId,
) -> Result<Event> {
    let game = games.get(&player).ok_or(Error::NoSuchGame)?;

    let event = match game.status {
        Status::VerificationPlacement(_) | Status::Registration => {
            return_bid_to_participants(game);
            game.participants_data.iter().for_each(|(id, _info)| {
                game_pair.remove(id);
            });
            games.remove(&player);
            Event::GameCanceled { game_id: player }
        }
        Status::Turn(_) | Status::PendingVerificationOfTheMove(_) => {
            let winner = game.get_opponent(&player);
            if game.bid != 0 {
                msg::send_with_gas(winner, "", 0, 2 * game.bid).expect("Error in sending value");
            }
            let total_time = exec::block_timestamp() - game.start_time.unwrap();
            let participants_info = game.participants_data.clone().into_iter().collect();
            let admin = game.admin;
            game_pair.remove(&admin);
            game_pair.remove(&winner);
            games.remove(&admin);
            Event::EndGame {
                admin,
                winner,
                total_time,
                participants_info,
                last_hit: None,
            }
        }
    };
    Ok(event)
}
/// Joins an existing multiplayer game as a participant.
///
/// # Arguments
///
/// * `games` - A mutable reference to the map storing multiple games.
/// * `game_pair` - A mutable reference to the map storing game pairs.
/// * `player` - The `ActorId` representing the player joining the game.
/// * `game_id` - The `ActorId` representing the ID of the game to join.
/// * `value` - A 128-bit unsigned integer representing the bid amount to join the game.
///
/// # Returns
///
/// * `Result<ActorId>` - Returns the `ActorId` of the player if successfully joined, otherwise returns an error.
///
/// # Errors
///
/// * `Error::SeveralGames` - If the player is already involved in another game.
/// * `Error::NoSuchGame` - If the game associated with `game_id` does not exist.
/// * `Error::WrongStatus` - If the game status is not `Status::Registration`.
/// * `Error::WrongBid` - If the bid amount does not match the game's bid.
pub fn join_game(
    games: &mut MultipleGamesMap,
    game_pair: &mut GamePairsMap,
    player: ActorId,
    name: String,
    game_id: ActorId,
    value: u128,
) -> Result<ActorId> {
    if game_pair.contains_key(&player) {
        return Err(Error::SeveralGames);
    }

    let game = games.get_mut(&game_id).ok_or(Error::NoSuchGame)?;

    if game.status != Status::Registration {
        return Err(Error::WrongStatus);
    }
    if game.bid != value {
        return Err(Error::WrongBid);
    }

    game.participants_data.insert(
        player,
        ParticipantInfo {
            name,
            board: vec![Entity::Unknown; 25],
            ship_hash: Vec::new(),
            total_shots: 0,
            succesfull_shots: 0,
        },
    );

    game_pair.insert(player, game_id);
    game.status = Status::VerificationPlacement(None);
    Ok(player)
}

/// Allows a player to leave a multiplayer game, handling different game statuses.
///
/// # Arguments
///
/// * `games` - A mutable reference to the map storing multiple games.
/// * `game_pair` - A mutable reference to the map storing game pairs.
/// * `player` - The `ActorId` representing the player leaving the game.
///
/// # Returns
///
/// * `Result<Event>` - Returns an event indicating the outcome of the player leaving the game.
///
/// # Errors
///
/// * `Error::NoSuchGame` - If the player's game does not exist.
/// * `Error::AccessDenied` - If the player attempts to leave when they are the owner of the game.
/// * `Error::WrongStatus` - If the game status does not allow the player to leave.
pub fn leave_game(
    games: &mut MultipleGamesMap,
    game_pair: &mut GamePairsMap,
    player: ActorId,
) -> Result<Event> {
    let game_id = game_pair.get(&player).ok_or(Error::NoSuchGame)?;

    if *game_id == player {
        return Err(Error::AccessDenied);
    }

    let game = games.get_mut(game_id).ok_or(Error::NoSuchGame)?;

    let event = match game.status {
        Status::VerificationPlacement(_) => {
            if game.bid != 0 {
                msg::send_with_gas(player, "", 0, game.bid).expect("Error in sending value");
            }
            game.participants_data.remove(&player);
            game.status = Status::Registration;
            Event::GameLeft { game_id: *game_id }
        }
        Status::Turn(_) | Status::PendingVerificationOfTheMove(_) => {
            let winner = *game_id;
            if game.bid != 0 {
                msg::send_with_gas(winner, "", 0, 2 * game.bid).expect("Error in sending value");
            }
            let total_time = exec::block_timestamp() - game.start_time.unwrap();
            let participants_info = game.participants_data.clone().into_iter().collect();
            let admin = game.admin;
            game_pair.remove(&winner);
            games.remove(&winner);
            Event::EndGame {
                admin,
                winner,
                total_time,
                participants_info,
                last_hit: None,
            }
        }
        Status::Registration => {
            return Err(Error::WrongStatus);
        }
    };

    // delete player from game pair
    game_pair.remove(&player);

    Ok(event)
}

pub fn delete_player(
    games: &mut MultipleGamesMap,
    game_pair: &mut GamePairsMap,
    player: ActorId,
    removable_player: ActorId,
) -> Result<Event> {
    let game_id = game_pair.get(&player).ok_or(Error::NoSuchGame)?;

    if *game_id != player {
        return Err(Error::AccessDenied);
    }

    let game = games.get_mut(game_id).ok_or(Error::NoSuchGame)?;

    match game.status {
        Status::VerificationPlacement(_) | Status::Registration => {
            if game.bid != 0 {
                msg::send_with_gas(removable_player, "", 0, game.bid)
                    .expect("Error in sending value");
            }
            game.participants_data.remove(&removable_player);
            game.status = Status::Registration;
        }
        _ => {
            return Err(Error::WrongStatus);
        }
    };
    let game_id = *game_id;
    // delete player from game pair
    game_pair.remove(&removable_player);

    Ok(Event::PlayerDeleted {
        game_id,
        removable_player,
    })
}

/// Sets the verification of ship placement for a player in a multiplayer game.
///
/// # Arguments
///
/// * `games` - A mutable reference to the map storing multiple games.
/// * `config` - The configuration settings for the game.
/// * `player` - The `ActorId` representing the player verifying ship placement.
/// * `game_id` - The `ActorId` representing the ID of the game.
/// * `hash` - A vector of bytes representing the hash of the ship placement.
/// * `block_timestamp` - The timestamp when the block was created.
///
/// # Returns
///
/// * `Result<Event>` - Returns an event indicating the outcome of the verification.
///
/// # Errors
///
/// * `Error::NoSuchGame` - If the game associated with `game_id` does not exist.
/// * `Error::AlreadyVerified` - If ship placement has already been verified by another player.
/// * `Error::WrongStatus` - If the game status does not allow setting ship placement verification.
pub fn set_verify_placement(
    games: &mut MultipleGamesMap,
    config: Configuration,
    player: ActorId,
    game_id: ActorId,
    hash: Vec<u8>,
    block_timestamp: u64,
) -> Result<Event> {
    let game = games.get_mut(&game_id).ok_or(Error::NoSuchGame)?;

    match &game.status {
        Status::VerificationPlacement(None) => {
            game.status = Status::VerificationPlacement(Some(player));
            let data = game
                .participants_data
                .get_mut(&player)
                .expect("At this status must be determined");
            data.ship_hash = hash;
            Ok(Event::PlacementVerified { admin: game.admin })
        }
        Status::VerificationPlacement(Some(verified_player)) if verified_player != &player => {
            game.status = Status::Turn(*verified_player);
            let data = game
                .participants_data
                .get_mut(&player)
                .expect("At this status must be determined");
            data.ship_hash = hash;
            game.start_time = Some(block_timestamp);
            game.last_move_time = block_timestamp;

            send_check_time_delayed_message(
                game_id,
                block_timestamp,
                config.gas_for_check_time,
                config.delay_for_check_time,
            );

            Ok(Event::PlacementVerified { admin: game.admin })
        }
        Status::VerificationPlacement(Some(_)) => Err(Error::AlreadyVerified),
        _ => Err(Error::WrongStatus),
    }
}

/// Executes a move in the game, processes the results of move verification, and updates the game state.
///
/// # Arguments
///
/// * `games` - A mutable reference to the `MultipleGamesMap`, representing the collection of ongoing games.
/// * `game_pair` - A mutable reference to the `GamePairsMap`, representing the mapping of players to their games.
/// * `config` - A configuration object containing gas limits and delay parameters.
/// * `player` - The `ActorId` of the player making the move.
/// * `game_id` - The `ActorId` of the game where the move is being made.
/// * `step` - An `Option<u8>` representing the move step (index on the game board). It should be `Some` if the player is making a move.
/// * `verification_result` - An `Option<VerificationResult>` representing the result of the move verification process, if available.
/// * `block_timestamp` - The current block timestamp used to track the timing of moves.
///
/// # Returns
///
/// * `Result<Event>` - Returns an event indicating the outcome of the move or the end of the game.
///
/// # Errors
///
/// * Returns `Error::NoSuchGame` if the game with the given `game_id` does not exist.
/// * Returns `Error::WrongStep` if the `step` value is out of bounds (greater than 24).
/// * Returns `Error::NotPlayer` if the player is not a participant in the game.
pub fn make_move(
    games: &mut MultipleGamesMap,
    game_pair: &mut GamePairsMap,
    config: Configuration,
    player: ActorId,
    game_id: ActorId,
    step: Option<u8>,
    verification_result: Option<VerificationResult>,
    block_timestamp: u64,
) -> Result<Event> {
    let game = games.get_mut(&game_id).ok_or(Error::NoSuchGame)?;

    let opponent = game.get_opponent(&player);
    let verified_result = if let Some(VerificationResult { res, hit }) = verification_result {
        game.shot(&player, hit, res);

        if game.check_end_game(&player) {
            msg::send_with_gas(opponent, "", 0, 2 * game.bid).expect("Error in sending value");
            let total_time = exec::block_timestamp() - game.start_time.unwrap();
            let participants_info: Vec<_> = game.participants_data.clone().into_iter().collect();
            let admin = game.admin;
            game.participants_data.iter().for_each(|(id, _info)| {
                game_pair.remove(id);
            });
            games.remove(&game_id);
            return Ok(Event::EndGame {
                admin,
                winner: opponent,
                total_time,
                participants_info,
                last_hit: Some(hit),
            });
        }
        let verified_result = match res {
            0 => Some((hit, StepResult::Missed)),
            1 => Some((hit, StepResult::Injured)),
            2 => Some((hit, StepResult::Killed)),
            _ => unimplemented!(),
        };
        if res != 0 {
            game.status = Status::Turn(opponent);
            game.last_move_time = block_timestamp;
            return Ok(Event::MoveMade {
                game_id,
                step,
                verified_result,
                turn: opponent,
            });
        }
        verified_result
    } else {
        None
    };

    let step = step.expect("`step` must not be None at this stage");
    if step > 24 {
        return Err(Error::WrongStep);
    }

    let data = game
        .participants_data
        .get_mut(&player)
        .ok_or(Error::NotPlayer)?;
    data.total_shots += 1;
    game.status = Status::PendingVerificationOfTheMove((opponent, step));
    game.last_move_time = block_timestamp;
    send_check_time_delayed_message(
        game_id,
        block_timestamp,
        config.gas_for_check_time,
        config.delay_for_check_time,
    );

    Ok(Event::MoveMade {
        game_id,
        step: Some(step),
        verified_result,
        turn: opponent,
    })
}

// Checks if a player is participating in a game and if the game is in the correct state for verifying ship placement.
pub fn check_game_for_verify_placement(
    games: &MultipleGamesMap,
    player: ActorId,
    game_id: ActorId,
) -> Result<()> {
    let game = games.get(&game_id).ok_or(Error::NoSuchGame)?;

    if !game.participants_data.contains_key(&player) {
        return Err(Error::NotPlayer);
    }

    if !matches!(game.status, Status::VerificationPlacement(_)) {
        return Err(Error::WrongStatus);
    }

    Ok(())
}

/// Checks if a player is allowed to verify a move based on the current game state and provided inputs.
pub fn check_game_for_move(
    games: &MultipleGamesMap,
    game_id: ActorId,
    player: ActorId,
    verify_variables: Option<VerificationVariables>,
    step: Option<u8>,
) -> Result<()> {
    let game = games.get(&game_id).ok_or(Error::NoSuchGame)?;
    if matches!(game.status, Status::PendingVerificationOfTheMove(_)) && verify_variables.is_none()
    {
        return Err(Error::WrongStatus);
    }
    if matches!(game.status, Status::Turn(_)) && step.is_none() {
        return Err(Error::WrongStatus);
    }
    if let Some(VerificationVariables {
        proof_bytes: _,
        public_input,
    }) = verify_variables
    {
        if game.status != Status::PendingVerificationOfTheMove((player, public_input.hit)) {
            return Err(Error::WrongStatus);
        }
        if public_input.out == 0 && step.is_none() {
            return Err(Error::StepIsNotTaken);
        }
        if game
            .participants_data
            .get(&player)
            .expect("At this status must be determined")
            .ship_hash
            != public_input.hash
        {
            return Err(Error::WrongShipsHash);
        }
    }

    Ok(())
}

/// Checks the timing of the last move in a game and determines if the game should end due to inactivity.
///
/// This function performs the following steps:
/// 1. Retrieves the game instance from the `games` map using the provided `game_id`.
/// 2. If the `last_move_time` of the game matches the provided `check_time`, it concludes that no move
///    has been made since the last check, indicating a timeout situation.
/// 3. Calculates the total time the game has been running by subtracting the start time from the current block timestamp.
/// 4. Retrieves the participants' information and determines the winner and loser based on the current game status:
///    - If the game is in a turn-based status, the opponent of the player who was supposed to move is declared the winner.
/// 5. Sends an `EndGame` event to both the winner and the loser, transferring the bid amount to the winner and notifying both players.
/// 6. Removes the game from both `games` and `game_pair` maps to clean up the game's data.
///
/// # Arguments
///
/// * `games` - A mutable reference to the map storing all the active games.
/// * `game_pair` - A mutable reference to the map storing pairs of players and their associated game IDs.
/// * `game_id` - The unique identifier of the game being checked.
/// * `check_time` - The timestamp of the last move, used to check if a timeout has occurred.
pub fn check_out_timing(
    games: &mut MultipleGamesMap,
    game_pair: &mut GamePairsMap,
    game_id: ActorId,
    check_time: u64,
) -> Result<Option<Event>> {
    let game = games.get_mut(&game_id).ok_or(Error::NoSuchGame)?;
    if game.last_move_time == check_time {
        let total_time = exec::block_timestamp() - game.start_time.unwrap();
        let participants_info = game.participants_data.clone().into_iter().collect();
        let (winner, loser) = match game.status {
            Status::Turn(id) => (game.get_opponent(&id), id),
            Status::PendingVerificationOfTheMove((id, _)) => (game.get_opponent(&id), id),
            _ => unimplemented!(),
        };
        let event = Event::EndGame {
            admin: game.admin,
            winner,
            total_time,
            participants_info,
            last_hit: None,
        };
        msg::send_with_gas(winner, "", 0, 2 * game.bid).expect("Error send message");
        msg::send_with_gas(loser, "", 0, 0).expect("Error send message");
        game_pair.remove(&winner);
        game_pair.remove(&loser);
        games.remove(&game_id);
        return Ok(Some(event));
    }

    Ok(None)
}
/// Deletes a game from storage based on the provided `game_id` and `create_time`.
///
/// # Arguments
///
/// * `games` - Mutable reference to the map storing multiple games.
/// * `game_pair` - Mutable reference to the map storing game pairs.
/// * `game_id` - The `ActorId` representing the ID of the game to delete.
/// * `create_time` - The timestamp indicating when the game was created.
///
/// # Returns
///
/// * `Result<Event>` - Returns `Ok` with an event indicating the game deletion.
///
/// # Errors
///
/// * `Error::NoSuchGame` - If the game associated with `game_id` does not exist.
///
/// This function deletes a game from the `games` map and `game_pair` map based on matching
/// `game_id` and `create_time`. If the deletion conditions are met, it returns the event
/// `Event::GameDeleted` with the `game_id`. Additionally, it returns the bid to participants
/// if the game had a bid associated with it.
pub fn delete_game(
    games: &mut MultipleGamesMap,
    game_pair: &mut GamePairsMap,
    game_id: ActorId,
    create_time: u64,
) -> Result<Event> {
    let game = games.get_mut(&game_id).ok_or(Error::NoSuchGame)?;

    if game.create_time == create_time {
        return_bid_to_participants(game);
        game.participants_data.iter().for_each(|(id, _info)| {
            game_pair.remove(id);
        });
        games.remove(&game_id);
    }
    Ok(Event::GameDeleted { game_id })
}

/// Returns the bid amount to all participants in the game if there was a bid associated with it.
///
/// # Arguments
///
/// * `game` - Immutable reference to the `MultipleGame` instance representing the game.
///
/// This function iterates through the participants of the game and sends the bid amount back
/// to each participant using `msg::send_with_gas`.
fn return_bid_to_participants(game: &MultipleGame) {
    if game.bid != 0 {
        game.participants_data.iter().for_each(|(id, _info)| {
            msg::send_with_gas(*id, "", 0, game.bid).expect("Error in sending value");
        });
    }
}

/// Sends a delayed message for checking game time out using the provided parameters.
///
/// # Arguments
///
/// * `game_id` - The `ActorId` representing the ID of the game to check timing for.
/// * `block_timestamp` - The current block timestamp.
/// * `gas_limit` - Gas limit for sending the message.
/// * `delay` - Delay in seconds for sending the message.
///
/// This function constructs a request message for checking game time out and sends it
/// using `msg::send_bytes_with_gas_delayed`.
fn send_check_time_delayed_message(
    game_id: ActorId,
    block_timestamp: u64,
    gas_limit: u64,
    delay: u32,
) {
    let request = [
        "Multiple".encode(),
        "CheckOutTiming".to_string().encode(),
        (game_id, block_timestamp).encode(),
    ]
    .concat();
    msg::send_bytes_with_gas_delayed(exec::program_id(), request, gas_limit, 0, delay)
        .expect("Error in sending message");
}
