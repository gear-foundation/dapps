use gstd::{msg, prelude::*, ActorId};
use primitive_types::U256;
pub type Payout = BTreeMap<ActorId, u128>;
use nft_io::*;

pub async fn nft_transfer(
    nft_program_id: &ActorId,
    to: &ActorId,
    token_id: U256,
    amount: u128,
) -> Payout {
    let response: NFTEvent = msg::send_for_reply_as(
        *nft_program_id,
        NFTAction::TransferPayout {
            to: *to,
            token_id,
            amount,
        },
        0,
    )
    .unwrap()
    .await
    .expect("error in transfer");
    let decoded_response = if let NFTEvent::TransferPayout(transfer_payout) = response {
        transfer_payout
    } else {
        unreachable!("Error in decoding payouts");
    };
    decoded_response.payouts
}

pub async fn nft_approve(nft_program_id: &ActorId, to: &ActorId, token_id: U256) {
    msg::send_for_reply(*nft_program_id, NFTAction::Approve { to: *to, token_id }, 0)
        .unwrap()
        .await
        .expect("error in transfer");
}
