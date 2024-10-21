#![no_std]
#![allow(clippy::new_without_default)]

use sails_rs::prelude::*;
mod player;
use player::PlayerService;
pub struct Program(());

#[program]
impl Program {
    pub async fn new() -> Self {
        Self(())
    }

    pub fn syndote_player(&self) -> PlayerService {
        PlayerService::new()
    }
}
