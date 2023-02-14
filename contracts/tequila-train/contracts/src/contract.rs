use gmeta::Metadata;
use gstd::{
    errors::{ContractError, Result as GstdResult},
    msg,
    prelude::*,
    util, ActorId, MessageId,
};
use hashbrown::HashMap;
use tequila_io::*;

static mut STATE: Option<HashMap<ActorId, u128>> = None;
static mut GAME_STATE: Option<GameState> = None;

fn static_mut_state() -> &'static mut HashMap<ActorId, u128> {
    match unsafe { &mut STATE } {
        Some(state) => state,
        None => unreachable!("State can't be uninitialized"),
    }
}

#[no_mangle]
extern "C" fn init() {
    unsafe { STATE = Some(HashMap::new()) }
}

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
            msg::reply_bytes(response.as_bytes(), 0);
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
        Command::Place { tile_id, track_id } => {
            game_state.make_turn(player, tile_id, track_id);
        }
    }

    if check_winner(game_state) {
        return Ok(());
    }

    Ok(())
}

fn common_state() -> <ContractMetadata as Metadata>::State {
    State(
        static_mut_state()
            .iter()
            .map(|(pinger, ping_count)| (*pinger, *ping_count))
            .collect(),
    )
}

#[no_mangle]
extern "C" fn meta_state() -> *const [i32; 2] {
    let query = msg::load().expect("Failed to load or decode `StateQuery` from `meta_state()`");
    let state = common_state();

    let reply = match query {
        StateQuery::AllState => StateQueryReply::AllState(state),
        StateQuery::Pingers => StateQueryReply::Pingers(state.pingers()),
        StateQuery::PingCount(actor) => StateQueryReply::PingCount(state.ping_count(actor)),
    };

    util::to_leak_ptr(reply.encode())
}

#[no_mangle]
extern "C" fn state() {
    reply(common_state()).expect(
        "Failed to encode or reply with `<ContractMetadata as Metadata>::State` from `state()`",
    );
}

#[no_mangle]
extern "C" fn metahash() {
    let metahash: [u8; 32] = include!("../.metahash");

    reply(metahash).expect("Failed to encode or reply with `[u8; 32]` from `metahash()`");
}

fn reply(payload: impl Encode) -> GstdResult<MessageId> {
    msg::reply(payload, 0)
}
