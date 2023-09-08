use gstd::{msg, ActorId};
use sharded_fungible_token_io::{FTokenAction, FTokenEvent, LogicAction};

/// Transfers `amount` tokens from `sender` account to `recipient` account.
/// Arguments:
/// * `transaction_id`: associated transaction id
/// * `from`: sender account
/// * `to`: recipient account
/// * `amount`: amount of tokens
pub async fn transfer_tokens(
    transaction_id: u64,
    token_address: &ActorId,
    from: &ActorId,
    to: &ActorId,
    amount_tokens: u128,
) -> Result<(), ()> {
    let reply = msg::send_for_reply_as::<FTokenAction, FTokenEvent>(
        *token_address,
        FTokenAction::Message {
            transaction_id,
            payload: LogicAction::Transfer {
                sender: *from,
                recipient: *to,
                amount: amount_tokens,
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
