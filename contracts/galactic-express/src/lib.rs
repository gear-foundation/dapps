#![no_std]

use galactic_express_io::*;
use gear_lib::tx_manager::TransactionManager;
use gstd::{
    collections::HashMap,
    errors::Error as GstdError,
    exec, iter, msg,
    ops::{Add, Rem, Sub},
    prelude::*,
    ActorId,
};
use num_traits::FromBytes;

struct Random {
    index: usize,
    random: [u8; 32],
}

impl Random {
    fn new() -> Result<Self, Error> {
        exec::random([0; 32])
            .map(|(random, _)| Self { index: 0, random })
            .map_err(|error| GstdError::from(error).into())
    }

    fn next(&mut self) -> u8 {
        let next = *self
            .random
            .get(self.index)
            .expect("index for the random array traversing must'n overflow");

        self.index += 1;

        next
    }

    fn generate<T, const N: usize>(&mut self, min: T, max: T) -> T
    where
        T: FromBytes<Bytes = [u8; N]>
            + Add<T, Output = T>
            + Sub<T, Output = T>
            + Rem<T, Output = T>
            + Copy,
    {
        min + T::from_le_bytes(&array::from_fn(|_| self.next())) % (max - min)
    }

    fn chance(&mut self, probability: u8) -> bool {
        assert!(probability < 101, "probability can't be more than 100");

        self.next() % 100 < probability
    }
}

static mut STATE: Option<(Contract, TransactionManager<()>)> = None;

fn state_mut() -> Result<&'static mut (Contract, TransactionManager<()>), Error> {
    unsafe { STATE.as_mut().ok_or(Error::StateUninitaliazed) }
}

enum Stage {
    Registration(HashMap<ActorId, Participant>),
    Results(Results),
}

impl Stage {
    fn mut_participants(&mut self) -> Result<&mut HashMap<ActorId, Participant>, Error> {
        if let Stage::Registration(participants) = self {
            Ok(participants)
        } else {
            Err(Error::SessionEnded)
        }
    }
}

impl Default for Stage {
    fn default() -> Self {
        Self::Results(Results::default())
    }
}

#[derive(Default)]
struct Contract {
    admin: ActorId,
    session_id: u128,
    altitude: u16,
    weather: Weather,
    reward: u128,
    stage: Stage,
}

impl Contract {
    fn check_admin(&self) -> Result<(), Error> {
        check_admin(self.admin)
    }

    fn create_new_session(&mut self) -> Result<Event, Error> {
        let stage = &mut self.stage;

        match stage {
            Stage::Registration(participants) => {
                check_admin(self.admin)?;
                participants.clear();
            }
            Stage::Results { .. } => *stage = Stage::Registration(HashMap::new()),
        }

        let mut random = Random::new()?;

        self.weather = match random.next() % (Weather::Tornado as u8 + 1) {
            0 => Weather::Clear,
            1 => Weather::Cloudy,
            2 => Weather::Rainy,
            3 => Weather::Stormy,
            4 => Weather::Thunder,
            5 => Weather::Tornado,
            _ => unreachable!(),
        };
        self.altitude = random.generate(TURN_ALTITUDE.0, TURN_ALTITUDE.1) * TURNS as u16;
        self.reward = random.generate(REWARD.0, REWARD.1);

        Ok(Event::NewSession(Session {
            session_id: self.session_id,
            altitude: self.altitude,
            weather: self.weather,
            reward: self.reward,
        }))
    }

    fn register(&mut self, participant: Participant) -> Result<Event, Error> {
        let msg_source = msg::source();

        if msg_source == self.admin {
            return Err(Error::AccessDenied);
        }
        if let Stage::Results(_) = self.stage {
            return Err(Error::SessionEnded);
        }

        let participants = self.stage.mut_participants()?;

        if participants.len() >= PARTICIPANTS - 1 {
            return Err(Error::SessionFull);
        }

        participant.check()?;
        participants.insert(msg_source, participant);

        Ok(Event::Registered(msg_source, participant))
    }

    async fn start_game(&mut self, mut participant: Participant) -> Result<Event, Error> {
        self.check_admin()?;

        let participants = self.stage.mut_participants()?;

        if participants.is_empty() {
            return Err(Error::NotEnoughParticipants);
        }

        participant.check()?;

        let mut random = Random::new()?;
        let mut turns = HashMap::new();

        for (actor, participant) in participants
            .into_iter()
            .chain(iter::once((&msg::source(), &mut participant)))
        {
            let mut actor_turns = Vec::with_capacity(TURNS);
            let mut remaining_fuel = participant.fuel_amount;

            for turn_index in 0..TURNS {
                match turn(
                    turn_index,
                    remaining_fuel,
                    &mut random,
                    self.weather,
                    participant.payload_amount,
                ) {
                    Ok(fuel_left) => {
                        remaining_fuel = fuel_left;

                        actor_turns.push(Turn::Alive {
                            fuel_left,
                            payload_amount: participant.payload_amount,
                        });
                    }
                    Err(halt_reason) => {
                        actor_turns.push(Turn::Destroyed(halt_reason));

                        break;
                    }
                }
            }

            turns.insert(*actor, actor_turns);
        }

        let mut scores: Vec<(ActorId, u128)> = turns
            .iter()
            .map(|(actor, turns)| {
                let last_turn = turns.last().expect("there must be at least 1 turn");

                (
                    *actor,
                    match last_turn {
                        Turn::Alive {
                            fuel_left,
                            payload_amount,
                        } => (*payload_amount as u128 + *fuel_left as u128) * self.altitude as u128,
                        Turn::Destroyed(_) => 0,
                    },
                )
            })
            .collect();

        scores.sort_by(|(_, score_a), (_, score_b)| score_a.cmp(score_b));

        let mut io_turns: Vec<Vec<(ActorId, Turn)>> = vec![vec![]; 3];

        for i in 0..TURNS {
            for (actor, actor_turns) in &turns {
                let turn = actor_turns
                    .get(i)
                    .unwrap_or_else(|| &actor_turns.last().expect("There must be at least 1 turn"));
                io_turns[i].push((*actor, *turn));
            }
        }

        let results = Results {
            turns: io_turns,
            rankings: scores,
        };

        self.session_id = self.session_id.wrapping_add(1);
        self.stage = Stage::Results(results.clone());

        Ok(Event::GameFinished(results))
    }
}

fn check_admin(admin: ActorId) -> Result<(), Error> {
    if msg::source() != admin {
        Err(Error::AccessDenied)
    } else {
        Ok(())
    }
}

fn turn(
    turn: usize,
    remaining_fuel: u8,
    random: &mut Random,
    weather: Weather,
    payload: u8,
) -> Result<u8, HaltReason> {
    let new_remaining_fuel =
        match remaining_fuel.checked_sub((payload + 2 * weather as u8) / TURNS as u8) {
            Some(actual_fuel) => actual_fuel,
            None => return Err(HaltReason::FuelShortage),
        };

    match turn {
        0 => {
            // values in "chance" are transmitted as percentages
            if random.chance(3) {
                return Err(HaltReason::EngineFailure);
            }
            // this trap for someone who specified a lot of fuel
            if remaining_fuel >= PENALTY_LEVEL - 2 * weather as u8 && random.chance(10) {
                return Err(HaltReason::FuelOverload);
            }
        }
        1 => {
            // this trap for someone who specified a lot of payload
            if payload >= PENALTY_LEVEL - 2 * weather as u8 && random.chance(10) {
                return Err(HaltReason::PayloadOverload);
            }

            if random.chance(5 + weather as u8) {
                return Err(HaltReason::SeparationFailure);
            }
        }
        2 => {
            if random.chance(10 + weather as u8) {
                return Err(HaltReason::AsteroidCollision);
            }
        }
        _ => unreachable!(),
    }

    Ok(new_remaining_fuel)
}

#[no_mangle]
extern fn init() {
    msg::reply(process_init(), 0).expect("failed to encode or reply from `main()`");
}

fn process_init() -> Result<(), Error> {
    unsafe {
        STATE = Some((
            Contract {
                admin: msg::source(),
                ..Default::default()
            },
            TransactionManager::new(),
        ));
    }

    Ok(())
}

#[gstd::async_main]
async fn main() {
    msg::reply(process_main().await, 0).expect("failed to encode or reply from `main()`");
}

async fn process_main() -> Result<Event, Error> {
    let action = msg::load()?;
    let (contract, _tx_manager) = state_mut()?;

    match action {
        Action::ChangeAdmin(actor) => {
            contract.check_admin()?;

            let old_admin = contract.admin;

            contract.admin = actor;

            Ok(Event::AdminChanged(old_admin, contract.admin))
        }
        Action::CreateNewSession => contract.create_new_session(),
        Action::Register(participant) => contract.register(participant),
        Action::StartGame(participant) => contract.start_game(participant).await,
    }
}

#[no_mangle]
extern fn state() {
    let (state, _tx_manager) = unsafe { STATE.take().expect("Unexpected error in taking state") };
    msg::reply::<State>(state.into(), 0)
        .expect("Failed to encode or reply with `State` from `state()`");
}

impl From<Contract> for State {
    fn from(value: Contract) -> Self {
        let Contract {
            admin,
            session_id,
            altitude,
            weather,
            reward,
            stage,
        } = value;

        let is_session_ended: bool;
        let participants: Vec<(ActorId, Participant)>;
        let turns: Vec<Vec<(ActorId, Turn)>>;
        let rankings: Vec<(ActorId, u128)>;

        match stage {
            Stage::Registration(participants_data) => {
                is_session_ended = false;
                participants = participants_data.into_iter().collect();
                turns = vec![vec![]];
                rankings = Vec::new();
            }
            Stage::Results(results) => {
                is_session_ended = true;
                participants = Vec::new();
                turns = results.turns;
                rankings = results.rankings;
            }
        };

        Self {
            admin,
            session: Session {
                session_id,
                altitude,
                weather,
                reward,
            },
            is_session_ended,
            participants,
            turns,
            rankings,
        }
    }
}
