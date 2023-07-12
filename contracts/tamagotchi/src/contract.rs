use gstd::{exec, msg, prelude::*, ActorId};
use tmg_io::*;

pub const HUNGER_PER_BLOCK: u64 = 1;
pub const BOREDOM_PER_BLOCK: u64 = 2;
pub const ENERGY_PER_BLOCK: u64 = 2;

pub const FILL_PER_FEED: u64 = 2_000;
pub const FILL_PER_ENTERTAINMENT: u64 = 2_000;
pub const FILL_PER_SLEEP: u64 = 2_000;

pub const MAX_VALUE: u64 = 10_000;

static mut TAMAGOTCHI: Option<Tamagotchi> = None;

#[no_mangle]
extern "C" fn handle() {
    let action: TmgAction = msg::load().expect("Unable to decode `TmgAction`");
    let tmg = unsafe { TAMAGOTCHI.get_or_insert(Default::default()) };
    match action {
        TmgAction::Name => {
            msg::reply(TmgReply::Name(tmg.name.clone()), 0)
                .expect("Error in a reply `TmgEvent::Name`");
        }
        TmgAction::Age => {
            let age = exec::block_timestamp() - tmg.date_of_birth;
            msg::reply(TmgReply::Age(age), 0).expect("Error in a reply `TmgEvent::Age`");
            // ⚠️ TODO: Send a reply about the Tamagotchi age
            // Hint: the message payload must be TmgReply::Age(age)
        }
        TmgAction::Feed => tmg.feed(),
        TmgAction::Play => tmg.play(),
        TmgAction::Sleep => tmg.sleep(),
        TmgAction::TmgInfo => tmg.tmg_info(),
    }
}

#[no_mangle]
unsafe extern "C" fn init() {
    let name: String = msg::load().expect("Failed to decode Tamagotchi name");
    // // ⚠️ TODO: Change the tamagotchi name
    // let name = String::from("Best-Tamagotchi");

    let current_block = exec::block_timestamp();

    let tmg = Tamagotchi {
        name,
        date_of_birth: current_block,
        owner: msg::source(),
        fed: MAX_VALUE,
        fed_block: current_block,
        entertained: MAX_VALUE,
        entertained_block: current_block,
        rested: MAX_VALUE,
        rested_block: current_block,
        ..Default::default()
    };
    TAMAGOTCHI = Some(tmg);
}

#[no_mangle]
extern "C" fn state() {
    let tmg = unsafe { TAMAGOTCHI.get_or_insert(Default::default()) };
    msg::reply(tmg, 0).expect("Failed to share state");
}
