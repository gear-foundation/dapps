use dex_factory_io::*;
use ft_io::*;
use gstd::{msg, ActorId};

/// Gets fee_to from a factory contract.
/// `factory_address` - MUST be a non-zero address
/// Arguments:
/// * `factory_address` - the address of factory which fee_to should be received
pub async fn get_fee_to(factory_address: &ActorId) -> ActorId {
    let fee_to_response: FactoryEvent =
        msg::send_for_reply_as(*factory_address, FactoryAction::FeeTo, 0)
            .unwrap()
            .await
            .expect("Error in get_fee_to");
    if let FactoryEvent::FeeTo(fee_to) = fee_to_response {
        return fee_to;
    }
    ActorId::zero()
}

/// Transfers token from the contract
/// `token_address` - MUST be a non-zero address
/// `from` - MUST be a non-zero address
/// `token_amount` - MUST be a non-zero number
/// Arguments:
/// * `token_address` - the address of FT contract
/// * `from` - tokens' sender
/// * `to` - tokens' receiver
/// * `token_amount` - the amount of tokens to be transferred
pub async fn transfer_tokens(
    token_address: &ActorId,
    from: &ActorId,
    to: &ActorId,
    token_amount: u128,
) {
    msg::send_for_reply_as::<_, FTEvent>(
        *token_address,
        FTAction::Transfer {
            from: *from,
            to: *to,
            amount: token_amount,
        },
        0,
    )
    .unwrap()
    .await
    .expect("Error in transfer");
}

pub async fn approve_tokens(token_address: &ActorId, to: &ActorId, token_amount: u128) {
    msg::send_for_reply_as::<_, FTEvent>(
        *token_address,
        FTAction::Approve {
            to: *to,
            amount: token_amount,
        },
        0,
    )
    .unwrap()
    .await
    .expect("Error in approve");
}
