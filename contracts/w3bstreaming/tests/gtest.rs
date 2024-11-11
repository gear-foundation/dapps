use sails_rs::{
    calls::*,
    gtest::{calls::*, System},
};

use w3bstreaming_client::{traits::*, Stream};

const ACTOR_ID: u64 = 42;

#[tokio::test]
async fn test_success() {
    let system = System::new();
    system.init_logger();
    system.mint_to(ACTOR_ID, 100_000_000_000_000);

    let remoting = GTestRemoting::new(system, ACTOR_ID.into());
    remoting.system().init_logger();

    // Submit program code into the system
    let program_code_id = remoting.system().submit_code(w3bstreaming::WASM_BINARY);

    let program_factory = w3bstreaming_client::W3BstreamingFactory::new(remoting.clone());

    let program_id = program_factory
        .new(None)
        .send_recv(program_code_id, b"salt")
        .await
        .unwrap();

    let mut service_client = w3bstreaming_client::W3Bstreaming::new(remoting.clone());

    service_client
        .edit_profile(None, None, None)
        .send_recv(program_id)
        .await
        .unwrap();

    let state = service_client.get_state().recv(program_id).await.unwrap();
    assert_eq!(state.users[0].0, ACTOR_ID.into());

    service_client
        .new_stream("Title".to_string(), None, 10, 100, "img_link".to_string())
        .send_recv(program_id)
        .await
        .unwrap();

    let state = service_client.get_state().recv(program_id).await.unwrap();
    assert_eq!(state.streams[0].1.broadcaster, ACTOR_ID.into());
    let stream_id = state.streams[0].0.clone();

    service_client
        .edit_stream(
            stream_id.clone(),
            Some(20),
            Some(200),
            Some("title_update".to_string()),
            None,
            None,
        )
        .send_recv(program_id)
        .await
        .unwrap();

    let stream = Stream {
        broadcaster: ACTOR_ID.into(),
        start_time: 20,
        end_time: 200,
        title: "title_update".to_string(),
        img_link: "img_link".to_string(),
        description: None,
    };

    let state = service_client.get_state().recv(program_id).await.unwrap();
    assert_eq!(state.streams[0].1, stream);

    service_client
        .delete_stream(stream_id)
        .send_recv(program_id)
        .await
        .unwrap();

    let state = service_client.get_state().recv(program_id).await.unwrap();
    assert!(state.streams.is_empty());
}
