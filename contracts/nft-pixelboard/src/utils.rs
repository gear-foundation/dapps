use crate::contract::NFTPixelboard;
use ft_main_io::{FTokenAction, FTokenEvent, LogicAction};
use gear_lib::non_fungible_token::token::{TokenId, TokenMetadata};
use gstd::{msg, prelude::*, ActorId};
use nft_io::{NFTAction, NFTEvent};
use nft_pixelboard_io::*;

pub async fn transfer_ftokens(
    transaction_id: u64,
    ft_contract_id: &ActorId,
    sender: &ActorId,
    recipient: &ActorId,
    amount: u128,
) -> Result<(), NFTPixelboardError> {
    let reply = msg::send_for_reply_as::<_, FTokenEvent>(
        *ft_contract_id,
        FTokenAction::Message {
            transaction_id,
            payload: LogicAction::Transfer {
                sender: *sender,
                recipient: *recipient,
                amount,
            },
        },
        0,
    )
    .expect("Error in sending a message `FTokenAction::Message`")
    .await;
    match reply {
        Ok(FTokenEvent::Ok) => Ok(()),
        _ => Err(NFTPixelboardError::FTokensTransferFailed),
    }
}

pub async fn transfer_nft(
    transaction_id: TransactionId,
    nft_program: &ActorId,
    to: &ActorId,
    token_id: TokenId,
) -> Result<(), NFTPixelboardError> {
    let reply = msg::send_for_reply_as::<_, NFTEvent>(
        *nft_program,
        NFTAction::Transfer {
            transaction_id,
            to: *to,
            token_id,
        },
        0,
    )
    .expect("Error during sending `NFTAction::Transfer` to an NFT program")
    .await;
    match reply {
        Ok(NFTEvent::Transfer(_)) => Ok(()),
        _ => Err(NFTPixelboardError::NFTTransferFailed),
    }
}

pub async fn mint_nft(
    transaction_id: TransactionId,
    nft_program: &ActorId,
    token_metadata: TokenMetadata,
) -> Result<TokenId, NFTPixelboardError> {
    let reply = msg::send_for_reply_as::<_, NFTEvent>(
        *nft_program,
        NFTAction::Mint {
            transaction_id,
            token_metadata,
        },
        0,
    )
    .expect("Error during sending `NFTAction::Mint` to an NFT program")
    .await;
    match reply {
        Ok(NFTEvent::Transfer(transfer)) => Ok(transfer.token_id),
        _ => Err(NFTPixelboardError::NFTMintFailed),
    }
}

impl From<&NFTPixelboard> for NFTPixelboardState {
    fn from(state: &NFTPixelboard) -> NFTPixelboardState {
        NFTPixelboardState {
            owner: state.owner,
            block_side_length: state.block_side_length,
            pixel_price: state.pixel_price,
            resolution: state.resolution,
            commission_percentage: state.commission_percentage,
            painting: state.painting.clone(),
            rectangles_by_token_ids: state
                .rectangles_by_token_ids
                .iter()
                .map(|(key, value)| (*key, *value))
                .collect(),
            tokens_by_rectangles: state
                .tokens_by_rectangles
                .iter()
                .map(|(key, value)| (*key, *value))
                .collect(),
            ft_program: state.ft_program,
            nft_program: state.nft_program,
            txs: state
                .txs
                .iter()
                .map(|(key, value)| (*key, value.clone()))
                .collect(),
            tx_id: state.tx_id,
        }
    }
}
