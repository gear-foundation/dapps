use ft_io::*;
use gstd::{exec, msg, ActorId};

pub async fn transfer_tokens(contract_id: &ActorId, from: &ActorId, to: &ActorId, amount: u128) {
    let _transfer_response: FTEvent = msg::send_and_wait_for_reply(
        *contract_id,
        FTAction::Transfer {
            from: *from,
            to: *to,
            amount,
        },
        0,
    )
    .unwrap()
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
        if to != &exec::program_id() {
            msg::send(*to, "", price).unwrap();
        }
    } else {
        transfer_tokens(&ft_contract_id.unwrap(), from, to, price).await;
    }
}

pub fn check_attached_value(ft_contract_id: Option<ActorId>, price: u128) {
    if ft_contract_id.is_none() && msg::value() != price {
        panic!("attached value is not equal the indicated price");
    }
}
