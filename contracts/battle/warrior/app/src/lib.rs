#![no_std]
#![allow(clippy::new_without_default)]
#![allow(static_mut_refs)]
use sails_rs::gstd::msg;
use sails_rs::prelude::*;

#[derive(Debug)]
struct WarriorStorage {
    owner: ActorId,
    appearance: Appearance,
}

#[derive(Debug, TypeInfo, Encode)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
struct Appearance {
    head_index: u16,
    hat_index: u16,
    body_index: u16,
    accessory_index: u16,
    body_color: String,
    back_color: String,
}

static mut STORAGE: Option<WarriorStorage> = None;

struct WarriorService(());

impl WarriorService {
    pub fn init() -> Self {
        unsafe {
            STORAGE = Some(WarriorStorage {
                owner: msg::source(),
                // YOUR CODE HERE: fill in the remaining fields
                // appearance: Appearance {
                //     head_index: ..,
                //     hat_index: ..,
                //     body_index: ..,
                //     accessory_index: ..,
                //     body_color: ..,
                //     back_color: ..,
                // }
                //
                // For example:
                appearance: Appearance {
                    head_index: 1,
                    hat_index: 2,
                    body_index: 3,
                    accessory_index: 4,
                    body_color: "#008000".to_string(),
                    back_color: "#0000FF".to_string(),
                },
            });
        }
        Self(())
    }
    pub fn get_warrior_storage(&self) -> &'static WarriorStorage {
        unsafe { STORAGE.as_ref().expect("Storage is not initialized") }
    }
}

#[sails_rs::service]
impl WarriorService {
    pub fn new() -> Self {
        Self(())
    }
    pub fn get_owner(&self) -> ActorId {
        self.get_warrior_storage().owner
    }
    pub fn get_appearance(&self) -> &'static Appearance {
        &self.get_warrior_storage().appearance
    }
}

pub struct WarriorProgram(());

#[sails_rs::program]
impl WarriorProgram {
    pub fn new() -> Self {
        WarriorService::init();
        Self(())
    }

    pub fn warrior(&self) -> WarriorService {
        WarriorService::new()
    }
}
