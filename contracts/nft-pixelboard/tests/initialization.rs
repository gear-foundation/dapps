use gstd::ActorId;

pub mod utils;
use utils::{prelude::*, FungibleToken, NonFungibleToken, FOREIGN_USER};

// # TODO:: remove ignore after fixing tests
#[ignore]
#[test]
fn initialization_failures() {
    let system = utils::initialize_system();

    let ft_program = FungibleToken::initialize(&system);
    let nft_program = NonFungibleToken::initialize(&system);

    let pixelboard_config = InitNFTPixelboard {
        ft_program: ft_program.actor_id(),
        block_side_length: 10,
        nft_program: nft_program.actor_id(),
        owner: FOREIGN_USER.into(),
        painting: vec![0; 100],
        pixel_price: MAX_PIXEL_PRICE,
        commission_percentage: 100,
        resolution: (10, 10).into(),
    };

    let mut failed_pixelboard_config = pixelboard_config.clone();
    failed_pixelboard_config.owner = ActorId::zero();
    // Should fail because `owner` address mustn't be `ActorId::zero()`.
    NFTPixelboard::initialize_custom(&system, failed_pixelboard_config).failed(NFTPixelboardError::ZeroAddress);

    failed_pixelboard_config = pixelboard_config.clone();
    failed_pixelboard_config.ft_program = ActorId::zero();
    // Should fail because `ft_program` address mustn't be `ActorId::zero()`.
    NFTPixelboard::initialize_custom(&system, failed_pixelboard_config).failed(NFTPixelboardError::ZeroAddress);

    failed_pixelboard_config = pixelboard_config.clone();
    failed_pixelboard_config.nft_program = ActorId::zero();
    // Should fail because `nft_program` address mustn't be `ActorId::zero()`.
    NFTPixelboard::initialize_custom(&system, failed_pixelboard_config).failed(NFTPixelboardError::ZeroAddress);

    failed_pixelboard_config = pixelboard_config.clone();
    failed_pixelboard_config.block_side_length = 0;
    // Should fail because `block_side_length` must be more than 0.
    NFTPixelboard::initialize_custom(&system, failed_pixelboard_config).failed(NFTPixelboardError::ZeroBlockSideLength);

    failed_pixelboard_config = pixelboard_config.clone();
    failed_pixelboard_config.resolution.width = 0;
    // Should fail because canvas `width` must be more than 0.
    NFTPixelboard::initialize_custom(&system, failed_pixelboard_config).failed(NFTPixelboardError::ZeroWidthOrHeight);

    failed_pixelboard_config = pixelboard_config.clone();
    failed_pixelboard_config.resolution.height = 0;
    // Should fail because canvas `height` must be more than 0.
    NFTPixelboard::initialize_custom(&system, failed_pixelboard_config).failed(NFTPixelboardError::ZeroWidthOrHeight);

    failed_pixelboard_config = pixelboard_config.clone();
    failed_pixelboard_config.resolution = (0, 0).into();
    // Should fail because a width & height of a canvas must be more than 0.
    NFTPixelboard::initialize_custom(&system, failed_pixelboard_config).failed(NFTPixelboardError::ZeroWidthOrHeight);

    failed_pixelboard_config = pixelboard_config.clone();
    failed_pixelboard_config.resolution.width = 15;
    failed_pixelboard_config.painting = vec![1; 150];
    // Should fail because each side of `resolution` must be a multiple of `block_side_length`.
    NFTPixelboard::initialize_custom(&system, failed_pixelboard_config).failed(NFTPixelboardError::WrongResolution);

    failed_pixelboard_config = pixelboard_config.clone();
    failed_pixelboard_config.resolution.height = 15;
    failed_pixelboard_config.painting = vec![1; 150];
    // Should fail because each side of `resolution` must be a multiple of `block_side_length`.
    NFTPixelboard::initialize_custom(&system, failed_pixelboard_config).failed(NFTPixelboardError::WrongResolution);

    failed_pixelboard_config = pixelboard_config.clone();
    failed_pixelboard_config.resolution = (15, 15).into();
    failed_pixelboard_config.painting = vec![1; 225];
    // Should fail because each side of `resolution` must be a multiple of `block_side_length`.
    NFTPixelboard::initialize_custom(&system, failed_pixelboard_config).failed(NFTPixelboardError::WrongResolution);

    failed_pixelboard_config = pixelboard_config.clone();
    failed_pixelboard_config.commission_percentage = 101;
    // Should fail because `commission_percentage` mustn't be more than 100.
    NFTPixelboard::initialize_custom(&system, failed_pixelboard_config).failed(NFTPixelboardError::WrongCommissionPercentage);

    failed_pixelboard_config = pixelboard_config;
    failed_pixelboard_config.pixel_price = MAX_PIXEL_PRICE + 1;
    // Should fail because `pixel_price` mustn't be more than `MAX_PIXEL_PRICE`.
    NFTPixelboard::initialize_custom(&system, failed_pixelboard_config).failed(NFTPixelboardError::PixelPriceExceeded);

    // failed_pixelboard_config = pixelboard_config.clone();
    // failed_pixelboard_config.painting = vec![1; 101];
    // // Should fail because `painting` length must equal a pixel count in a canvas.
    // NFTPixelboard::initialize_custom(&system, failed_pixelboard_config).failed();

    // failed_pixelboard_config = pixelboard_config;
    // failed_pixelboard_config.painting = vec![1; 99];
    // // Should fail because `painting` length must equal a pixel count in a canvas.
    // NFTPixelboard::initialize_custom(&system, failed_pixelboard_config).failed();
}

// #[test]
// fn initialization_n_meta_state() {
//     let system = utils::initialize_system();

//     let ft_program = FungibleToken::initialize(&system);
//     let nft_program = NonFungibleToken::initialize(&system);

//     let pixelboard_config = InitNFTPixelboard {
//         ft_program: ft_program.actor_id(),
//         block_side_length: 1,
//         nft_program: nft_program.actor_id(),
//         owner: FOREIGN_USER.into(),
//         painting: vec![0; 100],
//         pixel_price: MAX_PIXEL_PRICE,
//         commission_percentage: 100,
//         resolution: (10, 10).into(),
//     };
// let pixelboard_program =
//     NFTPixelboard::initialize_custom(&system, pixelboard_config.clone()).succeed();

// # TODO: uncomment when new meta will be ready for gtest

// pixelboard_program
//     .meta_state()
//     .ft_program()
//     .check(ft_program.actor_id());
// pixelboard_program
//     .meta_state()
//     .nft_program()
//     .check(nft_program.actor_id());
// pixelboard_program
//     .meta_state()
//     .block_side_length()
//     .check(pixelboard_config.block_side_length);
// pixelboard_program
//     .meta_state()
//     .painting()
//     .check(pixelboard_config.painting);
// pixelboard_program
//     .meta_state()
//     .pixel_price()
//     .check(pixelboard_config.pixel_price);
// pixelboard_program
//     .meta_state()
//     .commission_percentage()
//     .check(pixelboard_config.commission_percentage);
// pixelboard_program
//     .meta_state()
//     .resolution()
//     .check(pixelboard_config.resolution);
//}
