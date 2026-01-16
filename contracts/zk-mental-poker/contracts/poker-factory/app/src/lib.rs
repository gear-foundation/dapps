#![no_std]
#![allow(clippy::new_without_default)]
#![allow(static_mut_refs)]
mod services;
use crate::services::{Config, PokerFactoryService};
use sails_rs::ActorId;
pub struct PokerFactoryProgram(());

#[sails_rs::program]
impl PokerFactoryProgram {
    pub fn new(config: Config, pts_actor_id: ActorId, zk_verification_id: ActorId) -> Self {
        PokerFactoryService::init(config, pts_actor_id, zk_verification_id);
        Self(())
    }

    pub fn poker_factory(&self) -> PokerFactoryService {
        PokerFactoryService::new()
    }
}
