#![no_std]

use gear_lib::non_fungible_token::token::{TokenId, TokenMetadata};
use gstd::{async_main, exec, msg, prelude::*, ActorId};
use nft_pixelboard_io::*;

mod utils;

fn get_pixel_count<P: Into<usize>>(width: P, height: P) -> usize {
    let pixel_count = width.into() * height.into();
    if pixel_count == 0 {
        panic!("Width & height of a canvas/NFT must be more than 0");
    };
    pixel_count
}

fn check_painting(painting: &Vec<Color>, pixel_count: usize) {
    if painting.len() != pixel_count {
        panic!("`painting` length must equal a pixel count in a canvas/NFT");
    }
}

fn check_pixel_price(pixel_price: u128) {
    if pixel_price > MAX_PIXEL_PRICE {
        panic!("`pixel_price` mustn't be more than `MAX_PIXEL_PRICE`");
    }
}

fn get_mut_token<'a>(
    rectangles: &'a BTreeMap<TokenId, Rectangle>,
    tokens: &'a mut BTreeMap<Rectangle, TokenInfo>,
    token_id: TokenId,
) -> (&'a Rectangle, &'a mut TokenInfo) {
    let rectangle = rectangles.get(&token_id).expect("NFT not found by an ID");
    (
        rectangle,
        tokens
            .get_mut(rectangle)
            .expect("NFT not found by a rectangle"),
    )
}

fn paint(
    canvas_resolution: Resolution,
    rectangle: &Rectangle,
    rectangle_width: usize,
    rectangle_height: usize,
    canvas_painting: &mut [Color],
    token_painting: Vec<Color>,
) {
    let canvas_width = canvas_resolution.width as usize;

    let first_row_end = canvas_width * rectangle.top_left_corner.y as usize
        + rectangle.bottom_right_corner.x as usize;
    let first_row_start = first_row_end - rectangle_width;

    let (first_row_painting, rest_of_painting) = token_painting.split_at(rectangle_width);
    canvas_painting[first_row_start..first_row_end].copy_from_slice(first_row_painting);

    for (canvas_row, painting_row) in canvas_painting
        [first_row_end..first_row_end + (rectangle_height - 1) * canvas_width]
        .chunks_exact_mut(canvas_resolution.width as _)
        .zip(rest_of_painting.chunks_exact(rectangle_width))
    {
        canvas_row[canvas_width - rectangle_width..].copy_from_slice(painting_row);
    }
}

#[derive(Default)]
struct NFTPixelboard {
    owner: ActorId,
    block_side_length: BlockSideLength,
    pixel_price: u128,
    resolution: Resolution,
    commission_percentage: u8,
    painting: Vec<Color>,

    rectangles_by_token_ids: BTreeMap<TokenId, Rectangle>,
    tokens_by_rectangles: BTreeMap<Rectangle, TokenInfo>,

    ft_program: ActorId,
    nft_program: ActorId,
}

impl NFTPixelboard {
    async fn mint(
        &mut self,
        rectangle: Rectangle,
        token_metadata: TokenMetadata,
        painting: Vec<Color>,
    ) {
        let msg_source = msg::source();

        // Coordinates checks

        if rectangle.top_left_corner.x % self.block_side_length != 0
            || rectangle.top_left_corner.y % self.block_side_length != 0
            || rectangle.bottom_right_corner.x % self.block_side_length != 0
            || rectangle.bottom_right_corner.y % self.block_side_length != 0
        {
            panic!("Coordinates doesn't observe a block layout");
        }

        if rectangle.top_left_corner.x > rectangle.bottom_right_corner.x
            || rectangle.top_left_corner.y > rectangle.bottom_right_corner.y
        {
            panic!("Coordinates are mixed up or belong to wrong corners");
        }

        if rectangle.bottom_right_corner.x > self.resolution.width
            || rectangle.bottom_right_corner.y > self.resolution.height
        {
            panic!("Coordinates are out of a canvas");
        }

        if self.tokens_by_rectangles.keys().any(|existing_rectangle| {
            existing_rectangle.top_left_corner.x < rectangle.bottom_right_corner.x
                && existing_rectangle.bottom_right_corner.x > rectangle.top_left_corner.x
                && existing_rectangle.top_left_corner.y < rectangle.bottom_right_corner.y
                && existing_rectangle.bottom_right_corner.y > rectangle.top_left_corner.y
        }) {
            panic!("Given NFT rectangle collides with already minted one");
        }

        // Painting

        let rectangle_width = rectangle.width() as usize;
        let rectangle_height = rectangle.height() as usize;
        let rectangle_pixel_count = get_pixel_count(rectangle_width, rectangle_height);

        check_painting(&painting, rectangle_pixel_count);
        paint(
            self.resolution,
            &rectangle,
            rectangle_width,
            rectangle_height,
            &mut self.painting,
            painting,
        );

        // Payment and NFT minting

        utils::transfer_ftokens(
            self.ft_program,
            msg_source,
            self.owner,
            rectangle_pixel_count as u128 * self.pixel_price,
        )
        .await;

        let token_id = utils::mint_nft(self.nft_program, token_metadata).await;
        utils::transfer_nft(self.nft_program, msg_source, token_id).await;

        // Insertion and replying

        self.tokens_by_rectangles.insert(
            rectangle,
            TokenInfo {
                owner: msg_source,
                pixel_price: None,
                token_id,
            },
        );
        self.rectangles_by_token_ids.insert(token_id, rectangle);

        utils::reply(NFTPixelboardEvent::Minted(token_id));
    }

    async fn buy(&mut self, token_id: TokenId) {
        let msg_source = msg::source();
        let (rectangle, token) = get_mut_token(
            &self.rectangles_by_token_ids,
            &mut self.tokens_by_rectangles,
            token_id,
        );

        let pixel_price = token
            .pixel_price
            .unwrap_or_else(|| panic!("NFT isn't for sale"));
        // get_pixel_count() isn't used here because it checks an NFT area for
        // equality to 0, but here it's always not equal 0.
        let token_price =
            (rectangle.width() as usize * rectangle.height() as usize) as u128 * pixel_price;
        let resale_commission = token_price * self.commission_percentage as u128 / 100;

        utils::transfer_ftokens(self.ft_program, msg_source, self.owner, resale_commission).await;
        utils::transfer_ftokens(
            self.ft_program,
            msg_source,
            token.owner,
            token_price - resale_commission,
        )
        .await;

        token.pixel_price = None;
        token.owner = msg_source;
        utils::transfer_nft(self.nft_program, msg_source, token_id).await;

        utils::reply(NFTPixelboardEvent::Bought(token_id));
    }

    async fn change_sale_state(&mut self, token_id: TokenId, pixel_price: Option<u128>) {
        let msg_source = msg::source();
        let (_, token) = get_mut_token(
            &self.rectangles_by_token_ids,
            &mut self.tokens_by_rectangles,
            token_id,
        );
        assert_eq!(token.owner, msg_source);

        if let Some(price) = pixel_price {
            check_pixel_price(price);
            if token.pixel_price.is_none() {
                utils::transfer_nft(self.nft_program, exec::program_id(), token_id).await;
            }
        } else if token.pixel_price.is_some() {
            utils::transfer_nft(self.nft_program, msg_source, token_id).await;
        }
        token.pixel_price = pixel_price;

        utils::reply(NFTPixelboardEvent::SaleStateChanged(token_id));
    }

    fn paint(&mut self, token_id: TokenId, painting: Vec<Color>) {
        let (rectangle, token) = get_mut_token(
            &self.rectangles_by_token_ids,
            &mut self.tokens_by_rectangles,
            token_id,
        );
        assert_eq!(token.owner, msg::source());

        let rectangle_width = rectangle.width() as usize;
        let rectangle_height = rectangle.height() as usize;
        check_painting(
            &painting,
            get_pixel_count(rectangle_width, rectangle_height),
        );

        paint(
            self.resolution,
            rectangle,
            rectangle_width,
            rectangle_height,
            &mut self.painting,
            painting,
        );

        utils::reply(NFTPixelboardEvent::Painted(token_id));
    }
}

static mut PROGRAM: Option<NFTPixelboard> = None;

#[no_mangle]
extern "C" fn init() {
    let InitNFTPixelboard {
        owner,
        ft_program,
        nft_program,
        block_side_length,
        painting,
        resolution,
        commission_percentage,
        pixel_price,
    } = msg::load().expect("Unable to decode `InitNFTPixelboard`");

    if owner == ActorId::zero() {
        panic!("`owner` address mustn't be `ActorId::zero()`");
    }

    if ft_program == ActorId::zero() {
        panic!("`ft_program` address mustn't be `ActorId::zero()`");
    }

    if nft_program == ActorId::zero() {
        panic!("`nft_program` address mustn't be `ActorId::zero()`");
    }

    if block_side_length == 0 {
        panic!("`block_side_length` must be more than 0");
    }

    check_painting(
        &painting,
        get_pixel_count(resolution.width, resolution.height),
    );

    if resolution.width % block_side_length != 0 || resolution.height % block_side_length != 0 {
        panic!("Each side of `resolution` must be a multiple of `block_side_length`");
    }

    if commission_percentage > 100 {
        panic!("`commission_percentage` mustn't be more than 100");
    }

    check_pixel_price(pixel_price);

    let program = NFTPixelboard {
        owner,
        ft_program,
        nft_program,
        block_side_length,
        painting,
        pixel_price,
        commission_percentage,
        resolution,
        ..Default::default()
    };
    unsafe {
        PROGRAM = Some(program);
    }
}

#[async_main]
async fn main() {
    let action: NFTPixelboardAction = msg::load().expect("Unable to decode `NFTPixelboardAction`");
    let program = unsafe { PROGRAM.get_or_insert(Default::default()) };
    match action {
        NFTPixelboardAction::Mint {
            rectangle,
            token_metadata,
            painting,
        } => program.mint(rectangle, token_metadata, painting).await,
        NFTPixelboardAction::Buy(token_id) => program.buy(token_id).await,
        NFTPixelboardAction::ChangeSaleState {
            token_id,
            pixel_price,
        } => program.change_sale_state(token_id, pixel_price).await,
        NFTPixelboardAction::Paint { token_id, painting } => program.paint(token_id, painting),
    }
}

#[no_mangle]
extern "C" fn meta_state() -> *mut [i32; 2] {
    let query: NFTPixelboardStateQuery =
        msg::load().expect("Unable to decode `NFTPixelboardStateQuery`");
    let program = unsafe { PROGRAM.get_or_insert(Default::default()) };
    let encoded = match query {
        NFTPixelboardStateQuery::Painting => {
            NFTPixelboardStateReply::Painting(program.painting.clone())
        }
        NFTPixelboardStateQuery::Resolution => {
            NFTPixelboardStateReply::Resolution(program.resolution)
        }
        NFTPixelboardStateQuery::PixelPrice => {
            NFTPixelboardStateReply::PixelPrice(program.pixel_price)
        }
        NFTPixelboardStateQuery::BlockSideLength => {
            NFTPixelboardStateReply::BlockSideLength(program.block_side_length)
        }
        NFTPixelboardStateQuery::CommissionPercentage => {
            NFTPixelboardStateReply::CommissionPercentage(program.commission_percentage)
        }
        NFTPixelboardStateQuery::PixelInfo(coordinates) => {
            let mut token = Default::default();

            if coordinates.x < program.resolution.width && coordinates.y < program.resolution.height
            {
                let dot: Rectangle = (
                    (coordinates.x, coordinates.y + 1),
                    (coordinates.x, coordinates.y),
                )
                    .into();

                if let Some((rectangle, token_info)) =
                    program.tokens_by_rectangles.range(..dot).next_back()
                {
                    if coordinates.x < rectangle.bottom_right_corner.x
                        && coordinates.y < rectangle.bottom_right_corner.y
                    {
                        token = Token(*rectangle, *token_info)
                    }
                }
            }

            NFTPixelboardStateReply::PixelInfo(token)
        }
        NFTPixelboardStateQuery::TokenInfo(token_id) => {
            let mut token = Default::default();

            if let Some(rectangle) = program.rectangles_by_token_ids.get(&token_id) {
                if let Some(token_info) = program.tokens_by_rectangles.get(rectangle) {
                    token = Token(*rectangle, *token_info);
                }
            }

            NFTPixelboardStateReply::TokenInfo(token)
        }
        NFTPixelboardStateQuery::FTProgram => {
            NFTPixelboardStateReply::FTProgram(program.ft_program)
        }
        NFTPixelboardStateQuery::NFTProgram => {
            NFTPixelboardStateReply::NFTProgram(program.nft_program)
        }
    }
    .encode();
    gstd::util::to_leak_ptr(encoded)
}

gstd::metadata! {
    title: "NFT pixelboard",
    init:
        input: InitNFTPixelboard,
    handle:
        input: NFTPixelboardAction,
        output: NFTPixelboardEvent,
    state:
        input: NFTPixelboardStateQuery,
        output: NFTPixelboardStateReply,
}
