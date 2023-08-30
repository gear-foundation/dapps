#![no_std]

use gmeta::{In, InOut, Metadata, Out};
use gstd::{collections::BTreeSet, prelude::*, ActorId};

pub struct ContractMetadata;

impl Metadata for ContractMetadata {
    type Init = In<GameConfig>;
    type Handle = InOut<Action, Event>;
    type Reply = ();
    type Others = ();
    type Signal = ();
    type State = Out<ContractState>;
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum Move {
    Rock,
    Paper,
    Scissors,
    Lizard,
    Spock,
}

impl Move {
    pub fn new(number: u8) -> Move {
        match number {
            b'0' => Move::Rock,
            b'1' => Move::Paper,
            b'2' => Move::Scissors,
            b'3' => Move::Lizard,
            b'4' => Move::Spock,
            _ => panic!("Unknown symbol in move, {number}"),
        }
    }

    pub fn wins(&self, other: &Move) -> bool {
        match self {
            Move::Rock => match other {
                Move::Rock | Move::Paper | Move::Spock => false,
                Move::Scissors | Move::Lizard => true,
            },
            Move::Paper => match other {
                Move::Paper | Move::Scissors | Move::Lizard => false,
                Move::Rock | Move::Spock => true,
            },
            Move::Scissors => match other {
                Move::Rock | Move::Scissors | Move::Spock => false,
                Move::Paper | Move::Lizard => true,
            },
            Move::Lizard => match other {
                Move::Rock | Move::Scissors | Move::Lizard => false,
                Move::Paper | Move::Spock => true,
            },
            Move::Spock => match other {
                Move::Paper | Move::Lizard | Move::Spock => false,
                Move::Rock | Move::Scissors => true,
            },
        }
    }
}

#[derive(Debug, Default, Encode, Decode, TypeInfo, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct StageDescription {
    pub anticipated_players: BTreeSet<ActorId>,
    pub finished_players: BTreeSet<ActorId>,
}

#[derive(Debug, Default, Encode, Decode, TypeInfo, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum GameStage {
    #[default]
    Preparation,
    InProgress(StageDescription),
    Reveal(StageDescription),
}

impl GameStage {
    pub fn game_is_in_progress(&self) -> bool {
        match self {
            GameStage::Preparation => false,
            GameStage::InProgress(_) | GameStage::Reveal(_) => true,
        }
    }

    pub fn move_can_be_made(&self) -> bool {
        match self {
            GameStage::Preparation | GameStage::InProgress(_) => true,
            GameStage::Reveal(_) => false,
        }
    }

    pub fn is_player_in_game(&self, player: &ActorId) -> bool {
        match self {
            GameStage::Preparation => false,
            GameStage::InProgress(description) | GameStage::Reveal(description) => {
                description.anticipated_players.contains(player)
                    || description.finished_players.contains(player)
            }
        }
    }

    pub fn current_players(&self) -> Option<BTreeSet<ActorId>> {
        let description = match self {
            GameStage::Preparation => return None,
            GameStage::InProgress(progress_description) => progress_description,
            GameStage::Reveal(reveal_description) => reveal_description,
        };

        let players = description
            .anticipated_players
            .union(&description.finished_players)
            .copied()
            .collect();
        Some(players)
    }
}

#[derive(Debug, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct Duration {
    pub days: u64,
    pub hours: u64,
    pub minutes: u64,
}

#[derive(Debug, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum RevealResult {
    Continue,
    NextRoundStarted { players: BTreeSet<ActorId> },
    GameOver { winner: ActorId },
}

#[derive(Debug, Decode, Encode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum Action {
    /// Registers a player for the game.
    /// Player must send value to be registered
    ///
    /// # Requirements:
    /// * Game is not in progress yet. E.g. the `GameStage` must be `GameStage::Preparation`
    /// * `msg::value()` is greater or equal to `bet_size` in the config(refund will return to user).
    /// * Player not registred yet.
    /// * Lobby is not full.
    ///
    /// On success replies `Event::PlayerRegistred`.
    Register,

    /// Submits player's move to the program in encrypted form.
    /// Player can't change his move after it.
    ///
    /// # Arguments:
    /// * `Vec<u8>`: is the binary 256-bit blake2b hash of move("0" or "1" or "2" or "3" or "4") + "password".
    ///
    /// # Requirements:
    /// * The `GameStage` must be `GameStage::InProgress(StageDesciption)` where `StageDescription::anticipated_players` must contains `msg::source()`
    ///
    /// On success replies `Event::SuccessfulReveal(RevealResult)` where `RevealResult` will correspond to the situation after this reveal.
    MakeMove(Vec<u8>),

    /// Reveals the move of the player, with which players must confirm their moves.
    /// In this step the program validates that the hash submitted during the moves stage is equal
    /// to a hashed open string and save this move(first character from string) to determine the winners.
    ///
    /// # Arguments:
    /// * `Vec<u8>`: is the binary move("0" or "1" or "2" or "3" or "4") + "password" that should be equal to binary that was sent in `MakeMove(Vec<u8>)` without hashing.
    ///
    /// # Requirements:
    /// * The hashed(by program) `Reveal` binary must be equal to this round `MakeMove` binary.
    /// * The `GameStage` must be `GameStage::Reveal(StageDesciption)` where `StageDescription::anticipated_players` must contains `msg::source()`
    ///
    /// On success replies `Event::SuccessfulMove(ActorId)` where `ActorId` is the moved player's address.
    Reveal(Vec<u8>),

    /// Changes the game config of the next game.
    /// When the current game ends, this config will be applied.
    ///
    /// # Arguments:
    /// * `GameConfig`: is the config that will be applied to the next game.
    ///
    /// # Requirements:
    /// * The `msg::source()` must be the owner of the program.
    /// * `players_count_limit` of the `GameConfig` must be greater than 1
    /// * `entry_timeout` of the `GameConfig` must be greater than 5000(5 sec)
    /// * `move_timeout` of the `GameConfig` must be greater than 5000(5 sec)
    /// * `reveal_timeout` of the `GameConfig` must be greater than 5000(5 sec)
    ///
    /// On success replies `Event::GameConfigChanged`.
    ChangeNextGameConfig(GameConfig),

    /// Stops the game.
    /// This action can be used, for example, to change the configuration of the game,
    /// or if the players have gone on strike and do not want to continue playing,
    /// or if the game has gone on for a long time.
    /// When the admin stops the game, all funds are distributed among the players remaining in the game.
    /// If the game is in the registration stage, bets will be returned to the entire lobby.
    ///
    /// # Requirements:
    /// * The `msg::source()` must be the owner of the program.
    ///
    /// On success replies `Event::GameWasStopped(BTreeSet<ActorId>)` where inside are the players who got the money.
    StopGame,
}

#[derive(Debug, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum Event {
    PlayerRegistered,
    SuccessfulMove(ActorId),
    SuccessfulReveal(RevealResult),
    GameConfigChanged,
    GameStopped(BTreeSet<ActorId>),
}

#[derive(Debug, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum State {
    Config,
    LobbyList,
    GameStage,
    CurrentStageTimestamp,
}

#[derive(Debug, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum StateReply {
    Config(GameConfig),
    LobbyList(Vec<ActorId>),
    GameStage(GameStage),
    CurrentStageTimestamp(u64),
}

#[derive(Debug, Default, Clone, Encode, Decode, TypeInfo, PartialEq)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct GameConfig {
    pub bet_size: u128,
    pub players_count_limit: u8,
    pub entry_timeout_ms: u64,
    pub move_timeout_ms: u64,
    pub reveal_timeout_ms: u64,
}

#[derive(Debug, Default, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct ContractState {
    pub owner: ActorId,
    pub lobby: Vec<ActorId>,
    pub game_config: GameConfig,
    pub stage: GameStage,
    pub encrypted_moves: Vec<(ActorId, [u8; 32])>,
    pub player_moves: Vec<(ActorId, Move)>,
    pub next_game_config: Option<GameConfig>,
    pub current_stage_start_timestamp: u64,
}
