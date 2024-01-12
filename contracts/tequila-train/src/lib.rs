#![no_std]

use gstd::{msg, prelude::*};
use tequila_train_io::*;

/// All game initializing logic is inside `GameState` constructor.
static mut GAME_LAUNCHER: Option<GameLauncher> = None;

#[no_mangle]
extern fn init() {
    let maybe_limit: Option<u64> = msg::load().expect("Unexpected invalid payload.");

    unsafe {
        GAME_LAUNCHER = Some(if let Some(limit) = maybe_limit {
            GameLauncher::new_with_limit(limit)
        } else {
            GameLauncher::default()
        })
    }
}

#[no_mangle]
extern fn handle() {
    let reply = process_handle();
    msg::reply(reply, 0).expect("Failed to encode or reply with `Result<Event, Error>`.");
}

fn process_handle() -> Result<Event, Error> {
    let game_launcher = unsafe {
        GAME_LAUNCHER
            .as_mut()
            .expect("The contract is not initialized")
    };

    if let Some(game_state) = &game_launcher.game_state {
        match game_state.state() {
            State::Stalled => {
                return Err(Error(
                    "The game stalled. No one is able to make a turn".to_owned(),
                ));
            }
            State::Winner(winner) => {
                return Err(Error(format!(
                    "The game is already finished. The winner is: {winner:?}"
                )));
            }
            _ => (),
        };
    }

    let command: Command = msg::load().expect("Unexpected invalid command payload.");
    let player = msg::source();

    match command {
        Command::Skip => {
            if let Some(game_state) = &mut game_launcher.game_state {
                game_state.skip_turn(player)
            } else {
                Err(Error("Game is not started!".to_owned()))
            }
        }
        Command::Place {
            tile_id,
            track_id,
            remove_train,
        } => {
            if let Some(game_state) = &mut game_launcher.game_state {
                game_state.make_turn(player, tile_id, track_id, remove_train)
            } else {
                Err(Error("Game is not started!".to_owned()))
            }
        }
        Command::Register { player, name } => game_launcher.register(player, name),
        Command::StartGame => game_launcher.start(),
        Command::RestartGame(maybe_limit) => game_launcher.restart(maybe_limit),
    }
}

#[no_mangle]
extern fn state() {
    msg::reply(
        unsafe {
            GAME_LAUNCHER
                .clone()
                .expect("Game launcher is not initialized")
        },
        0,
    )
    .expect("Failed to encode or reply with the game state");
}
