mod utils;

use channel_io::Message;
use gstd::ActorId;
use gtest::{Program, System};
use utils::*;

#[test]
fn channels_initialization() {
    let sys = System::new();
    sys.init_logger();

    // upload and init a router program
    let _router = Program::router(&sys);

    // upload and init 2 channels
    let channel_1 = Program::channel(&sys);
    channel_1.register();
    let channel_2 = Program::channel(&sys);
    channel_2.register();

    // check that channels were registered at router contract
    let mut expected_channels: Vec<router_io::Channel> = Vec::new();

    // first channel info
    let mut channel = router_io::Channel {
        id: 2.into(),
        name: String::from("channel_io::Channel-Coolest-Name"),
        owner_id: OWNER.into(),
        description: String::from("channel_io::Channel-Coolest-Description"),
    };
    // read info about that channel from the router contract
    // router.check_channel_info(channel.clone());

    expected_channels.push(channel.clone());
    // change id to get the second channel info
    channel.id = 3.into();
    // router.check_channel_info(channel.clone());

    expected_channels.push(channel);

    // check that channels are in the router state
    // router.check_all_channel(expected_channels);

    // check that OWNER subscribes to 2 channels
    channel_1.add_subscriber(OWNER);
    channel_2.add_subscriber(OWNER);
    let _expected_subscriptions: Vec<ActorId> = vec![2.into(), 3.into()];
    // router.check_user_subscriptions(OWNER, expected_subscriptions);
}

#[test]
fn subscriptions() {
    let sys = System::new();
    sys.init_logger();

    let _router = Program::router(&sys);
    let channel = Program::channel(&sys);
    channel.register();
    channel.add_subscriber(OWNER);
    let _channel_id: ActorId = CHANNEL_ID.into();
    // add subscribers
    for subscriber in SUBSCRIBERS {
        channel.add_subscriber(*subscriber);
        // check a subscription in the router contract
        // router.check_user_subscriptions(*subscriber, vec![channel_id]);
    }

    // unsubscribe
    channel.unsubscribe(SUBSCRIBERS[1]);
    // check that subscriptions of SUBSCRIBERS[1] are empty
    // router.check_user_subscriptions(SUBSCRIBERS[1], vec![]);
}

#[test]
fn post() {
    let sys = System::new();
    sys.init_logger();

    // upload and init a router program
    Program::router(&sys);

    // upload and init a channel
    let channel = Program::channel(&sys);
    channel.register();
    channel.add_subscriber(OWNER);
    let mut expected_messages: Vec<Message> = Vec::new();
    // init message
    let mut message = Message {
        owner: OWNER.into(),
        text: String::from("channel_io::Channel \"channel_io::Channel-Coolest-Name\" was created"),
        timestamp: 0,
    };
    expected_messages.push(message.clone());
    // add subscribers
    for subscriber in SUBSCRIBERS {
        channel.add_subscriber(*subscriber);
    }

    // message for post
    message.text = String::from("Hello");
    expected_messages.push(message.clone());

    channel.post(OWNER, String::from("Hello"), message);

    // let messages: Vec<Message> = channel.meta_state(()).expect("Meta_state failed");
    // assert_eq!(expected_messages, messages);

    // must fail since not owner posted a message
    channel.post_fail(SUBSCRIBERS[0], String::from("Hello"));
}
