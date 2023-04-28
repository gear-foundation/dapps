use app_io::*;
use app_state::{WASM_BINARY, WASM_EXPORTS};
use gmeta::Metadata;
use gstd::ActorId;
use gtest::{Log, Program, System};

#[test]
fn test() {
    let system = System::new();

    system.init_logger();

    let program = Program::current(&system);
    let mut result = program.send_bytes(2, []);

    assert!(!result.main_failed());

    result = program.send(2, PingPong::Pong);

    assert!(result.log().is_empty());

    // State reading

    // All state

    let mut expected_state = vec![];

    for mut actor in 0..=100 {
        actor += 2;
        result = program.send(actor, PingPong::Ping);

        assert!(result.contains(&Log::builder().payload(PingPong::Pong)));

        expected_state.push((actor.into(), 1))
    }

    let mut state: <ContractMetadata as Metadata>::State = program.read_state().unwrap();

    expected_state.sort_unstable();
    state.sort_unstable();

    assert_eq!(state, expected_state);

    // `ping_count` metafunction

    result = program.send(2, PingPong::Ping);

    assert!(result.contains(&Log::builder().payload(PingPong::Pong)));

    let ping_count: u128 = program
        .read_state_using_wasm(WASM_EXPORTS[2], WASM_BINARY.into(), Some(ActorId::from(2)))
        .unwrap();

    assert_eq!(ping_count, 2);
}
