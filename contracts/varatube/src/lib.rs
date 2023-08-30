#![no_std]

use fungible_token_io::{FTAction, FTEvent};
use gstd::{async_main, exec, msg, prelude::*, ActorId};
use varatube_io::{Actions, Period, Price, SubscriberData, SubscriptionState};

// TODO [cleanness] control tokens are of erc20 standard
// TODO [cleanness] error workflow done by eco-system guys
// TODO [cleanness + optimization] control errors between async calls
// TODO [release] add readme + docs
// TODO [cleanness + release] add tests (for ex: UpdateSubscription execution is the same as end date)

/// Supported means of payment
static mut CURRENCIES: BTreeMap<ActorId, Price> = BTreeMap::new();
/// Subscribers
static mut SUBSCRIBERS: BTreeMap<ActorId, SubscriberData> = BTreeMap::new();

/// Subscribers state manager
struct Subscribers;

/// Payment methods state.
struct Currencies;

impl Subscribers {
    /// Add subscriber.
    fn add_subscriber(subscriber: ActorId, data: SubscriberData) {
        unsafe {
            SUBSCRIBERS.insert(subscriber, data);
        }
    }

    /// Add pending subscription.
    ///
    /// Inserting `data` is actually currency id and subscription period.
    fn add_pending_subscriber(subscriber: ActorId, data: (ActorId, Period)) {
        let (currency_id, period) = data;
        unsafe {
            SUBSCRIBERS.insert(
                subscriber,
                SubscriberData {
                    currency_id,
                    period,
                    subscription_start: None,
                    renewal_date: None,
                },
            );
        }
    }

    /// Ger subscriber.
    fn get_subscriber(subscriber: &ActorId) -> Option<SubscriberData> {
        unsafe { SUBSCRIBERS.get(subscriber).copied() }
    }

    /// Remove subscriber.
    fn delete_subscriber(subscriber: &ActorId) {
        unsafe {
            SUBSCRIBERS.remove(subscriber);
        }
    }
}

impl Currencies {
    /// Add a new mean of payment.
    fn add_currency(currency: (ActorId, Price)) {
        let (id, price) = currency;
        unsafe {
            CURRENCIES.insert(id, price);
        }
    }

    /// Get price of subscription when paid by `currency_id`.
    fn get_price(currency_id: &ActorId) -> Option<Price> {
        unsafe { CURRENCIES.get(currency_id).copied() }
    }
}

#[no_mangle]
extern fn init() {
    Currencies::add_currency(msg::load().expect("init: wrong payload: expected token id"));
}

#[async_main]
async fn main() {
    match msg::load().expect("handle: wrong payload: expected `Actions`") {
        Actions::RegisterSubscription {
            period,
            currency_id,
            with_renewal,
        } => {
            let price = Currencies::get_price(&currency_id)
                .expect("RegisterSubscription: unregistered payment method");
            let subscriber = msg::source();
            // Check subscription requirements
            if Subscribers::get_subscriber(&subscriber).is_some() {
                panic!("RegisterSubscription: invalid subscription state");
            }
            // Withdraw subscription fee.
            let _: FTEvent = msg::send_for_reply_as(
                currency_id,
                FTAction::Transfer {
                    from: subscriber,
                    to: exec::program_id(),
                    amount: price * period.as_units(),
                },
                0,
                0,
            )
            .unwrap_or_else(|e| panic!("RegisterSubscription: error sending async message: {e:?}"))
            .await
            .unwrap_or_else(|e| {
                panic!("RegisterSubscription: token transfer ended up with an error {e:?}")
            });
            gstd::debug!("Before delayed msg");
            // Send delayed message for state invalidation:
            // - subscription renewal
            // - subscription deletion
            if msg::send_delayed(
                exec::program_id(),
                Actions::UpdateSubscription { subscriber },
                0,
                period.to_blocks(),
            )
            .is_ok()
            {
                gstd::debug!("Delayed ok");
                let start_date = exec::block_timestamp();
                let start_block = exec::block_height();
                let renewal_date = if with_renewal {
                    Some((
                        start_date + period.as_millis(),
                        start_block + period.to_blocks(),
                    ))
                } else {
                    None
                };
                Subscribers::add_subscriber(
                    subscriber,
                    SubscriberData {
                        currency_id,
                        period,
                        renewal_date,
                        subscription_start: Some((start_date, start_block)),
                    },
                );
            } else {
                // Delayed message sending is needed for storage invalidation and update.
                // If this "sanity" message sending was failed, then we consider subscriber
                // as pending, so he can enable/withdraw his subscription receiving back
                // money.
                gstd::debug!("Delayed NOT ok");
                Subscribers::add_pending_subscriber(subscriber, (currency_id, period));
            }
        }
        Actions::UpdateSubscription { subscriber } => {
            let this_program = exec::program_id();

            // This message is only intended to be send from this program
            if msg::source() != this_program {
                panic!("UpdateSubscription: message allowed only for this program");
            }

            let SubscriberData {
                currency_id,
                period,
                renewal_date,
                ..
            } = Subscribers::get_subscriber(&subscriber)
                .expect("UpdateSubscription: subscriber not found");

            let current_block = exec::block_height();
            let current_date = exec::block_timestamp();

            // If user want to renew his subscription...
            if renewal_date.is_some() {
                let price = Currencies::get_price(&currency_id)
                    .expect("UpdateSubscription: payment method was deleted");
                let _: FTEvent = msg::send_for_reply_as(
                    currency_id,
                    FTAction::Transfer {
                        from: subscriber,
                        to: this_program,
                        amount: price * period.as_units(),
                    },
                    0,
                    0,
                )
                .unwrap_or_else(|e| {
                    panic!("UpdateSubscription: error sending async message: {e:?}")
                })
                .await
                .unwrap_or_else(|e| {
                    Subscribers::delete_subscriber(&subscriber);
                    panic!("UpdateSubscription: transfer ended up with an error {e:?}")
                });

                // Send delayed message for state invalidation:
                // - subscription renewal
                // - subscription deletion
                if msg::send_delayed(
                    this_program,
                    Actions::UpdateSubscription { subscriber },
                    0,
                    period.to_blocks(),
                )
                .is_ok()
                {
                    Subscribers::add_subscriber(
                        subscriber,
                        SubscriberData {
                            currency_id,
                            period,
                            subscription_start: Some((current_date, current_block)),
                            renewal_date: Some((
                                current_date + period.as_millis(),
                                current_block + period.to_blocks(),
                            )),
                        },
                    );
                } else {
                    // Delayed message sending is needed for storage invalidation and update.
                    // If this "sanity" message sending was failed, then we consider subscriber
                    // as pending, so he can enable/withdraw his subscription receiving back
                    // money.
                    Subscribers::add_pending_subscriber(subscriber, (currency_id, period));
                }
            } else {
                Subscribers::delete_subscriber(&subscriber);
            }
        }
        Actions::CancelSubscription => {
            let subscriber = msg::source();
            let subscription = Subscribers::get_subscriber(&subscriber);
            if subscription.is_none() {
                panic!("CancelSubscription: subscription not found");
            }

            let updated_subscription = {
                let mut new_data = subscription.expect("checked");
                new_data.renewal_date = None;

                new_data
            };

            Subscribers::add_subscriber(subscriber, updated_subscription);
        }
        Actions::ManagePendingSubscription { enable } => {
            let subscriber = msg::source();
            let this_program = exec::program_id();

            if let Some(SubscriberData {
                subscription_start,
                period,
                currency_id,
                ..
            }) = Subscribers::get_subscriber(&subscriber)
            {
                if subscription_start.is_some() {
                    panic!("ManagePendingSubscription: subscription is not pending");
                }

                if enable {
                    msg::send_delayed(
                        this_program,
                        Actions::UpdateSubscription { subscriber },
                        0,
                        period.to_blocks(),
                    )
                    .unwrap_or_else(|e| {
                        panic!("ManagePendingSubscription: sending delayed message failed {e:?}")
                    });

                    let current_date = exec::block_timestamp();
                    let current_block = exec::block_height();
                    Subscribers::add_subscriber(
                        subscriber,
                        SubscriberData {
                            currency_id,
                            period,
                            subscription_start: Some((current_date, current_block)),
                            renewal_date: Some((
                                current_date + period.as_millis(),
                                current_block + period.to_blocks(),
                            )),
                        },
                    );
                } else {
                    let price = Currencies::get_price(&currency_id)
                        .expect("ManagePendingSubscription: payment method was deleted");
                    let _: FTEvent = msg::send_for_reply_as(
                        currency_id,
                        FTAction::Transfer {
                            from: this_program,
                            to: subscriber,
                            amount: price * period.as_units(),
                        },
                        0,
                        0,
                    )
                    .unwrap_or_else(|e| {
                        panic!("ManagePendingSubscription: error sending async message: {e:?}")
                    })
                    .await
                    .unwrap_or_else(|e| {
                        panic!("ManagePendingSubscription: transfer ended up with an error {e:?}")
                    });

                    Subscribers::delete_subscriber(&subscriber);
                }
            } else {
                panic!("ManagePendingSubscription: can't manage non existing subscription");
            }
        }
    }
}

#[no_mangle]
extern fn state() {
    let ret_state = unsafe { SUBSCRIBERS.clone() };
    let ret_state2 = unsafe { CURRENCIES.clone() };
    let _ = msg::reply::<SubscriptionState>((ret_state, ret_state2).into(), 0);
}
