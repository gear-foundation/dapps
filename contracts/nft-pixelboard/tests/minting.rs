use gear_lib_old::non_fungible_token::token::TokenMetadata;

pub mod utils;
use utils::{prelude::*, FungibleToken, NonFungibleToken, FOREIGN_USER, OWNER};

#[test]
fn minting_failures() {
    let system = utils::initialize_system();

    let mut ft_program = FungibleToken::initialize(&system);
    ft_program.mint(FOREIGN_USER, MAX_PIXEL_PRICE * 36);

    let nft_program = NonFungibleToken::initialize(&system);

    let pixelboard_config = InitNFTPixelboard {
        ft_program: ft_program.actor_id(),
        block_side_length: 2,
        nft_program: nft_program.actor_id(),
        owner: OWNER.into(),
        painting: vec![0; 100],
        pixel_price: MAX_PIXEL_PRICE,
        commission_percentage: 100,
        resolution: (10, 10).into(),
    };
    let pixelboard_program = NFTPixelboard::initialize_custom(&system, pixelboard_config).succeed();
    let default_painting = vec![0; 36];
    let default_rectangle = ((2, 2), (8, 8)).into();

    // Should fail because the coordinates doesn't observe a block layout.
    pixelboard_program
        .mint(
            FOREIGN_USER,
            default_painting.clone(),
            ((8, 3), (3, 8)).into(),
        )
        .failed(NFTPixelboardError::CoordinatesNotObserveBlockLayout);
    // Should fail because the coordinates doesn't observe a block layout.
    pixelboard_program
        .mint(
            FOREIGN_USER,
            default_painting.clone(),
            ((3, 8), (8, 3)).into(),
        )
        .failed(NFTPixelboardError::CoordinatesNotObserveBlockLayout);
    // Should fail because the coordinates are mixed up or belong to wrong corners.
    pixelboard_program
        .mint(
            FOREIGN_USER,
            default_painting.clone(),
            ((8, 2), (2, 8)).into(),
        )
        .failed(NFTPixelboardError::CoordinatesWithWrongCorners);
    // Should fail because the coordinates are mixed up or belong to wrong corners.
    pixelboard_program
        .mint(
            FOREIGN_USER,
            default_painting.clone(),
            ((8, 8), (2, 2)).into(),
        )
        .failed(NFTPixelboardError::CoordinatesWithWrongCorners);
    // Should fail because the coordinates are mixed up or belong to wrong corners.
    pixelboard_program
        .mint(
            FOREIGN_USER,
            default_painting.clone(),
            ((2, 8), (8, 2)).into(),
        )
        .failed(NFTPixelboardError::CoordinatesWithWrongCorners);
    // Should fail because the coordinates are out of a canvas.
    pixelboard_program
        .mint(
            FOREIGN_USER,
            default_painting.clone(),
            ((2, 2), (12, 12)).into(),
        )
        .failed(NFTPixelboardError::CoordinatesOutOfCanvas);
    // Should fail because the coordinates are mixed up or belong to wrong corners.
    pixelboard_program
        .mint(
            FOREIGN_USER,
            default_painting.clone(),
            ((12, 12), (8, 8)).into(),
        )
        .failed(NFTPixelboardError::CoordinatesWithWrongCorners);
    // Should fail because pixel `painting` length must equal a pixel count in an NFT.
    pixelboard_program
        .mint(FOREIGN_USER, vec![0; 35], default_rectangle)
        .failed(NFTPixelboardError::WrongPaintingLength);
    // Should fail because pixel `painting` length must equal a pixel count in an NFT.
    pixelboard_program
        .mint(FOREIGN_USER, vec![0; 37], default_rectangle)
        .failed(NFTPixelboardError::CoordinatesWithWrongCorners);
    // Should fail because a width & height of an NFT must be more than 0.
    pixelboard_program
        .mint(FOREIGN_USER, vec![], ((4, 4), (4, 4)).into())
        .failed(NFTPixelboardError::ZeroWidthOrHeight);
    // Should fail because a width & height of an NFT must be more than 0.
    pixelboard_program
        .mint(FOREIGN_USER, vec![], ((0, 4), (10, 4)).into())
        .failed(NFTPixelboardError::ZeroWidthOrHeight);
    // Should fail because a width & height of an NFT must be more than 0.
    pixelboard_program
        .mint(FOREIGN_USER, vec![], ((4, 0), (4, 10)).into())
        .failed(NFTPixelboardError::ZeroWidthOrHeight);

    pixelboard_program
        .mint(FOREIGN_USER, default_painting.clone(), default_rectangle)
        .succeed(0);

    // Should fail because the given NFT rectangle collides with already minted one.
    pixelboard_program
        .mint(FOREIGN_USER, default_painting.clone(), default_rectangle)
        .failed(NFTPixelboardError::CoordinatesCollision);
    // Should fail because the given NFT rectangle collides with already minted one.
    pixelboard_program
        .mint(
            FOREIGN_USER,
            default_painting.clone(),
            ((0, 0), (4, 4)).into(),
        )
        .failed(NFTPixelboardError::CoordinatesCollision);
    // Should fail because the given NFT rectangle collides with already minted one.
    pixelboard_program
        .mint(
            FOREIGN_USER,
            default_painting.clone(),
            ((4, 0), (10, 4)).into(),
        )
        .failed(NFTPixelboardError::CoordinatesCollision);
    // Should fail because the given NFT rectangle collides with already minted one.
    pixelboard_program
        .mint(
            FOREIGN_USER,
            default_painting.clone(),
            ((0, 4), (4, 10)).into(),
        )
        .failed(NFTPixelboardError::CoordinatesCollision);
    // Should fail because the given NFT rectangle collides with already minted one.
    pixelboard_program
        .mint(FOREIGN_USER, default_painting, ((4, 4), (10, 10)).into())
        .failed(NFTPixelboardError::CoordinatesCollision);
}

#[test]
fn minting_n_meta_state() {
    let system = utils::initialize_system();

    let mut ft_program = FungibleToken::initialize(&system);
    ft_program.mint(FOREIGN_USER, MAX_PIXEL_PRICE * (6 + 25 + 1));

    let nft_program = NonFungibleToken::initialize(&system);
    let pixelboard_program =
        NFTPixelboard::initialize(&system, ft_program.actor_id(), nft_program.actor_id());

    let mut token = Token(
        ((1, 1), (2, 7)).into(),
        TokenInfo {
            owner: FOREIGN_USER.into(),
            pixel_price: None,
            token_id: Some(0.into()),
        },
    );

    pixelboard_program
        .mint(FOREIGN_USER, vec![0; 6], token.0)
        .succeed(0);

    ft_program
        .balance(FOREIGN_USER)
        .succeed(MAX_PIXEL_PRICE * (25 + 1));
    ft_program.balance(OWNER).succeed(MAX_PIXEL_PRICE * 6);
    //pixelboard_program.meta_state().token_info(0).check(token);
    // pixelboard_program
    //     .meta_state()
    //     .pixel_info((1, 4).into())
    //     .check(token);

    token.0 = ((3, 3), (8, 8)).into();
    token.1.token_id = Some(1.into());
    let token_metadata = TokenMetadata {
        name: "The incredibly ordinary rectangle".into(),
        description: "Really, it can't be more boring".into(),
        media: "1029384756".into(),
        reference: "https://youtu.be/dQw4w9WgXcQ".into(),
    };
    pixelboard_program
        .mint_with_metadata(FOREIGN_USER, vec![0; 25], token.0, token_metadata)
        .succeed(1);

    // ft_program.balance(FOREIGN_USER).check(MAX_PIXEL_PRICE);
    // ft_program.balance(OWNER).check(MAX_PIXEL_PRICE * (6 + 25));
    // pixelboard_program.meta_state().token_info(1).check(token);
    // nft_program.meta_state().token_metadata(1).check(NFToken {
    //     owner_id: FOREIGN_USER.into(),
    //     description: token_metadata.description,
    //     media: token_metadata.media,
    //     name: token_metadata.name,
    //     reference: token_metadata.reference,
    //     id: token.1.token_id,
    //     approved_account_ids: Default::default(),
    // });

    // // NFT center
    // pixelboard_program
    //     .meta_state()
    //     .pixel_info((5, 5).into())
    //     .check(token);
    // // NFT corners
    // pixelboard_program
    //     .meta_state()
    //     .pixel_info((3, 3).into())
    //     .check(token);
    // pixelboard_program
    //     .meta_state()
    //     .pixel_info((7, 7).into())
    //     .check(token);
    // pixelboard_program
    //     .meta_state()
    //     .pixel_info((3, 7).into())
    //     .check(token);
    // pixelboard_program
    //     .meta_state()
    //     .pixel_info((7, 3).into())
    //     .check(token);
    // // Pixels outside of the NFT
    // pixelboard_program
    //     .meta_state()
    //     .pixel_info((2, 2).into())
    //     .check(Token::default());
    // pixelboard_program
    //     .meta_state()
    //     .pixel_info((8, 8).into())
    //     .check(Token::default());
    // pixelboard_program
    //     .meta_state()
    //     .pixel_info((2, 8).into())
    //     .check(Token::default());
    // pixelboard_program
    //     .meta_state()
    //     .pixel_info((8, 2).into())
    //     .check(Token::default());
    // // Pixel between NFTs
    // pixelboard_program
    //     .meta_state()
    //     .pixel_info((2, 5).into())
    //     .check(Token::default());
    // // Pixels on the edge/outside of the canvas
    // pixelboard_program
    //     .meta_state()
    //     .pixel_info((0, 0).into())
    //     .check(Token::default());
    // pixelboard_program
    //     .meta_state()
    //     .pixel_info((9, 9).into())
    //     .check(Token::default());
    // pixelboard_program
    //     .meta_state()
    //     .pixel_info((10, 10).into())
    //     .check(Token::default());

    token.0 = ((9, 9), (10, 10)).into();
    token.1.token_id = Some(2.into());
    // Minting a one pixel NFT
    pixelboard_program
        .mint(FOREIGN_USER, vec![0], token.0)
        .succeed(2);

    ft_program.balance(FOREIGN_USER).succeed(0);
    ft_program
        .balance(OWNER)
        .succeed(MAX_PIXEL_PRICE * (6 + 25 + 1));
    // pixelboard_program.meta_state().token_info(2).check(token);
    // pixelboard_program
    //     .meta_state()
    //     .pixel_info((9, 9).into())
    //     .check(token);

    // // Pixels outside of the one pixel NFT
    // pixelboard_program
    //     .meta_state()
    //     .pixel_info((8, 8).into())
    //     .check(Token::default());
    // pixelboard_program
    //     .meta_state()
    //     .pixel_info((10, 10).into())
    //     .check(Token::default());
    // pixelboard_program
    //     .meta_state()
    //     .pixel_info((8, 10).into())
    //     .check(Token::default());
    // pixelboard_program
    //     .meta_state()
    //     .pixel_info((10, 8).into())
    //     .check(Token::default());
}
