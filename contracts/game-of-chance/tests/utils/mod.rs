use common::{InitResult, MetaStateReply, Program, RunResult, TransactionalProgram};
use game_of_chance_io::*;
use gstd::{prelude::*, ActorId};
use gtest::{Program as InnerProgram, System, EXISTENTIAL_DEPOSIT};
use rand::{RngCore, SeedableRng};
use rand_xoshiro::Xoshiro128PlusPlus;

mod fungible_token;

pub mod common;
pub mod prelude;

pub use common::initialize_system;
pub use fungible_token::FungibleToken;

pub const FOREIGN_USER: u64 = 9999999;

type GOCRunResult<T> = RunResult<T, Event, Error>;

pub struct Goc<'a>(InnerProgram<'a>);

impl Program for Goc<'_> {
    fn inner_program(&self) -> &InnerProgram {
        &self.0
    }
}

impl<'a> Goc<'a> {
    pub fn initialize(system: &'a System, admin: impl Into<ActorId>) -> InitResult<Goc<'a>, Error> {
        Self::common_initialize(system, admin, |_, _| {})
    }

    pub fn initialize_with_existential_deposit(
        system: &'a System,
        admin: impl Into<ActorId>,
    ) -> InitResult<Goc<'a>, Error> {
        Self::common_initialize(system, admin, |system, program| {
            system.mint_to(program.id(), EXISTENTIAL_DEPOSIT)
        })
    }

    fn common_initialize(
        system: &'a System,
        admin: impl Into<ActorId>,
        mint: fn(&System, &InnerProgram),
    ) -> InitResult<Goc<'a>, Error> {
        let program = InnerProgram::current(system);

        mint(system, &program);

        let result = program.send(
            FOREIGN_USER,
            Initialize {
                admin: admin.into(),
            },
        );
        let is_active = system.is_active_program(program.id());

        InitResult::new(Self(program), result, is_active)
    }

    pub fn state(&self) -> GOCMetaState {
        GOCMetaState(&self.0)
    }

    pub fn start(
        &mut self,
        from: u64,
        duration: u64,
        participation_cost: u128,
        fungible_token: Option<ActorId>,
    ) -> GOCRunResult<(u64, u128, Option<ActorId>)> {
        RunResult::new(
            self.0.send(
                from,
                Action::Start {
                    duration,
                    participation_cost,
                    fungible_token,
                },
            ),
            |(ending, participation_cost, fungible_token)| Event::Started {
                ending,
                participation_cost,
                fungible_token,
            },
        )
    }

    pub fn enter(&mut self, from: u64) -> GOCRunResult<u64> {
        self.enter_with_value(from, 0)
    }

    pub fn enter_with_value(&mut self, from: u64, value: u128) -> GOCRunResult<u64> {
        RunResult::new(
            self.0.send_with_value(from, Action::Enter, value),
            |actor_id| Event::PlayerAdded(actor_id.into()),
        )
    }

    pub fn pick_winner(&mut self, from: u64) -> GOCRunResult<ActorId> {
        RunResult::new(self.0.send(from, Action::PickWinner), Event::Winner)
    }
}

pub struct GOCMetaState<'a>(&'a InnerProgram<'a>);

impl GOCMetaState<'_> {
    pub fn all(self) -> MetaStateReply<State> {
        MetaStateReply(self.0.read_state().unwrap())
    }
}

pub fn predict_winner(system: &System, players: &[u64]) -> ActorId {
    let mut random_data = [0; 4];

    Xoshiro128PlusPlus::seed_from_u64(system.block_timestamp()).fill_bytes(&mut random_data);

    let mystical_number = u32::from_le_bytes(random_data) as usize;

    players[mystical_number % players.len()].into()
}
