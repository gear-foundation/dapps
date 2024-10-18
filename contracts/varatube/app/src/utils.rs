use crate::Storage;
use core::fmt::Debug;
use gstd::{ext, format};
use sails_rs::prelude::*;

pub type TokenData = (ActorId, Price);
pub type Price = u128;

#[derive(Debug, Default, Encode, Clone, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct Config {
    pub gas_for_token_transfer: u64,
    pub gas_to_start_subscription_update: u64,
    pub block_duration: u32,
    pub min_gas_limit: u64,
}

#[derive(Debug, Clone, Copy, Default, Encode, Decode, TypeInfo, PartialEq, Eq)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct SubscriberData {
    /// Id of the payment method.
    pub currency_id: ActorId,
    /// Subscription period
    pub period: Period,
    /// If `None`, means that subscriber has paid for the
    /// subscription, but didn't succeed sending delayed
    /// message for subscription check/renewal.
    pub subscription_start: Option<(u64, u32)>,
    // TODO [optimization] this must be calculated off-chain
    /// Subscription renewal date.
    ///
    /// If None, then no renewal desired.
    pub renewal_date: Option<(u64, u32)>,
}

/// Set of time periods for which a subscription can be purchased
/// in context of the sucbscription contract.
#[derive(Debug, Clone, Copy, Default, Encode, Decode, TypeInfo, PartialEq, Eq)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum Period {
    Year,
    NineMonths,
    SixMonths,
    ThreeMonths,
    #[default]
    Month,
}

impl Period {
    const SECOND: u32 = 1;

    pub fn minimal_unit() -> Self {
        Self::Month
    }

    pub fn as_units(&self) -> u128 {
        match self {
            Period::Year => 12,
            Period::NineMonths => 9,
            Period::SixMonths => 6,
            Period::ThreeMonths => 3,
            Period::Month => 1,
        }
    }

    pub fn to_blocks(&self, target_block_time: u32) -> u32 {
        self.as_secs().div_ceil(target_block_time)
    }

    pub fn as_millis(&self) -> u64 {
        self.as_secs() as u64 * 1000
    }

    fn as_secs(&self) -> u32 {
        match self {
            Period::Year => Self::Month.as_secs() * 12,
            Period::NineMonths => Self::Month.as_secs() * 9,
            Period::SixMonths => Self::Month.as_secs() * 6,
            Period::ThreeMonths => Self::Month.as_secs() * 3,
            Period::Month => Self::SECOND * 30 * 24 * 60 * 60,
        }
    }
}

/// The contract's error replies in case of unsuccessful message execution.
#[derive(Debug, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum VaraTubeError {
    AccountAlreadyRegistered,
    ErrorInSendingMsgToTransferTokens,
    ErrorInReceivingReplyFromToken,
    ErrorDuringSendingDelayedMsg,
    AccountDoesNotExist,
    WrongMsgSource,
    UnregisteredPaymentMethod,
    SubscriptionIsNotPending,
    NotAdmin,
}

impl Storage {
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
    pub fn get_subscriber(&self, subscriber: &ActorId) -> Result<SubscriberData, VaraTubeError> {
        self.subscribers
            .get(subscriber)
            .copied()
            .ok_or(VaraTubeError::AccountDoesNotExist)
    }

    /// Remove subscriber.
    pub fn delete_subscriber(&mut self, subscriber: &ActorId) {
        self.subscribers.remove(subscriber);
    }

    /// Get price of subscription when paid by `currency_id`.
    pub fn get_price(&self, currency_id: &ActorId) -> Result<Price, VaraTubeError> {
        if let Some(price) = self.currencies.get(currency_id) {
            Ok(*price)
        } else {
            Err(VaraTubeError::UnregisteredPaymentMethod)
        }
    }

    pub fn check_if_subscriber_doesnt_exist(
        &self,
        subscriber: &ActorId,
    ) -> Result<(), VaraTubeError> {
        if self.subscribers.contains_key(subscriber) {
            return Err(VaraTubeError::AccountAlreadyRegistered);
        }
        Ok(())
    }

    pub fn check_if_admin(&self, account: &ActorId) -> Result<(), VaraTubeError> {
        if !self.admins.contains(account) {
            return Err(VaraTubeError::NotAdmin);
        };
        Ok(())
    }
}

pub fn panicking<T, E: Debug, F: FnOnce() -> Result<T, E>>(f: F) -> T {
    match f() {
        Ok(v) => v,
        Err(e) => panic(e),
    }
}

pub fn panic(err: impl Debug) -> ! {
    ext::panic(&format!("{err:?}"))
}

#[derive(Debug, Clone, Default, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct SubscriberDataState {
    pub is_active: bool,
    pub start_date: u64,
    pub start_block: u32,
    pub end_date: u64,
    pub end_block: u32,
    pub period: Period,
    pub will_renew: bool,
    pub price: u128,
}
