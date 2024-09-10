use super::{error::Error, Event};
use sails_rs::prelude::*;

#[macro_export]
macro_rules! event_or_panic_async {
    ($self:expr, $expr:expr) => {{
        let result: Result<Event, Error> = $expr().await;
        match result {
            Ok(value) => {
                if let Err(e) = $self.notify_on(value) {
                    panic!("Error in depositing events: {:?}", e);
                }
            }
            Err(e) => {
                panic!("Message processing failed with error: {:?}", e);
            }
        }
    }};
}
