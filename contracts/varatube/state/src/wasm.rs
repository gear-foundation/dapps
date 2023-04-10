use gear_subscription_io::{SubscriberDataState, SubscriptionMetadata};
use gmeta::{metawasm, BTreeMap, Metadata};
use gstd::{ActorId, ToString};

#[metawasm]
pub trait Metawasm {
    type State = <SubscriptionMetadata as Metadata>::State;

    fn all_subscriptions(state: Self::State) -> BTreeMap<ActorId, SubscriberDataState> {
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
                        end_date: start_date + period.to_millis(),
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
