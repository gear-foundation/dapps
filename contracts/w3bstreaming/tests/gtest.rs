use sails_rs::client::*;
use sails_rs::gtest::constants::DEFAULT_USERS_INITIAL_BALANCE;
use sails_rs::{gtest::System, ActorId};
use w3bstreaming_client::w_3_bstreaming::W3Bstreaming;
use w3bstreaming_client::Stream;
use w3bstreaming_client::W3Bstreaming as ClientW3Bstreaming;
use w3bstreaming_client::W3BstreamingCtors;

const ACTOR_ID: u64 = 42;

#[tokio::test]
async fn test_success() {
    let system = System::new();
    system.init_logger_with_default_filter("gwasm=debug,gtest=info,sails_rs=debug");
    system.mint_to(ACTOR_ID, DEFAULT_USERS_INITIAL_BALANCE);

    let env = GtestEnv::new(system, ACTOR_ID.into());

    // NOTE: if `w3bstreaming::WASM_BINARY` is not in scope in your workspace,
    // import it directly: `use w3bstreaming::WASM_BINARY;`
    let code_id = env.system().submit_code(w3bstreaming::WASM_BINARY);

    let program = env
        .deploy::<w3bstreaming_client::W3BstreamingProgram>(code_id, b"salt".to_vec())
        .new(None)
        .await
        .unwrap();

    // Service accessor name depends on codegen.
    // Most likely: `program.w3_bstreaming()`. If it doesn't compile, try `program.w3bstreaming()`.
    let mut svc = program.w_3_bstreaming();

    svc.edit_profile(None, None, None, None).await.unwrap();

    let state = svc.get_state().await.unwrap();
    assert_eq!(state.users[0].0, ActorId::from(ACTOR_ID));

    svc.new_stream("Title".to_string(), None, 10, 100, "img_link".to_string())
        .await
        .unwrap();

    let state = svc.get_state().await.unwrap();
    assert_eq!(state.streams[0].1.broadcaster, ActorId::from(ACTOR_ID));
    let stream_id = state.streams[0].0.clone();

    svc.edit_stream(
        stream_id.clone(),
        Some(20),
        Some(200),
        Some("title_update".to_string()),
        None,
        None,
    )
    .await
    .unwrap();

    let expected = Stream {
        broadcaster: ActorId::from(ACTOR_ID),
        start_time: 20,
        end_time: 200,
        title: "title_update".to_string(),
        img_link: "img_link".to_string(),
        description: None,
    };

    let state = svc.get_state().await.unwrap();
    assert_eq!(state.streams[0].1, expected);

    svc.delete_stream(stream_id).await.unwrap();

    let state = svc.get_state().await.unwrap();
    assert!(state.streams.is_empty());
}
