use gstd::{collections::BTreeMap, msg, prelude::*, ActorId};
use nft_marketplace_io::*;
use non_fungible_token_io::{NFTAction, NFTEvent};
use primitive_types::U256;

pub type Payout = BTreeMap<ActorId, u128>;

pub async fn nft_transfer(
    transaction_id: TransactionId,
    nft_program_id: &ActorId,
    to: &ActorId,
    token_id: U256,
) -> Result<(), ()> {
    msg::send_for_reply_as::<NFTAction, NFTEvent>(
        *nft_program_id,
        NFTAction::Transfer {
            transaction_id,
            to: *to,
            token_id,
        },
        0,
        0,
    )
    .expect("Error in sending a message `NFTAction::Transfer`")
    .await
    .map(|_| ())
    .map_err(|_| ())
}

pub async fn payouts(nft_program_id: &ActorId, owner: &ActorId, amount: u128) -> Payout {
    let reply: NFTEvent = msg::send_for_reply_as(
        *nft_program_id,
        NFTAction::NFTPayout {
            owner: *owner,
            amount,
        },
        0,
        0,
    )
    .expect("Error in sending a message `NFTAction::NFTPayout`")
    .await
    .expect("Unable to decode `NFTEvent`");

    match reply {
        NFTEvent::NFTPayout(payout) => payout,
        _ => panic!("Wrong received reply"),
    }
}

pub async fn get_owner(nft_contract_id: &ContractId, token_id: TokenId) -> ActorId {
    let reply: NFTEvent =
        msg::send_for_reply_as(*nft_contract_id, NFTAction::Owner { token_id }, 0, 0)
            .expect("Error in sending a message `NFTAction::Owner`")
            .await
            .expect("Unable to decode `NFTEvent`");

    match reply {
        NFTEvent::Owner { owner, token_id: _ } => owner,
        _ => panic!("Wrong received message"),
    }
}
