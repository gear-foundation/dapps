use sails_rs::gstd::msg;
use sails_rs::prelude::*;

pub async fn transfer_tokens(
    ft_contract_id: &ActorId,
    sender: &ActorId,
    recipient: &ActorId,
    value: U256,
) {
    let request = [
        "Vft".encode(),
        "TransferFrom".to_string().encode(),
        (*sender, *recipient, value).encode(),
    ]
    .concat();


    msg::send_bytes_with_gas_for_reply(*ft_contract_id, request, 5_000_000_000, 0, 0)
        .expect("Error in sending message to nft contract: `TransferFrom`")
        .await
        .expect("Error in receiving message to nft contract: `TransferFrom`");
}
