//! Data types for the contract input/output.

#![no_std]
use codec::{Decode, Encode};
use gmeta::{In, InOut, Metadata};
use gstd::{collections::BTreeMap, prelude::*, ActorId, MessageId};
// prices for acceleration and shuffles are constants for simple implementation
pub const ACCELERATION_COST: u32 = 10;
pub const SHELL_COST: u32 = 20;

pub const DEFAULT_ACC_AMOUNT: u32 = 20;
pub const DEFAULT_SHELL_AMOUNT: u32 = 25;

pub const MAX_ACC_AMOUNT: u32 = 40;
pub const MAX_SHELL_AMOUNT: u32 = 40;
pub const MAX_DISTANCE: u32 = 3_242;
pub const TIME: u32 = 1;

pub const DEFAULT_SPEED: u32 = 100;

pub const GAS_FOR_STRATEGY: u64 = 20_000_000_000;
pub const GAS_FOR_ROUND: u64 = 150_000_000_000;
pub const RESERVATION_AMOUNT: u64 = 240_000_000_000;
pub const RESERVATION_TIME: u32 = 86_400;
pub const GAS_MIN_AMOUNT: u64 = 30_000_000_000;

/// Time deadline for player turn(30_000ms).
pub const TURN_DEADLINE_MS: u64 = 30_000;

pub const MIN_SPEED: u32 = 10;

/// Time after which the game instance must be removed
/// 1 block = 3s (1 minutes)
pub const TIME_INTERVAL: u32 = 20;

/// Gas for deleting the game instance
pub const GAS_TO_REMOVE_GAME: u64 = 20_000_000_000;

/// Contract metadata. This is the contract's interface description.
///
/// It defines the types of messages that can be sent to the contract.
pub struct ContractMetadata;

impl Metadata for ContractMetadata {
    /// Init message type.
    ///
    /// Describes incoming/outgoing types for the `init()` function.
    ///
    /// The [`GameInit`] type is passed for initial smart-contract data(i.e config..) if any.
    type Init = In<GameInit>;
    /// Handle message type.
    ///
    /// Describes incoming/outgoing types for the `handle()` function.
    ///
    /// We use the [`GameAction`] type for incoming and [`GameReply`] for outgoing
    /// messages.
    type Handle = InOut<GameAction, Result<GameReply, GameError>>;
    /// Asynchronous handle message type.
    ///
    /// Describes incoming/outgoing types for the `main()` function in case of
    /// asynchronous interaction.
    ///
    /// The unit tuple is used as we don't use asynchronous interaction in this
    /// contract.
    type Others = InOut<(), RoundInfo>;
    /// Reply message type.
    ///
    /// Describes incoming/outgoing types of messages performed using the
    /// `handle_reply()` function.
    ///
    /// The unit tuple is used as we don't process any replies in this contract.
    type Reply = ();
    /// Signal message type.
    ///
    /// Describes only the outgoing type from the program while processing the
    /// system signal.
    ///
    /// The unit tuple is used as we don't process any signals in this contract.
    type Signal = ();

    type State = InOut<StateQuery, StateReply>;
}

// This structure is for creating a gaming session, which allows players to predefine certain actions for an account that will play the game on their behalf for a certain period of time.
// Sessions can be used to send transactions from a dApp on behalf of a user without requiring their confirmation with a wallet.
// The user is guaranteed that the dApp can only execute transactions that comply with the allowed_actions of the session until the session expires.
#[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
pub struct Session {
    // the address of the player who will play on behalf of the user
    pub key: ActorId,
    // until what time the session is valid
    pub expires: u64,
    // what messages are allowed to be sent by the account (key)
    pub allowed_actions: Vec<ActionsForSession>,

    pub expires_at_block: u32,
}

#[derive(Encode, Decode, TypeInfo)]
pub struct SignatureData {
    pub key: ActorId,
    pub duration: u64,
    pub allowed_actions: Vec<ActionsForSession>,
}

#[derive(Encode, Decode, TypeInfo)]
pub enum StateQuery {
    Admins,
    StrategyIds,
    Game { account_id: ActorId },
    AllGames,
    MsgIdToGameId,
    Config,
    MessagesAllowed,
    SessionForTheAccount(ActorId),
}

#[derive(Encode, Decode, TypeInfo)]
pub enum StateReply {
    Admins(Vec<ActorId>),
    StrategyIds(Vec<ActorId>),
    Game(Option<Game>),
    AllGames(Vec<(ActorId, Game)>),
    MsgIdToGameId(Vec<(MessageId, ActorId)>),
    WaitingMsgs(Vec<(MessageId, MessageId)>),
    Config(Config),
    MessagesAllowed(bool),
    SessionForTheAccount(Option<Session>),
}

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

#[derive(Encode, Decode, TypeInfo)]
pub struct GameInit {
    pub config: Config,
}

#[derive(Encode, Decode, TypeInfo)]
pub enum GameAction {
    AddAdmin(ActorId),
    RemoveAdmin(ActorId),
    AddStrategyIds {
        car_ids: Vec<ActorId>,
    },
    StartGame {
        session_for_account: Option<ActorId>,
    },
    Play {
        account: ActorId,
    },
    PlayerMove {
        session_for_account: Option<ActorId>,
        strategy_action: StrategyAction,
    },
    UpdateConfig {
        gas_to_remove_game: Option<u64>,
        initial_speed: Option<u32>,
        min_speed: Option<u32>,
        max_speed: Option<u32>,
        gas_for_round: Option<u64>,
        time_interval: Option<u32>,
        max_distance: Option<u32>,
        time: Option<u32>,
        time_for_game_storage: Option<u64>,
        block_duration_ms: Option<u64>,
        gas_to_delete_session: Option<u64>,
    },
    RemoveGameInstance {
        account_id: ActorId,
    },
    RemoveGameInstances {
        players_ids: Option<Vec<ActorId>>,
    },
    AllowMessages(bool),
    CreateSession {
        key: ActorId,
        duration: u64,
        allowed_actions: Vec<ActionsForSession>,
        signature: Option<Vec<u8>>,
    },
    DeleteSessionFromProgram {
        account: ActorId,
    },
    DeleteSessionFromAccount,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
pub enum ActionsForSession {
    StartGame,
    PlayerMove, 
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

#[derive(Encode, Decode, TypeInfo, PartialEq, Eq)]
pub enum GameReply {
    GameFinished,
    GameStarted,
    StrategyAdded,
    MoveMade,
    GameInstanceRemoved,
    InstancesRemoved,
    AdminAdded,
    AdminRemoved,
    ConfigUpdated,
    StatusMessagesUpdated,
    SessionCreated,
    SessionDeleted,
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

    pub fn update_positions(&mut self, config: &Config) {
        let mut winners = Vec::with_capacity(3);
        for (car_id, car) in self.cars.iter_mut() {
            car.position = car.position.saturating_add(car.speed * config.time);
            if car.position >= config.max_distance {
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
}

#[derive(Debug, Default, Encode, Decode, TypeInfo, Clone)]
pub struct Config {
    pub gas_to_remove_game: u64,
    pub initial_speed: u32,
    pub min_speed: u32,
    pub max_speed: u32,
    pub gas_for_round: u64,
    pub time_interval: u32,
    pub max_distance: u32,
    pub time: u32,
    pub time_for_game_storage: u64,
    pub block_duration_ms: u64,
    pub gas_to_delete_session: u64
}

#[derive(Debug, Default, Encode, Decode, TypeInfo, Clone)]
pub struct RoundInfo {
    pub cars: Vec<(ActorId, u32, Option<RoundAction>)>,
    pub result: Option<GameResult>,
}
