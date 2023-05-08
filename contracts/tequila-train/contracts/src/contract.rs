use gstd::{
    errors::{ContractError, Result as GstdResult},
    msg,
    prelude::*,
    MessageId,
};
use tequila_io::*;

/// All game initializing logic is inside `GameState` constructor.
static mut GAME_LAUNCHER: Option<GameLauncher> = None;

#[no_mangle]
extern "C" fn init() {
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
extern "C" fn handle() {
    process_handle()
        .expect("Failed to load, decode, encode, or reply with `PingPong` from `handle()`")
}

fn process_handle() -> Result<(), ContractError> {
    let game_launcher = unsafe {
        GAME_LAUNCHER
            .as_mut()
            .expect("The contract is not initialized")
    };
    let check_winner = |game_state: &GameState| match game_state.state() {
        State::Stalled => {
            msg::reply_bytes("The game stalled. No one is able to make a turn", 0)
                .expect("failed to reply");
            true
        }
        State::Winner(winner) => {
            let response = format!("The game is already finished. The winner is: {winner:?}");
            msg::reply_bytes(response.as_bytes(), 0).expect("failed to reply");
            true
        }
        State::Playing | State::Registration => false,
    };

    if let Some(game_state) = &game_launcher.game_state {
        if check_winner(game_state) {
            return Ok(());
        }
    }

    let command = msg::load()?;
    let player = msg::source();
    match command {
        Command::Skip => {
            if let Some(game_state) = &mut game_launcher.game_state {
                game_state.skip_turn(player)
            } else {
                panic!("Game is not started!");
            }
        }
        Command::Place {
            tile_id,
            track_id,
            remove_train,
        } => {
            if let Some(game_state) = &mut game_launcher.game_state {
                game_state.make_turn(player, tile_id, track_id, remove_train);
            } else {
                panic!("Game is not started!");
            }
        }
        Command::Register { player, name } => {
            game_launcher.register(player, name);
        }
        Command::StartGame => {
            game_launcher.start();
        }
        Command::RestartGame(maybe_limit) => {
            game_launcher.restart(maybe_limit);
        }
    }

    if let Some(game_state) = &game_launcher.game_state {
        if check_winner(game_state) {
            return Ok(());
        }
    }

    Ok(())
}

#[no_mangle]
extern "C" fn state() {
    reply(unsafe {
        GAME_LAUNCHER
            .clone()
            .expect("Game launcher is not initialized")
    })
    .expect("Failed to encode or reply with the game state");
}

#[no_mangle]
extern "C" fn metahash() {
    let metahash: [u8; 32] = include!("../.metahash");
    reply(metahash).expect("Failed to encode or reply with `[u8; 32]` from `metahash()`");
}

fn reply(payload: impl Encode) -> GstdResult<MessageId> {
    msg::reply(payload, 0)
}
