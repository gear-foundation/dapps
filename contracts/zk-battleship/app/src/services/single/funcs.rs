use super::{
    utils::{Result, *},
    Event,
};
use crate::admin::storage::configuration::Configuration;
use crate::services::verify::{PublicMoveInput, VerificationResult};
use gstd::{exec, msg, prelude::*, ActorId};

static mut SEED: u8 = 0;

/// Function for saving the state of the beginning of a single-player game.
///
/// # Arguments
///
/// * `games` - A mutable reference to the map that stores single-player game instances.
/// * `player` - The `ActorId` representing the player starting the game.
/// * `hash` - A vector of bytes representing the hash of the player's ships' positions.
/// * `gas_limit` - The gas limit for the delayed delete game message.
/// * `delay` - The delay time in blocks for the delete game message.
///
/// # Returns
///
/// * `Result<()>` - Returns `Ok(())` if the game is successfully started and saved, otherwise returns an error.
///
/// This function generates the positions of the bot's ships, creates a new game instance with the provided player,
/// if the game exists it replaces it with a new one
/// hash, gas limit, and delay, and then inserts the game instance into the provided games map.
/// It also schedules a delayed message to delete the game after a specified delay.
pub fn start_single_game(
    games: &mut SingleGamesMap,
    player: ActorId,
    hash: Vec<u8>,
    config: Configuration,
    block_timestamp: u64,
) -> Result<()> {
    let bot_ships = generate_field();
    let game_instance = SingleGame {
        player_board: vec![Entity::Unknown; 25],
        ship_hash: hash,
        bot_ships,
        start_time: block_timestamp,
        total_shots: 0,
        succesfull_shots: 0,
        verification_requirement: None,
        last_move_time: block_timestamp,
    };
    games.insert(player, game_instance);
    send_delete_game_delayed_message(
        player,
        block_timestamp,
        config.gas_for_delete_single_game,
        config.delay_for_delete_single_game,
    );
    send_check_time_delayed_message(
        player,
        block_timestamp,
        config.gas_for_check_time,
        config.delay_for_check_time,
    );
    Ok(())
}

/// This function verifies that the game's current state matches the expected values based on the provided public input.
/// It checks the status of the game, the hash of the player's ships, and the validity of the 'out' value.
///
/// # Errors
///
/// * `Error::NoSuchGame` - Returned if there is no game associated with the given player.
/// * `Error::WrongStatusOrHit` - Returned if the game's status or hit does not match the expected values based on the input.
/// * `Error::WrongShipHash` - Returned if the game's ship hash does not match the expected hash based on the input.
/// * `Error::WrongOut` - Returned if the public input's 'out' value is not valid (neither 0 nor 1).
pub fn check_game(
    games: &SingleGamesMap,
    player: ActorId,
    public_input: PublicMoveInput,
    step: Option<u8>,
) -> Result<()> {
    let game = games.get(&player).ok_or(Error::NoSuchGame)?;

    if game.verification_requirement.is_none() {
        return Err(Error::WrongVerificationRequirement);
    }
    if public_input.out == 0 && step.is_none() {
        return Err(Error::StepIsNotTaken);
    }
    if game.ship_hash != public_input.hash {
        return Err(Error::WrongShipHash);
    }
    match public_input.out {
        0..=2 => Ok(()),
        _ => Err(Error::WrongOut),
    }
}
/// Processes a player's move in a single-player battleship game against a bot.
///
/// # Parameters
/// - `games`: Mutable reference to the map of ongoing single-player games.
/// - `player`: The `ActorId` of the player making the move.
/// - `verification_result`: Optional result from verification, indicating the outcome of a move.
/// - `step`: Optional `u8` representing the player's move (board position).
///
/// # Returns
/// - `Result<Event>`: An event indicating the move outcome or game end.
///
/// # Errors
/// - `Error::NoSuchGame` if the game doesn't exist.
/// - `Error::WrongVerificationRequirement` if verification is required but not provided.
/// - `Error::WrongOut` for an invalid verification result.
/// - `Error::WrongStep` if the step is out of range.
///
/// # Function Flow
/// - If a verification result is provided, update the game state accordingly. If the game ends, declare the winner.
/// - If no verification, process the player's move. If the game ends, declare the winner.
/// - Determine the bot's next move and prepare for possible verification.
pub fn make_move(
    games: &mut SingleGamesMap,
    player: ActorId,
    verification_result: Option<VerificationResult>,
    step: Option<u8>,
    config: Configuration,
    block_timestamp: u64,
) -> Result<Event> {
    let game = games.get_mut(&player).ok_or(Error::NoSuchGame)?;

    if game.verification_requirement.is_some() && verification_result.is_none() {
        return Err(Error::WrongVerificationRequirement);
    }

    if let Some(VerificationResult { res, hit }) = verification_result {
        match res {
            0 => game.player_board[hit as usize] = Entity::Boom,
            1 => game.player_board[hit as usize] = Entity::BoomShip,
            2 => game.dead_ship(hit),
            _ => return Err(Error::WrongOut),
        }

        if game.check_end_game() {
            let time = exec::block_timestamp() - game.start_time;
            let total_shots = game.total_shots;
            let succesfull_shots = game.succesfull_shots;
            games.remove(&player);
            return Ok(Event::EndGame {
                player,
                winner: BattleshipParticipants::Bot,
                time,
                total_shots,
                succesfull_shots,
                last_hit: Some(hit),
            });
        }

        if res != 0 {
            let bot_step = move_analysis(&game.player_board);
            game.verification_requirement = Some(bot_step);
            game.last_move_time = block_timestamp;
            send_check_time_delayed_message(
                player,
                block_timestamp,
                config.gas_for_check_time,
                config.delay_for_check_time,
            );
            return Ok(Event::MoveMade {
                player,
                step,
                step_result: None,
                bot_step: Some(bot_step),
            });
        }
    }

    let step = step.expect("`step` must not be None at this stage");
    if step > 24 {
        return Err(Error::WrongStep);
    }
    let step_result = game.bot_ships.bang(step);
    game.total_shots += 1;
    if step_result != StepResult::Missed {
        game.succesfull_shots += 1;
    }
    if game.bot_ships.check_end_game() {
        let time = exec::block_timestamp() - game.start_time;
        let total_shots = game.total_shots;
        let succesfull_shots = game.succesfull_shots;
        games.remove(&player);
        return Ok(Event::EndGame {
            player,
            winner: BattleshipParticipants::Player,
            time,
            total_shots,
            succesfull_shots,
            last_hit: Some(step),
        });
    }

    let bot_step = if step_result != StepResult::Missed {
        None
    } else {
        Some(move_analysis(&game.player_board))
    };
    game.verification_requirement = bot_step;
    game.last_move_time = block_timestamp;
    send_check_time_delayed_message(
        player,
        block_timestamp,
        config.gas_for_check_time,
        config.delay_for_check_time,
    );
    Ok(Event::MoveMade {
        player,
        step: Some(step),
        step_result: Some(step_result),
        bot_step,
    })
}

/// This function checks if a game with the specified player and start time exists in the map.
/// If found, it removes the game from the map. If not found, it returns an error indicating
/// that no such game exists.
pub fn delete_game(games: &mut SingleGamesMap, player: ActorId, start_time: u64) -> Result<()> {
    let game = games.get_mut(&player).ok_or(Error::NoSuchGame)?;

    if game.start_time == start_time {
        games.remove(&player);
    }

    Ok(())
}

/// This function constructs a request message to delete a single-player game instance
/// and sends it with a specified gas limit and delay using a delayed message mechanism.
/// It expects successful message sending; otherwise, it panics with an error message.
fn send_delete_game_delayed_message(player: ActorId, start_time: u64, gas_limit: u64, delay: u32) {
    let request = [
        "Single".encode(),
        "DeleteGame".to_string().encode(),
        (player, start_time).encode(),
    ]
    .concat();

    msg::send_bytes_with_gas_delayed(exec::program_id(), request, gas_limit, 0, delay)
        .expect("Error in sending message");
}

fn send_check_time_delayed_message(
    actor_id: ActorId,
    block_timestamp: u64,
    gas_limit: u64,
    delay: u32,
) {
    let request = [
        "Single".encode(),
        "CheckOutTiming".to_string().encode(),
        (actor_id, block_timestamp).encode(),
    ]
    .concat();
    msg::send_bytes_with_gas_delayed(exec::program_id(), request, gas_limit, 0, delay)
        .expect("Error in sending message");
}

pub fn check_out_timing(
    games: &mut SingleGamesMap,
    actor_id: ActorId,
    check_time: u64,
) -> Result<Option<Event>> {
    let game = games.get_mut(&actor_id).ok_or(Error::NoSuchGame)?;
    let event = if game.last_move_time == check_time {
        let time = exec::block_timestamp() - game.start_time;
        let event = Event::EndGame {
            player: actor_id,
            winner: BattleshipParticipants::Bot,
            time,
            last_hit: None,
            total_shots: game.total_shots,
            succesfull_shots: game.succesfull_shots,
        };
        games.remove(&actor_id);
        Some(event)
    } else {
        None
    };

    Ok(event)
}

/// This function is responsible for randomly or strategically placing ships on the game field,
/// ensuring that the positions are valid according to the game's rules.
fn generate_field() -> Ships {
    // let board = vec![Entity::Empty; 25];
    let mut ships = vec![];
    // Each ship is randomized and it can happen that at some point there is no room for a ship on the field,
    // so you need this cycle
    'mark: loop {
        let mut board = vec![Entity::Empty; 25];
        let ship_sizes = [3, 2, 2, 1];
        for &size in &ship_sizes {
            if check_empty_field(&board, size) {
                ships.push(place_ship(&mut board, size));
            } else {
                ships = vec![];
                continue 'mark;
            }
        }
        break;
    }
    Ships {
        ship_1: ships[0].clone(),
        ship_2: ships[1].clone(),
        ship_3: ships[2].clone(),
        ship_4: ships[3].clone(),
    }
}

fn check_empty_field(board: &[Entity], size: usize) -> bool {
    let empty_count = board.iter().filter(|&entity| entity.is_empty()).count();
    empty_count >= size
}

fn place_ship(board: &mut [Entity], size: usize) -> Vec<u8> {
    let mut placed = false;
    let mut ship: Vec<u8> = vec![];
    // the ship can be positioned in four positions,
    // the direction is chosen randomly and a check is made
    while !placed {
        let empty_indices = get_empty_indices(board);
        let random_index = get_random_value(empty_indices.len() as u8);
        let position = empty_indices[random_index as usize];
        let directions = [-1, -5, 1, 5]; // 0-left 1-up 2-right 3-down
        let random_direction = get_random_value(3);
        let direction = directions[random_direction as usize];
        if can_place_ship(board, position as isize, direction, size) {
            for i in 0..size {
                let target_position = (position as isize + direction * i as isize) as usize;
                ship.push(target_position as u8);
                board[target_position] = Entity::Ship;
                occupy_cells(board, target_position);
            }
            placed = true;
        }
    }
    ship
}

fn get_empty_indices(board: &[Entity]) -> Vec<usize> {
    board
        .iter()
        .enumerate()
        .filter_map(|(index, entity)| if entity.is_empty() { Some(index) } else { None })
        .collect()
}

pub fn get_random_value(range: u8) -> u8 {
    let seed = unsafe { SEED };
    unsafe { SEED = SEED.wrapping_add(1) };
    let mut random_input: [u8; 32] = exec::program_id().into();
    random_input[0] = random_input[0].wrapping_add(seed);
    let (random, _) = exec::random(random_input).expect("Error in getting random number");
    random[0] % range
}

fn can_place_ship(board: &[Entity], position: isize, direction: isize, size: usize) -> bool {
    if size == 3 {
        let is_valid = match direction {
            -1 if position % 5 == 0 || (position - 1) % 5 == 0 => false,
            -5 if (position + direction) < 0 || (position + 2 * direction) < 0 => false,
            1 if (position + 1) % 5 == 0 || (position + 2) % 5 == 0 => false,
            5 if (position + direction) > 24 || (position + 2 * direction) > 24 => false,
            _ => true,
        };

        if is_valid {
            return check_cells(board, position + direction)
                && check_cells(board, position + 2 * direction);
        }
        return false;
    }

    if size == 2 {
        let is_valid = match direction {
            -1 if position % 5 == 0 => false,
            -5 if (position + direction) < 0 => false,
            1 if (position + 1) % 5 == 0 => false,
            5 if (position + direction) > 24 => false,
            _ => true,
        };

        if is_valid {
            return check_cells(board, position + direction);
        }
        return false;
    }

    true
}

fn occupy_cells(board: &mut [Entity], position: usize) {
    let cells = match position {
        0 => vec![1, 5, 6],
        4 => vec![-1, 4, 5],
        20 => vec![1, -4, -5],
        24 => vec![-1, -5, -6],
        p if p % 5 == 0 => vec![-4, -5, 1, 5, 6],
        p if (p + 1) % 5 == 0 => vec![-1, -5, -6, 4, 5],
        _ => vec![-1, -4, -5, -6, 1, 4, 5, 6],
    };

    for &cell in &cells {
        if position as isize + cell < 0 || position as isize + cell > 24 {
            continue;
        }
        let target_position = (position as isize + cell) as usize;
        match board[target_position] {
            Entity::Empty | Entity::Unknown => {
                board[target_position] = Entity::Occupied;
            }
            _ => (),
        }
    }
}

fn check_cells(board: &[Entity], position: isize) -> bool {
    let cells = match position {
        0 => vec![1, 5, 6],
        4 => vec![-1, 4, 5],
        20 => vec![1, -4, -5],
        24 => vec![-1, -5, -6],
        p if p % 5 == 0 => vec![-4, -5, 1, 5, 6],
        p if (p + 1) % 5 == 0 => vec![-1, -5, -6, 4, 5],
        _ => vec![-1, -4, -5, -6, 1, 4, 5, 6],
    };
    for &cell in &cells {
        if position + cell < 0 || position + cell > 24 {
            continue;
        }
        let target_position = (position + cell) as usize;
        if board[target_position] == Entity::Ship {
            return false;
        }
    }

    true
}

fn move_analysis(board: &[Entity]) -> u8 {
    // Firstly, if we hit a ship, we have to finish it off and kill it
    for (index, status) in board.iter().enumerate() {
        if *status == Entity::BoomShip {
            let possible_bang = possible_bang(board, index as u8);
            if !possible_bang.is_empty() {
                let random_index = get_random_value(possible_bang.len() as u8);
                return possible_bang[random_index as usize];
            }
        }
    }
    // If there are no wounded ships, randomly select a free cell
    let mut possible_bang: Vec<u8> = vec![];
    for (index, status) in board.iter().enumerate() {
        if *status == Entity::Unknown {
            possible_bang.push(index as u8)
        }
    }
    let random_index = get_random_value(possible_bang.len() as u8);
    possible_bang[random_index as usize]
}

fn possible_bang(board: &[Entity], position: u8) -> Vec<u8> {
    let directions: Vec<i8> = match position {
        0 => vec![1, 5],
        4 => vec![-1, 5],
        20 => vec![-5, 1],
        24 => vec![-5, -1],
        p if p % 5 == 0 => vec![-5, 1, 5],
        p if (p + 1) % 5 == 0 => vec![-5, -1, 5],
        _ => vec![-5, -1, 1, 5],
    };

    let mut possible_bang: Vec<u8> = vec![];
    let mut single_boom_ship = true;
    for &direction in &directions {
        let current_position = position as i8 + direction;
        if !(0..=24).contains(&current_position) {
            continue;
        }
        if board[current_position as usize] == Entity::BoomShip {
            single_boom_ship = false;
            if check_bang(current_position as u8, direction)
                && board[(current_position + direction) as usize] == Entity::Unknown
            {
                possible_bang.push((current_position + direction) as u8);
            }
            if check_bang(position, -direction)
                && board[(position as i8 - direction) as usize] == Entity::Unknown
            {
                possible_bang.push((position as i8 - direction) as u8);
            }
        }
    }
    if possible_bang.is_empty() && single_boom_ship {
        for &direction in &directions {
            let current_position = position as i8 + direction;
            if !(0..=24).contains(&current_position) {
                continue;
            }
            if board[current_position as usize] == Entity::Unknown {
                possible_bang.push(current_position as u8);
            }
        }
    }
    possible_bang
}

fn check_bang(position: u8, direction: i8) -> bool {
    let check_cell = position as i8 + direction;
    if !(0..=24).contains(&check_cell) {
        return false;
    }
    match direction {
        -1 if position % 5 == 0 => return false,
        1 if (position + 1) % 5 == 0 => return false,
        _ => (),
    }
    true
}
