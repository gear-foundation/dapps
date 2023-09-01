#![no_std]

use gear_lib_old::non_fungible_token::token::TokenId;
use gstd::{prelude::*, ActorId};
use nft_pixelboard_io::*;

#[gmeta::metawasm]
pub mod metafns {
    pub type State = NFTPixelboardState;

    pub fn painting(state: State) -> Vec<Color> {
        state.painting
    }

    pub fn resolution(state: State) -> Resolution {
        state.resolution
    }

    pub fn pixel_price(state: State) -> u128 {
        state.pixel_price
    }

    pub fn block_side_length(state: State) -> BlockSideLength {
        state.block_side_length
    }

    pub fn commission_percentage(state: State) -> u8 {
        state.commission_percentage
    }

    pub fn pixel_info(state: State, coordinates: Coordinates) -> Token {
        let mut token = Default::default();

        if coordinates.x < state.resolution.width && coordinates.y < state.resolution.height {
            for (rectangle, token_info) in state.tokens_by_rectangles.iter() {
                if coordinates.x < rectangle.bottom_right_corner.x
                    && coordinates.y < rectangle.bottom_right_corner.y
                {
                    token = Token(*rectangle, *token_info)
                }
            }
        }

        token
    }

    pub fn token_info(state: State, token_id: TokenId) -> Token {
        let mut token = Default::default();

        if let Some((_, rectangle)) = state
            .rectangles_by_token_ids
            .iter()
            .find(|&(x, _)| x == &token_id)
        {
            if let Some((_, token_info)) = state
                .tokens_by_rectangles
                .iter()
                .find(|&(x, _)| x == rectangle)
            {
                token = Token(*rectangle, *token_info);
            }
        }
        token
    }

    pub fn ft_program(state: State) -> ActorId {
        state.ft_program
    }

    pub fn nft_program(state: State) -> ActorId {
        state.nft_program
    }
}
