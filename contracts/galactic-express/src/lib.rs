#![no_std]

use galactic_express_io::*;
use gear_lib::tx_manager::{Stepper, TransactionKind, TransactionManager};
use gstd::{
    collections::HashMap,
    errors::Error as GstdError,
    exec, iter, msg,
    ops::{Add, Rem, Sub},
    prelude::*,
    ActorId,
};
use num_traits::FromBytes;
use sharded_fungible_token_io::{FTokenAction, FTokenEvent, LogicAction};

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
    sft: ActorId,

    session_id: u128,
    altitude: u16,
    weather: Weather,
    fuel_price: u8,
    reward: u128,
    stage: Stage,
}

impl Contract {
    fn assert_admin(&self) -> Result<(), Error> {
        assert_admin(self.admin)
    }

    fn create_new_session(&mut self) -> Result<Event, Error> {
        let stage = &mut self.stage;

        match stage {
            Stage::Registration(participants) => {
                assert_admin(self.admin)?;

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
        self.fuel_price = random.generate(FUEL_PRICE.0, FUEL_PRICE.1);
        self.reward = random.generate(REWARD.0, REWARD.1);

        Ok(Event::NewSession(Session {
            id: self.session_id,
            altitude: self.altitude,
            weather: self.weather,
            fuel_price: self.fuel_price,
            reward: self.reward,
        }))
    }

    async fn start_game(
        &mut self,
        mut participant: Participant,
        stepper: &mut Stepper,
    ) -> Result<Event, Error> {
        self.assert_admin()?;

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
            let mut remaining_fuel = participant.fuel;

            for turn_index in 0..TURNS {
                match turn(
                    turn_index,
                    remaining_fuel,
                    &mut random,
                    self.weather,
                    participant.payload,
                ) {
                    Ok(fuel_left) => {
                        remaining_fuel = fuel_left;

                        actor_turns.push(TurnOutcome::Alive {
                            fuel_left,
                            payload: participant.payload,
                        });
                    }
                    Err(halt_reason) => {
                        actor_turns.push(TurnOutcome::Destroyed(halt_reason));

                        break;
                    }
                }
            }

            turns.insert(*actor, actor_turns);
        }

        let mut scores: Vec<_> = turns
            .iter()
            .map(|(actor, turns)| {
                let last_turn = turns.last().expect("there must be at least 1 turn");

                (
                    *actor,
                    match last_turn {
                        TurnOutcome::Alive { fuel_left, payload } => {
                            *payload as u32 * self.altitude as u32 + *fuel_left as u32
                        }
                        TurnOutcome::Destroyed(_) => 0,
                    },
                )
            })
            .collect();

        scores.sort_by(|(_, score_a), (_, score_b)| score_a.cmp(score_b));

        let mut rankings = Vec::with_capacity(scores.len());
        let mut deductible = 0;

        for (recipient, _) in scores.into_iter().rev() {
            let amount = self.reward - deductible;

            if FTokenEvent::Ok
                != msg::send_for_reply_as(
                    self.sft,
                    FTokenAction::Message {
                        transaction_id: stepper.step()?,
                        payload: LogicAction::Mint { recipient, amount },
                    },
                    0,
                    0,
                )?
                .await?
            {
                return Err(Error::TransferFailed);
            }

            if matches!(rankings.last(), Some((_, reward)) if *reward != amount) {
                deductible += self.reward / 10 * 6 / PARTICIPANTS as u128;
            }

            rankings.push((recipient, amount));
        }

        let mut io_turns: Vec<Vec<(ActorId, TurnOutcome)>> = vec![];

        for (actor, actor_turns) in turns {
            let mut last_turn = actor_turns.get(0).expect("there must be at least 1 turn");

            for i in 0..TURNS {
                if let Some(turns) = io_turns.get_mut(i) {
                    turns.push((
                        actor,
                        if let Some(turn) = actor_turns.get(i) {
                            last_turn = turn;

                            *turn
                        } else {
                            *last_turn
                        },
                    ))
                } else {
                    io_turns.push(vec![(actor, *last_turn)])
                }
            }
        }

        let results = Results {
            turns: io_turns,
            rankings,
        };

        self.session_id = self.session_id.wrapping_add(1);
        self.stage = Stage::Results(results.clone());

        Ok(Event::GameFinished(results))
    }
}

fn assert_admin(admin: ActorId) -> Result<(), Error> {
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
    let Some(new_remaining_fuel) = remaining_fuel.checked_sub((payload + 2 * weather as u8) / TURNS as u8) else {
        return Err(HaltReason::FuelShortage);
    };

    match turn {
        0 => {
            if random.chance(3) {
                return Err(HaltReason::EngineFailure);
            }

            if remaining_fuel >= 80 - 2 * weather as u8 && random.chance(10) {
                return Err(HaltReason::FuelOverload);
            }
        }
        1 => {
            if payload >= 80 - 2 * weather as u8 && random.chance(10) {
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
    reply(process_init()).expect("failed to encode or reply from `init()`");
}

fn process_init() -> Result<(), Error> {
    let Initialize { admin, sft } = msg::load()?;

    unsafe {
        STATE = Some((
            Contract {
                admin,
                sft,
                ..Default::default()
            },
            TransactionManager::new(),
        ));
    }

    Ok(())
}

#[gstd::async_main]
async fn main() {
    reply(process_main().await).expect("failed to encode or reply from `main()`");
}

async fn process_main() -> Result<Event, Error> {
    let action = msg::load()?;
    let (contract, tx_manager) = state_mut()?;

    match action {
        Action::ChangeAdmin(actor) => {
            contract.assert_admin()?;

            let old_admin = contract.admin;

            contract.admin = actor;

            Ok(Event::AdminChanged(old_admin, contract.admin))
        }
        Action::ChangeSft(actor) => {
            contract.assert_admin()?;

            let old_sft = contract.sft;

            contract.sft = actor;

            Ok(Event::SftChanged(old_sft, contract.sft))
        }
        Action::CreateNewSession => contract.create_new_session(),
        Action::Register(participant) => {
            let msg_source = msg::source();

            if msg_source == contract.admin {
                return Err(Error::AccessDenied);
            }

            let participants = contract.stage.mut_participants()?;

            if participants.len() >= PARTICIPANTS - 1 {
                return Err(Error::SessionFull);
            }

            participant.check()?;
            participants.insert(msg_source, participant);

            Ok(Event::Registered(msg_source, participant))
        }
        Action::StartGame(participant) => {
            contract
                .start_game(
                    participant,
                    &mut tx_manager
                        .acquire_transaction(msg::source(), TransactionKind::New(()))?
                        .stepper,
                )
                .await
        }
    }
}

#[no_mangle]
extern fn state() {
    let (
        Contract {
            admin,
            sft,
            session_id,
            altitude,
            weather,
            fuel_price,
            reward,
            stage,
        },
        _,
    ) = state_mut().expect("state uninitialized");

    let stage = match stage {
        Stage::Registration(participants) => {
            galactic_express_io::Stage::Registration(participants.clone().into_iter().collect())
        }
        Stage::Results(results) => galactic_express_io::Stage::Results(results.clone()),
    };

    reply(State {
        admin: *admin,
        sft: *sft,
        session: Session {
            id: *session_id,
            altitude: *altitude,
            weather: *weather,
            fuel_price: *fuel_price,
            reward: *reward,
        },
        stage,
    })
    .expect("failed to encode or reply from `state()`");
}

fn reply(payload: impl Encode) -> Result<(), GstdError> {
    msg::reply(payload, 0).map(|_| ())
}
