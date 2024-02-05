use fungible_token_io::FTEvent;

use crate::{msg, Actions, ActorId, Error, Period, Price, SubscriberData, VaraTube};

impl VaraTube {
    /// Add subscriber.
    pub fn add_subscriber(&mut self, subscriber: &ActorId, data: SubscriberData) {
        self.subscribers.insert(*subscriber, data);
    }

    /// Add pending subscription.
    ///
    /// Inserting `data` is actually currency id and subscription period.
    pub fn add_pending_subscriber(
        &mut self,
        subscriber: &ActorId,
        (currency_id, period): (ActorId, Period),
    ) {
        self.subscribers.insert(
            *subscriber,
            SubscriberData {
                currency_id,
                period,
                subscription_start: None,
                renewal_date: None,
            },
        );
    }

    /// Ger subscriber.
    pub fn get_subscriber(&self, subscriber: &ActorId) -> Result<SubscriberData, Error> {
        self.subscribers
            .get(subscriber)
            .copied()
            .ok_or(Error::AccountDoesNotExist)
    }

    /// Remove subscriber.
    pub fn delete_subscriber(&mut self, subscriber: &ActorId) {
        self.subscribers.remove(subscriber);
    }

    /// Get price of subscription when paid by `currency_id`.
    pub fn get_price(&self, currency_id: &ActorId) -> Result<Price, Error> {
        if let Some(price) = self.currencies.get(currency_id) {
            Ok(*price)
        } else {
            Err(Error::UnregisteredPaymentMethod)
        }
    }

    pub fn check_if_subscriber_doesnt_exist(&self, subscriber: &ActorId) -> Result<(), Error> {
        if self.subscribers.contains_key(subscriber) {
            return Err(Error::AccountAlreadyRegistered);
        }
        Ok(())
    }

    pub fn check_if_admin(&self, account: &ActorId) -> Result<(), Error> {
        if !self.admins.contains(account) {
            return Err(Error::NotAdmin);
        };
        Ok(())
    }
}

pub fn check_msg_source(msg_source: ActorId, expected_account: ActorId) -> Result<(), Error> {
    if msg_source != expected_account {
        return Err(Error::WrongMsgSource);
    };
    Ok(())
}

pub async fn transfer_tokens(
    token_id: &ActorId,
    from: &ActorId,
    to: &ActorId,
    amount: u128,
    gas_limit: u64,
) -> Result<(), Error> {
    match msg::send_with_gas_for_reply_as::<_, FTEvent>(
        *token_id,
        FTEvent::Transfer {
            from: *from,
            to: *to,
            amount,
        },
        gas_limit,
        0,
        0,
    ) {
        Ok(msg_future) => match msg_future.await {
            Ok(_) => Ok(()),
            Err(_) => Err(Error::ErrorInReceivingReplyFromToken),
        },
        Err(_) => Err(Error::ErrorInSendingMsgToTransferTokens),
    }
}

pub fn send_delayed_subscription_renewal(
    program_id: &ActorId,
    subsciber: &ActorId,
    delay: u32,
    gas_limit: Option<u64>,
) -> Result<(), Error> {
    if let Some(gas_limit) = gas_limit {
        msg::send_with_gas_delayed(
            *program_id,
            Actions::UpdateSubscription {
                subscriber: *subsciber,
            },
            gas_limit,
            0,
            delay,
        )
        .map_err(|_| Error::ErrorDuringSendingDelayedMsg)?;
    } else if msg::send_delayed(
        *program_id,
        Actions::UpdateSubscription {
            subscriber: *subsciber,
        },
        0,
        delay,
    )
    .is_err()
    {
        return Err(Error::ErrorDuringSendingDelayedMsg);
    }

    Ok(())
}
