use dex_io::*;
use gear_lib::tx_manager::Stepper;
use gstd::{
    errors::CoreError,
    msg::{self, CodecMessageFuture},
    prelude::*,
    ActorId,
};
use sharded_fungible_token_io::{FTokenAction, FTokenEvent, LogicAction};

pub fn send<T: Decode>(
    to: ActorId,
    payload: impl Encode,
) -> Result<CodecMessageFuture<T>, CoreError> {
    msg::send_for_reply_as(to, payload, 0, 0)
}

pub async fn transfer_tokens(
    stepper: &mut Stepper,
    token: ActorId,
    sender: ActorId,
    recipient: ActorId,
    amount: u128,
) -> Result<(), Error> {
    let payload = FTokenAction::Message {
        transaction_id: stepper.step()?,
        payload: LogicAction::Transfer {
            sender,
            recipient,
            amount,
        },
    };

    match send(token, payload).unwrap().await.unwrap() {
        FTokenEvent::Ok => Ok(()),
        FTokenEvent::Err => Err(Error::TransferFailed),
        _ => unreachable!("received an unexpected `FTokenEvent` variant"),
    }
}

pub async fn balance_of(token: ActorId, actor: ActorId) -> Result<u128, CoreError> {
    if let FTokenEvent::Balance(balance) = send(token, FTokenAction::GetBalance(actor))
        .unwrap()
        .await
        .unwrap()
    {
        Ok(balance)
    } else {
        unreachable!("received an unexpected `FTokenEvent` variant");
    }
}
