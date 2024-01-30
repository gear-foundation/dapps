#![no_std]

use gstd::{exec, msg, prelude::*};
use w3bstreaming_io::{Action, ActionResult, Contract, Profile, Role, State, Stream, Subscription};

static mut CONTRACT: Option<Contract> = None;

#[no_mangle]
extern fn init() {
    let contract = Contract {
        ..Default::default()
    };
    unsafe { CONTRACT = Some(contract) };
}

#[no_mangle]
extern fn handle() {
    let input: Action = msg::load().expect("Unable to load message");
    let contract = unsafe { CONTRACT.as_mut().expect("The contract is not initialized") };

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
            if let Some(profile) = contract.users.get_mut(&msg_src) {
                profile.stream_ids.push(stream_id.clone());
            } else {
                panic!("Account is no registered");
            }
            contract.streams.insert(
                stream_id.clone(),
                Stream {
                    broadcaster: msg_src,
                    img_link,
                    start_time,
                    end_time,
                    title,
                    description,
                    watchers: Vec::new(),
                },
            );
            msg::reply(ActionResult::StreamIsScheduled { id: stream_id }, 0)
                .expect("Unable to send reply");
        }
        Action::DeleteStream { stream_id } => {
            let msg_src = msg::source();
            if let Some(profile) = contract.users.get_mut(&msg_src) {
                if let Some(index) = profile.stream_ids.iter().position(|x| *x == stream_id) {
                    profile.stream_ids.remove(index);
                } else {
                    panic!("Id is not exist");
                }
            } else {
                panic!("Account is no registered");
            }

            if let Some(stream) = contract.streams.get(&stream_id) {
                if stream.broadcaster == msg_src {
                    contract.streams.remove(&stream_id);
                } else {
                    panic!("You are not broadcaster");
                }
            } else {
                panic!("Id is not exist");
            }

            msg::reply(ActionResult::StreamDeleted { id: stream_id }, 0)
                .expect("Unable to send reply");
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

            if let Some(stream) = contract.streams.get_mut(&stream_id) {
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

            msg::reply(ActionResult::StreamEdited, 0).expect("Unable to send reply");
        }
        Action::Subscribe { account_id } => {
            if contract.users.get(&account_id).is_none() {
                panic!("The user is not found");
            }

            let msg_src = msg::source();

            if contract.users.get(&msg_src).is_none() {
                panic!("You are not registered");
            }

            contract
                .users
                .entry(account_id)
                .and_modify(|profile| profile.subscribers.push(msg_src));

            contract.users.entry(msg_src).and_modify(|profile| {
                profile.subscriptions.push(Subscription {
                    account_id,
                    sub_date: exec::block_timestamp(),
                    next_write_off: None,
                })
            });

            msg::reply(ActionResult::Subscribed, 0).expect("Unable to send reply");
        }
        Action::EditProfile {
            name,
            surname,
            img_link,
        } => {
            contract
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
                    role: Role::Speaker,
                });

            msg::reply(ActionResult::ProfileEdited, 0).expect("Unable to send reply");
        }
    };
}

#[no_mangle]
extern fn state() {
    let contract = unsafe { CONTRACT.take().expect("Unexpected error in taking state") };
    msg::reply::<State>(contract.into(), 0).expect("Failed to share state");
}
