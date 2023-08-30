#![no_std]

use gstd::{exec, msg, prelude::*, ActorId};
use tamagotchi_io::*;

#[derive(Default, Clone)]
struct Tamagotchi {
    name: String,
    date_of_birth: u64,
    owner: ActorId,
    fed: u64,
    fed_block: u64,
    entertained: u64,
    entertained_block: u64,
    rested: u64,
    rested_block: u64,
    allowed_account: Option<ActorId>,
}

static mut TAMAGOTCHI: Option<Tamagotchi> = None;

impl Tamagotchi {
    fn feed(&mut self) {
        assert!(!self.tmg_is_dead(), "Tamagotchi has died");
        self.fed_block = exec::block_timestamp();
        self.fed += FILL_PER_FEED - self.calculate_hunger();
        self.fed = if self.fed > MAX_VALUE {
            MAX_VALUE
        } else {
            self.fed
        };
        msg::reply(TmgReply::Fed, 0).expect("Error in a reply `TmgEvent::Fed`");
    }

    fn play(&mut self) {
        assert!(!self.tmg_is_dead(), "Tamagotchi has died");
        self.entertained_block = exec::block_timestamp();
        self.entertained += FILL_PER_ENTERTAINMENT - self.calculate_boredom();
        self.entertained = if self.entertained > MAX_VALUE {
            MAX_VALUE
        } else {
            self.entertained
        };
        msg::reply(TmgReply::Entertained, 0).expect("Error in a reply `TmgEvent::Entertained`");
    }

    fn sleep(&mut self) {
        assert!(!self.tmg_is_dead(), "Tamagotchi has died");
        self.rested_block = exec::block_timestamp();
        self.rested += FILL_PER_SLEEP - self.calculate_energy();
        self.rested = if self.rested > MAX_VALUE {
            MAX_VALUE
        } else {
            self.rested
        };
        msg::reply(TmgReply::Slept, 0).expect("Error in a reply `TmgEvent::Slept`");
    }

    fn calculate_hunger(&self) -> u64 {
        HUNGER_PER_BLOCK * ((exec::block_timestamp() - self.fed_block) / 1_000)
    }

    fn calculate_boredom(&self) -> u64 {
        BOREDOM_PER_BLOCK * ((exec::block_timestamp() - self.entertained_block) / 1000)
    }

    fn calculate_energy(&self) -> u64 {
        ENERGY_PER_BLOCK * ((exec::block_timestamp() - self.rested_block) / 1000)
    }

    fn tmg_info(&self) {
        msg::reply(
            TmgReply::TmgInfo {
                owner: self.owner,
                name: self.name.clone(),
                date_of_birth: self.date_of_birth,
            },
            0,
        )
        .expect("Error in a reply `TmgEvent::TmgInfo");
    }

    fn tmg_is_dead(&self) -> bool {
        let fed = self.fed.saturating_sub(self.calculate_hunger());
        let entertained = self.entertained.saturating_sub(self.calculate_boredom());
        let rested = self.rested.saturating_sub(self.calculate_energy());
        fed == 0 && entertained == 0 && rested == 0
    }
}

#[no_mangle]
extern fn handle() {
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
unsafe extern fn init() {
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
extern fn state() {
    let tmg = unsafe { TAMAGOTCHI.get_or_insert(Default::default()) };
    msg::reply(tamagotchi_io::Tamagotchi::from(tmg.clone()), 0).expect("Failed to share state");
}

impl From<Tamagotchi> for tamagotchi_io::Tamagotchi {
    fn from(value: Tamagotchi) -> Self {
        Self {
            name: value.name,
            date_of_birth: value.date_of_birth,
            owner: value.owner,
            fed: value.fed,
            fed_block: value.fed_block,
            entertained: value.entertained,
            entertained_block: value.entertained_block,
            rested: value.rested,
            rested_block: value.rested_block,
            allowed_account: value.allowed_account,
        }
    }
}
