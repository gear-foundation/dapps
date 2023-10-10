use common::{InitResult, Program, RunResult};
use galactic_express_io::*;
use gstd::collections::HashSet;
use gstd::{prelude::*, ActorId};
use gtest::{Program as InnerProgram, System};

mod common;
pub mod prelude;

pub use common::initialize_system;

pub const FOREIGN_USER: u64 = 1029384756123;
pub const ADMINS: [u64; 2] = [123, 321];
pub const PLAYERS: [u64; 3] = [1234, 4321, 2332];

type RocketsRunResult<T, C = ()> = RunResult<T, C, Event, Error>;

pub struct Rockets<'a>(InnerProgram<'a>);

impl Program for Rockets<'_> {
    fn inner_program(&self) -> &InnerProgram<'_> {
        &self.0
    }
}

impl<'a> Rockets<'a> {
    pub fn initialize(system: &'a System, from: u64) -> Self {
        let program = InnerProgram::current(system);

        let result = program.send(from, 0);
        let is_active = system.is_active_program(program.id());

        InitResult::<_, Error>::new(Self(program), result, is_active).succeed()
    }

    pub fn change_admin(
        &mut self,
        from: u64,
        actor: impl Into<ActorId>,
    ) -> RocketsRunResult<(u64, u64)> {
        RunResult::new(
            self.0.send(from, Action::ChangeAdmin(actor.into())),
            |event, (old, new)| assert_eq!(Event::AdminChanged(old.into(), new.into()), event),
        )
    }

    pub fn create_new_session(&mut self, from: u64) -> RocketsRunResult<u128, Session> {
        RunResult::new(
            self.0.send(from, Action::CreateNewSession),
            |event, session_id| {
                if let Event::NewSession(session) = event {
                    assert_eq!(session.session_id, session_id);

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
    ) -> RocketsRunResult<(u64, Participant)> {
        RunResult::new(
            self.0.send(from, Action::Register(participant)),
            |event, (actor, participant)| {
                assert_eq!(Event::Registered(actor.into(), participant), event)
            },
        )
    }

    pub fn start_game(
        &mut self,
        from: u64,
        participant: Participant,
    ) -> RocketsRunResult<HashSet<u64>> {
        RunResult::new(
            self.0.send(from, Action::StartGame(participant)),
            |event, players| {
                if let Event::GameFinished(turns) = event {
                    assert!(turns.len() == TOTAL_TURNS);
                    assert!(turns.iter().all(|players| players.len() == 4));
                    let players = players.into_iter().map(|p| p.into()).collect();
                    assert!(turns
                        .iter()
                        .map(|players| players
                            .iter()
                            .map(|(actor, _)| *actor)
                            .collect::<HashSet<ActorId>>())
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
