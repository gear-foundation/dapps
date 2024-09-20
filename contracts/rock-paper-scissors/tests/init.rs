use gtest::{Program, System};
use rock_paper_scissors_io::*;

mod routines;
pub use routines::*;

pub fn init(
    sys: &System,
    owner_user: u64,
    bet_size: u128,
    players_count_limit: u8,
    entry_timeout_ms: u64,
    move_timeout_ms: u64,
    reveal_timeout_ms: u64,
) -> Program<'_> {
    sys.init_logger();
    USERS
        .iter()
        .copied()
        .for_each(|id| sys.mint_to(id, 100_000_000_000_000));

    let program = Program::current_opt(sys);
    let result = program.send(
        owner_user,
        GameConfig {
            bet_size,
            players_count_limit,
            entry_timeout_ms,
            move_timeout_ms,
            reveal_timeout_ms,
        },
    );

    assert!(!result.main_failed());

    program
}

#[test]
#[ignore]
fn check_all_users_bet() {
    let sys = System::new();
    let entry_timout_ms = COMMON_TIMEOUT;
    let move_timout_ms = COMMON_TIMEOUT + 1;
    let reveal_timout_ms = COMMON_TIMEOUT + 2;

    let game = init(
        &sys,
        USERS[0],
        COMMON_BET,
        COMMON_PLAYERS_COUNT_LIMIT,
        entry_timout_ms,
        move_timout_ms,
        reveal_timout_ms,
    );

    register_players(&game, &USERS[0..3], COMMON_BET);
    failure_register_player(&game, USERS[3], COMMON_BET - 1);
    failure_user_move(&game, USERS[0], Move::Spock);

    sys.spend_blocks(blocks_count(entry_timout_ms / 3_000));
    failure_user_move(&game, USERS[0], Move::Spock);
    sys.spend_blocks(1);
    check_user_move(&game, USERS[0], Move::Spock);
    check_user_move(&game, USERS[1], Move::Spock);
    failure_user_move(&game, USERS[1], Move::Lizard);
    failure_user_move(&game, USERS[3], Move::Spock);

    failure_user_reveal(&game, USERS[0], Move::Spock);
    sys.spend_blocks(blocks_count(move_timout_ms / 3_000));
    failure_user_reveal(&game, USERS[0], Move::Spock);
    sys.spend_blocks(1);
    check_user_reveal_with_continue(&game, USERS[0], Move::Spock);
    failure_user_reveal(&game, USERS[2], Move::Lizard);
    failure_user_reveal(&game, USERS[1], Move::Lizard);
    sys.spend_blocks(blocks_count(reveal_timout_ms / 3_000));
    sys.spend_blocks(1);

    register_players(&game, &USERS[0..3], COMMON_BET);
}
