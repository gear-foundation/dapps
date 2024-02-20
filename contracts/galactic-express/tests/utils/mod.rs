use common::{InitResult, Program, RunResult};
use galactic_express_io::*;
use gstd::{collections::HashSet, prelude::*, ActorId};
use gtest::{Program as InnerProgram, System};

mod common;

pub mod prelude;

pub use common::initialize_system;

pub const FOREIGN_USER: u64 = 1029384756123;
pub const ADMIN: u64 = 10;
pub const PLAYERS: [u64; 3] = [12, 13, 14];

type GalExResult<T, C = ()> = RunResult<T, C, Event, Error>;

pub struct GalEx<'a>(InnerProgram<'a>);

impl Program for GalEx<'_> {
    fn inner_program(&self) -> &InnerProgram<'_> {
        &self.0
    }
}

impl<'a> GalEx<'a> {
    pub fn initialize(system: &'a System, from: u64) -> Self {
        let program = InnerProgram::current(system);

        let result = program.send(from, 0);
        let is_active = system.is_active_program(program.id());

        InitResult::<_, Error>::new(Self(program), result, is_active).succeed()
    }

    pub fn create_new_session(
        &mut self,
        from: u64,
        name: String,
        bid: u128,
    ) -> GalExResult<u128, u128> {
        RunResult::new(
            self.0
                .send_with_value(from, Action::CreateNewSession { name }, bid),
            |event, _id| {
                if let Event::NewSessionCreated {
                    altitude, reward, ..
                } = event
                {
                    assert!(((TURN_ALTITUDE.0 * (TURNS as u16))
                        ..(TURN_ALTITUDE.1 * (TURNS as u16)))
                        .contains(&altitude));
                    assert!((REWARD.0..REWARD.1).contains(&reward));
                    reward
                } else {
                    unreachable!()
                }
            },
        )
    }

    pub fn register(
        &mut self,
        from: u64,
        creator: ActorId,
        participant: Participant,
        bid: u128,
    ) -> GalExResult<(u64, Participant)> {
        RunResult::new(
            self.0.send_with_value(
                from,
                Action::Register {
                    creator,
                    participant,
                },
                bid,
            ),
            |event, (actor, participant)| {
                assert_eq!(Event::Registered(actor.into(), participant), event)
            },
        )
    }

    pub fn cancel_register(&mut self, from: u64) -> GalExResult<(u64, Participant)> {
        RunResult::new(
            self.0.send(from, Action::CancelRegistration),
            |event, (_actor, _participant)| assert_eq!(Event::RegistrationCanceled, event),
        )
    }

    pub fn delete_player(
        &mut self,
        from: u64,
        player_id: ActorId,
    ) -> GalExResult<(u64, Participant)> {
        RunResult::new(
            self.0.send(from, Action::DeletePlayer { player_id }),
            |_, _| {},
        )
    }

    pub fn start_game(
        &mut self,
        from: u64,
        fuel_amount: u8,
        payload_amount: u8,
    ) -> GalExResult<HashSet<u64>> {
        RunResult::new(
            self.0.send(
                from,
                Action::StartGame {
                    fuel_amount,
                    payload_amount,
                },
            ),
            |event, players| {
                if let Event::GameFinished(results) = event {
                    assert!(results.turns.len() == TURNS);
                    assert!(results.rankings.len() == MAX_PARTICIPANTS);
                    assert!(results
                        .turns
                        .iter()
                        .all(|players| players.len() == MAX_PARTICIPANTS));

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

    pub fn state(&self) -> Option<State> {
        let reply = self
            .0
            .read_state(StateQuery::All)
            .expect("Unexpected invalid state.");
        if let StateReply::All(state) = reply {
            Some(state)
        } else {
            None
        }
    }
}
