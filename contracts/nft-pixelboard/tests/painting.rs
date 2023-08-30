pub mod utils;
use utils::{prelude::*, FungibleToken, NonFungibleToken, FOREIGN_USER, USER};

// TODO: uncomment & remove `#[allow(unused)]` after fixing tests.
// #[test]
#[allow(unused)]
fn painting_failures() {
    let system = utils::initialize_system();

    let mut ft_program = FungibleToken::initialize(&system);
    ft_program.mint(USER[0], MAX_PIXEL_PRICE * 25);

    let nft_program = NonFungibleToken::initialize(&system);
    let pixelboard_program =
        NFTPixelboard::initialize(&system, ft_program.actor_id(), nft_program.actor_id());

    pixelboard_program
        .mint(USER[0], vec![0; 25], ((3, 3), (8, 8)).into())
        .succeed(0);

    // Should fail because USER[0] isn't the owner of the NFT.
    pixelboard_program
        .paint(USER[1], 0, vec![0; 25])
        .failed(NFTPixelboardError::NotOwner);
    // Should fail because `painting` length must equal a pixel count in an NFT.
    pixelboard_program
        .paint(USER[0], 0, vec![0; 24])
        .failed(NFTPixelboardError::WrongPaintingLength);
    // Should fail because `painting` length must equal a pixel count in an NFT.
    pixelboard_program
        .paint(USER[0], 0, vec![0; 26])
        .failed(NFTPixelboardError::WrongPaintingLength);
}

// TODO: uncomment & remove `#[allow(unused)]` after fixing tests.
// #[test]
#[allow(unused)]
fn painting() {
    let system = utils::initialize_system();

    let mut ft_program = FungibleToken::initialize(&system);
    ft_program.mint(FOREIGN_USER, MAX_PIXEL_PRICE * (25 + 7 + 20 + 1));

    let nft_program = NonFungibleToken::initialize(&system);

    let mut pixelboard_program =
        NFTPixelboard::initialize(&system, ft_program.actor_id(), nft_program.actor_id());

    pixelboard_program
        .mint(FOREIGN_USER, vec![0; 25], ((3, 3), (8, 8)).into())
        .succeed(0);
    #[rustfmt::skip]
    pixelboard_program
        .paint(
            FOREIGN_USER,
            0,
            vec![
                1, 0, 7, 0, 1,
                0, 7, 0, 7, 0,
                7, 0, 0, 0, 7,
                0, 7, 0, 7, 0,
                1, 0, 7, 0, 1,
            ],
        )
        .succeed(0);
    // #[rustfmt::skip]
    // pixelboard_program.meta_state().painting().check(vec![
    //     0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    //     0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    //     0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    //     0, 0, 0, 1, 0, 7, 0, 1, 0, 0,
    //     0, 0, 0, 0, 7, 0, 7, 0, 0, 0,
    //     0, 0, 0, 7, 0, 0, 0, 7, 0, 0,
    //     0, 0, 0, 0, 7, 0, 7, 0, 0, 0,
    //     0, 0, 0, 1, 0, 7, 0, 1, 0, 0,
    //     0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    //     0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    // ]);

    pixelboard_program
        .mint(FOREIGN_USER, vec![0; 7], ((3, 9), (10, 10)).into())
        .succeed(1);
    pixelboard_program
        .paint(FOREIGN_USER, 1, vec![4, 4, 4, 4, 4, 4, 4])
        .succeed(1);
    // #[rustfmt::skip]
    // pixelboard_program.meta_state().painting().check(vec![
    //     0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    //     0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    //     0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    //     0, 0, 0, 1, 0, 7, 0, 1, 0, 0,
    //     0, 0, 0, 0, 7, 0, 7, 0, 0, 0,
    //     0, 0, 0, 7, 0, 0, 0, 7, 0, 0,
    //     0, 0, 0, 0, 7, 0, 7, 0, 0, 0,
    //     0, 0, 0, 1, 0, 7, 0, 1, 0, 0,
    //     0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    //     0, 0, 0, 4, 4, 4, 4, 4, 4, 4,
    // ]);

    pixelboard_program
        .mint(FOREIGN_USER, vec![0; 20], ((0, 0), (2, 10)).into())
        .succeed(2);
    #[rustfmt::skip]
    pixelboard_program
        .paint(
            FOREIGN_USER,
            2,
            vec![
                8, 9,
                9, 8,
                8, 9,
                9, 8,
                8, 9,
                9, 8,
                8, 9,
                9, 8,
                8, 9,
                9, 8,
            ],
        )
        .succeed(2);
    // #[rustfmt::skip]
    // pixelboard_program.meta_state().painting().check(vec![
    //     8, 9, 0, 0, 0, 0, 0, 0, 0, 0,
    //     9, 8, 0, 0, 0, 0, 0, 0, 0, 0,
    //     8, 9, 0, 0, 0, 0, 0, 0, 0, 0,
    //     9, 8, 0, 1, 0, 7, 0, 1, 0, 0,
    //     8, 9, 0, 0, 7, 0, 7, 0, 0, 0,
    //     9, 8, 0, 7, 0, 0, 0, 7, 0, 0,
    //     8, 9, 0, 0, 7, 0, 7, 0, 0, 0,
    //     9, 8, 0, 1, 0, 7, 0, 1, 0, 0,
    //     8, 9, 0, 0, 0, 0, 0, 0, 0, 0,
    //     9, 8, 0, 4, 4, 4, 4, 4, 4, 4,
    // ]);

    pixelboard_program
        .mint(FOREIGN_USER, vec![0], ((9, 0), (10, 1)).into())
        .succeed(3);
    pixelboard_program
        .paint(FOREIGN_USER, 3, vec![3])
        .succeed(3);
    // #[rustfmt::skip]
    // pixelboard_program.meta_state().painting().check(vec![
    //     8, 9, 0, 0, 0, 0, 0, 0, 0, 3,
    //     9, 8, 0, 0, 0, 0, 0, 0, 0, 0,
    //     8, 9, 0, 0, 0, 0, 0, 0, 0, 0,
    //     9, 8, 0, 1, 0, 7, 0, 1, 0, 0,
    //     8, 9, 0, 0, 7, 0, 7, 0, 0, 0,
    //     9, 8, 0, 7, 0, 0, 0, 7, 0, 0,
    //     8, 9, 0, 0, 7, 0, 7, 0, 0, 0,
    //     9, 8, 0, 1, 0, 7, 0, 1, 0, 0,
    //     8, 9, 0, 0, 0, 0, 0, 0, 0, 0,
    //     9, 8, 0, 4, 4, 4, 4, 4, 4, 4,
    // ]);

    // A few extreme cases...

    ft_program.mint(FOREIGN_USER, MAX_PIXEL_PRICE * (1 + 10 + 10));

    // One pixel canvas
    let mut pixelboard_config = InitNFTPixelboard {
        ft_program: ft_program.actor_id(),
        block_side_length: 1,
        nft_program: nft_program.actor_id(),
        owner: FOREIGN_USER.into(),
        painting: vec![0],
        pixel_price: MAX_PIXEL_PRICE,
        commission_percentage: 100,
        resolution: (1, 1).into(),
    };
    pixelboard_program =
        NFTPixelboard::initialize_custom(&system, pixelboard_config.clone()).succeed();

    pixelboard_program
        .mint(FOREIGN_USER, vec![0], ((0, 0), (1, 1)).into())
        .succeed(4);

    pixelboard_program
        .paint(FOREIGN_USER, 4, vec![4])
        .succeed(4);
    // pixelboard_program.meta_state().painting().check(vec![4]);

    // One column canvas
    pixelboard_config.resolution = (1, 10).into();
    pixelboard_config.painting = vec![0; 10];
    pixelboard_program = NFTPixelboard::initialize_custom(&system, pixelboard_config).succeed();

    pixelboard_program
        .mint(FOREIGN_USER, vec![0; 3], ((0, 2), (1, 5)).into())
        .succeed(5);

    pixelboard_program
        .paint(FOREIGN_USER, 5, vec![1, 2, 3])
        .succeed(5);
    // pixelboard_program
    //     .meta_state()
    //     .painting()
    //     .check(vec![0, 0, 1, 2, 3, 0, 0, 0, 0, 0]);

    // // One row canvas
    // pixelboard_config.resolution = (10, 1).into();
    // pixelboard_program = NFTPixelboard::initialize_custom(&system, pixelboard_config).succeed();

    // pixelboard_program
    //     .mint(FOREIGN_USER, vec![0; 8], ((1, 0), (9, 1)).into())
    //     .check(6);

    // pixelboard_program
    //     .paint(FOREIGN_USER, 6, vec![8, 7, 6, 5, 4, 3, 2, 1])
    //     .check(6);
    // pixelboard_program
    //     .meta_state()
    //     .painting()
    //     .check(vec![0, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
}
