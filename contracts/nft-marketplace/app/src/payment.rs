use sails_rs::gstd::msg;
use sails_rs::prelude::*;
use extended_vft_client::vft::io as vft;

pub async fn transfer_tokens(
    ft_contract_id: &ActorId,
    sender: &ActorId,
    recipient: &ActorId,
    value: U256,
) {
    let request = vft::TransferFrom::encode_call(*sender, *recipient, value);
    msg::send_bytes_with_gas_for_reply(*ft_contract_id, request, 5_000_000_000, 0, 0)
        .expect("Error in sending message to nft contract: `TransferFrom`")
        .await
        .expect("Error in receiving message to nft contract: `TransferFrom`");
}
