#![no_std]

use gstd::{async_main, collections::BTreeMap, exec, msg, prelude::*, ActorId};
use varatube_io::*;

pub mod utils;
use utils::*;

#[derive(Default)]
struct VaraTube {
    admins: Vec<ActorId>,
    currencies: BTreeMap<ActorId, Price>,
    subscribers: BTreeMap<ActorId, SubscriberData>,
    config: Config,
}

static mut VARATUBE: Option<VaraTube> = None;

impl VaraTube {
    /// Add a new mean of payment.
    pub fn add_token_data(&mut self, token_id: ActorId, price: Price) -> Result<Reply, Error> {
        self.check_if_admin(&msg::source())?;
        self.currencies.insert(token_id, price);
        Ok(Reply::PaymentAdded)
    }
    async fn register_subscription(
        &mut self,
        period: Period,
        currency_id: &ActorId,
        with_renewal: bool,
    ) -> Result<Reply, Error> {
        let price = self.get_price(currency_id)?;
        let subscriber = msg::source();
        // Check subscription requirements

        self.check_if_subscriber_doesnt_exist(&subscriber)?;

        let program_id = exec::program_id();
        // Withdraw subscription fee.
        transfer_tokens(
            currency_id,
            &subscriber,
            &program_id,
            price * period.as_units(),
            self.config.gas_for_token_transfer,
        )
        .await?;

        // Send delayed message for state invalidation:
        // - subscription renewal
        // - subscription deletion
        match send_delayed_subscription_renewal(
            &program_id,
            &subscriber,
            period.to_blocks(self.config.block_duration),
            self.config.gas_for_delayed_msg,
        ) {
            Ok(_) => {
                let start_date = exec::block_timestamp();
                let start_block = exec::block_height();
                let renewal_date = if with_renewal {
                    Some((
                        start_date + period.as_millis(),
                        start_block + period.to_blocks(self.config.block_duration),
                    ))
                } else {
                    None
                };
                self.add_subscriber(
                    &subscriber,
                    SubscriberData {
                        currency_id: *currency_id,
                        period,
                        renewal_date,
                        subscription_start: Some((start_date, start_block)),
                    },
                );
            }
            Err(_) => {
                self.add_pending_subscriber(&subscriber, (*currency_id, period));
            }
        };

        Ok(Reply::SubscriptionRegistered)
    }

    async fn update_subscription(&mut self, subscriber: &ActorId) -> Result<Reply, Error> {
        let this_program = exec::program_id();

        // This message is only intended to be send from this program
        check_msg_source(msg::source(), this_program)?;

        let SubscriberData {
            currency_id,
            period,
            renewal_date,
            ..
        } = self.get_subscriber(&subscriber)?;

        let current_block = exec::block_height();
        let current_date = exec::block_timestamp();

        // If user want to renew his subscription...
        if renewal_date.is_some() {
            let price = self.get_price(&currency_id)?;

            transfer_tokens(
                &currency_id,
                subscriber,
                &this_program,
                price * period.as_units(),
                self.config.gas_for_token_transfer,
            )
            .await?;

            // Send delayed message for state invalidation:
            // - subscription renewal
            // - subscription deletion
            match send_delayed_subscription_renewal(
                &this_program,
                subscriber,
                period.to_blocks(self.config.block_duration),
                self.config.gas_for_delayed_msg,
            ) {
                Ok(_) => {
                    self.add_subscriber(
                        subscriber,
                        SubscriberData {
                            currency_id,
                            period,
                            subscription_start: Some((current_date, current_block)),
                            renewal_date: Some((
                                current_date + period.as_millis(),
                                current_block + period.to_blocks(self.config.block_duration),
                            )),
                        },
                    );
                }
                Err(_) => {
                    // Delayed message sending is needed for storage invalidation and update.
                    // If this "sanity" message sending was failed, then we consider subscriber
                    // as pending, so he can enable/withdraw his subscription receiving back
                    // money.
                    self.add_pending_subscriber(subscriber, (currency_id, period));
                }
            }
        } else {
            self.delete_subscriber(subscriber);
        }
        Ok(Reply::SubscriptionUpdated)
    }

    fn cancel_subscription(&mut self) -> Result<Reply, Error> {
        let subscriber = msg::source();
        let mut subscription_data = self.get_subscriber(&subscriber)?;

        subscription_data.renewal_date = None;

        self.add_subscriber(&subscriber, subscription_data);

        Ok(Reply::SubscriptionCancelled)
    }

    async fn manage_pending_subscription(&mut self, enable: bool) -> Result<Reply, Error> {
        let subscriber = msg::source();
        let this_program = exec::program_id();

        let SubscriberData {
            subscription_start,
            period,
            currency_id,
            ..
        } = self.get_subscriber(&subscriber)?;

        if subscription_start.is_some() {
            return Err(Error::SubscriptionIsNotPending);
        }

        if enable {
            send_delayed_subscription_renewal(
                &this_program,
                &subscriber,
                period.to_blocks(self.config.block_duration),
                self.config.gas_for_delayed_msg,
            )?;

            let current_date = exec::block_timestamp();
            let current_block = exec::block_height();
            self.add_subscriber(
                &subscriber,
                SubscriberData {
                    currency_id,
                    period,
                    subscription_start: Some((current_date, current_block)),
                    renewal_date: Some((
                        current_date + period.as_millis(),
                        current_block + period.to_blocks(self.config.block_duration),
                    )),
                },
            );
        } else {
            let price = self.get_price(&currency_id)?;
            transfer_tokens(
                &currency_id,
                &this_program,
                &subscriber,
                price * period.as_units(),
                self.config.gas_for_token_transfer,
            )
            .await?;

            self.delete_subscriber(&subscriber);
        };
        Ok(Reply::PendingSubscriptionManaged)
    }

    fn update_config(
        &mut self,
        gas_for_token_transfer: Option<u64>,
        gas_for_delayed_msg: Option<u64>,
        block_duration: Option<u32>,
    ) -> Result<Reply, Error> {
        self.check_if_admin(&msg::source())?;

        if let Some(gas_for_token_transfer) = gas_for_token_transfer {
            self.config.gas_for_token_transfer = gas_for_token_transfer;
        }

        if let Some(gas_for_delayed_msg) = gas_for_delayed_msg {
            self.config.gas_for_delayed_msg = gas_for_delayed_msg;
        }

        if let Some(block_duration) = block_duration {
            self.config.block_duration = block_duration;
        }

        Ok(Reply::ConfigUpdated)
    }
}

#[no_mangle]
extern fn init() {
    let config: Config = msg::load().expect("Unable to decode the initial msg");
    unsafe {
        VARATUBE = Some(VaraTube {
            admins: vec![msg::source()],
            config,
            ..Default::default()
        })
    }
}

#[async_main]
async fn main() {
    let action: Actions = msg::load().expect("handle: wrong payload: expected `Actions`");
    let varatube = unsafe { VARATUBE.as_mut().expect("The contract is not initiazlied") };
    let reply = match action {
        Actions::RegisterSubscription {
            period,
            currency_id,
            with_renewal,
        } => {
            varatube
                .register_subscription(period, &currency_id, with_renewal)
                .await
        }
        Actions::UpdateSubscription { subscriber } => {
            varatube.update_subscription(&subscriber).await
        }
        Actions::CancelSubscription => varatube.cancel_subscription(),
        Actions::ManagePendingSubscription { enable } => {
            varatube.manage_pending_subscription(enable).await
        }
        Actions::AddTokenData { token_id, price } => varatube.add_token_data(token_id, price),
        Actions::UpdateConfig {
            gas_for_token_transfer,
            gas_for_delayed_msg,
            block_duration,
        } => varatube.update_config(gas_for_token_transfer, gas_for_delayed_msg, block_duration),
    };
    msg::reply(reply, 0).expect("Error during sending a reply");
}

#[no_mangle]
extern fn state() {
    let query: StateQuery = msg::load().expect("state: wrong payload: expected `StateQuery`");
    let varatube = unsafe { VARATUBE.take().expect("The contract is not initiazlied") };
    let reply = match query {
        StateQuery::Admins => {
            StateReply::Admins(varatube.admins)
        }
        StateQuery::Currencies => {
            StateReply::Currencies(varatube.currencies)
        }
        StateQuery::Subscribers => {
            StateReply::Subscribers(varatube.subscribers)
        }
        StateQuery::Config => {
            StateReply::Config(varatube.config)
        }
    };
    msg::reply(reply, 0).expect("Error in sharing state");
}
