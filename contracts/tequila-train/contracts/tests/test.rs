use gstd::ActorId;
use gtest::{Program, System};
use tequila_io::*;

#[test]
fn test() {
    let system = System::new();

    system.init_logger();

    let program = Program::current(&system);
    let players: Players = [ActorId::zero(), ActorId::zero()].into();
    let result = program.send(2, players);

    assert!(!result.main_failed());
}
