#![no_std]

use galactic_express_io::*;
use gear_lib::tx_manager::TransactionManager;
use gstd::{
    collections::HashMap,
    errors::Error as GstdError,
    exec, msg,
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
    games: HashMap<ActorId, Game>,
    player_to_game_id: HashMap<ActorId, ActorId>,
}

#[derive(Default)]
pub struct Game {
    admin: ActorId,
    admin_name: String,
    bid: u128,
    altitude: u16,
    weather: Weather,
    reward: u128,
    stage: Stage,
}

impl Contract {
    fn create_new_session(&mut self, name: String) -> Result<Event, Error> {
        let msg_src = msg::source();
        let msg_value = msg::value();

        if self.player_to_game_id.contains_key(&msg_src) {
            return Err(Error::SeveralRegistrations);
        }

        if !self.games.contains_key(&msg_src) {
            let game = Game {
                admin: msg_src,
                admin_name: name,
                bid: msg_value,
                ..Default::default()
            };
            self.games.insert(msg_src, game);
        }
        let game = self.games.get_mut(&msg_src).expect("Critical error");

        let stage = &mut game.stage;

        match stage {
            Stage::Registration(participants) => {
                participants.clear();
            }
            Stage::Results { .. } => *stage = Stage::Registration(HashMap::new()),
        }

        let mut random = Random::new()?;

        game.weather = match random.next() % (Weather::Tornado as u8 + 1) {
            0 => Weather::Clear,
            1 => Weather::Cloudy,
            2 => Weather::Rainy,
            3 => Weather::Stormy,
            4 => Weather::Thunder,
            5 => Weather::Tornado,
            _ => unreachable!(),
        };
        game.altitude = random.generate(TURN_ALTITUDE.0, TURN_ALTITUDE.1) * TURNS as u16;
        game.reward = random.generate(REWARD.0, REWARD.1);
        self.player_to_game_id.insert(msg_src, msg_src);

        Ok(Event::NewSession {
            altitude: game.altitude,
            weather: game.weather,
            reward: game.reward,
            bid: msg_value,
        })
    }

    fn cancel_game(&mut self) -> Result<Event, Error> {
        let msg_src = msg::source();
        let game = self.games.get(&msg_src).ok_or(Error::NoSuchGame)?;

        match &game.stage {
            Stage::Registration(players) => {
                players.iter().for_each(|(id, _)| {
                    send_value(*id, game.bid);
                    self.player_to_game_id.remove(id);
                });
            }
            Stage::Results(results) => {
                results.rankings.iter().for_each(|(id, _)| {
                    self.player_to_game_id.remove(id);
                });
            }
        } 

        self.player_to_game_id.remove(&msg_src);
        self.games.remove(&msg_src);
        Ok(Event::GameCanceled)
    }

    fn leave_game(&mut self) -> Result<Event, Error> {
        let msg_src = msg::source();
        self.player_to_game_id.remove(&msg_src);
        Ok(Event::GameLeft)
    }

    fn register(
        &mut self,
        creator: ActorId,
        participant: Participant,
        msg_source: ActorId,
        msg_value: u128,
    ) -> Result<Event, Error> {
        if self.player_to_game_id.contains_key(&msg_source) {
            return Err(Error::SeveralRegistrations);
        }

        if let Some(game) = self.games.get_mut(&creator) {
            if msg_value != game.bid {
                return Err(Error::WrongBid);
            }
            if let Stage::Results(_) = game.stage {
                return Err(Error::SessionEnded);
            }

            let participants = game.stage.mut_participants()?;

            if participants.contains_key(&msg_source) {
                return Err(Error::AlreadyRegistered);
            }

            if participants.len() >= PARTICIPANTS - 1 {
                return Err(Error::SessionFull);
            }

            participant.check()?;
            participants.insert(msg_source, participant.clone());
            self.player_to_game_id.insert(msg_source, creator);

            Ok(Event::Registered(msg_source, participant))
        } else {
            Err(Error::NoSuchGame)
        }
    }

    fn cancel_register(&mut self) -> Result<Event, Error> {
        let msg_source = msg::source();

        let creator = self
            .player_to_game_id
            .get(&msg_source)
            .ok_or(Error::Unregistered)?;
        let game = self.games.get_mut(creator).ok_or(Error::NoSuchGame)?;

        if msg_source != game.admin {
            let participants = game.stage.mut_participants()?;
            if participants.contains_key(&msg_source) {
                send_value(msg_source, game.bid);
                participants.remove(&msg_source).expect("Critical error");
                self.player_to_game_id.remove(&msg_source);
            } else {
                return Err(Error::NoSuchPlayer);
            }
            Ok(Event::CancelRegistration)
        } else {
            Err(Error::NotForAdmin)
        }
    }
    fn delete_player(&mut self, player_id: ActorId) -> Result<Event, Error> {
        let msg_source = msg::source();

        if let Some(game) = self.games.get_mut(&msg_source) {
            if let Stage::Results(_) = game.stage {
                return Err(Error::SessionEnded);
            }

            let participants = game.stage.mut_participants()?;

            if participants.contains_key(&player_id) {
                send_value(player_id, game.bid);
                participants.remove(&player_id).expect("Critical error");
                self.player_to_game_id.remove(&player_id);
            } else {
                return Err(Error::NoSuchPlayer);
            }

            Ok(Event::PlayerDeleted { player_id })
        } else {
            Err(Error::NoSuchGame)
        }
    }

    async fn start_game(&mut self, fuel_amount: u8, payload_amount: u8) -> Result<Event, Error> {
        let msg_source = msg::source();

        let game = self.games.get_mut(&msg_source).ok_or(Error::NoSuchGame)?;
        let participant = Participant {
            id: msg_source,
            name: game.admin_name.clone(),
            fuel_amount,
            payload_amount,
        };
        participant.check()?;
        let participants = game.stage.mut_participants()?;

        if participants.is_empty() {
            return Err(Error::NotEnoughParticipants);
        }
        participants.insert(msg_source, participant);

        let mut random = Random::new()?;
        let mut turns = HashMap::new();

        for (actor, participant) in participants.into_iter() {
            let mut actor_turns = Vec::with_capacity(TURNS);
            let mut remaining_fuel = participant.fuel_amount;

            for turn_index in 0..TURNS {
                match turn(
                    turn_index,
                    remaining_fuel,
                    &mut random,
                    game.weather,
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
                        } => (*payload_amount as u128 + *fuel_left as u128) * game.altitude as u128,
                        Turn::Destroyed(_) => 0,
                    },
                )
            })
            .collect();

        scores.sort_by(|(_, score_a), (_, score_b)| score_a.cmp(score_b));

        let mut io_turns: Vec<Vec<(ActorId, Turn)>> = vec![vec![]; TURNS];

        for (i, io_turn) in io_turns.iter_mut().enumerate().take(TURNS) {
            for (actor, actor_turns) in &turns {
                let turn = actor_turns
                    .get(i)
                    .unwrap_or_else(|| actor_turns.last().expect("There must be at least 1 turn"));
                io_turn.push((*actor, *turn));
            }
        }

        let max_value = scores.iter().map(|(_, value)| value).max().unwrap();
        let winners: Vec<_> = scores
            .iter()
            .filter_map(|(actor_id, value)| {
                if value == max_value {
                    Some(*actor_id)
                } else {
                    None
                }
            })
            .collect();
        let prize = game.bid * scores.len() as u128 / winners.len() as u128;

        if game.bid != 0 {
            winners.iter().for_each(|id| {
                send_value(*id, prize);
            });
        }

        let results = Results {
            turns: io_turns,
            rankings: scores.clone(),
        };
        game.stage = Stage::Results(results.clone());

        Ok(Event::GameFinished(results))
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

fn send_value(destination: ActorId, value: u128) {
    if value != 0 {
        msg::send_with_gas(destination, "", 0, value).expect("Error in sending value");
    }
}

#[no_mangle]
extern fn init() {
    msg::reply(process_init(), 0).expect("failed to encode or reply from `main()`");
}

fn process_init() -> Result<(), Error> {
    unsafe {
        STATE = Some((
            Contract {
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
        Action::CreateNewSession { name } => contract.create_new_session(name),
        Action::Register {
            creator,
            participant,
        } => {
            let msg_source = msg::source();
            let msg_value = msg::value();
            let reply = contract.register(creator, participant, msg_source, msg_value);
            if reply.is_err() {
                send_value(msg_source, msg_value);
            }
            reply
        }
        Action::CancelRegistration => contract.cancel_register(),
        Action::DeletePlayer { player_id } => contract.delete_player(player_id),
        Action::CancelGame => contract.cancel_game(),
        Action::LeaveGame => contract.leave_game(),
        Action::StartGame {
            fuel_amount,
            payload_amount,
        } => contract.start_game(fuel_amount, payload_amount).await,
    }
}

#[no_mangle]
extern fn state() {
    let (state, _tx_manager) = unsafe { STATE.take().expect("Unexpected error in taking state") };
    let query: StateQuery = msg::load().expect("Unable to load the state query");
    let reply = match query {
        StateQuery::All => StateReply::All(state.into()),
        StateQuery::GetGame { player_id } => {
            let game_state = state
                .player_to_game_id
                .get(&player_id)
                .and_then(|creator_id| state.games.get(creator_id))
                .map(|game| {
                    let stage = match &game.stage {
                        Stage::Registration(participants_data) => StageState::Registration(
                            participants_data.clone().into_iter().collect(),
                        ),
                        Stage::Results(results) => StageState::Results(results.clone()),
                    };

                    GameState {
                        admin: game.admin,
                        altitude: game.altitude,
                        weather: game.weather,
                        reward: game.reward,
                        stage,
                        bid: game.bid,
                    }
                });

            StateReply::Game(game_state)
        }
    };
    msg::reply(reply, 0).expect("Unable to share the state");
}

impl From<Contract> for State {
    fn from(value: Contract) -> Self {
        let Contract {
            games,
            player_to_game_id,
        } = value;

        let games = games
            .into_iter()
            .map(|(id, game)| {
                let stage = match game.stage {
                    Stage::Registration(participants_data) => {
                        StageState::Registration(participants_data.into_iter().collect())
                    }
                    Stage::Results(results) => StageState::Results(results),
                };

                let game_state = GameState {
                    admin: game.admin,
                    altitude: game.altitude,
                    weather: game.weather,
                    reward: game.reward,
                    stage,
                    bid: game.bid,
                };
                (id, game_state)
            })
            .collect();

        let player_to_game_id = player_to_game_id.into_iter().collect();

        Self {
            games,
            player_to_game_id,
        }
    }
}
