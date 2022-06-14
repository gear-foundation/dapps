use codec::Encode;
use gtest::{Program, System};
#[path = "../src/common.rs"]
pub mod common;
use common::*;

const OWNER: [u8; 32] = [1; 32];
const SUBSCRIBER: [u8; 32] = [2; 32];

fn init_with_msg(sys: &System) {
    let feeds_channel = Program::from_file(
        sys,
        "../target/wasm32-unknown-unknown/release/gear_feeds_channel.wasm",
    );
    // ⚠️ TODO: Change the text message
    let res = feeds_channel.send(
        OWNER,
        Message {
            text: "".to_string(),
            timestamp: 0,
        },
    );

    assert!(res.log().is_empty());
}

#[test]
fn meta() {
    let sys = System::new();
    sys.init_logger();
    init_with_msg(&sys);

    let feeds_channel = sys.get_program(1);
    let res = feeds_channel.send(OWNER, ChannelAction::Meta);

    // ⚠️ TODO: Change the channel name and description
    let meta = Meta {
        name: "Channel-Coolest-Name".to_string(),
        description: "Channel-Coolest-Description".to_string(),
        owner_id: OWNER.into(),
    };

    assert!(res.contains(&(OWNER, ChannelOutput::Metadata(meta).encode())));
}

#[test]
fn subscribe_and_unsubscribe() {
    let sys = System::new();
    sys.init_logger();
    init_with_msg(&sys);

    let feeds_channel = sys.get_program(1);
    // subscribes to the channel
    feeds_channel.send(SUBSCRIBER, ChannelAction::Subscribe);

    // ⚠️ TODO: Change the post message
    let res = feeds_channel.send(OWNER, ChannelAction::Post("hello".to_string()));

    // checks that the message was sent to the owner
    // ⚠️ TODO: Change the received message
    assert!(res.contains(&(
        OWNER,
        ChannelOutput::SingleMessage(Message {
            text: "hello".to_string(),
            timestamp: 0,
        })
        .encode()
    )));

    // checks that the message was sent to the subscriber
    // ⚠️ TODO: Change the received message
    assert!(res.contains(&(
        SUBSCRIBER,
        ChannelOutput::SingleMessage(Message {
            text: "hello".to_string(),
            timestamp: 0,
        })
        .encode()
    )));

    // unsubscribes from the channel
    feeds_channel.send(SUBSCRIBER, ChannelAction::Unsubscribe);

    let res = feeds_channel.send(OWNER, ChannelAction::Post("hello".to_string()));

    // checks that the subscriber didn't receive the message
    assert!(!res.contains(&(
        SUBSCRIBER,
        ChannelOutput::SingleMessage(Message {
            text: "hello".to_string(),
            timestamp: 0,
        })
        .encode()
    )));
}

#[test]
fn check_for_failure() {
    let sys = System::new();
    sys.init_logger();
    init_with_msg(&sys);

    let feeds_channel = sys.get_program(1);

    // must fails since a subscriber is not the channel owner
    let res = feeds_channel.send(SUBSCRIBER, ChannelAction::Post("hello".to_string()));
    assert!(res.main_failed());
}
