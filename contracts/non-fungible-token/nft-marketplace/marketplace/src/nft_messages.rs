use gstd::{msg, prelude::*, ActorId};
use primitive_types::U256;
pub type Payout = BTreeMap<ActorId, u128>;
use nft_io::*;

pub async fn nft_transfer(nft_program_id: &ActorId, to: &ActorId, token_id: U256) {
    let _transfer_response: NFTEvent = msg::send_and_wait_for_reply(
        *nft_program_id,
        NFTAction::Transfer { to: *to, token_id },
        0,
    )
    .unwrap()
    .await
    .expect("error in transfer");
}

pub async fn nft_approve(nft_program_id: &ActorId, to: &ActorId, token_id: U256) {
    let _approve_response: NFTEvent =
        msg::send_and_wait_for_reply(*nft_program_id, NFTAction::Approve { to: *to, token_id }, 0)
            .unwrap()
            .await
            .expect("error in transfer");
}

pub async fn nft_payouts(nft_program_id: &ActorId, owner: &ActorId, amount: u128) -> Payout {
    let payouts: NFTEvent = msg::send_and_wait_for_reply(
        *nft_program_id,
        NFTAction::NFTPayout {
            owner: *owner,
            amount,
        },
        0,
    )
    .unwrap()
    .await
    .expect("Error in function 'nft_payout' call");
    match payouts {
        NFTEvent::NFTPayout(payouts) => payouts,
        _ => BTreeMap::new(),
    }
}
