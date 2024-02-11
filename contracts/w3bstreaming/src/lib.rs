#![no_std]

use gstd::{exec, msg, prelude::*};
use w3bstreaming_io::{Action, Event, Profile, Program, State, Stream, Subscription};

static mut PROGRAM: Option<Program> = None;

#[no_mangle]
extern fn init() {
    let program = Program {
        ..Default::default()
    };
    unsafe { PROGRAM = Some(program) };
}

#[no_mangle]
extern fn handle() {
    let input: Action = msg::load().expect("Unable to load message");
    let program = unsafe { PROGRAM.as_mut().expect("The program is not initialized") };

    match input {
        Action::NewStream {
            title,
            description,
            start_time,
            end_time,
            img_link,
        } => {
            let stream_id = exec::block_timestamp().to_string() + &title;
            let msg_src = msg::source();
            if let Some(profile) = program.users.get_mut(&msg_src) {
                profile.stream_ids.push(stream_id.clone());
            } else {
                panic!("Account is no registered");
            }
            program.streams.insert(
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
            msg::reply(Event::StreamIsScheduled { id: stream_id }, 0)
                .expect("Unable to send reply");
        }
        Action::DeleteStream { stream_id } => {
            let msg_src = msg::source();
            let profile = program
                .users
                .get_mut(&msg_src)
                .expect("Account is no registered");
            let index = profile
                .stream_ids
                .iter()
                .position(|x| *x == stream_id)
                .expect("Id is not exist");
            profile.stream_ids.remove(index);

            let stream = program.streams.get(&stream_id).expect("Id is not exist");
            if stream.broadcaster == msg_src {
                program.streams.remove(&stream_id);
            } else {
                panic!("You are not broadcaster");
            }

            msg::reply(Event::StreamDeleted { id: stream_id }, 0).expect("Unable to send reply");
        }
        Action::EditStream {
            stream_id,
            start_time,
            end_time,
            title,
            img_link,
            description,
        } => {
            let msg_src = msg::source();

            if let Some(stream) = program.streams.get_mut(&stream_id) {
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

            msg::reply(Event::StreamEdited, 0).expect("Unable to send reply");
        }
        Action::Subscribe { account_id } => {
            if !program.users.contains_key(&account_id) {
                panic!("The user is not found");
            }

            let msg_src = msg::source();

            if !program.users.contains_key(&msg_src) {
                panic!("You are not registered");
            }

            program
                .users
                .entry(account_id)
                .and_modify(|profile| profile.subscribers.push(msg_src));

            program.users.entry(msg_src).and_modify(|profile| {
                profile.subscriptions.push(Subscription {
                    account_id,
                    sub_date: exec::block_timestamp(),
                })
            });

            msg::reply(Event::Subscribed, 0).expect("Unable to send reply");
        }
        Action::EditProfile {
            name,
            surname,
            img_link,
        } => {
            program
                .users
                .entry(msg::source())
                .and_modify(|profile| {
                    profile.name = name.clone();
                    profile.surname = surname.clone();
                    profile.img_link = img_link.clone();
                })
                .or_insert_with(|| Profile {
                    name,
                    surname,
                    img_link,
                    stream_ids: Vec::new(),
                    subscribers: Vec::new(),
                    subscriptions: Vec::new(),
                });

            msg::reply(Event::ProfileEdited, 0).expect("Unable to send reply");
        }
    };
}

#[no_mangle]
extern fn state() {
    let program = unsafe { PROGRAM.take().expect("Unexpected error in taking state") };
    msg::reply::<State>(program.into(), 0).expect("Failed to share state");
}
