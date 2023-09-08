use fungible_token_io::{FTAction, FTEvent};
use gstd::{msg, ActorId};

#[allow(unused)]
pub async fn transfer_from_tokens(token_id: &ActorId, from: &ActorId, to: &ActorId, amount: u128) {
    let _transfer_response: FTEvent = msg::send_for_reply_as(
        *token_id,
        FTAction::Transfer {
            from: *from,
            to: *to,
            amount,
        },
        0,
        0,
    )
    .unwrap()
    .await
    .expect("Error in transfer");
}

pub async fn transfer_tokens(token_id: &ActorId, from: &ActorId, to: &ActorId, amount: u128) {
    let _transfer_response: FTEvent = msg::send_for_reply_as(
        *token_id,
        FTAction::Transfer {
            from: *from,
            to: *to,
            amount,
        },
        0,
        0,
    )
    .unwrap()
    .await
    .expect("Error in transfer");
}

#[allow(unused)]
pub async fn approve_tokens(token_id: &ActorId, to: &ActorId, amount: u128) {
    let _approve_response: FTEvent =
        msg::send_for_reply_as(*token_id, FTAction::Approve { to: *to, amount }, 0, 0)
            .unwrap()
            .await
            .expect("Error in approve tokens");
}

pub async fn balance(token_id: &ActorId, account: &ActorId) -> u128 {
    let balance_response = msg::send_for_reply_as(*token_id, FTAction::BalanceOf(*account), 0, 0)
        .unwrap()
        .await
        .expect("Error in balance");

    if let FTEvent::Balance(balance_response) = balance_response {
        balance_response
    } else {
        0
    }
}
