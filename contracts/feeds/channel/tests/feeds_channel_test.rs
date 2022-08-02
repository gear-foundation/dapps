use channel_io::*;
use gstd::{ActorId, BTreeSet};
use gtest::{Program, System};
use router_io::*;
mod utils;
use utils::*;

#[test]
fn channels_initialization() {
    let sys = System::new();
    sys.init_logger();

    // upload and init a router program
    let router = Program::router(&sys);

    // upload and init 2 channels
    Program::channel(&sys);
    Program::channel(&sys);

    // check that channels were registered at router contract
    let mut expected_channels: Vec<Channel> = Vec::new();

    // first channel info
    let mut channel = Channel {
        id: 2.into(),
        name: String::from("Channel-Coolest-Name"),
        owner_id: OWNER.into(),
        description: String::from("Channel-Coolest-Description"),
    };
    // read info about that channel from the router contract
    router.check_channel_info(channel.clone());

    expected_channels.push(channel.clone());
    // change id to get the second channel info
    channel.id = 3.into();
    router.check_channel_info(channel.clone());

    expected_channels.push(channel);

    // check that channels are in the router state
    router.check_all_channel(expected_channels);

    // check that OWNER subscribes to 2 channels
    let mut expected_subscriptions: BTreeSet<ActorId> = BTreeSet::new();
    expected_subscriptions.insert(2.into());
    expected_subscriptions.insert(3.into());

    router.check_user_subscriptions(OWNER, expected_subscriptions);
}

#[test]
fn subscriptions() {
    let sys = System::new();
    sys.init_logger();

    let router = Program::router(&sys);
    let channel = Program::channel(&sys);

    let channel_id: ActorId = CHANNEL_ID.into();
    // add subscribers
    for subscriber in SUBSCRIBERS {
        channel.add_subscriber(*subscriber);
        // check a subscription in the router contract
        router.check_user_subscriptions(*subscriber, BTreeSet::from([channel_id]));
    }

    // must fail since already subscribed to the channel
    channel.add_subscriber_fail(SUBSCRIBERS[0]);

    // unsubscribe
    channel.unsubscribe(SUBSCRIBERS[1]);
    // check that subscriptions of SUBSCRIBERS[1] are empty
    router.check_user_subscriptions(SUBSCRIBERS[1], BTreeSet::new());

    // must fail since a sender does not subscribe to channel
    channel.unsubscribe_fail(SUBSCRIBERS[1]);
}

#[test]
fn post() {
    let sys = System::new();
    sys.init_logger();

    // upload and init a router program
    Program::router(&sys);

    // upload and init a channel
    let channel = Program::channel(&sys);

    let mut expected_messages: Vec<Message> = Vec::new();
    // init message
    let mut message = Message {
        owner: OWNER.into(),
        text: String::from("Channel \"Channel-Coolest-Name\" was created"),
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

    channel.post(OWNER, SUBSCRIBERS, String::from("Hello"), message);

    let messages: Vec<Message> = channel.meta_state(()).expect("Meta_state failed");
    assert_eq!(expected_messages, messages);

    // must fail since not owner posted a message
    channel.post_fail(SUBSCRIBERS[0], String::from("Hello"));
}
