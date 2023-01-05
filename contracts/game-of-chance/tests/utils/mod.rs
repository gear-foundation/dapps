use common::{InitResult, MetaStateReply, Program, RunResult, TransactionalProgram};
use gstd::{prelude::*, ActorId};
use gtest::{Program as InnerProgram, System, EXISTENTIAL_DEPOSIT};
use rand::{RngCore, SeedableRng};
use rand_xoshiro::Xoshiro128PlusPlus;

use game_of_chance::*;

mod ft;

pub mod common;
pub mod prelude;

pub use common::initialize_system;
pub use ft::FungibleToken;

pub const FOREIGN_USER: u64 = 9999999;

type GOCRunResult<T> = RunResult<T, GOCEvent, GOCError>;

pub struct Goc<'a>(InnerProgram<'a>);

impl Program for Goc<'_> {
    fn inner_program(&self) -> &InnerProgram {
        &self.0
    }
}

impl<'a> Goc<'a> {
    pub fn initialize(
        system: &'a System,
        admin: impl Into<ActorId>,
    ) -> InitResult<Goc<'a>, GOCError> {
        Self::common_initialize(system, admin, |_, _| {})
    }

    pub fn initialize_with_existential_deposit(
        system: &'a System,
        admin: impl Into<ActorId>,
    ) -> InitResult<Goc<'a>, GOCError> {
        Self::common_initialize(system, admin, |system, program| {
            system.mint_to(program.id(), EXISTENTIAL_DEPOSIT)
        })
    }

    fn common_initialize(
        system: &'a System,
        admin: impl Into<ActorId>,
        mint: fn(&System, &InnerProgram),
    ) -> InitResult<Goc<'a>, GOCError> {
        let program = InnerProgram::current(system);

        mint(system, &program);

        let result = program.send(
            FOREIGN_USER,
            GOCInit {
                admin: admin.into(),
            },
        );
        let is_active = system.is_active_program(program.id());

        InitResult::new(Self(program), result, is_active)
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
        RunResult::new(
            self.0.send(
                from,
                GOCAction::Start {
                    duration,
                    participation_cost,
                    ft_actor_id: ft_address,
                },
            ),
            |(ending, participation_cost, ft_actor_id)| GOCEvent::Started {
                ending,
                participation_cost,
                ft_actor_id,
            },
        )
    }

    pub fn enter(&mut self, from: u64) -> GOCRunResult<u64> {
        self.enter_with_value(from, 0)
    }

    pub fn enter_with_value(&mut self, from: u64, value: u128) -> GOCRunResult<u64> {
        RunResult::new(
            self.0.send_with_value(from, GOCAction::Enter, value),
            |actor_id| GOCEvent::PlayerAdded(actor_id.into()),
        )
    }

    pub fn pick_winner(&mut self, from: u64) -> GOCRunResult<ActorId> {
        RunResult::new(self.0.send(from, GOCAction::PickWinner), GOCEvent::Winner)
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

pub fn predict_winner(system: &System, players: &[u64]) -> ActorId {
    let mut random_data = [0; 4];

    Xoshiro128PlusPlus::seed_from_u64(system.block_timestamp()).fill_bytes(&mut random_data);

    let mystical_number = u32::from_le_bytes(random_data) as usize;

    players[mystical_number % players.len()].into()
}
