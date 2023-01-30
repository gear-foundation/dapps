use gear_lib::non_fungible_token::token::TokenId;
use gmeta::{metawasm, Metadata};
use gstd::{prelude::*, ActorId};
use nft_pixelboard_io::*;

#[metawasm]
pub trait Metawasm {
    type State = <ContractMetadata as Metadata>::State;

    fn painting(state: Self::State) -> Vec<Color> {
        state.painting
    }

    fn resolution(state: Self::State) -> Resolution {
        state.resolution
    }

    fn pixel_price(state: Self::State) -> u128 {
        state.pixel_price
    }

    fn block_side_length(state: Self::State) -> BlockSideLength {
        state.block_side_length
    }

    fn commission_percentage(state: Self::State) -> u8 {
        state.commission_percentage
    }

    fn pixel_info(coordinates: Coordinates, state: Self::State) -> Token {
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

    fn token_info(token_id: TokenId, state: Self::State) -> Token {
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

    fn ft_program(state: Self::State) -> ActorId {
        state.ft_program
    }

    fn nft_program(state: Self::State) -> ActorId {
        state.nft_program
    }
}
