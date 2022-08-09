use ft_io::*;
use gstd::{msg, ActorId};

pub async fn transfer_tokens(token_id: &ActorId, from: &ActorId, to: &ActorId, amount: u128) {
    let _transfer_response = msg::send_for_reply(
        *token_id,
        FTAction::Transfer {
            from: *from,
            to: *to,
            amount,
        },
        0,
    )
    .expect("Error in message")
    .await
    .expect("Error in transfer");
}
