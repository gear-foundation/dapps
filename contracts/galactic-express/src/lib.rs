#![no_std]

use core::array;
use gstd::collections::HashMap;
use gstd::{
    errors::Error as GstdError,
    exec, msg,
    ops::{Add, Rem, Sub},
    prelude::*,
    ActorId,
};
use num_traits::FromBytes;
use galactic_express_io::*;

#[cfg(feature = "binary-vendor")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

static mut STATE: Option<Contract> = None;

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
        let next = self.random[self.index];

        self.index += 1;
        debug_assert!(
            self.index < 33,
            "overflow of the index for the random array traversing"
        );

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
        debug_assert!(probability < 101, "probability can't be more than 100");

        self.next() % 100 < probability
    }
}

struct Contract {
    admin: ActorId,

    session_id: u128,
    is_session_ended: bool,

    altitude: u16,
    weather: Weather,
    fuel_price: u8,
    reward: u128,
    participants: HashMap<ActorId, Participant>,
    turns: Vec<HashMap<ActorId, Turn>>,
    rankings: Vec<(ActorId, u128)>,
}

impl Default for Contract {
    fn default() -> Self {
        let State {
            admin,
            session,
            is_session_ended,
            ..
        } = State::default();

        Self {
            admin,

            session_id: session.session_id,
            is_session_ended,

            altitude: session.altitude,
            weather: session.weather,
            fuel_price: session.fuel_price,
            reward: session.reward,
            participants: HashMap::new(),
            turns: vec![],
            rankings: vec![],
        }
    }
}

impl Contract {
    fn change_admin(&mut self, actor: ActorId) -> Result<Event, Error> {
        let msg_source = msg::source();

        self.assert_admin(msg_source)?;

        self.admin = actor;

        Ok(Event::AdminChanged(msg_source, self.admin))
    }

    fn assert_admin(&self, actor: ActorId) -> Result<(), Error> {
        if self.admin == actor {
            Ok(())
        } else {
            Err(Error::AccessDenied)
        }
    }

    fn new_session(&mut self) -> Result<Event, Error> {
        let mut random = Random::new()?;

        let random_weather = match random.next() % 6 {
            0 => Weather::Clear,
            1 => Weather::Cloudy,
            2 => Weather::Rainy,
            3 => Weather::Stormy,
            4 => Weather::Thunder,
            5 => Weather::Tornado,
            _ => unreachable!(),
        };
        let random_fuel_price = random.generate(MIN_FUEL_PRICE, MAX_FUEL_PRICE);
        let random_reward = random.generate(MIN_REWARD, MAX_REWARD);
        let random_altitude =
            random.generate(MIN_TURN_ALTITUDE, MAX_TURN_ALTITUDE) * TOTAL_TURNS as u16;

        self.is_session_ended = false;

        self.altitude = random_altitude;
        self.weather = random_weather;
        self.fuel_price = random_fuel_price;
        self.reward = random_reward;

        self.participants.clear();
        self.turns.clear();
        self.rankings.clear();

        Ok(Event::NewSession(Session {
            session_id: self.session_id,

            altitude: self.altitude,
            weather: self.weather,
            fuel_price: self.fuel_price,
            reward: self.reward,
        }))
    }

    fn turn(
        &self,
        turn_index: usize,
        random: &mut Random,
        payload_amount: u8,
        fuel_left: u8,
    ) -> Result<u8, HaltReason> {
        let Some(new_fuel_left) = fuel_left.checked_sub((payload_amount + 2 * self.weather as u8) / TOTAL_TURNS as u8) else {
            return Err(HaltReason::FuelShortage);
        };

        match turn_index {
            0 => {
                if random.chance(3) {
                    return Err(HaltReason::EngineFailure);
                }

                if fuel_left >= 80 - 2 * self.weather as u8 && random.chance(10) {
                    return Err(HaltReason::FuelOverload);
                }
            }
            1 => {
                if payload_amount >= 80 - 2 * self.weather as u8 && random.chance(10) {
                    return Err(HaltReason::PayloadOverload);
                }

                if random.chance(5 + self.weather as u8) {
                    return Err(HaltReason::SeparationFailure);
                }
            }
            2 => {
                if random.chance(10 + self.weather as u8) {
                    return Err(HaltReason::AsteroidCollision);
                }
            }
            _ => unreachable!(),
        }

        Ok(new_fuel_left)
    }

    async fn start_game(&mut self, participant: Participant) -> Result<Event, Error> {
        self.assert_admin(msg::source())?;

        if self.participants.is_empty() {
            return Err(Error::NotEnoughParticipants);
        }

        self.register(participant, false)?;

        let mut random = Random::new()?;
        let mut turns = vec![];

        for turn_index in 0..TOTAL_TURNS {
            let mut turn: HashMap<ActorId, Turn> = HashMap::new();

            let first_turn: HashMap<ActorId, Turn> = self
                .participants
                .iter()
                .map(|(actor, participant)| {
                    (
                        *actor,
                        Turn::Alive {
                            fuel_left: participant.fuel_amount,
                            payload_amount: participant.payload_amount,
                        },
                    )
                })
                .collect();

            let participants_data = if turn_index == 0 {
                &first_turn
            } else {
                &turns[turn_index - 1]
            };

            for (actor, turn_entry) in participants_data
                .into_iter()
                .map(|(actor, turn_entry)| (*actor, *turn_entry))
            {
                let (fuel_left, payload_amount) = match turn_entry {
                    Turn::Alive {
                        fuel_left,
                        payload_amount,
                    } => (fuel_left, payload_amount),
                    destroyed => {
                        turn.insert(actor, destroyed);

                        continue;
                    }
                };

                turn.insert(
                    actor,
                    match self.turn(turn_index, &mut random, payload_amount, fuel_left) {
                        Ok(new_fuel_left) => Turn::Alive {
                            fuel_left: new_fuel_left,
                            payload_amount,
                        },
                        Err(reason) => Turn::Destroyed(reason),
                    },
                );
            }

            turns.push(turn);
        }

        self.is_session_ended = true;
        self.turns = turns.clone();
        self.session_id = self.session_id.wrapping_add(1);

        let mut rankings = vec![];

        for (participant, turn_info) in &turns[TOTAL_TURNS - 1] {
            rankings.push((
                participant,
                match turn_info {
                    Turn::Alive {
                        fuel_left,
                        payload_amount,
                    } => {
                        *payload_amount as u128 * self.reward * self.altitude as u128
                            + *fuel_left as u128
                    }
                    Turn::Destroyed(_) => 0,
                },
            ))
        }

        rankings.sort_unstable_by(|a, b| a.1.cmp(&b.1));

        let mut min_earnings = u128::MAX;
        let mut factor = 10;

        let rankings = rankings
            .into_iter()
            .rev()
            .map(|(actor, earnings)| {
                if earnings < min_earnings {
                    min_earnings = earnings;
                    factor -= 2;
                }
                (*actor, self.reward / 10 * factor)
            })
            .collect();

        self.rankings = rankings;

        Ok(Event::GameFinished(
            turns
                .into_iter()
                .map(|turn| turn.into_iter().collect())
                .collect(),
        ))
    }

    fn register(&mut self, participant: Participant, is_not_admin: bool) -> Result<Event, Error> {
        if self.is_session_ended {
            return Err(Error::EndedSession);
        }

        let msg_source = msg::source();

        if is_not_admin && msg_source == self.admin {
            return Err(Error::AccessDenied);
        }

        if is_not_admin && self.participants.len() >= 3 {
            return Err(Error::FullSession);
        }

        if participant.fuel_amount > 100 || participant.payload_amount > 100 {
            return Err(Error::FuelOrPayloadOverload);
        }

        self.participants.insert(msg_source, participant);

        Ok(Event::Registered(msg_source, participant))
    }
}

#[gstd::async_main]
async fn main() {
    msg::reply(process_handle().await, 0).expect("failed to encode or reply from `handle()`");
}

async fn process_handle() -> Result<Event, Error> {
    let action = msg::load()?;
    let contract = unsafe { STATE.as_mut().expect("state isn't initialized") };

    match action {
        Action::ChangeAdmin(actor) => contract.change_admin(actor),
        Action::CreateNewSession => contract.new_session(),
        Action::Register(participant) => contract.register(participant, true),
        Action::StartGame(participant) => contract.start_game(participant).await,
    }
}

#[gstd::async_init]
async fn init() {
    let result = process_init().await;
    let is_err = result.is_err();

    msg::reply(result, 0).expect("failed to encode or reply from `init()`");

    if is_err {
        exec::exit(msg::source());
    }
}

async fn process_init() -> Result<(), Error> {
    unsafe {
        STATE = Some(Contract {
            admin: msg::source(),
            ..Default::default()
        })
    }

    Ok(())
}

#[no_mangle]
extern fn state() {
    let state = unsafe { STATE.take().expect("Unexpected error in taking state") };
    msg::reply::<State>(state.into(), 0)
        .expect("Failed to encode or reply with `IoNft` from `state()`");
}

impl From<Contract> for State {
    fn from(value: Contract) -> Self {
        let Contract {
            admin,
            session_id,
            is_session_ended,
            altitude,
            weather,
            fuel_price,
            reward,
            participants,
            turns,
            rankings,
        } = value;

        Self {
            admin,
            session: Session {
                session_id,
                altitude,
                weather,
                fuel_price,
                reward,
            },
            is_session_ended,

            participants: participants.into_iter().collect(),
            turns: turns
                .into_iter()
                .map(|turn| turn.into_iter().collect())
                .collect(),
            rankings,
        }
    }
}
