use gstd::{prelude::*, ActorId, Encode};
use gtest::{Program, RunResult, System};
use rock_paper_scissors_io::*;

pub const USERS: &[u64] = &[3, 4, 5, 6];
pub const COMMON_USERS_SET: &[u64] = &[3, 4, 5];
pub const DEFAULT_PASSWORD: &str = "pass12";
pub const COMMON_BET: u128 = 1_000_000;
pub const START_BALANCE: u128 = 1_000_000_000;
pub const COMMON_PLAYERS_COUNT_LIMIT: u8 = 5;
pub const COMMON_TIMEOUT: u64 = 5_000;
pub const COMMON_CONFIG: GameConfig = GameConfig {
    bet_size: COMMON_BET,
    players_count_limit: COMMON_PLAYERS_COUNT_LIMIT,
    entry_timeout_ms: COMMON_TIMEOUT,
    move_timeout_ms: COMMON_TIMEOUT,
    reveal_timeout_ms: COMMON_TIMEOUT,
};

pub trait NumberConvertable {
    fn number(&self) -> u8;
}

impl NumberConvertable for Move {
    fn number(&self) -> u8 {
        match self {
            Move::Rock => 0,
            Move::Paper => 1,
            Move::Scissors => 2,
            Move::Lizard => 3,
            Move::Spock => 4,
        }
    }
}

pub fn blocks_count(timout: u64) -> u32 {
    timout as _
}

pub fn common_init(sys: &System) -> Program {
    common_init_with_owner_and_bet(sys, USERS[0], COMMON_BET)
}

pub fn common_init_with_owner_and_bet(sys: &System, owner_user: u64, bet_size: u128) -> Program {
    sys.init_logger();
    USERS
        .iter()
        .copied()
        .for_each(|id| sys.mint_to(id, START_BALANCE));
    let program = Program::current_opt(sys);
    let result = program.send(
        owner_user,
        GameConfig {
            bet_size,
            players_count_limit: COMMON_PLAYERS_COUNT_LIMIT,
            entry_timeout_ms: COMMON_TIMEOUT,
            move_timeout_ms: COMMON_TIMEOUT,
            reveal_timeout_ms: COMMON_TIMEOUT,
        },
    );

    assert!(!result.main_failed());

    program
}

pub fn common_init_and_register(sys: &System) -> Program {
    init_and_register_with_users(sys, COMMON_USERS_SET)
}

pub fn init_and_register_with_users<'a>(sys: &'a System, users: &[u64]) -> Program<'a> {
    init_register_users_and_wait_until_move_stage(sys, USERS[0], users, COMMON_BET)
}

fn init_register_users_and_wait_until_move_stage<'a>(
    sys: &'a System,
    owner_user: u64,
    players: &[u64],
    bet_size: u128,
) -> Program<'a> {
    let program = common_init_with_owner_and_bet(sys, owner_user, bet_size);
    register_players(&program, players, bet_size);
    sys.spend_blocks(blocks_count(COMMON_TIMEOUT / 1_000 + 1));

    program
}

pub fn register_players(program: &Program, players: &[u64], bet_size: u128) {
    players
        .iter()
        .for_each(|player| check_register_player(program, *player, bet_size));
}

pub fn reach_reveal_stage_with_init<'a>(
    sys: &'a System,
    users: &[u64],
    moves: &[Move],
) -> Program<'a> {
    let game = init_and_register_with_users(sys, users);
    reach_reveal_stage(&game, users, moves);

    game
}

pub fn reach_reveal_stage(game: &Program, users: &[u64], moves: &[Move]) {
    assert_eq!(users.len(), moves.len());

    users
        .iter()
        .copied()
        .zip(moves.iter().cloned())
        .for_each(|(user, users_move)| check_user_move(game, user, users_move));
}

pub fn play_round(game: &Program, users: &[u64], moves: &[Move]) -> RunResult {
    reach_reveal_stage(game, users, moves);

    for (user, users_move) in users
        .iter()
        .take(users.len() - 1)
        .zip(moves.iter().take(users.len() - 1))
    {
        check_user_reveal_with_continue(game, *user, users_move.clone());
    }

    try_to_reveal(game, *users.last().unwrap(), moves.last().cloned().unwrap())
}

pub fn check_user_move(program: &Program, player: u64, users_move: Move) {
    let result = try_to_move(program, player, users_move);

    assert!(result.contains(&(player, Event::SuccessfulMove(player.into()).encode())));
}

pub fn failure_user_move(program: &Program, player: u64, users_move: Move) {
    let result = try_to_move(program, player, users_move);

    assert!(result.main_failed());
}

pub fn try_to_move(program: &Program, player: u64, users_move: Move) -> RunResult {
    let move_with_pass = users_move.number().to_string() + DEFAULT_PASSWORD;
    let hash_bytes = sp_core_hashing::blake2_256(move_with_pass.as_bytes());
    program.send(player, Action::MakeMove(hash_bytes.to_vec()))
}

pub fn check_user_reveal_with_continue(program: &Program, player: u64, users_move: Move) {
    let result = try_to_reveal(program, player, users_move);

    assert!(result.contains(&(
        player,
        Event::SuccessfulReveal(RevealResult::Continue).encode()
    )));
}

pub fn check_user_reveal_with_next_round(
    program: &Program,
    player: u64,
    users_move: Move,
    next_round_players: BTreeSet<ActorId>,
) {
    let result = try_to_reveal(program, player, users_move);

    assert!(result.contains(&(
        player,
        Event::SuccessfulReveal(RevealResult::NextRoundStarted {
            players: next_round_players
        })
        .encode()
    )));
}

pub fn check_user_reveal_with_game_over(
    program: &Program,
    player: u64,
    users_move: Move,
    winner: ActorId,
) {
    let result = try_to_reveal(program, player, users_move);

    assert!(result.contains(&(
        player,
        Event::SuccessfulReveal(RevealResult::GameOver { winner }).encode()
    )));
}

pub fn failure_user_reveal(program: &Program, player: u64, users_move: Move) {
    let result = try_to_reveal(program, player, users_move);

    assert!(result.main_failed());
}

pub fn failure_user_reveal_with_password(
    program: &Program,
    player: u64,
    users_move: Move,
    password: &str,
) {
    let result = try_to_reveal_with_password(program, player, users_move, password);

    assert!(result.main_failed());
}

fn try_to_reveal(program: &Program, player: u64, users_move: Move) -> RunResult {
    try_to_reveal_with_password(program, player, users_move, DEFAULT_PASSWORD)
}

fn try_to_reveal_with_password(
    program: &Program,
    player: u64,
    users_move: Move,
    password: &str,
) -> RunResult {
    let move_with_pass = users_move.number().to_string() + password;

    program.send(player, Action::Reveal(move_with_pass.as_bytes().to_vec()))
}

pub fn check_register_player(program: &Program, from: u64, bet: u128) {
    let result = program.send_with_value(from, Action::Register, bet);

    assert!(result.contains(&(from, Event::PlayerRegistered.encode())));
}

pub fn failure_register_player(program: &Program, from: u64, bet: u128) {
    let result = program.send_with_value(from, Action::Register, bet);

    assert!(result.main_failed());
}

pub fn check_change_next_game_config(program: &Program, from: u64, config: GameConfig) {
    let result = program.send(from, Action::ChangeNextGameConfig(config));

    assert!(result.contains(&(from, Event::GameConfigChanged.encode())));
}

pub fn failure_change_next_game_config(program: &Program, from: u64, config: GameConfig) {
    let result = program.send(from, Action::ChangeNextGameConfig(config));

    assert!(result.main_failed());
}

pub fn check_stop_the_game(program: &Program, from: u64, rewarded_users: &[u64]) {
    let result = program.send(from, Action::StopGame);
    let rewarded_users = rewarded_users.iter().cloned().map(Into::into).collect();
    assert!(result.contains(&(from, Event::GameStopped(rewarded_users).encode())));
}

pub fn failure_stop_the_game(program: &Program, from: u64) {
    let result = program.send(from, Action::StopGame);

    assert!(result.main_failed());
}

pub fn check_users_balance(sys: &System, user: &u64, balance: u128) {
    let user_balance = sys.balance_of(*user);
    assert_eq!(balance, user_balance);
}
