#![no_std]

use gstd::{collections::BTreeMap, ActorId};
use varatube_io::*;

#[gmeta::metawasm]
pub mod metafns {
    pub type State = SubscriptionState;

    pub fn all_subscriptions(state: State) -> BTreeMap<ActorId, SubscriberDataState> {
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
                        end_block: start_block + period.to_blocks(),
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
