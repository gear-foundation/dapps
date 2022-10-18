use ft_io::*;
use gstd::{exec, msg, ActorId};
const MINIMUM_VALUE: u64 = 500;
pub async fn transfer_tokens(contract_id: &ActorId, from: &ActorId, to: &ActorId, amount: u128) {
    msg::send_for_reply(
        *contract_id,
        FTAction::Transfer {
            from: *from,
            to: *to,
            amount,
        },
        0,
    )
    .expect("Error in sending message to FT contract")
    .await
    .expect("Error in transfer");
}

pub async fn transfer_payment(
    from: &ActorId,
    to: &ActorId,
    ft_contract_id: Option<ActorId>,
    price: u128,
) {
    if ft_contract_id.is_none() {
        if to != &exec::program_id() && price > MINIMUM_VALUE.into() {
            msg::send(*to, "", price).expect("Error in sending payment in value");
        }
    } else {
        transfer_tokens(
            &ft_contract_id.expect("There must no be an error here"),
            from,
            to,
            price,
        )
        .await;
    }
}

pub fn check_attached_value(ft_contract_id: Option<ActorId>, price: u128) {
    if ft_contract_id.is_none() && msg::value() != price {
        panic!("attached value is not equal the indicated price");
    }
}
