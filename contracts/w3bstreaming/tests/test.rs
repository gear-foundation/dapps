use gtest::{Program, System};
use w3bstreaming_io::*;

pub const USERS: [u64; 3] = [10, 11, 12];

fn edit_profile(
    system: &System,
    web_stream: &Program<'_>,
    from: u64,
    name: Option<String>,
    surname: Option<String>,
    img_link: Option<String>,
    error: bool,
) {
    let mid = web_stream.send(
        from,
        Action::EditProfile {
            name,
            surname,
            img_link,
        },
    );
    let res = system.run_next_block();
    assert_eq!(error, res.failed.contains(&mid));
}

#[allow(clippy::too_many_arguments)]
fn new_stream(
    system: &System,
    web_stream: &Program<'_>,
    from: u64,
    title: String,
    description: Option<String>,
    start_time: u64,
    end_time: u64,
    img_link: String,
    error: bool,
) {
    let mid = web_stream.send(
        from,
        Action::NewStream {
            title,
            description,
            start_time,
            end_time,
            img_link,
        },
    );
    let res = system.run_next_block();
    assert_eq!(error, res.failed.contains(&mid));
}

#[allow(clippy::too_many_arguments)]
fn edit_stream(
    system: &System,
    web_stream: &Program<'_>,
    from: u64,
    stream_id: String,
    start_time: Option<u64>,
    end_time: Option<u64>,
    title: Option<String>,
    img_link: Option<String>,
    description: Option<String>,
    error: bool,
) {
    let mid = web_stream.send(
        from,
        Action::EditStream {
            stream_id,
            start_time,
            end_time,
            title,
            img_link,
            description,
        },
    );
    let res = system.run_next_block();
    assert_eq!(error, res.failed.contains(&mid));
}

fn delete_stream(
    system: &System,
    web_stream: &Program<'_>,
    from: u64,
    stream_id: String,
    error: bool,
) {
    let mid = web_stream.send(from, Action::DeleteStream { stream_id });
    let res = system.run_next_block();
    assert_eq!(error, res.failed.contains(&mid));
}

#[test]
fn success() {
    let system = System::new();
    system.init_logger();
    system.mint_to(USERS[0], 100_000_000_000_000);
    let web_stream = Program::current(&system);

    let mid = web_stream.send(USERS[0], 0);
    let res = system.run_next_block();
    assert!(res.succeed.contains(&mid));

    edit_profile(&system, &web_stream, USERS[0], None, None, None, false);

    let state: State = web_stream.read_state(0).expect("Can't read state");
    assert_eq!(state.users[0].0, USERS[0].into());

    new_stream(
        &system,
        &web_stream,
        USERS[0],
        "Title".to_string(),
        None,
        10,
        100,
        "img_link".to_string(),
        false,
    );

    let state: State = web_stream.read_state(0).expect("Can't read state");
    assert_eq!(state.streams[0].1.broadcaster, USERS[0].into());
    let stream_id = state.streams[0].0.clone();

    edit_stream(
        &system,
        &web_stream,
        USERS[0],
        stream_id.clone(),
        Some(20),
        Some(200),
        Some("title_update".to_string()),
        None,
        None,
        false,
    );

    let stream = Stream {
        broadcaster: USERS[0].into(),
        start_time: 20,
        end_time: 200,
        title: "title_update".to_string(),
        img_link: "img_link".to_string(),
        description: None,
    };

    let state: State = web_stream.read_state(0).expect("Can't read state");
    assert_eq!(state.streams[0].1, stream);

    delete_stream(&system, &web_stream, USERS[0], stream_id, false);

    let state: State = web_stream.read_state(0).expect("Can't read state");
    assert!(state.streams.is_empty());
}

#[test]
fn failures() {
    let system = System::new();
    system.init_logger();
    system.mint_to(USERS[0], 100_000_000_000_000);
    system.mint_to(USERS[1], 100_000_000_000_000);
    let web_stream = Program::current(&system);

    let mid = web_stream.send(USERS[0], 0);
    let res = system.run_next_block();
    assert!(res.succeed.contains(&mid));

    // not registered
    new_stream(
        &system,
        &web_stream,
        USERS[0],
        "Title".to_string(),
        None,
        10,
        100,
        "img_link".to_string(),
        true,
    );

    edit_profile(&system, &web_stream, USERS[0], None, None, None, false);

    let state: State = web_stream.read_state(0).expect("Can't read state");
    assert_eq!(state.users[0].0, USERS[0].into());

    new_stream(
        &system,
        &web_stream,
        USERS[0],
        "Title".to_string(),
        None,
        10,
        100,
        "img_link".to_string(),
        false,
    );

    let state: State = web_stream.read_state(0).expect("Can't read state");
    assert_eq!(state.streams[0].1.broadcaster, USERS[0].into());
    let stream_id = state.streams[0].0.clone();

    // Not broadcaster
    edit_stream(
        &system,
        &web_stream,
        USERS[1],
        stream_id.clone(),
        Some(20),
        Some(200),
        Some("title_update".to_string()),
        None,
        None,
        true,
    );

    // Account is no registered
    delete_stream(&system, &web_stream, USERS[1], stream_id, true);
}
