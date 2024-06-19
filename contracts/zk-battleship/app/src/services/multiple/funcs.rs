use super::{
    utils::{Result, Status, *},
    Event,
};
use crate::admin::storage::configuration::Configuration;
use crate::single::Entity;
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
/// # Returns
///
/// * `Result<ActorId>` - Returns the `ActorId` of the canceled game's player if the game exists, otherwise returns an error.
///
/// # Errors
///
/// * `Error::NoSuchGame` - If the game associated with the player does not exist.
pub fn cancel_game(
    games: &mut MultipleGamesMap,
    game_pair: &mut GamePairsMap,
    player: ActorId,
) -> Result<ActorId> {
    let game = games.get(&player).ok_or(Error::NoSuchGame)?;

    return_bid_to_participants(game);
    game.participants_data.iter().for_each(|(id, _info)| {
        game_pair.remove(id);
    });
    games.remove(&player);

    Ok(player)
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
            game_pair.remove(&winner);
            games.remove(&winner);
            Event::EndGame {
                winner,
                total_time,
                participants_info,
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
            Ok(Event::PlacementVerified)
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
                false,
                config.gas_for_check_time,
                config.delay_for_check_time,
            );

            Ok(Event::PlacementVerified)
        }
        Status::VerificationPlacement(Some(_)) => Err(Error::AlreadyVerified),
        _ => Err(Error::WrongStatus),
    }
}

/// Executes a move by the player in a multiplayer game, sending a message to the opponent
/// and updating game state accordingly.
///
/// # Arguments
///
/// * `games` - A mutable reference to the map storing multiple games.
/// * `config` - The configuration settings for the game.
/// * `player` - The `ActorId` representing the player making the move.
/// * `game_id` - The `ActorId` representing the ID of the game.
/// * `step` - The step or move to be executed, representing a shot at the opponent's board.
/// * `block_timestamp` - The timestamp when the block was created.
///
/// # Returns
///
/// * `Result<Event>` - Returns an event indicating the outcome of the move.
///
/// # Errors
///
/// * `Error::WrongStep` - If the `step` exceeds the allowable range (0-24).
/// * `Error::NoSuchGame` - If the game associated with `game_id` does not exist.
/// * `Error::WrongStatus` - If the game status does not permit the player to make a move.
/// * `Error::NotPlayer` - If the `player` is not recognized as a participant in the game.
pub fn make_move(
    games: &mut MultipleGamesMap,
    config: Configuration,
    player: ActorId,
    game_id: ActorId,
    step: u8,
    block_timestamp: u64,
) -> Result<Event> {
    if step > 24 {
        return Err(Error::WrongStep);
    }

    let game = games.get_mut(&game_id).ok_or(Error::NoSuchGame)?;
    if game.status != Status::Turn(player) {
        return Err(Error::WrongStatus);
    }
    let data = game
        .participants_data
        .get_mut(&player)
        .ok_or(Error::NotPlayer)?;
    data.total_shots += 1;
    let opponent = game.get_opponent(&player);
    msg::send(opponent, Event::MoveMade { step }, 0).expect("Error send message");
    game.status = Status::PendingVerificationOfTheMove((opponent, step));
    game.last_move_time = block_timestamp;
    send_check_time_delayed_message(
        game_id,
        block_timestamp,
        false,
        config.gas_for_check_time * 2,
        config.delay_for_check_time,
    );

    Ok(Event::MoveMade { step })
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
pub fn check_game_for_verify_move(
    games: &MultipleGamesMap,
    game_id: ActorId,
    player: ActorId,
    hit: u8,
    hash: Vec<u8>,
) -> Result<()> {
    let game = games.get(&game_id).ok_or(Error::NoSuchGame)?;

    if game.status != Status::PendingVerificationOfTheMove((player, hit)) {
        return Err(Error::WrongStatus);
    }
    if game
        .participants_data
        .get(&player)
        .expect("At this status must be determined")
        .ship_hash
        != hash
    {
        return Err(Error::WrongShipsHash);
    }

    Ok(())
}

/// Checks the timing of game moves and handles game state transitions based on the provided parameters.
///
/// # Arguments
///
/// * `games` - Mutable reference to the map storing multiple games.
/// * `game_pair` - Mutable reference to the map storing game pairs.
/// * `config` - Configuration settings for gas and delay parameters.
/// * `game_id` - The `ActorId` representing the ID of the game to be checked.
/// * `check_time` - The timestamp to check against the last move time in the game.
/// * `repeated_pass` - Boolean indicating if the check is a repeated pass.
///
/// # Returns
///
/// * `Result<()>` - Returns `Ok` if the timing check is successful and game state transitions are handled accordingly.
///
/// # Errors
///
/// * `Error::NoSuchGame` - If the game associated with `game_id` does not exist.
///
/// This function checks the timing of game moves and transitions game states based on the provided parameters. If the `check_time`
/// matches the last move time in the game, it either transitions to the next turn or ends the game, depending on whether `repeated_pass`
/// is `false` or `true`, respectively. It also manages participant data and game removal from storage when necessary.
pub fn check_out_timing(
    games: &mut MultipleGamesMap,
    game_pair: &mut GamePairsMap,
    config: Configuration,
    game_id: ActorId,
    check_time: u64,
    repeated_pass: bool,
) -> Result<()> {
    let game = games.get_mut(&game_id).ok_or(Error::NoSuchGame)?;
    if game.last_move_time == check_time {
        if !repeated_pass {
            let player = match game.status {
                Status::Turn(id) => id,
                Status::PendingVerificationOfTheMove((id, _)) => id,
                _ => unimplemented!(),
            };
            let following_player = game.get_opponent(&player);
            game.status = Status::Turn(following_player);
            let block_timestamp = exec::block_timestamp();
            game.last_move_time = block_timestamp;
            send_check_time_delayed_message(
                game_id,
                block_timestamp,
                true,
                config.gas_for_check_time,
                config.delay_for_check_time,
            )
        } else {
            return_bid_to_participants(game);
            game.participants_data.iter().for_each(|(id, _info)| {
                game_pair.remove(id);
            });
            games.remove(&game_id);
        }
    }

    Ok(())
}

/// Verifies the move made by a player in a multiple-player game and updates the game state accordingly.
///
/// # Arguments
///
/// * `games` - Mutable reference to the map storing multiple games.
/// * `game_pair` - Mutable reference to the map storing game pairs.
/// * `game_id` - The `ActorId` representing the ID of the game to verify the move in.
/// * `player` - The `ActorId` representing the ID of the player making the move.
/// * `res` - The result of the move (0 for miss, 1 for hit).
/// * `hit` - The specific position or step of the move.
///
/// # Returns
///
/// * `Result<Event>` - Returns `Ok` with an event indicating the result of the verified move.
///
/// # Errors
///
/// * `Error::NoSuchGame` - If the game associated with `game_id` does not exist.
///
/// This function verifies a move made by a player in a multiple-player game, updates the game state based on the move result (`res`),
/// and checks if the game has ended. If the game ends, it sends the appropriate message to the winner and removes the game from storage.
/// It transitions the game state to the next player's turn if the game continues.
pub fn verified_move(
    games: &mut MultipleGamesMap,
    game_pair: &mut GamePairsMap,
    game_id: ActorId,
    player: ActorId,
    res: u8,
    hit: u8,
) -> Result<Event> {
    let game = games.get_mut(&game_id).ok_or(Error::NoSuchGame)?;
    game.shot(&player, hit, res);

    if game.check_end_game(&player) {
        let winner = game.get_opponent(&player);
        msg::send_with_gas(winner, "", 0, 2 * game.bid).expect("Error in sending value");
        let total_time = exec::block_timestamp() - game.start_time.unwrap();
        let participants_info = game.participants_data.clone().into_iter().collect();
        game.participants_data.iter().for_each(|(id, _info)| {
            game_pair.remove(id);
        });
        games.remove(&game_id);
        return Ok(Event::EndGame {
            winner,
            total_time,
            participants_info,
        });
    }
    game.status = Status::Turn(player);

    Ok(Event::MoveVerified {
        step: hit,
        result: res,
    })
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
    }
    games.remove(&game_id);
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
/// * `repeated_pass` - Boolean indicating whether it's a repeated pass for checking time.
/// * `gas_limit` - Gas limit for sending the message.
/// * `delay` - Delay in seconds for sending the message.
///
/// This function constructs a request message for checking game time out and sends it
/// using `msg::send_bytes_with_gas_delayed`.
fn send_check_time_delayed_message(
    game_id: ActorId,
    block_timestamp: u64,
    repeated_pass: bool,
    gas_limit: u64,
    delay: u32,
) {
    let request = [
        "Multiple".encode(),
        "CheckOutTiming".to_string().encode(),
        (game_id, block_timestamp, repeated_pass).encode(),
    ]
    .concat();
    msg::send_bytes_with_gas_delayed(exec::program_id(), request, gas_limit, 0, delay)
        .expect("Error in sending message");
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::services::multiple::funcs;
//     use utils::*;

//     #[test]
//     fn create_game() {
//         // Initializing thread logger.
//         let _ = env_logger::try_init();

//         // Creating game_pair_map.
//         let alice = alice();
//         let mut game_pair_map = game_pair_map([]);
//         assert!(game_pair_map.is_empty());
//         let mut multiple_game_map = multiple_game_map([]);
//         assert!(multiple_game_map.is_empty());

//         // # Test case #1.
//         // Ok: Create game
//         {
//             funcs::create_game(&mut multiple_game_map, &mut game_pair_map, alice, 100).unwrap();
//             assert_eq!(*game_pair_map.get(&alice).unwrap(), alice);
//             assert_eq!(
//                 *multiple_game_map.get(&alice).unwrap(),
//                 multiple_game(alice)
//             );
//         }
//         // # Test case #2.
//         // Error: Several games
//         {
//             let res = funcs::create_game(&mut multiple_game_map, &mut game_pair_map, alice, 100);
//             assert!(res.is_err_and(|err| err == Error::SeveralGames));
//         }
//     }

//     #[test]
//     fn cancel_game() {
//         // Initializing thread logger.
//         let _ = env_logger::try_init();

//         // Creating game_pair_map.
//         let alice = alice();
//         let bob = bob();
//         let mut game_pair_map = game_pair_map([(alice, alice), (bob, alice)]);
//         assert!(!game_pair_map.is_empty());
//         let mut game = multiple_game(alice);
//         game.second_player_board = Some((alice, vec![Entity::Unknown; 25]));
//         game.participants.1 = bob;
//         let mut multiple_game_map = multiple_game_map([(alice, game)]);
//         assert!(!multiple_game_map.is_empty());

//         // # Test case #1.
//         // Ok: Cancel game
//         {
//             funcs::cancel_game(&mut multiple_game_map, &mut game_pair_map, alice).unwrap();
//             assert!(game_pair_map.is_empty());
//             assert!(multiple_game_map.is_empty());
//         }
//         // # Test case #2.
//         // Error: No such game
//         {
//             let res = funcs::cancel_game(&mut multiple_game_map, &mut game_pair_map, alice);
//             assert!(res.is_err_and(|err| err == Error::NoSuchGame));
//         }
//     }

//     #[test]
//     fn join_game() {
//         // Initializing thread logger.
//         let _ = env_logger::try_init();

//         // Creating game_pair_map.
//         let alice = alice();
//         let bob = bob();
//         let mut pair_map = game_pair_map([(alice, alice), (bob, bob)]);
//         assert!(!pair_map.is_empty());
//         let mut game = multiple_game(alice);
//         let mut game_map = multiple_game_map([(alice, game.clone()), (bob, game.clone())]);
//         assert!(!game_map.is_empty());

//         // # Test case #1.
//         // Error: several games
//         {
//             let res = funcs::join_game(&mut game_map, &mut pair_map, bob, alice, 100);
//             assert!(res.is_err_and(|err| err == Error::SeveralGames));
//         }

//         // # Test case #2.
//         // Ok: join to game
//         let mut pair_map = game_pair_map([(alice, alice)]);
//         let mut game_map = multiple_game_map([(alice, game.clone())]);
//         {
//             funcs::join_game(&mut game_map, &mut pair_map, bob, alice, 100).unwrap();
//             game.second_player_board = Some((bob, vec![Entity::Unknown; 25]));
//             game.participants.1 = bob;
//             game.status = Status::VerificationPlacement(None);
//             assert_eq!(*pair_map.get(&bob).unwrap(), alice);
//             assert_eq!(*game_map.get(&alice).unwrap(), game);
//         }
//         // # Test case #3.
//         // Error: there's already a player registered
//         let john = john();
//         {
//             let res = funcs::join_game(&mut game_map, &mut pair_map, john, alice, 100);
//             assert!(res.is_err_and(|err| err == Error::WrongStatus));
//         }
//     }

//     #[test]
//     fn leave_game() {
//         // Initializing thread logger.
//         let _ = env_logger::try_init();

//         // Creating game_pair_map.
//         let alice = alice();
//         let bob = bob();
//         let mut pair_map = game_pair_map([(alice, alice), (bob, alice)]);
//         assert!(!pair_map.is_empty());
//         let mut game = multiple_game(alice);
//         game.second_player_board = Some((alice, vec![Entity::Unknown; 25]));
//         game.participants.1 = bob;
//         let mut game_map = multiple_game_map([(alice, game.clone())]);
//         assert!(!game_map.is_empty());

//         // # Test case #1.
//         // Error: wrong status
//         {
//             let res = funcs::leave_game(&mut game_map, &mut pair_map, bob);
//             assert!(res.is_err_and(|err| err == Error::WrongStatus));
//         }

//         // # Test case #2.
//         // Ok: leave game when status is verification placement
//         game.status = Status::VerificationPlacement(None);
//         let mut game_map = multiple_game_map([(alice, game.clone())]);
//         {
//             funcs::leave_game(&mut game_map, &mut pair_map, bob).unwrap();
//             game.second_player_board = None;
//             game.participants.1 = ActorId::zero();
//             game.status = Status::Registration;
//             assert_eq!(pair_map.len(), 1);
//             assert_eq!(*game_map.get(&alice).unwrap(), game);
//         }
//         // # Test case #3.
//         // Ok: leave game when status is turn
//         game.status = Status::Turn(alice);
//         let mut game_map = multiple_game_map([(alice, game.clone())]);
//         let mut pair_map = game_pair_map([(alice, alice), (bob, alice)]);
//         {
//             let event = funcs::leave_game(&mut game_map, &mut pair_map, bob).unwrap();
//             assert_eq!(event, Event::EndGame { winner: alice });
//             assert_eq!(pair_map.len(), 1);
//         }
//         // # Test case #4.
//         // Ok: leave game when status is game over
//         game.status = Status::GameOver(alice);
//         let mut game_map = multiple_game_map([(alice, game.clone())]);
//         let mut pair_map = game_pair_map([(alice, alice), (bob, alice)]);
//         {
//             let event = funcs::leave_game(&mut game_map, &mut pair_map, bob).unwrap();
//             assert_eq!(event, Event::GameLeft { game_id: alice });
//             assert_eq!(pair_map.len(), 1);
//         }
//     }

//     #[test]
//     fn set_verify_placement() {
//         // Initializing thread logger.
//         let _ = env_logger::try_init();

//         // Creating game_pair_map.
//         let alice = alice();
//         let bob = bob();
//         let mut game = multiple_game(alice);
//         let mut game_map = multiple_game_map([(alice, game.clone())]);
//         assert!(!game_map.is_empty());

//         // # Test case #1.
//         // Error: wrong status
//         {
//             let res = funcs::set_verify_placement(&mut game_map, alice, alice, 0);

//             assert!(res.is_err_and(|err| err == Error::WrongStatus));
//         }
//         // # Test case #2.
//         // Ok: status VerificationPlacement(None)
//         game.status = Status::VerificationPlacement(None);
//         let mut game_map = multiple_game_map([(alice, game.clone())]);
//         {
//             funcs::set_verify_placement(&mut game_map, alice, alice, 0).unwrap();
//             game.status = Status::VerificationPlacement(Some(alice));
//             assert_eq!(*game_map.get(&alice).unwrap(), game);
//         }
//         // # Test case #3.
//         // Error: a case where Alice wants to double verify ships
//         {
//             let res = funcs::set_verify_placement(&mut game_map, alice, alice, 0);
//             assert!(res.is_err_and(|err| err == Error::AlreadyVerified));
//         }
//         // # Test case #4.
//         // Ok: status VerificationPlacement(Some(alice))
//         {
//             funcs::set_verify_placement(&mut game_map, bob, alice, 0).unwrap();
//             game.status = Status::Turn(alice);
//             game.start_time = Some(0);
//             assert_eq!(*game_map.get(&alice).unwrap(), game);
//         }
//     }

//     mod utils {
//         use super::{GamePairsMap, MultipleGame, MultipleGamesMap};
//         use crate::multiple::Status;
//         use crate::single::Entity;
//         use gstd::{vec, ActorId};

//         pub fn game_pair_map<const N: usize>(content: [(ActorId, ActorId); N]) -> GamePairsMap {
//             content.into_iter().collect()
//         }
//         pub fn multiple_game_map<const N: usize>(
//             content: [(ActorId, MultipleGame); N],
//         ) -> MultipleGamesMap {
//             content.into_iter().collect()
//         }
//         pub fn multiple_game(player: ActorId) -> MultipleGame {
//             MultipleGame {
//                 first_player_board: (player, vec![Entity::Unknown; 25]),
//                 second_player_board: None,
//                 participants: (player, ActorId::zero()),
//                 ship_hashes: ((ActorId::zero(), vec![]), (ActorId::zero(), vec![])),
//                 start_time: None,
//                 end_time: None,
//                 status: Status::Registration,
//                 bid: 100,
//             }
//         }

//         pub fn alice() -> ActorId {
//             1u64.into()
//         }
//         pub fn bob() -> ActorId {
//             2u64.into()
//         }
//         pub fn john() -> ActorId {
//             3u64.into()
//         }
//     }
// }
