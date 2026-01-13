use extended_vft_client::vft::io as vft_io;
use sails_rs::gstd::msg;
use sails_rs::prelude::*;

pub async fn transfer_tokens(
    ft_contract_id: &ActorId,
    sender: &ActorId,
    recipient: &ActorId,
    value: U256,
) {
    let request =
        vft_io::TransferFrom::encode_params_with_prefix("Vft", *sender, *recipient, value);
    msg::send_bytes_with_gas_for_reply(*ft_contract_id, request, 5_000_000_000, 0, 5_000_000_000)
        .expect("Error in sending message to ft contract: `TransferFrom`")
        .await
        .expect("Error in receiving message to ft contract: `TransferFrom`");
}
