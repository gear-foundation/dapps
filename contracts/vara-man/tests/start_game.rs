mod utils;
use crate::utils::*;
use gtest::{Program, System};
use utils::VaraMan;
use vara_man_io::{Level, Stage, Status};

#[test]
fn success_play_single_game() {
    let system = System::new();
    system.init_logger();

    let vara_man = Program::vara_man(&system);
    system.mint_to(VARA_MAN_ID, VARA_MAN_FUND);
    vara_man.change_status(ADMIN, Status::StartedWithNativeToken);
    let old_balance = system.balance_of(PLAYERS[0]);
    vara_man.finish_single_game(PLAYERS[0], 1, 5, None);
    system.claim_value_from_mailbox(PLAYERS[0]);
    let new_balance = system.balance_of(PLAYERS[0]);
    assert_eq!(new_balance - old_balance, 100_000_000_000_000);
}

#[test]
fn success_play_tournament() {
    let system = System::new();
    system.init_logger();

    let vara_man = Program::vara_man(&system);
    system.mint_to(VARA_MAN_ID, VARA_MAN_FUND);
    system.mint_to(PLAYERS[0], VARA_MAN_FUND);
    system.mint_to(PLAYERS[1], VARA_MAN_FUND);

    vara_man.change_status(ADMIN, Status::StartedWithNativeToken);

    vara_man.create_tournament(
        PLAYERS[0],
        "TOURNAMENT".to_string(),
        "Admin tournament".to_string(),
        Level::Easy,
        180_000,
        10_000_000_000_000,
        None,
    );

    let state = vara_man.get_state().expect("Unexpected invalid state.");
    assert_eq!(state.tournaments.len(), 1);
    assert_eq!(state.players_to_game_id.len(), 1);

    vara_man.register(
        PLAYERS[1],
        PLAYERS[0].into(),
        "player #1".to_string(),
        10_000_000_000_000,
        None,
    );
    let state = vara_man.get_state().expect("Unexpected invalid state.");
    assert_eq!(state.tournaments[0].1.participants.len(), 2);

    let old_balance = system.balance_of(PLAYERS[1]);
    vara_man.cancel_register(PLAYERS[1], None);
    system.claim_value_from_mailbox(PLAYERS[1]);
    let new_balance = system.balance_of(PLAYERS[1]);
    assert_eq!(new_balance - old_balance, 10_000_000_000_000);

    vara_man.register(
        PLAYERS[1],
        PLAYERS[0].into(),
        "player #1".to_string(),
        10_000_000_000_000,
        None,
    );

    vara_man.start_tournament(PLAYERS[0], None);
    vara_man.record_tournament_result(PLAYERS[0], 1_000, 1, 5, None);
    vara_man.record_tournament_result(PLAYERS[1], 1_000, 1, 5, None);
    let state = vara_man.get_state().expect("Unexpected invalid state.");
    assert_eq!(state.tournaments[0].1.participants[1].1.points, 10);

    system.spend_blocks(61);
    let state = vara_man.get_state().expect("Unexpected invalid state.");
    assert_eq!(
        state.tournaments[0].1.stage,
        Stage::Finished(vec![PLAYERS[1].into(), PLAYERS[0].into()])
    );
}
