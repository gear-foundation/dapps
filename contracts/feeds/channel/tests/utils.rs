use channel_io::*;
use gstd::{ActorId, BTreeSet};
use gtest::{Log, Program, System};
use router_io::*;

pub const CHANNEL_ID: u64 = 2;
pub const ROUTER_ID: u64 = 1;
pub const OWNER: u64 = 100;
pub const SUBSCRIBERS: &[u64] = &[10, 11, 12, 13, 14];

pub trait FeedsChannel {
    fn router(sys: &System) -> Program;
    fn channel(sys: &System) -> Program;
    fn add_subscriber(&self, subscriber: u64);
    fn add_subscriber_fail(&self, subscriber: u64);
    fn unsubscribe(&self, subscriber: u64);
    fn unsubscribe_fail(&self, subscriber: u64);
    fn post(&self, owner: u64, subscribers: &[u64], text: String, message: Message);
    fn post_fail(&self, owner: u64, text: String);
    fn check_user_subscriptions(&self, user: u64, expected_subscriptions: BTreeSet<ActorId>);
    fn check_channel_info(&self, channel: Channel);
    fn check_all_channel(&self, expected_channels: Vec<Channel>);
}

impl FeedsChannel for Program<'_> {
    fn router(sys: &System) -> Program {
        let router = Program::from_file(
            sys,
            "../target/wasm32-unknown-unknown/release/gear_feeds_router.wasm",
        );

        let res = router.send_bytes(OWNER, "INIT");

        assert!(res.log().is_empty());

        router
    }

    fn channel(sys: &System) -> Program {
        let channel = Program::current(sys);

        let res = channel.send(
            OWNER,
            ChannelInit {
                router_contract_id: ROUTER_ID.into(),
            },
        );
        let log = Log::builder()
            .dest(OWNER)
            .payload(ChannelOutput::SubscriberAdded(OWNER.into()));
        assert!(res.contains(&log));

        channel
    }

    fn add_subscriber(&self, subscriber: u64) {
        let res = self.send(subscriber, ChannelAction::Subscribe);
        let log = Log::builder()
            .dest(subscriber)
            .payload(ChannelOutput::SubscriberAdded(subscriber.into()));
        assert!(res.contains(&log));
    }

    fn add_subscriber_fail(&self, subscriber: u64) {
        assert!(self
            .send(subscriber, ChannelAction::Subscribe)
            .main_failed());
    }

    fn unsubscribe(&self, subscriber: u64) {
        let res = self.send(subscriber, ChannelAction::Unsubscribe);
        let log = Log::builder()
            .dest(subscriber)
            .payload(ChannelOutput::SubscriberRemoved(subscriber.into()));
        assert!(res.contains(&log));
    }

    fn unsubscribe_fail(&self, subscriber: u64) {
        assert!(self
            .send(subscriber, ChannelAction::Unsubscribe)
            .main_failed());
    }

    fn post(&self, owner: u64, subscribers: &[u64], text: String, message: Message) {
        let res = self.send(owner, ChannelAction::Post(text));
        let log = Log::builder()
            .dest(OWNER)
            .payload(ChannelOutput::MessagePosted(message.clone()));
        assert!(res.contains(&log));
        let log = Log::builder()
            .dest(OWNER)
            .payload(ChannelOutput::SingleMessage(message.clone()));
        assert!(res.contains(&log));
        for subscriber in subscribers {
            let log = Log::builder()
                .dest(*subscriber)
                .payload(ChannelOutput::SingleMessage(message.clone()));
            assert!(res.contains(&log));
        }
    }
    fn post_fail(&self, owner: u64, text: String) {
        assert!(self.send(owner, ChannelAction::Post(text)).main_failed())
    }
    fn check_user_subscriptions(&self, user: u64, expected_subscriptions: BTreeSet<ActorId>) {
        let subscriptions: RouterStateReply = self
            .meta_state(RouterState::SubscribedToChannels(user.into()))
            .expect("Meta_state failed");

        assert_eq!(
            subscriptions,
            RouterStateReply::SubscribedToChannels(expected_subscriptions),
        );
    }

    fn check_channel_info(&self, channel: Channel) {
        let channel_info: RouterStateReply = self
            .meta_state(&RouterState::Channel(channel.id))
            .expect("Meta_state failed");
        assert_eq!(channel_info, RouterStateReply::Channel(channel));
    }

    fn check_all_channel(&self, expected_channels: Vec<Channel>) {
        let channels: RouterStateReply = self
            .meta_state(&RouterState::AllChannels)
            .expect("Meta_state failed");
        assert_eq!(channels, RouterStateReply::AllChannels(expected_channels));
    }
}
