use crate::transfer_tokens;
use crate::{ActorId, ContractId, Item};
use sails_rs::gstd::msg;

pub async fn buy_item_with_value(item: &mut Item, new_owner: &ActorId) {
    let price = item.price.expect("Can't be None");
    if msg::value() != price {
        panic!("Wrong price");
    }

    // send value
    msg::send_with_gas(item.owner, "", 0, price).expect("Error in sending value");

    item.owner = *new_owner;
    item.price = None;
}

pub async fn buy_item_with_fungible_tokens(
    item: &mut Item,
    ft_contract_id: &ContractId,
    new_owner: &ActorId,
) {
    let price = item.price.expect("Can't be None");

    // transfer FT to the owner
    transfer_tokens(ft_contract_id, new_owner, &item.owner, price.into()).await;

    item.owner = *new_owner;
    item.price = None;
}
