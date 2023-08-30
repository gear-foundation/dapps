use super::common::{InitResult, Program, RunResult};
use super::{FOREIGN_USER, OWNER};
use gear_lib_old::non_fungible_token::token::TokenMetadata;
use gstd::ActorId;
use gtest::{Program as InnerProgram, System, EXISTENTIAL_DEPOSIT};
use nft_pixelboard_io::*;

type NFTPixelboardRunResult<T> = RunResult<T, NFTPixelboardEvent, NFTPixelboardError>;

pub struct NFTPixelboard<'a>(InnerProgram<'a>);

impl Program for NFTPixelboard<'_> {
    fn inner_program(&self) -> &InnerProgram {
        &self.0
    }
}

impl<'a> NFTPixelboard<'a> {
    pub fn initialize(system: &'a System, ft_program: ActorId, nft_program: ActorId) -> Self {
        Self::initialize_custom(
            system,
            InitNFTPixelboard {
                ft_program,
                block_side_length: 1,
                nft_program,
                owner: OWNER.into(),
                painting: vec![0; 100],
                pixel_price: MAX_PIXEL_PRICE,
                commission_percentage: 100,
                resolution: (10, 10).into(),
            },
        )
        .succeed()
    }

    pub fn initialize_custom(
        system: &'a System,
        config: InitNFTPixelboard,
    ) -> InitResult<NFTPixelboard<'a>, NFTPixelboardError> {
        let program = InnerProgram::current_opt(system);

        system.mint_to(program.id(), EXISTENTIAL_DEPOSIT);

        let result = program.send(FOREIGN_USER, config);
        let is_active = system.is_active_program(program.id());
        InitResult::new(Self(program), result, is_active)
    }

    pub fn mint(
        &self,
        from: u64,
        painting: Vec<Color>,
        rectangle: Rectangle,
    ) -> NFTPixelboardRunResult<u128> {
        self.mint_with_metadata(from, painting, rectangle, Default::default())
    }

    pub fn mint_with_metadata(
        &self,
        from: u64,
        painting: Vec<Color>,
        rectangle: Rectangle,
        token_metadata: TokenMetadata,
    ) -> NFTPixelboardRunResult<u128> {
        RunResult::new(
            self.0.send(
                from,
                NFTPixelboardAction::Mint {
                    painting,
                    rectangle,
                    token_metadata,
                },
            ),
            |value| NFTPixelboardEvent::Minted(value.into()),
        )
    }

    pub fn change_sale_state(
        &self,
        from: u64,
        token_id: u128,
        pixel_price: Option<u128>,
    ) -> NFTPixelboardRunResult<u128> {
        RunResult::new(
            self.0.send(
                from,
                NFTPixelboardAction::ChangeSaleState {
                    token_id: token_id.into(),
                    pixel_price,
                },
            ),
            |token_id| NFTPixelboardEvent::SaleStateChanged(token_id.into()),
        )
    }

    pub fn buy(&self, from: u64, token_id: u128) -> NFTPixelboardRunResult<u128> {
        RunResult::new(
            self.0.send(from, NFTPixelboardAction::Buy(token_id.into())),
            |token_id| NFTPixelboardEvent::Bought(token_id.into()),
        )
    }

    pub fn paint(
        &self,
        from: u64,
        token_id: u128,
        painting: Vec<Color>,
    ) -> NFTPixelboardRunResult<u128> {
        RunResult::new(
            self.0.send(
                from,
                NFTPixelboardAction::Paint {
                    token_id: token_id.into(),
                    painting,
                },
            ),
            |token_id| NFTPixelboardEvent::Painted(token_id.into()),
        )
    }
}

pub struct NFTPixelboardInit<'a>(InnerProgram<'a>, bool);

impl<'a> NFTPixelboardInit<'a> {
    #[track_caller]
    pub fn failed(self) {
        assert!(self.1)
    }

    #[track_caller]
    pub fn succeed(self) -> NFTPixelboard<'a> {
        assert!(!self.1);
        NFTPixelboard(self.0)
    }
}

// # TODO: uncomment when new meta will be ready for gtest

// pub fn meta_state(&self) -> NFTPixelboardMetaState {
//     NFTPixelboardMetaState(&self.0)
// }

// pub struct NFTPixelboardMetaState<'a>(&'a InnerProgram<'a>);

// impl NFTPixelboardMetaState<'_> {
//     pub fn ft_program(self) -> MetaStateReply<ActorId> {
//         if let NFTPixelboardStateReply::FTProgram(reply) = self
//             .0
//             .meta_state(NFTPixelboardStateQuery::FTProgram)
//             .unwrap()
//         {
//             MetaStateReply(reply)
//         } else {
//             unreachable!();
//         }
//     }

//     pub fn nft_program(self) -> MetaStateReply<ActorId> {
//         if let NFTPixelboardStateReply::NFTProgram(reply) = self
//             .0
//             .meta_state(NFTPixelboardStateQuery::NFTProgram)
//             .unwrap()
//         {
//             MetaStateReply(reply)
//         } else {
//             unreachable!();
//         }
//     }

//     pub fn block_side_length(self) -> MetaStateReply<BlockSideLength> {
//         if let NFTPixelboardStateReply::BlockSideLength(reply) = self
//             .0
//             .meta_state(NFTPixelboardStateQuery::BlockSideLength)
//             .unwrap()
//         {
//             MetaStateReply(reply)
//         } else {
//             unreachable!();
//         }
//     }

//     pub fn painting(self) -> MetaStateReply<Vec<Color>> {
//         if let NFTPixelboardStateReply::Painting(reply) = self
//             .0
//             .meta_state(NFTPixelboardStateQuery::Painting)
//             .unwrap()
//         {
//             MetaStateReply(reply)
//         } else {
//             unreachable!();
//         }
//     }

//     pub fn pixel_info(self, coordinates: Coordinates) -> MetaStateReply<Token> {
//         if let NFTPixelboardStateReply::PixelInfo(reply) = self
//             .0
//             .meta_state(NFTPixelboardStateQuery::PixelInfo(coordinates))
//             .unwrap()
//         {
//             MetaStateReply(reply)
//         } else {
//             unreachable!();
//         }
//     }

//     pub fn pixel_price(self) -> MetaStateReply<u128> {
//         if let NFTPixelboardStateReply::PixelPrice(reply) = self
//             .0
//             .meta_state(NFTPixelboardStateQuery::PixelPrice)
//             .unwrap()
//         {
//             MetaStateReply(reply)
//         } else {
//             unreachable!();
//         }
//     }

//     pub fn commission_percentage(self) -> MetaStateReply<u8> {
//         if let NFTPixelboardStateReply::CommissionPercentage(reply) = self
//             .0
//             .meta_state(NFTPixelboardStateQuery::CommissionPercentage)
//             .unwrap()
//         {
//             MetaStateReply(reply)
//         } else {
//             unreachable!();
//         }
//     }

//     pub fn resolution(self) -> MetaStateReply<Resolution> {
//         if let NFTPixelboardStateReply::Resolution(reply) = self
//             .0
//             .meta_state(NFTPixelboardStateQuery::Resolution)
//             .unwrap()
//         {
//             MetaStateReply(reply)
//         } else {
//             unreachable!();
//         }
//     }

//     pub fn token_info(self, token_id: u128) -> MetaStateReply<Token> {
//         if let NFTPixelboardStateReply::TokenInfo(reply) = self
//             .0
//             .meta_state(NFTPixelboardStateQuery::TokenInfo(token_id.into()))
//             .unwrap()
//         {
//             MetaStateReply(reply)
//         } else {
//             unreachable!();
//         }
//     }
// }
