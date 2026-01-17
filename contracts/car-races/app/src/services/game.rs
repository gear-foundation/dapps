use super::{Error, config};
use sails_rs::{collections::BTreeMap, prelude::*};

pub const DEFAULT_SPEED: u32 = 100;

#[derive(Encode, Decode, TypeInfo, Clone, Debug)]
pub struct Car {
    pub position: u32,
    pub speed: u32,
    pub car_actions: Vec<RoundAction>,
    pub round_result: Option<RoundAction>,
}

#[derive(Encode, Decode, TypeInfo, Default, PartialEq, Eq, Debug, Clone)]
pub enum GameState {
    #[default]
    ReadyToStart,
    Race,
    Stopped,
    Finished,
    PlayerAction,
}

#[derive(Encode, Decode, TypeInfo, Default, Clone, Debug)]
pub struct Game {
    pub cars: BTreeMap<ActorId, Car>,
    pub car_ids: Vec<ActorId>,
    pub current_turn: u8,
    pub state: GameState,
    pub result: Option<GameResult>,
    pub current_round: u32,
    pub last_time_step: u64,
}

#[derive(Encode, Decode, TypeInfo, Clone, Debug)]
pub enum GameResult {
    Win,
    Draw,
    Lose,
}

#[derive(Encode, Decode, TypeInfo, Debug)]
pub enum StrategyAction {
    BuyAcceleration,
    BuyShell,
    Skip,
}

#[derive(Encode, Decode, TypeInfo, Debug, Clone, PartialEq, Eq)]
pub enum RoundAction {
    Accelerated,
    SlowedDown,
    SlowedDownAndAccelerated,
}

#[derive(Encode, Decode, TypeInfo)]
pub enum CarAction {
    YourTurn(BTreeMap<ActorId, Car>),
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum GameError {
    NotAdmin,
    MustBeTwoStrategies,
    GameAlreadyStarted,
    NotPlayerTurn,
    NotProgram,
    MessageProcessingSuspended,
}

impl Game {
    pub async fn process_car_turn(&mut self) -> Result<(), Error> {
        let car_id = self.get_current_car_id();

        let payload_bytes = [
            "CarStrategy".encode(),
            "MakeMove".encode(),
            self.cars.clone().encode(),
        ]
        .concat();

        let bytes = gstd::msg::send_bytes_with_gas_for_reply(
            car_id,
            payload_bytes,
            config().gas_for_round,
            0,
            config().gas_for_reply_deposit,
        )
        .expect("Error in sending a message")
        .await
        .expect("Error in receiving reply");

        if let Ok((_, _, strategy_action)) =
            <(String, String, StrategyAction)>::decode(&mut bytes.as_ref())
        {
            self.apply_strategy_move(strategy_action);
        } else {
            // car eliminated from race for wrong payload
            self.car_ids.retain(|id| *id != car_id);
        }

        let num_of_cars = self.car_ids.len() as u8;
        self.current_turn = (self.current_turn + 1) % num_of_cars;

        Ok(())
    }

    pub fn apply_strategy_move(&mut self, strategy_move: StrategyAction) {
        match strategy_move {
            StrategyAction::BuyAcceleration => {
                self.buy_acceleration();
            }
            StrategyAction::BuyShell => {
                self.buy_shell();
            }
            StrategyAction::Skip => {}
        }
    }

    pub fn buy_acceleration(&mut self) {
        let car_id = self.get_current_car_id();
        let car = self.cars.get_mut(&car_id).expect("Get Car: Can't be None");
        car.speed = car.speed.saturating_add(DEFAULT_SPEED);
        car.car_actions.push(RoundAction::Accelerated);
    }

    pub fn buy_shell(&mut self) {
        let car_id = self.get_current_car_id();

        let shelled_car_id = self.find_car_to_shell(&car_id);
        self.cars.entry(shelled_car_id).and_modify(|car| {
            let new_speed = car.speed.saturating_sub(DEFAULT_SPEED);
            car.speed = new_speed.max(DEFAULT_SPEED);
            car.car_actions.push(RoundAction::SlowedDown);
        });
    }

    pub fn get_current_car_id(&self) -> ActorId {
        self.car_ids[self.current_turn as usize]
    }

    pub fn update_positions(&mut self) {
        let mut winners = Vec::with_capacity(3);
        for (car_id, car) in self.cars.iter_mut() {
            car.position = car.position.saturating_add(car.speed * config().time);
            if car.position >= config().max_distance {
                self.state = GameState::Finished;
                winners.push((*car_id, car.position));
            }

            if !car.car_actions.is_empty() {
                car.round_result = if car.car_actions.contains(&RoundAction::Accelerated)
                    && car.car_actions.contains(&RoundAction::SlowedDown)
                {
                    Some(RoundAction::SlowedDownAndAccelerated)
                } else if car.car_actions.contains(&RoundAction::Accelerated) {
                    Some(RoundAction::Accelerated)
                } else {
                    Some(RoundAction::SlowedDown)
                };
                car.car_actions = Vec::new();
            } else {
                car.round_result = None;
            }
        }
        winners.sort_by(|a, b| b.1.cmp(&a.1));
        if self.state == GameState::Finished {
            match winners.len() {
                1 => {
                    if winners[0].0 == self.car_ids[0] {
                        self.result = Some(GameResult::Win);
                    } else {
                        self.result = Some(GameResult::Lose);
                    }
                }
                2 => {
                    if winners[0].0 == self.car_ids[0] || winners[1].0 == self.car_ids[0] {
                        if winners[0].1 == winners[1].1 {
                            self.result = Some(GameResult::Draw);
                        } else if winners[0].0 == self.car_ids[0] {
                            self.result = Some(GameResult::Win);
                        } else {
                            self.result = Some(GameResult::Lose);
                        }
                    } else {
                        self.result = Some(GameResult::Lose);
                    }
                }
                3 => {
                    if winners[0].1 == winners[1].1 && winners[0].1 == winners[2].1 {
                        self.result = Some(GameResult::Draw);
                    } else if winners[0].1 == winners[1].1 {
                        if winners[0].0 == self.car_ids[0] || winners[1].0 == self.car_ids[0] {
                            self.result = Some(GameResult::Draw);
                        } else {
                            self.result = Some(GameResult::Lose);
                        }
                    } else if winners[0].0 == self.car_ids[0] {
                        self.result = Some(GameResult::Win);
                    } else {
                        self.result = Some(GameResult::Lose);
                    }
                }
                _ => {
                    unreachable!();
                }
            }
        }
    }

    fn find_car_to_shell(&self, car_id: &ActorId) -> ActorId {
        let mut cars_vec: Vec<(ActorId, Car)> = self
            .cars
            .iter()
            .map(|(car_id, car)| (*car_id, car.clone()))
            .collect();
        cars_vec.sort_by(|a, b| b.1.position.cmp(&a.1.position));

        // if the car is the first
        // then we slowed the car that is behind it
        if cars_vec[0].0 == *car_id {
            return cars_vec[1].0;
        }

        // if the car is the second or the last
        // then we slowed the first car
        cars_vec[0].0
    }

    pub fn verify_game_state(&mut self) -> Result<(), Error> {
        if self.state != GameState::PlayerAction {
            Err(Error::NotPlayerTurn)
        } else {
            Ok(())
        }
    }

    pub fn is_player_action_or_finished(&self) -> bool {
        self.state == GameState::PlayerAction || self.state == GameState::Finished
    }
}

#[derive(Debug, Default, Encode, Decode, TypeInfo, Clone)]
pub struct RoundInfo {
    pub cars: Vec<(ActorId, u32, Option<RoundAction>)>,
    pub result: Option<GameResult>,
}
