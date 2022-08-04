use ft_io::{FTAction, FTEvent};
use gear_lib::non_fungible_token::{
    io::NFTTransfer,
    token::{TokenId, TokenMetadata},
};
use gstd::{msg, prelude::*, ActorId};
use nft_io::NFTAction;
use nft_pixelboard_io::*;

pub fn reply(nft_pixelboard_event: NFTPixelboardEvent) {
    msg::reply(nft_pixelboard_event, 0).expect("Error during replying with `NFTPixelboardEvent`");
}

pub async fn transfer_ftokens(ft_program: ActorId, from: ActorId, to: ActorId, amount: u128) {
    msg::send_for_reply_as::<_, FTEvent>(ft_program, FTAction::Transfer { from, to, amount }, 0)
        .expect("Error during sending `FTAction::Transfer` to a FT program")
        .await
        .expect("Unable to decode `FTEvent`");
}

pub async fn transfer_nft(nft_program: ActorId, to: ActorId, token_id: TokenId) {
    msg::send_for_reply_as::<_, NFTTransfer>(nft_program, NFTAction::Transfer { to, token_id }, 0)
        .expect("Error during sending `NFTAction::Transfer` to an NFT program")
        .await
        .expect("Unable to decode `NFTTransfer`");
}

pub async fn mint_nft(nft_program: ActorId, token_metadata: TokenMetadata) -> TokenId {
    let raw_reply: Vec<u8> =
        msg::send_for_reply_as(nft_program, NFTAction::Mint { token_metadata }, 0)
            .expect("Error during sending `NFTAction::Mint` to an NFT program")
            .await
            .expect("Unable to decode `Vec<u8>`");
    NFTTransfer::decode(&mut &raw_reply[..])
        .expect("Unable to decode `NFTTransfer`")
        .token_id
}
