use gstd::{
    errors::{ContractError, Result as GstdResult},
    msg,
    prelude::*,
    MessageId,
};
use tequila_io::*;

static mut GAME_STATE: Option<GameState> = None;

#[no_mangle]
extern "C" fn init() {}

#[no_mangle]
extern "C" fn handle() {
    process_handle()
        .expect("Failed to load, decode, encode, or reply with `PingPong` from `handle()`")
}

fn process_handle() -> Result<(), ContractError> {
    let game_state = unsafe { GAME_STATE.as_mut().unwrap() };
    let check_winner = |game_state: &GameState| {
        if let Some(winner) = game_state.winner() {
            let response = format!("The game is already finished. The winner is: {winner:?}");
            msg::reply_bytes(response.as_bytes(), 0).expect("failed to reply");
            return true;
        }

        false
    };

    if check_winner(game_state) {
        return Ok(());
    }

    let command = msg::load()?;
    let player = msg::source();
    match command {
        Command::Skip => game_state.skip_turn(player),
        Command::Place {
            tile_id,
            track_id,
            remove_train,
        } => {
            game_state.make_turn(player, tile_id, track_id, remove_train);
        }
    }

    if check_winner(game_state) {
        return Ok(());
    }

    Ok(())
}

#[no_mangle]
extern "C" fn state() {
    reply(unsafe {
        GAME_STATE
            .as_ref()
            .expect("Game state is not initialized")
            .clone()
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
