#![no_std]

pub use w3bstreaming_io::*;

use gstd::{exec, msg, prelude::*};
use w3bstreaming_io::{Action, ActionResult, Contract, Profile, Role, Stream, Subscription};

static mut CONTRACT: Option<Contract> = None;

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
            if let Some(profile) = contract.users.get_mut(&msg::source()) {
                profile.stream_ids.push(stream_id.clone());
            } else {
                panic!("Account is no registered");
            }
            contract.streams.insert(
                stream_id.clone(),
                Stream {
                    broadcaster: msg::source(),
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
        Action::Subscribe { account_id } => {
            if contract.users.get(&account_id).is_none() {
                panic!("The user is not found");
            }

            if contract.users.get(&msg::source()).is_none() {
                panic!("You are not registered");
            }

            contract
                .users
                .entry(account_id)
                .and_modify(|profile| profile.subscribers.push(msg::source()));

            contract.users.entry(msg::source()).and_modify(|profile| {
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
    msg::reply::<Contract>(
        unsafe {
            CONTRACT
                .as_mut()
                .expect("The contract is not initialized")
                .clone()
        },
        0,
    )
    .expect("`state()` failed");
}
