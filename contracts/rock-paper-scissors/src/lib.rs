#![no_std]

use gstd::{
    collections::{HashMap, HashSet},
    debug, exec, msg,
    prelude::*,
    ActorId,
};
use rock_paper_scissors_io::*;
use validations::validate_game_config;

mod helper_functions;
mod validations;

include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

static mut RPS_GAME: Option<RPSGame> = None;

#[derive(Debug, Default)]
pub struct RPSGame {
    pub owner: ActorId,
    pub lobby: HashSet<ActorId>,
    pub game_config: GameConfig,
    pub stage: GameStage,
    pub encrypted_moves: HashMap<ActorId, [u8; 32]>,
    pub player_moves: HashMap<ActorId, Move>,
    pub next_game_config: Option<GameConfig>,
    pub current_stage_start_timestamp: u64,
}

impl RPSGame {
    fn register(&mut self) {
        self.validate_game_is_not_in_progress();
        self.validate_bet(msg::value());
        self.validate_there_is_no_such_player(&msg::source());
        self.validate_there_is_place_for_player();

        let change = msg::value() - self.game_config.bet_size;
        self.lobby.insert(msg::source());

        msg::reply(Event::PlayerRegistered, change).expect("Can't send reply");
    }

    fn make_move(&mut self, move_hash: Vec<u8>) {
        let player_id = &msg::source();
        self.validate_player_can_make_a_move(player_id);

        self.save_move(&msg::source(), move_hash);
        self.try_to_transit_to_reveal_stage_after_move();

        msg::reply(Event::SuccessfulMove(*player_id), 0).expect("Reply error");
    }

    fn reveal(&mut self, real_move: Vec<u8>) {
        let player = &msg::source();

        self.validate_player_can_reveal(player);
        self.validate_reveal(player, real_move.as_slice());

        self.save_real_move(player, real_move);
        let result = self.end_round_if_needed();

        msg::reply(Event::SuccessfulReveal(result), 0).expect("Reply error");
    }

    fn set_next_game_config(&mut self, config: GameConfig) {
        validate_game_config(&config);
        self.validate_source_is_owner();

        self.next_game_config = Some(config);

        msg::reply(Event::GameConfigChanged, 0).expect("Reply error");
    }

    fn stop_the_game(&mut self) {
        self.validate_source_is_owner();

        let players = if matches!(self.stage, GameStage::Preparation) {
            self.lobby.iter().for_each(|player| {
                msg::send(*player, "STOP", self.game_config.bet_size).expect("Can't send reward");
            });

            self.lobby.iter().copied().collect()
        } else {
            let players = self.stage.current_players().expect("Game is not started");

            let part = exec::value_available() / players.len() as u128;

            for player in players.iter() {
                msg::send(*player, "STOP", part).expect("Can't send reward");
            }

            players
        };

        msg::reply(Event::GameStopped(players), 0).expect("Reply error");

        self.start_new_game();
    }
}

#[no_mangle]
extern fn init() {
    let config: GameConfig = msg::load().expect("Could not load Action");
    debug!("init(): {:?}", config);

    validate_game_config(&config);

    let game = RPSGame {
        owner: msg::source(),
        game_config: config,
        current_stage_start_timestamp: exec::block_timestamp(),
        ..Default::default()
    };

    unsafe { RPS_GAME = Some(game) };
}

#[no_mangle]
extern fn handle() {
    let action: Action = msg::load().expect("Could not load Action");
    let game: &mut RPSGame = unsafe { RPS_GAME.get_or_insert(RPSGame::default()) };

    game.change_stage_by_timeout_if_needed();

    match action {
        Action::Register => game.register(),
        Action::MakeMove(hashed_move) => game.make_move(hashed_move),
        Action::Reveal(real_move) => game.reveal(real_move),
        Action::ChangeNextGameConfig(config) => game.set_next_game_config(config),
        Action::StopGame => game.stop_the_game(),
    }
}

#[no_mangle]
extern fn state() {
    let game = unsafe { RPS_GAME.as_ref().expect("Unexpected error in taking state") };
    msg::reply::<ContractState>(game.into(), 0)
        .expect("Failed to encode or reply with `ContractState` from `state()`");
}

impl From<&RPSGame> for ContractState {
    fn from(value: &RPSGame) -> Self {
        let RPSGame {
            owner,
            lobby,
            game_config,
            stage,
            encrypted_moves,
            player_moves,
            next_game_config,
            current_stage_start_timestamp,
        } = value;

        let encrypted_moves = encrypted_moves
            .iter()
            .map(|(id, arr)| (*id, *arr))
            .collect();
        let player_moves = player_moves
            .iter()
            .map(|(id, mv)| (*id, mv.clone()))
            .collect();
        let lobby = lobby.iter().cloned().collect();

        Self {
            owner: *owner,
            lobby,
            game_config: game_config.clone(),
            stage: stage.clone(),
            encrypted_moves,
            player_moves,
            next_game_config: next_game_config.clone(),
            current_stage_start_timestamp: *current_stage_start_timestamp,
        }
    }
}
