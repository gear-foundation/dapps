use gstd::{msg, prelude::*, ActorId};
use sharded_fungible_token_io::{FTokenAction, FTokenEvent, LogicAction};

pub async fn transfer_tokens(
    transaction_id: u64,
    token_id: &ActorId,
    sender: &ActorId,
    recipient: &ActorId,
    amount: u128,
) -> Result<(), ()> {
    let reply = msg::send_for_reply_as::<_, FTokenEvent>(
        *token_id,
        FTokenAction::Message {
            transaction_id,
            payload: LogicAction::Transfer {
                sender: *sender,
                recipient: *recipient,
                amount,
            },
        },
        0,
        0,
    )
    .expect("Error in sending a message `FTokenAction::Message`")
    .await;

    match reply {
        Ok(FTokenEvent::Ok) => Ok(()),
        _ => Err(()),
    }
}
