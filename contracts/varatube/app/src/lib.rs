#![no_std]
#![allow(clippy::new_without_default)]
#![allow(static_mut_refs)]
use gstd::{exec, msg};
use sails_rs::{collections::HashMap, prelude::*};
pub mod funcs;
pub mod utils;
use utils::*;

#[derive(Default)]
pub struct Storage {
    admins: Vec<ActorId>,
    currencies: HashMap<ActorId, Price>,
    subscribers: HashMap<ActorId, SubscriberData>,
    config: Config,
    dns_info: Option<(ActorId, String)>,
}

static mut STORAGE: Option<Storage> = None;

#[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum Event {
    SubscriptionRegistered,
    SubscriptionUpdated,
    SubscriptionCancelled,
    PendingSubscriptionManaged,
    PaymentAdded,
    ConfigUpdated,
    Killed { inheritor: ActorId },
}

#[derive(Clone)]
pub struct Service(());

impl Service {
    pub async fn init(config: Config, dns_id_and_name: Option<(ActorId, String)>) -> Self {
        unsafe {
            STORAGE = Some(Storage {
                admins: vec![msg::source()],
                config,
                ..Default::default()
            });
        }
        if let Some((id, name)) = dns_id_and_name {
            let request = [
                "Dns".encode(),
                "AddNewProgram".to_string().encode(),
                (name, exec::program_id()).encode(),
            ]
            .concat();

            msg::send_bytes_with_gas_for_reply(id, request, 5_000_000_000, 0, 0)
                .expect("Error in sending message")
                .await
                .expect("Error in `AddNewProgram`");
        }
        Self(())
    }
    pub fn get_mut(&mut self) -> &'static mut Storage {
        unsafe { STORAGE.as_mut().expect("Storage is not initialized") }
    }
    pub fn get(&self) -> &'static Storage {
        unsafe { STORAGE.as_ref().expect("Storage is not initialized") }
    }
}

#[service(events = Event)]
impl Service {
    pub fn new() -> Self {
        Self(())
    }
    pub fn add_token_data(&mut self, token_id: ActorId, price: Price) {
        let storage = self.get_mut();
        let event = panicking(|| funcs::add_token_data(storage, token_id, price));
        self.emit_event(event.clone()).expect("Notification Error");
    }
    pub async fn register_subscription(
        &mut self,
        period: Period,
        currency_id: ActorId,
        with_renewal: bool,
    ) {
        let storage = self.get_mut();
        let res = funcs::register_subscription(storage, period, currency_id, with_renewal).await;
        let event = match res {
            Ok(v) => v,
            Err(e) => panic(e),
        };

        self.emit_event(event.clone()).expect("Notification Error");
    }

    pub async fn update_subscription(&mut self, subscriber: ActorId) {
        let storage = self.get_mut();
        let res = funcs::update_subscription(storage, subscriber).await;
        let event = match res {
            Ok(v) => v,
            Err(e) => panic(e),
        };

        self.emit_event(event.clone()).expect("Notification Error");
    }

    pub fn cancel_subscription(&mut self) {
        let storage = self.get_mut();
        let event = panicking(|| funcs::cancel_subscription(storage));
        self.emit_event(event.clone()).expect("Notification Error");
    }

    pub async fn manage_pending_subscription(&mut self, enable: bool) {
        let storage = self.get_mut();
        let res = funcs::manage_pending_subscription(storage, enable).await;
        let event = match res {
            Ok(v) => v,
            Err(e) => panic(e),
        };

        self.emit_event(event.clone()).expect("Notification Error");
    }

    pub fn update_config(
        &mut self,
        gas_for_token_transfer: Option<u64>,
        gas_to_start_subscription_update: Option<u64>,
        block_duration: Option<u32>,
    ) {
        let storage = self.get_mut();
        let event = panicking(|| {
            funcs::update_config(
                storage,
                gas_for_token_transfer,
                gas_to_start_subscription_update,
                block_duration,
            )
        });
        self.emit_event(event.clone()).expect("Notification Error");
    }

    pub async fn kill(&mut self, inheritor: ActorId) {
        let storage = self.get();
        if let Some((id, _name)) = &storage.dns_info {
            let request = ["Dns".encode(), "DeleteMe".to_string().encode(), ().encode()].concat();

            msg::send_bytes_with_gas_for_reply(*id, request, 5_000_000_000, 0, 0)
                .expect("Error in sending message")
                .await
                .expect("Error in `AddNewProgram`");
        }

        if !storage.admins.contains(&msg::source()) {
            panic(VaraTubeError::NotAdmin);
        }

        self.emit_event(Event::Killed { inheritor })
            .expect("Notification Error");
        exec::exit(inheritor);
    }

    pub fn admins(&self) -> &'static Vec<ActorId> {
        &self.get().admins
    }

    pub fn config(&self) -> &'static Config {
        &self.get().config
    }

    pub fn currencies(&self) -> Vec<(ActorId, Price)> {
        self.get().currencies.clone().into_iter().collect()
    }

    pub fn subscribers(&self) -> Vec<(ActorId, SubscriberData)> {
        self.get().subscribers.clone().into_iter().collect()
    }

    pub fn get_subscriber(&self, account: ActorId) -> Option<SubscriberData> {
        self.get().subscribers.get(&account).cloned()
    }
    pub fn all_subscriptions(&self) -> Vec<(ActorId, SubscriberDataState)> {
        let state = self.get();
        state
            .subscribers
            .iter()
            .filter_map(|(k, v)| {
                if let Some((start_date, start_block)) = v.subscription_start {
                    let period = v.period;
                    let will_renew = v.renewal_date.is_some();

                    let ret_data = SubscriberDataState {
                        is_active: true,
                        start_date,
                        start_block,
                        end_date: start_date + period.as_millis(),
                        end_block: start_block + period.to_blocks(state.config.block_duration),
                        period,
                        will_renew,
                        price: state.currencies.get(&v.currency_id).copied().unwrap(),
                    };

                    Some((*k, ret_data))
                } else {
                    None
                }
            })
            .collect()
    }
}

pub struct VaratubeProgram(());

#[program]
impl VaratubeProgram {
    pub async fn new(config: Config, dns_id_and_name: Option<(ActorId, String)>) -> Self {
        Service::init(config, dns_id_and_name).await;
        Self(())
    }
    pub fn varatube(&self) -> Service {
        Service(())
    }
}
