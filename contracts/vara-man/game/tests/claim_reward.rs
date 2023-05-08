mod utils;

use gtest::{Program, System};
use utils::{FToken, VaraMan};
use vara_man_io::Status;

#[test]
fn success() {
    let system = System::new();
    system.init_logger();

    let _ft = Program::ftoken(&system);
    let vara_man = Program::vara_man(&system);
    // TODO: Fund contract
    vara_man.change_status(Status::Started);

    let state = vara_man.get_state();
    assert!(state.players.is_empty() && state.games.is_empty());

    vara_man.register_player(utils::PLAYERS[0], "John", false);

    // TODO: Implement positive claim tests
}
