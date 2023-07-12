use dex_pair_io::*;
use ft_main_io::{FTokenAction, FTokenEvent, LogicAction};
use gear_lib::tx_manager::Stepper;
use gstd::{
    errors::Result,
    msg::{self, CodecMessageFuture},
    prelude::*,
    ActorId,
};

pub fn send<T: Decode>(to: ActorId, payload: impl Encode) -> Result<CodecMessageFuture<T>> {
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

    match send(token, payload)?.await? {
        FTokenEvent::Ok => Ok(()),
        FTokenEvent::Err => Err(Error::TransferFailed),
        _ => unreachable!("received an unexpected `FTokenEvent` variant"),
    }
}

pub async fn balance_of(token: ActorId, actor: ActorId) -> Result<u128> {
    if let FTokenEvent::Balance(balance) = send(token, FTokenAction::GetBalance(actor))?.await? {
        Ok(balance)
    } else {
        unreachable!("received an unexpected `FTokenEvent` variant");
    }
}
