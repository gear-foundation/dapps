#![no_std]

use gstd::{debug, exec, msg, prelude::*, ActorId};
use rps_io::*;

mod validations;
use validations::validate_game_config;

mod helper_functions;

static mut RPS_GAME: Option<RPSGame> = None;

#[derive(Debug, Default)]
struct RPSGame {
    owner: ActorId,
    lobby: BTreeSet<ActorId>,
    game_config: GameConfig,
    stage: GameStage,
    encrypted_moves: BTreeMap<ActorId, [u8; 32]>,
    player_moves: BTreeMap<ActorId, Move>,
    next_game_config: Option<GameConfig>,
    current_stage_start_timestamp: u64,
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

            self.lobby.clone()
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
unsafe extern "C" fn init() {
    let config: GameConfig = msg::load().expect("Could not load Action");
    debug!("init(): {:?}", config);

    validate_game_config(&config);

    let game = RPSGame {
        owner: msg::source(),
        game_config: config,
        current_stage_start_timestamp: exec::block_timestamp(),
        ..Default::default()
    };

    RPS_GAME = Some(game);
}

#[no_mangle]
unsafe extern "C" fn handle() {
    let action: Action = msg::load().expect("Could not load Action");
    let game: &mut RPSGame = RPS_GAME.get_or_insert(RPSGame::default());

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
unsafe extern "C" fn meta_state() -> *mut [i32; 2] {
    let query: State = msg::load().expect("failed to decode input argument");
    let game: &RPSGame = RPS_GAME.get_or_insert(RPSGame::default());

    let encoded = match query {
        State::Config => StateReply::Config(game.game_config.clone()),
        State::LobbyList => StateReply::LobbyList(game.lobby.clone().into_iter().collect()),
        State::GameStage => StateReply::GameStage(game.stage.clone()),
        State::CurrentStageTimestamp => {
            StateReply::CurrentStageTimestamp(game.current_stage_start_timestamp)
        }
    }
    .encode();

    gstd::util::to_leak_ptr(encoded)
}

gstd::metadata! {
    title: "RockPaperScissors",
    init:
        input : GameConfig,
    handle:
        input: Action,
        output: Event,
    state:
        input: State,
        output: StateReply,
}
