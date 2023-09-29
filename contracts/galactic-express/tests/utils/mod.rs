use common::{InitResult, Program, RunResult};
use galactic_express_io::*;
use gstd::{collections::HashSet, prelude::*, ActorId};
use gtest::{Program as InnerProgram, System};

mod common;
mod sft;

pub mod prelude;

pub use common::initialize_system;
pub use sft::Sft;

pub const FOREIGN_USER: u64 = 1029384756123;
pub const ADMINS: [u64; 2] = [123, 321];
pub const PLAYERS: [u64; 3] = [1234, 4321, 2332];

type GalExResult<T, C = ()> = RunResult<T, C, Event, Error>;

pub struct GalEx<'a>(InnerProgram<'a>);

impl Program for GalEx<'_> {
    fn inner_program(&self) -> &InnerProgram<'_> {
        &self.0
    }
}

impl<'a> GalEx<'a> {
    pub fn initialize(system: &'a System, admin: u64, sft: ActorId) -> Self {
        let program = InnerProgram::current(system);

        let result = program.send(
            FOREIGN_USER,
            Initialize {
                admin: admin.into(),
                sft,
            },
        );
        let is_active = system.is_active_program(program.id());

        InitResult::<_, Error>::new(Self(program), result, is_active).succeed()
    }

    pub fn change_admin(
        &mut self,
        from: u64,
        actor: impl Into<ActorId>,
    ) -> GalExResult<(u64, u64)> {
        RunResult::new(
            self.0.send(from, Action::ChangeAdmin(actor.into())),
            |event, (old, new)| assert_eq!(Event::AdminChanged(old.into(), new.into()), event),
        )
    }

    pub fn create_new_session(&mut self, from: u64) -> GalExResult<u128, Session> {
        RunResult::new(
            self.0.send(from, Action::CreateNewSession),
            |event, session_id| {
                if let Event::NewSession(session) = event {
                    assert_eq!(session.id, session_id);
                    assert!(((TURN_ALTITUDE.0 * (TURNS as u16))
                        ..(TURN_ALTITUDE.1 * (TURNS as u16)))
                        .contains(&session.altitude));
                    assert!((FUEL_PRICE.0..FUEL_PRICE.1).contains(&session.fuel_price));
                    assert!((REWARD.0..REWARD.1).contains(&session.reward));

                    session
                } else {
                    unreachable!()
                }
            },
        )
    }

    pub fn register(
        &mut self,
        from: u64,
        participant: Participant,
    ) -> GalExResult<(u64, Participant)> {
        RunResult::new(
            self.0.send(from, Action::Register(participant)),
            |event, (actor, participant)| {
                assert_eq!(Event::Registered(actor.into(), participant), event)
            },
        )
    }

    pub fn start_game(&mut self, from: u64, participant: Participant) -> GalExResult<HashSet<u64>> {
        RunResult::new(
            self.0.send(from, Action::StartGame(participant)),
            |event, players| {
                if let Event::GameFinished(results) = event {
                    assert!(results.turns.len() == TURNS);
                    assert!(results.rankings.len() == PARTICIPANTS);
                    assert!(results
                        .turns
                        .iter()
                        .all(|players| players.len() == PARTICIPANTS));

                    let players: HashSet<ActorId> = players.into_iter().map(|p| p.into()).collect();

                    assert!(results
                        .turns
                        .iter()
                        .map(|players| players
                            .iter()
                            .map(|(actor, _)| *actor)
                            .collect::<HashSet<_>>())
                        .all(|true_players| true_players == players));
                } else {
                    unreachable!()
                }
            },
        )
    }

    pub fn state(&self) -> State {
        self.0.read_state().unwrap()
    }
}
