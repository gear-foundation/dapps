use common::{InitResult, MetaStateReply, Program, RunResult, TransactionProgram};
use gstd::{prelude::*, ActorId};
use gtest::{Program as InnerProgram, System};
use rand::{RngCore, SeedableRng};
use rand_xoshiro::Xoshiro128PlusPlus;

use game_of_chance::*;

mod sft;

pub mod common;
pub mod prelude;

pub use common::initialize_system;
pub use sft::Sft;

pub const FOREIGN_USER: u64 = 9999999;

type GOCRunResult<T> = RunResult<T, GOCEvent>;
type GOCInitResult<'a> = InitResult<Goc<'a>>;

pub struct Goc<'a>(InnerProgram<'a>);

impl Program for Goc<'_> {
    fn inner_program(&self) -> &InnerProgram {
        &self.0
    }
}

impl<'a> From<InnerProgram<'a>> for Goc<'a> {
    fn from(program: InnerProgram<'a>) -> Self {
        Self(program)
    }
}

impl<'a> Goc<'a> {
    pub fn initialize(system: &'a System, admin: impl Into<ActorId>) -> GOCInitResult {
        let program = InnerProgram::current(system);

        let failed = program
            .send(
                FOREIGN_USER,
                GOCInit {
                    admin: admin.into(),
                },
            )
            .main_failed();

        InitResult(Self(program), failed)
    }

    pub fn meta_state(&self) -> GOCMetaState {
        GOCMetaState(&self.0)
    }

    pub fn start(
        &mut self,
        from: u64,
        duration: u64,
        participation_cost: u128,
        ft_address: Option<ActorId>,
    ) -> GOCRunResult<(u64, u128, Option<ActorId>)> {
        RunResult(
            self.0.send(
                from,
                GOCAction::Start {
                    duration,
                    participation_cost,
                    ft_actor_id: ft_address,
                },
            ),
            |(ending, participation_cost, ft_address)| GOCEvent::Started {
                ending,
                participation_cost,
                ft_actor_id: ft_address,
            },
        )
    }

    pub fn enter(&mut self, from: u64) -> GOCRunResult<u64> {
        self.enter_with_value(from, 0)
    }

    pub fn enter_with_value(&mut self, from: u64, value: u128) -> GOCRunResult<u64> {
        RunResult(
            self.0.send_with_value(from, GOCAction::Enter, value),
            |actor_id| GOCEvent::PlayerAdded(actor_id.into()),
        )
    }

    pub fn pick_winner(&mut self, from: u64) -> GOCRunResult<ActorId> {
        RunResult(self.0.send(from, GOCAction::PickWinner), GOCEvent::Winner)
    }
}

pub struct GOCMetaState<'a>(&'a InnerProgram<'a>);

impl GOCMetaState<'_> {
    pub fn state(self) -> MetaStateReply<GOCState> {
        MetaStateReply(
            self.0
                .meta_state_empty()
                .expect("Failed to decode `GOCStateReply`"),
        )
    }
}

pub fn predict_winner(system: &System, players: &[u64]) -> u64 {
    let mut random_data = [0; 4];

    Xoshiro128PlusPlus::seed_from_u64(system.block_timestamp()).fill_bytes(&mut random_data);

    let mystical_number = u32::from_le_bytes(random_data) as usize;

    players[mystical_number % players.len()]
}
