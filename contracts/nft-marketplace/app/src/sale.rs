use crate::{nft_transfer, transfer_tokens};
use crate::{ActorId, ContractId, Item, TokenId};
use sails_rs::gstd::msg;

pub async fn buy_item_with_value(
    item: &mut Item,
    nft_contract_id: &ContractId,
    old_owner: &ActorId,
    new_owner: &ActorId,
    token_id: TokenId,
) {
    let price = item.price.expect("Can't be None");
    if msg::value() != price {
        panic!("Wrong price");
    }

    // transfer NFT
    nft_transfer(nft_contract_id, old_owner, new_owner, token_id).await;
    // send value
    msg::send_with_gas(item.owner, "", 0, price).expect("Error in sending value");

    item.owner = *new_owner;
    item.price = None;
}

pub async fn buy_item_with_fungible_tokens(
    item: &mut Item,
    nft_contract_id: &ContractId,
    ft_contract_id: &ContractId,
    old_owner: &ActorId,
    new_owner: &ActorId,
    token_id: TokenId,
) {
    let price = item.price.expect("Can't be None");

    // transfer FT to the owner
    transfer_tokens(ft_contract_id, new_owner, &item.owner, price.into()).await;
    // transfer NFT to the buyer
    nft_transfer(nft_contract_id, old_owner, new_owner, token_id).await;

    item.owner = *new_owner;
    item.price = None;
}
