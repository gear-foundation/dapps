use crate::{Event, Period, Price, Storage, SubscriberData, VaraTubeError};
use gstd::{exec, msg, ActorId};
use sails_rs::prelude::*;

pub fn add_token_data(
    storage: &mut Storage,
    token_id: ActorId,
    price: Price,
) -> Result<Event, VaraTubeError> {
    storage.check_if_admin(&msg::source())?;
    storage.currencies.insert(token_id, price);
    Ok(Event::PaymentAdded)
}

pub async fn register_subscription(
    storage: &mut Storage,
    period: Period,
    currency_id: ActorId,
    with_renewal: bool,
) -> Result<Event, VaraTubeError> {
    let price = storage.get_price(&currency_id)?;
    let subscriber = msg::source();
    // Check subscription requirements

    storage.check_if_subscriber_doesnt_exist(&subscriber)?;

    let program_id = exec::program_id();
    // Withdraw subscription fee.
    transfer_tokens(
        &currency_id,
        &subscriber,
        &program_id,
        (price * period.as_units()).into(),
        storage.config.gas_for_token_transfer,
    )
    .await;

    // Send delayed message for state invalidation:
    // - subscription renewal
    // - subscription deletion
    match send_delayed_subscription_renewal(
        &program_id,
        &subscriber,
        period.to_blocks(storage.config.block_duration),
        Some(storage.config.gas_to_start_subscription_update),
    ) {
        Ok(_) => {
            let start_date = exec::block_timestamp();
            let start_block = exec::block_height();
            let renewal_date = if with_renewal {
                Some((
                    start_date + period.as_millis(),
                    start_block + period.to_blocks(storage.config.block_duration),
                ))
            } else {
                None
            };
            storage.add_subscriber(
                &subscriber,
                SubscriberData {
                    currency_id,
                    period,
                    renewal_date,
                    subscription_start: Some((start_date, start_block)),
                },
            );
        }
        Err(_) => {
            storage.add_pending_subscriber(&subscriber, (currency_id, period));
        }
    };

    Ok(Event::SubscriptionRegistered)
}

pub async fn update_subscription(
    storage: &mut Storage,
    subscriber: ActorId,
) -> Result<Event, VaraTubeError> {
    let program_id = exec::program_id();

    // This message is only intended to be send from this program
    if msg::source() != program_id {
        return Err(VaraTubeError::WrongMsgSource);
    };

    let SubscriberData {
        currency_id,
        period,
        renewal_date,
        ..
    } = storage.get_subscriber(&subscriber)?;

    let current_block = exec::block_height();
    let current_date = exec::block_timestamp();

    // If user want to renew his subscription...
    if renewal_date.is_some() {
        let price = storage.get_price(&currency_id)?;

        transfer_tokens(
            &currency_id,
            &subscriber,
            &program_id,
            (price * period.as_units()).into(),
            storage.config.gas_for_token_transfer,
        )
        .await;

        // Send delayed message for state invalidation:
        // - subscription renewal
        // - subscription deletion
        match send_delayed_subscription_renewal(
            &program_id,
            &subscriber,
            period.to_blocks(storage.config.block_duration),
            None,
        ) {
            Ok(_) => {
                // It's necessary to check if there is enough gas for the next auto-subscription.
                // If not, then the `renewal_date` should be set to None
                let renewal_date = if exec::gas_available() > storage.config.min_gas_limit {
                    Some((
                        current_date + period.as_millis(),
                        current_block + period.to_blocks(storage.config.block_duration),
                    ))
                } else {
                    None
                };
                storage.add_subscriber(
                    &subscriber,
                    SubscriberData {
                        currency_id,
                        period,
                        subscription_start: Some((current_date, current_block)),
                        renewal_date,
                    },
                );
            }
            Err(_) => {
                // Delayed message sending is needed for storage invalidation and update.
                // If this "sanity" message sending was failed, then we consider subscriber
                // as pending, so he can enable/withdraw his subscription receiving back
                // money.
                storage.add_pending_subscriber(&subscriber, (currency_id, period));
            }
        }
    } else {
        storage.delete_subscriber(&subscriber);
    }
    Ok(Event::SubscriptionUpdated)
}

pub fn cancel_subscription(storage: &mut Storage) -> Result<Event, VaraTubeError> {
    let subscriber = msg::source();
    let mut subscription_data = storage.get_subscriber(&subscriber)?;
    subscription_data.renewal_date = None;
    storage.add_subscriber(&subscriber, subscription_data);
    Ok(Event::SubscriptionCancelled)
}

pub async fn manage_pending_subscription(
    storage: &mut Storage,
    enable: bool,
) -> Result<Event, VaraTubeError> {
    let subscriber = msg::source();
    let this_program = exec::program_id();

    let SubscriberData {
        subscription_start,
        period,
        currency_id,
        ..
    } = storage.get_subscriber(&subscriber)?;

    if subscription_start.is_some() {
        return Err(VaraTubeError::SubscriptionIsNotPending);
    }

    if enable {
        send_delayed_subscription_renewal(
            &this_program,
            &subscriber,
            period.to_blocks(storage.config.block_duration),
            Some(storage.config.gas_to_start_subscription_update),
        )?;

        let current_date = exec::block_timestamp();
        let current_block = exec::block_height();
        storage.add_subscriber(
            &subscriber,
            SubscriberData {
                currency_id,
                period,
                subscription_start: Some((current_date, current_block)),
                renewal_date: Some((
                    current_date + period.as_millis(),
                    current_block + period.to_blocks(storage.config.block_duration),
                )),
            },
        );
    } else {
        let price = storage.get_price(&currency_id)?;
        transfer_tokens(
            &currency_id,
            &this_program,
            &subscriber,
            (price * period.as_units()).into(),
            storage.config.gas_for_token_transfer,
        )
        .await;

        storage.delete_subscriber(&subscriber);
    };
    Ok(Event::PendingSubscriptionManaged)
}

pub fn update_config(
    storage: &mut Storage,
    gas_for_token_transfer: Option<u64>,
    gas_to_start_subscription_update: Option<u64>,
    block_duration: Option<u32>,
) -> Result<Event, VaraTubeError> {
    storage.check_if_admin(&msg::source())?;

    if let Some(gas_for_token_transfer) = gas_for_token_transfer {
        storage.config.gas_for_token_transfer = gas_for_token_transfer;
    }

    if let Some(gas_to_start_subscription_update) = gas_to_start_subscription_update {
        storage.config.gas_to_start_subscription_update = gas_to_start_subscription_update;
    }

    if let Some(block_duration) = block_duration {
        storage.config.block_duration = block_duration;
    }

    Ok(Event::ConfigUpdated)
}

async fn transfer_tokens(
    ft_address: &ActorId,
    from: &ActorId,
    to: &ActorId,
    value: U256,
    gas_limit: u64,
) {
    let request = [
        "Vft".encode(),
        "TransferFrom".to_string().encode(),
        (*from, *to, value).encode(),
    ]
    .concat();

    msg::send_bytes_with_gas_for_reply(*ft_address, request, gas_limit, 0, 0)
        .expect("Error in sending a message")
        .await
        .expect("Error in transfer Fungible Token");
}

fn send_delayed_subscription_renewal(
    program_id: &ActorId,
    subscriber: &ActorId,
    delay: u32,
    gas_limit: Option<u64>,
) -> Result<(), VaraTubeError> {
    let request = [
        "Varatube".encode(),
        "UpdateSubscription".to_string().encode(),
        (*subscriber).encode(),
    ]
    .concat();

    if let Some(gas_limit) = gas_limit {
        msg::send_bytes_with_gas_delayed(*program_id, request, gas_limit, 0, delay)
            .map_err(|_| VaraTubeError::ErrorDuringSendingDelayedMsg)?;
    } else if msg::send_bytes_delayed(*program_id, request, 0, delay).is_err() {
        return Err(VaraTubeError::ErrorDuringSendingDelayedMsg);
    }

    Ok(())
}
