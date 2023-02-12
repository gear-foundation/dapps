#![no_std]
use gstd::{exec, msg, prelude::*, ActorId};
use tmg_io::*;

pub const HUNGER_PER_BLOCK: u64 = 1;
pub const BOREDOM_PER_BLOCK: u64 = 2;
pub const ENERGY_PER_BLOCK: u64 = 2;

pub const FILL_PER_FEED: u64 = 2_000;
pub const FILL_PER_ENTERTAINMENT: u64 = 2_000;
pub const FILL_PER_SLEEP: u64 = 2_000;

pub const MAX_VALUE: u64 = 10_000;

#[derive(Default, Encode, Decode, TypeInfo)]
pub struct Tamagotchi {
    pub name: String,
    pub date_of_birth: u64,
    pub owner: ActorId,
    pub fed: u64,
    pub fed_block: u64,
    pub entertained: u64,
    pub entertained_block: u64,
    pub rested: u64,
    pub rested_block: u64,
    pub allowed_account: Option<ActorId>,
}

static mut TAMAGOTCHI: Option<Tamagotchi> = None;

impl Tamagotchi {
    fn name(&self) {
        msg::reply(TmgEvent::Name(self.name.clone()), 0)
            .expect("Error in a reply `TmgEvent::Name`");
    }
    fn age(&self) {
        let age = exec::block_timestamp() - self.date_of_birth;
        msg::reply(TmgEvent::Age(age), 0).expect("Error in a reply `TmgEvent::Age`");
    }
    fn feed(&mut self) {
        assert!(!self.tmg_is_dead(), "Tamagotchi has died");
        self.fed_block = exec::block_timestamp();
        self.fed += FILL_PER_FEED - self.calculate_hunger();
        self.fed = if self.fed > MAX_VALUE {
            MAX_VALUE
        } else {
            self.fed
        };
        msg::reply(TmgEvent::Fed, 0).expect("Error in a reply `TmgEvent::Fed`");
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
        msg::reply(TmgEvent::Entertained, 0).expect("Error in a reply `TmgEvent::Entertained`");
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
        msg::reply(TmgEvent::Slept, 0).expect("Error in a reply `TmgEvent::Slept`");
    }

    fn transfer(&mut self, new_owner: &ActorId) {
        assert!(
            self.owner == msg::source() || self.allowed_account == Some(msg::source()),
            "Not allowed to change the owner"
        );
        self.owner = *new_owner;
        self.allowed_account = None;
        msg::reply(TmgEvent::Transfer(*new_owner), 0)
            .expect("Error in a reply `TmgEvent::OwnerChanged`");
    }

    fn approve(&mut self, allowed_account: &ActorId) {
        assert_eq!(
            self.owner,
            msg::source(),
            "Only owner can allow another account to change the owner"
        );
        assert!(self.owner != *allowed_account, "Approve to owner");

        self.allowed_account = Some(*allowed_account);
        msg::reply(TmgEvent::Approve(*allowed_account), 0)
            .expect("Error in a reply `TmgEvent::AllowedAccount`");
    }

    fn revoke_approval(&mut self) {
        assert_eq!(self.owner, msg::source(), "Only owner can revoke approval");
        self.allowed_account = None;
        msg::reply(TmgEvent::RevokeApproval, 0)
            .expect("Error in a reply `TmgEvent::ApprovalRevoked`");
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
            TmgEvent::TmgInfo {
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

#[gstd::async_main]
async fn main() {
    let action: TmgAction = msg::load().expect("Unable to decode `TmgAction`");
    let tmg = unsafe { TAMAGOTCHI.get_or_insert(Default::default()) };
    match action {
        TmgAction::Name => tmg.name(),
        TmgAction::Age => tmg.age(),
        TmgAction::Feed => tmg.feed(),
        TmgAction::Play => tmg.play(),
        TmgAction::Sleep => tmg.sleep(),
        TmgAction::Transfer(new_owner) => tmg.transfer(&new_owner),
        TmgAction::Approve(allowed_account) => tmg.approve(&allowed_account),
        TmgAction::RevokeApproval => tmg.revoke_approval(),
        TmgAction::TmgInfo => tmg.tmg_info(),
    }
}

#[no_mangle]
unsafe extern "C" fn init() {
    let name: String = String::from_utf8(msg::load_bytes().expect("Cant load init message"))
        .expect("Error in decoding");
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

#[derive(Encode, Decode, TypeInfo)]
pub struct TmgState {
    name: String,
    fed: u64,
    entertained: u64,
    rested: u64,
    age: u64,
    is_dead: bool,
}

#[no_mangle]
extern "C" fn state() {
    let tmg = unsafe { TAMAGOTCHI.get_or_insert(Default::default()) };
    msg::reply(tmg, 0).expect("Failed to share state");
}

#[no_mangle]
extern "C" fn metahash() {
    let metahash: [u8; 32] = include!("../.metahash");
    msg::reply(metahash, 0).expect("Failed to share metahash");
}
