pub mod utils;
use utils::{prelude::*, FungibleToken, NonFungibleToken, FOREIGN_USER, OWNER, USER};

// TODO: uncomment & remove `#[allow(unused)]` after fixing tests.
// #[test]
#[allow(unused)]
fn reselling() {
    let system = utils::initialize_system();

    let mut ft_program = FungibleToken::initialize(&system);
    ft_program.mint(USER[0], MAX_PIXEL_PRICE * 25);
    ft_program.mint(USER[1], 25);

    let nft_program = NonFungibleToken::initialize(&system);

    let pixelboard_config = InitNFTPixelboard {
        ft_program: ft_program.actor_id(),
        block_side_length: 1,
        nft_program: nft_program.actor_id(),
        owner: OWNER.into(),
        painting: vec![0; 100],
        pixel_price: MAX_PIXEL_PRICE,
        commission_percentage: 13,
        resolution: (10, 10).into(),
    };
    let pixelboard_program =
        NFTPixelboard::initialize_custom(&system, pixelboard_config.clone()).succeed();

    let mut token = Token(
        ((3, 3), (8, 8)).into(),
        TokenInfo {
            owner: USER[0].into(),
            token_id: Some(0.into()),
            pixel_price: Some(MAX_PIXEL_PRICE),
        },
    );

    pixelboard_program
        .mint(USER[0], vec![0; 25], token.0)
        .succeed(0);
    // Putting the NFT up for sale
    pixelboard_program
        .change_sale_state(USER[0], 0, Some(MAX_PIXEL_PRICE))
        .succeed(0);

    // pixelboard_program.meta_state().token_info(0).check(token);
    // nft_program
    //     .meta_state()
    //     .owner(0)
    //     .check(pixelboard_program.actor_id());

    // Removing the NFT from sale
    pixelboard_program
        .change_sale_state(USER[0], 0, None)
        .succeed(0);
    token.1.pixel_price = None;

    // pixelboard_program.meta_state().token_info(0).check(token);
    // nft_program.meta_state().owner(0).check(USER[0].into());

    pixelboard_program
        .change_sale_state(USER[0], 0, Some(0))
        .succeed(0);
    // Updating an NFT pixel price
    pixelboard_program
        .change_sale_state(USER[0], 0, Some(1))
        .succeed(0);
    token.1.pixel_price = Some(1);

    // pixelboard_program.meta_state().token_info(0).check(token);
    // nft_program
    //     .meta_state()
    //     .owner(0)
    //     .check(pixelboard_program.actor_id());

    pixelboard_program.buy(USER[1], 0).succeed(0);
    token.1.owner = USER[1].into();
    token.1.pixel_price = None;

    let commission = 25 * pixelboard_config.commission_percentage as u128 / 100;
    ft_program
        .balance(OWNER)
        .succeed(MAX_PIXEL_PRICE * 25 + commission);
    ft_program.balance(USER[0]).succeed(25 - commission);
    ft_program.balance(USER[1]).succeed(0);
    // nft_program.meta_state().owner(0).(USER[1].into());
    // pixelboard_program.meta_state().token_info(0).check(token);
}

// TODO: uncomment & remove `#[allow(unused)]` after fixing tests.
// #[test]
#[allow(unused)]
fn reselling_failures() {
    let system = utils::initialize_system();

    let mut ft_program = FungibleToken::initialize(&system);
    ft_program.mint(USER[0], MAX_PIXEL_PRICE * 25);
    ft_program.mint(USER[1], MAX_PIXEL_PRICE * 24);

    let nft_program = NonFungibleToken::initialize(&system);

    let pixelboard_config = InitNFTPixelboard {
        ft_program: ft_program.actor_id(),
        block_side_length: 1,
        nft_program: nft_program.actor_id(),
        owner: OWNER.into(),
        painting: vec![0; 100],
        pixel_price: MAX_PIXEL_PRICE,
        commission_percentage: 13,
        resolution: (10, 10).into(),
    };
    let pixelboard_program =
        NFTPixelboard::initialize_custom(&system, pixelboard_config.clone()).succeed();

    pixelboard_program
        .mint(USER[0], vec![0; 25], ((3, 3), (8, 8)).into())
        .succeed(0);
    // Should fail because FOREIGN_USER isn't the owner of the NFT.
    pixelboard_program
        .change_sale_state(FOREIGN_USER, 0, Some(MAX_PIXEL_PRICE))
        .failed(NFTPixelboardError::NotOwner);
    // Should fail because `pixel_price` mustn't be more than `MAX_PIXEL_PRICE`.
    pixelboard_program
        .change_sale_state(USER[0], 0, Some(MAX_PIXEL_PRICE + 1))
        .failed(NFTPixelboardError::PixelPriceExceeded);
    // Should fail because the NFT isn't for sale.
    pixelboard_program
        .buy(USER[1], 0)
        .failed(NFTPixelboardError::NFTIsNotOnSale);

    pixelboard_program
        .change_sale_state(USER[0], 0, Some(MAX_PIXEL_PRICE))
        .succeed(0);

    // Should fail because USER[0] doesn't have enough fungible tokens to buy this NFT.
    pixelboard_program
        .buy(USER[1], 0)
        .failed(NFTPixelboardError::FTokensTransferFailed);

    // But a commission should still be debited from USER[0] because USER[0] has enough tokens for it.
    let commission = MAX_PIXEL_PRICE * 25 * pixelboard_config.commission_percentage as u128 / 100;
    ft_program
        .balance(USER[1])
        .succeed(MAX_PIXEL_PRICE * 24 - commission);
    ft_program
        .balance(OWNER)
        .succeed(MAX_PIXEL_PRICE * 25 + commission);
}
