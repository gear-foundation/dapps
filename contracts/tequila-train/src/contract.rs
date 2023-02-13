use app_io::*;
use gmeta::Metadata;
use gstd::{
    errors::{ContractError, Result as GstdResult},
    msg,
    prelude::*,
    util, ActorId, MessageId,
};
use hashbrown::HashMap;

static mut STATE: Option<HashMap<ActorId, u128>> = None;

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
    let payload = msg::load()?;

    if let PingPong::Ping = payload {
        let pingers = static_mut_state();

        pingers
            .entry(msg::source())
            .and_modify(|ping_count| *ping_count = ping_count.saturating_add(1))
            .or_insert(1);

        reply(PingPong::Pong)?;
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
