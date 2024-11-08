#![no_std]

use sails_rs::{
    collections::HashMap,
    gstd::{exec, msg},
    prelude::*,
};
mod utils;
use utils::*;

static mut PROGRAM: Option<Program> = None;

#[derive(Default, Clone, Debug)]

pub struct Program {
    pub streams: HashMap<String, Stream>,
    pub users: HashMap<ActorId, Profile>,
}

struct W3bstreamingService(());

impl W3bstreamingService {
    pub fn new() -> Self {
        Self(())
    }
    pub fn get_mut(&mut self) -> &'static mut Program {
        unsafe { PROGRAM.as_mut().expect("Ping counter is not initialized") }
    }
    pub fn get(&self) -> &'static Program {
        unsafe { PROGRAM.as_ref().expect("Ping counter is not initialized") }
    }
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum Event {
    StreamIsScheduled { id: String },
    StreamDeleted { id: String },
    StreamEdited,
    Subscribed,
    ProfileEdited,
}

#[sails_rs::service(events = Event)]
impl W3bstreamingService {
    fn init() -> Self {
        unsafe {
            PROGRAM = Some(Program {
                ..Default::default()
            });
        }
        Self(())
    }

    pub fn new_stream(
        &mut self,
        title: String,
        description: Option<String>,
        start_time: u64,
        end_time: u64,
        img_link: String,
    ) {
        let stream_id = exec::block_timestamp().to_string() + &title;
        let msg_src = msg::source();
        let storage = self.get_mut();
        if let Some(profile) = storage.users.get_mut(&msg_src) {
            profile.stream_ids.push(stream_id.clone());
        } else {
            panic!("Account is no registered");
        }
        storage.streams.insert(
            stream_id.clone(),
            Stream {
                broadcaster: msg_src,
                img_link,
                start_time,
                end_time,
                title,
                description,
            },
        );
        self.notify_on(Event::StreamIsScheduled { id: stream_id })
            .expect("Notification Error");
    }

    pub fn delete_stream(&mut self, stream_id: String) {
        let storage = self.get_mut();
        let msg_src = msg::source();
        let profile = storage
            .users
            .get_mut(&msg_src)
            .expect("Account is no registered");
        let index = profile
            .stream_ids
            .iter()
            .position(|x| *x == stream_id)
            .expect("Id is not exist");
        profile.stream_ids.remove(index);

        let stream = storage.streams.get(&stream_id).expect("Id is not exist");
        if stream.broadcaster == msg_src {
            storage.streams.remove(&stream_id);
        } else {
            panic!("You are not broadcaster");
        }
        self.notify_on(Event::StreamDeleted { id: stream_id })
            .expect("Notification Error");
    }
    pub fn edit_stream(
        &mut self,
        stream_id: String,
        start_time: Option<u64>,
        end_time: Option<u64>,
        title: Option<String>,
        img_link: Option<String>,
        description: Option<String>,
    ) {
        let storage = self.get_mut();
        let msg_src = msg::source();

        if let Some(stream) = storage.streams.get_mut(&stream_id) {
            if stream.broadcaster == msg_src {
                if let Some(start_time) = start_time {
                    stream.start_time = start_time;
                }
                if let Some(end_time) = end_time {
                    stream.end_time = end_time;
                }
                if let Some(title) = title {
                    stream.title = title;
                }
                if let Some(img_link) = img_link {
                    stream.img_link = img_link;
                }
                stream.description = description;
            } else {
                panic!("You are not broadcaster");
            }
        } else {
            panic!("Id is not exist");
        }

        self.notify_on(Event::StreamEdited)
            .expect("Notification Error");
    }

    pub fn subscribe(&mut self, account_id: ActorId) {
        let storage = self.get_mut();
        if !storage.users.contains_key(&account_id) {
            panic!("The user is not found");
        }

        let msg_src = msg::source();

        if !storage.users.contains_key(&msg_src) {
            panic!("You are not registered");
        }

        storage
            .users
            .entry(account_id)
            .and_modify(|profile| profile.subscribers.push(msg_src));

        storage.users.entry(msg_src).and_modify(|profile| {
            profile.subscriptions.push(Subscription {
                account_id,
                sub_date: exec::block_timestamp(),
            })
        });

        self.notify_on(Event::Subscribed)
            .expect("Notification Error");
    }

    pub fn edit_profile(
        &mut self,
        name: Option<String>,
        surname: Option<String>,
        img_link: Option<String>,
    ) {
        let storage = self.get_mut();

        storage
            .users
            .entry(msg::source())
            .and_modify(|profile| {
                profile.name.clone_from(&name);
                profile.surname.clone_from(&surname);
                profile.img_link.clone_from(&img_link);
            })
            .or_insert_with(|| Profile {
                name,
                surname,
                img_link,
                stream_ids: Vec::new(),
                subscribers: Vec::new(),
                subscriptions: Vec::new(),
            });

        self.notify_on(Event::ProfileEdited)
            .expect("Notification Error");
    }
    // Service's query
    pub fn get_state(&self) -> ProgramState {
        self.get().clone().into()
    }
}

pub struct W3bstreamingProgram(());

#[allow(clippy::new_without_default)]
#[sails_rs::program]
impl W3bstreamingProgram {
    // Program's constructor
    pub fn new() -> Self {
        W3bstreamingService::init();
        Self(())
    }

    // Exposed service
    pub fn w3bstreaming(&self) -> W3bstreamingService {
        W3bstreamingService::new()
    }
}
