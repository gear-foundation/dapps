use crate::{ContractId, TokenId};
use extended_vnft_client::vnft::io as vnft_io;
use sails_rs::gstd::msg;
use sails_rs::prelude::*;

pub async fn nft_transfer(
    nft_contract_id: &ContractId,
    from: &ActorId,
    to: &ActorId,
    token_id: U256,
) {
    let request = vnft_io::TransferFrom::encode_call(*from, *to, token_id);
    msg::send_bytes_with_gas_for_reply(*nft_contract_id, request, 5_000_000_000, 0, 0)
        .expect("Error in sending message to nft contract: `TransferFrom`")
        .await
        .expect("Error in receiving message to nft contract: `TransferFrom`");
}

pub async fn get_owner(nft_contract_id: &ContractId, token_id: TokenId) -> ActorId {
    let request = vnft_io::OwnerOf::encode_call(token_id);
    let (_, _, owner): (String, String, ActorId) =
        msg::send_bytes_with_gas_for_reply_as(*nft_contract_id, request, 5_000_000_000, 0, 0)
            .expect("Error in sending message to nft contract: `OwnerOf`")
            .await
            .expect("Error in receiving message to nft contract: `OwnerOf`");

    owner
}
