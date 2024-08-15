use crate::services::game::{
    GameError, GameInstance, Event, GameResult, Mark, Storage, VICTORIES,
};
use gstd::{exec, msg};
use sails_rs::prelude::*;

pub fn start_game(storage: &mut Storage, msg_source: ActorId) -> Result<Event, GameError> {
    check_allow_messages(&storage, msg_source)?;
    if let Some(current_game) = storage.current_games.get(&msg_source) {
        if !current_game.game_over {
            return Err(GameError::GameIsAlreadyStarted);
        }
    }

    let turn = random_turn(msg_source);

    let (player_mark, bot_mark) = if turn == 0 {
        (Mark::O, Mark::X)
    } else {
        (Mark::X, Mark::O)
    };
    let mut game_instance = GameInstance {
        board: vec![None; 9],
        player_mark,
        bot_mark,
        last_time: exec::block_timestamp(),
        game_result: None,
        game_over: false,
    };

    if bot_mark == Mark::X {
        game_instance.board[4] = Some(Mark::X);
    }

    storage
        .current_games
        .insert(msg_source, game_instance.clone());

    Ok(Event::GameStarted {
        game: game_instance,
    })
}

pub fn turn(storage: &mut Storage, msg_source: ActorId, step: u8) -> Result<Event, GameError> {
    check_allow_messages(&storage, msg_source)?;
    let game_instance = storage
        .current_games
        .get_mut(&msg_source)
        .ok_or(GameError::GameIsNotStarted)?;

    if game_instance.board[step as usize].is_some() {
        return Err(GameError::CellIsAlreadyOccupied);
    }
    if game_instance.game_over {
        return Err(GameError::GameIsAlreadyOver);
    }
    let block_timestamp = exec::block_timestamp();
    if game_instance.last_time + storage.config.turn_deadline_ms < block_timestamp {
        return Err(GameError::MissedYourTurn);
    }
    game_instance.board[step as usize] = Some(game_instance.player_mark);
    game_instance.last_time = block_timestamp;

    if let Some(mark) = get_result(&game_instance.board.clone()) {
        game_instance.game_over = true;
        if mark == game_instance.player_mark {
            game_instance.game_result = Some(GameResult::Player);
            send_delayed_message_to_remove_game(
                msg_source,
                storage.config.gas_to_remove_game,
                storage.config.time_interval,
            );
        } else {
            game_instance.game_result = Some(GameResult::Bot);
            send_delayed_message_to_remove_game(
                msg_source,
                storage.config.gas_to_remove_game,
                storage.config.time_interval,
            );
        }
        return Ok(Event::GameFinished {
            game: game_instance.clone(),
        });
    }

    let bot_step = make_move(game_instance);

    if let Some(step_num) = bot_step {
        game_instance.board[step_num] = Some(game_instance.bot_mark);
    }

    if let Some(mark) = get_result(&game_instance.board.clone()) {
        game_instance.game_over = true;
        if mark == game_instance.player_mark {
            game_instance.game_result = Some(GameResult::Player);
            send_delayed_message_to_remove_game(
                msg_source,
                storage.config.gas_to_remove_game,
                storage.config.time_interval,
            );
        } else {
            game_instance.game_result = Some(GameResult::Bot);
            send_delayed_message_to_remove_game(
                msg_source,
                storage.config.gas_to_remove_game,
                storage.config.time_interval,
            );
        }
        return Ok(Event::GameFinished {
            game: game_instance.clone(),
        });
    } else if !game_instance.board.contains(&None) || bot_step.is_none() {
        game_instance.game_over = true;
        game_instance.game_result = Some(GameResult::Draw);
        send_delayed_message_to_remove_game(
            msg_source,
            storage.config.gas_to_remove_game,
            storage.config.time_interval,
        );
        return Ok(Event::GameFinished {
            game: game_instance.clone(),
        });
    }

    Ok(Event::MoveMade {
        game: game_instance.clone(),
    })
}

pub fn skip(storage: &mut Storage, msg_source: ActorId) -> Result<Event, GameError> {
    check_allow_messages(&storage, msg_source)?;
    let game_instance = storage
        .current_games
        .get_mut(&msg_source)
        .ok_or(GameError::GameIsNotStarted)?;

    if game_instance.game_over {
        return Err(GameError::GameIsAlreadyOver);
    }
    let block_timestamp = exec::block_timestamp();
    if game_instance.last_time + storage.config.turn_deadline_ms >= block_timestamp {
        return Err(GameError::NotMissedTurnMakeMove);
    }

    let bot_step = make_move(game_instance);
    game_instance.last_time = block_timestamp;

    match bot_step {
        Some(step_num) => {
            game_instance.board[step_num] = Some(game_instance.bot_mark);
            let win = get_result(&game_instance.board.clone());
            if let Some(mark) = win {
                game_instance.game_over = true;
                if mark == game_instance.player_mark {
                    game_instance.game_result = Some(GameResult::Player);
                    send_delayed_message_to_remove_game(
                        msg_source,
                        storage.config.gas_to_remove_game,
                        storage.config.time_interval,
                    );
                } else {
                    game_instance.game_result = Some(GameResult::Bot);
                    send_delayed_message_to_remove_game(
                        msg_source,
                        storage.config.gas_to_remove_game,
                        storage.config.time_interval,
                    );
                }
                return Ok(Event::GameFinished {
                    game: game_instance.clone(),
                });
            } else if !game_instance.board.contains(&None) {
                game_instance.game_over = true;
                game_instance.game_result = Some(GameResult::Draw);
                send_delayed_message_to_remove_game(
                    msg_source,
                    storage.config.gas_to_remove_game,
                    storage.config.time_interval,
                );
                return Ok(Event::GameFinished {
                    game: game_instance.clone(),
                });
            }
        }
        None => {
            game_instance.game_over = true;
            game_instance.game_result = Some(GameResult::Draw);
            send_delayed_message_to_remove_game(
                msg_source,
                storage.config.gas_to_remove_game,
                storage.config.time_interval,
            );
            return Ok(Event::GameFinished {
                game: game_instance.clone(),
            });
        }
    }

    Ok(Event::MoveMade {
        game: game_instance.clone(),
    })
}

pub fn remove_game_instance(storage: &mut Storage, msg_source: ActorId, account: ActorId) -> Result<Event, GameError> {
    if msg_source != exec::program_id() {
        return Err(GameError::MessageOnlyForProgram);
    }

    let game_instance = storage
        .current_games
        .get(&account)
        .expect("Unexpected: the game does not exist");

    if game_instance.game_over {
        storage.current_games.remove(&account);
    }
    Ok(Event::GameInstanceRemoved)
}

pub fn remove_game_instances(storage: &mut Storage, msg_source: ActorId, accounts: Option<Vec<ActorId>>) -> Result<Event, GameError> {
    if !storage.admins.contains(&msg_source) {
        return Err(GameError::NotAdmin);
    }
    match accounts {
        Some(accounts) => {
            for account in accounts {
                storage.current_games.remove(&account);
            }
        }
        None => {
            storage.current_games.retain(|_, game_instance| {
                exec::block_timestamp() - game_instance.last_time
                    < storage.config.time_interval as u64 * storage.config.s_per_block
            });
        }
    }
    Ok(Event::GameInstanceRemoved)
}

pub fn add_admin(storage: &mut Storage, msg_source: ActorId, admin: ActorId) -> Result<Event, GameError> {
    if !storage.admins.contains(&msg_source) {
        return Err(GameError::NotAdmin);
    }
    storage.admins.push(admin);
    Ok(Event::AdminAdded)
}
pub fn remove_admin(storage: &mut Storage, msg_source: ActorId, admin: ActorId) -> Result<Event, GameError> {
    if !storage.admins.contains(&msg_source) {
        return Err(GameError::NotAdmin);
    }
    storage.admins.retain(|id| *id != admin);
    Ok(Event::AdminRemoved)
}

pub fn update_config(
    storage: &mut Storage,
    msg_source: ActorId,
    s_per_block: Option<u64>,
    gas_to_remove_game: Option<u64>,
    time_interval: Option<u32>,
    turn_deadline_ms: Option<u64>,
) -> Result<Event, GameError> {
    if !storage.admins.contains(&msg_source) {
        return Err(GameError::NotAdmin);
    }

    if let Some(s_per_block) = s_per_block {
        storage.config.s_per_block = s_per_block;
    }
    if let Some(gas_to_remove_game) = gas_to_remove_game {
        storage.config.gas_to_remove_game = gas_to_remove_game;
    }
    if let Some(time_interval) = time_interval {
        storage.config.time_interval = time_interval;
    }
    if let Some(turn_deadline_ms) = turn_deadline_ms {
        storage.config.turn_deadline_ms = turn_deadline_ms;
    }
    Ok(Event::ConfigUpdated)
}

pub fn allow_messages(
    storage: &mut Storage,
    msg_source: ActorId,
    messages_allowed: bool,
) -> Result<Event, GameError> {
    if !storage.admins.contains(&msg_source) {
        return Err(GameError::NotAdmin);
    }
    storage.messages_allowed = messages_allowed;
    Ok(Event::StatusMessagesUpdated)
}

fn check_allow_messages(storage: &Storage, msg_source: ActorId) -> Result<(), GameError> {
    if !storage.messages_allowed && !storage.admins.contains(&msg_source) {
        return Err(GameError::NotAllowedToSendMessages);
    }
    Ok(())
}

fn make_move(game: &GameInstance) -> Option<usize> {
    match game.bot_mark {
        Mark::O => {
            // if on any of the winning lines there are 2 own pieces and 0 strangers
            // make move
            let step = check_line(&game.board, 2, 0);
            if let Some(step_num) = step {
                return Some(step_num);
            }

            // if on any of the winning lines there are 2 stranger pieces and 0 own
            // make move
            let step = check_line(&game.board, 0, 2);
            if let Some(step_num) = step {
                return Some(step_num);
            }
            // if on any of the winning lines there are 1 own pieces and 0 strangers
            // make move
            let step = check_line(&game.board, 1, 0);
            if let Some(step_num) = step {
                return Some(step_num);
            }
            // if the center is empty, then we occupy the center
            if game.board[4] != Some(Mark::O) && game.board[4] != Some(Mark::X) {
                return Some(4);
            }
            // occupy the first cell
            if game.board[0] != Some(Mark::O) && game.board[0] != Some(Mark::X) {
                return Some(0);
            }
        }
        Mark::X => {
            // if on any of the winning lines there are 2 own pieces and 0 strangers
            // make move
            let step = check_line(&game.board, 0, 2);

            if let Some(step_num) = step {
                return Some(step_num);
            }
            // if on any of the winning lines there are 2 stranger pieces and 0 own
            // make move
            let step = check_line(&game.board, 2, 0);
            if let Some(step_num) = step {
                return Some(step_num);
            }
            // if on any of the winning lines there are 1 own pieces and 0 strangers
            // make move
            let step = check_line(&game.board, 0, 1);

            if let Some(step_num) = step {
                return Some(step_num);
            }
            // if the center is empty, then we occupy the center
            if game.board[4] != Some(Mark::O) && game.board[4] != Some(Mark::X) {
                return Some(4);
            }
            // occupy the first cell
            if game.board[0] != Some(Mark::O) && game.board[0] != Some(Mark::X) {
                return Some(0);
            }
        }
    }
    None
}

fn check_line(map: &[Option<Mark>], sum_o: u8, sum_x: u8) -> Option<usize> {
    for line in VICTORIES.iter() {
        let mut o = 0;
        let mut x = 0;
        for i in 0..3 {
            if map[line[i]] == Some(Mark::O) {
                o += 1;
            }
            if map[line[i]] == Some(Mark::X) {
                x += 1;
            }
        }

        if sum_o == o && sum_x == x {
            for i in 0..3 {
                if map[line[i]] != Some(Mark::O) && map[line[i]] != Some(Mark::X) {
                    return Some(line[i]);
                }
            }
        }
    }
    None
}

fn get_result(map: &[Option<Mark>]) -> Option<Mark> {
    for i in VICTORIES.iter() {
        if map[i[0]] == Some(Mark::X) && map[i[1]] == Some(Mark::X) && map[i[2]] == Some(Mark::X) {
            return Some(Mark::X);
        }

        if map[i[0]] == Some(Mark::O) && map[i[1]] == Some(Mark::O) && map[i[2]] == Some(Mark::O) {
            return Some(Mark::O);
        }
    }
    None
}

fn random_turn(msg_source: ActorId) -> u8 {
    let random_input: [u8; 32] = msg_source.into();
    let (random, _) = exec::random(random_input).expect("Error in getting random number");
    random[0] % 2
}

fn send_delayed_message_to_remove_game(
    account: ActorId,
    gas_to_remove_game: u64,
    time_interval: u32,
) {
    let request = [
        "TicTacToe".encode(),
        "RemoveGameInstance".to_string().encode(),
        (account).encode(),
    ]
    .concat();

    msg::send_bytes_with_gas_delayed(
        exec::program_id(),
        request,
        gas_to_remove_game,
        0,
        time_interval,
    )
    .expect("Error in sending message");
}
