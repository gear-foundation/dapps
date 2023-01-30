use crate::utils;
use gear_lib::non_fungible_token::token::{TokenId, TokenMetadata};
use gstd::{async_main, errors::Result as GstdResult, exec, msg, prelude::*, ActorId, MessageId};
use nft_pixelboard_io::*;
pub const MIN_STEP_FOR_TX: u64 = 3;

fn get_pixel_count<P: Into<usize>>(width: P, height: P) -> Result<usize, NFTPixelboardError> {
    let pixel_count = width.into() * height.into();
    if pixel_count == 0 {
        return Err(NFTPixelboardError::ZeroWidthOrHeight);
    };
    Ok(pixel_count)
}

fn check_painting(painting: &Vec<Color>, pixel_count: usize) -> Result<(), NFTPixelboardError> {
    if painting.len() != pixel_count {
        return Err(NFTPixelboardError::WrongPaintingLength);
    }
    Ok(())
}

fn check_pixel_price(pixel_price: u128) -> Result<(), NFTPixelboardError> {
    if pixel_price > MAX_PIXEL_PRICE {
        return Err(NFTPixelboardError::PixelPriceExceeded);
    }
    Ok(())
}

fn get_mut_token<'a>(
    rectangles: &'a BTreeMap<TokenId, Rectangle>,
    tokens: &'a mut BTreeMap<Rectangle, TokenInfo>,
    token_id: TokenId,
) -> Result<(&'a Rectangle, &'a mut TokenInfo), NFTPixelboardError> {
    let rectangle = if let Some(rectangle) = rectangles.get(&token_id) {
        rectangle
    } else {
        return Err(NFTPixelboardError::NFTNotFoundById);
    };
    let tokens = if let Some(tokens) = tokens.get_mut(rectangle) {
        tokens
    } else {
        return Err(NFTPixelboardError::NFTNotFountByRectangle);
    };
    Ok((rectangle, tokens))
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
pub struct NFTPixelboard {
    pub owner: ActorId,
    pub block_side_length: BlockSideLength,
    pub pixel_price: u128,
    pub resolution: Resolution,
    pub commission_percentage: u8,
    pub painting: Vec<Color>,

    pub rectangles_by_token_ids: BTreeMap<TokenId, Rectangle>,
    pub tokens_by_rectangles: BTreeMap<Rectangle, TokenInfo>,

    pub ft_program: ActorId,
    pub nft_program: ActorId,

    pub txs: BTreeMap<ActorId, (TransactionId, NFTPixelboardAction)>,
    pub tx_id: TransactionId,
}

impl NFTPixelboard {
    async fn mint(
        &mut self,
        mut tx_id: TransactionId,
        rectangle: Rectangle,
        token_metadata: TokenMetadata,
        painting: Vec<Color>,
    ) -> Result<NFTPixelboardEvent, NFTPixelboardError> {
        let msg_source = msg::source();
        let rectangle_width = rectangle.width() as usize;
        let rectangle_height = rectangle.height() as usize;
        let rectangle_pixel_count = get_pixel_count(rectangle_width, rectangle_height)?;

        // Payment: transfer to contract account
        utils::transfer_ftokens(
            tx_id,
            &self.ft_program,
            &msg_source,
            &exec::program_id(),
            rectangle_pixel_count as u128 * self.pixel_price,
        )
        .await?;

        if let Err(error) = self.coordinates_check(rectangle, painting.clone()) {
            // transfer tokens back to user
            utils::transfer_ftokens(
                tx_id,
                &self.ft_program,
                &exec::program_id(),
                &msg_source,
                rectangle_pixel_count as u128 * self.pixel_price,
            )
            .await?;
            return Err(error);
        }

        // Painting
        paint(
            self.resolution,
            &rectangle,
            rectangle_width,
            rectangle_height,
            &mut self.painting,
            painting,
        );
        let token_info = self
            .tokens_by_rectangles
            .entry(rectangle)
            .or_insert_with(|| TokenInfo {
                owner: msg_source,
                pixel_price: None,
                token_id: None,
            });

        let token_id = utils::mint_nft(tx_id, &self.nft_program, token_metadata).await?;
        tx_id = tx_id.wrapping_add(1);
        utils::transfer_nft(tx_id, &self.nft_program, &msg_source, token_id).await?;
        utils::transfer_ftokens(
            tx_id,
            &self.ft_program,
            &exec::program_id(),
            &self.owner,
            rectangle_pixel_count as u128 * self.pixel_price,
        )
        .await?;
        // Insertion and replying
        token_info.token_id = Some(token_id);
        self.rectangles_by_token_ids.insert(token_id, rectangle);
        Ok(NFTPixelboardEvent::Minted(token_id))
    }

    async fn buy(
        &mut self,
        mut tx_id: TransactionId,
        token_id: TokenId,
    ) -> Result<NFTPixelboardEvent, NFTPixelboardError> {
        let msg_source = msg::source();
        let (rectangle, token) = get_mut_token(
            &self.rectangles_by_token_ids,
            &mut self.tokens_by_rectangles,
            token_id,
        )?;

        let pixel_price = if let Some(pixel_price) = token.pixel_price {
            pixel_price
        } else {
            return Err(NFTPixelboardError::NFTIsNotOnSale);
        };

        // get_pixel_count() isn't used here because it checks an NFT area for
        // equality to 0, but here it's always not equal 0.
        let token_price =
            (rectangle.width() as usize * rectangle.height() as usize) as u128 * pixel_price;
        let resale_commission = token_price * self.commission_percentage as u128 / 100;

        utils::transfer_ftokens(
            tx_id,
            &self.ft_program,
            &msg_source,
            &exec::program_id(),
            token_price,
        )
        .await?;

        tx_id = tx_id.wrapping_add(1);

        utils::transfer_ftokens(
            tx_id,
            &self.ft_program,
            &exec::program_id(),
            &self.owner,
            resale_commission,
        )
        .await?;

        tx_id = tx_id.wrapping_add(1);

        utils::transfer_ftokens(
            tx_id,
            &self.ft_program,
            &exec::program_id(),
            &token.owner,
            token_price - resale_commission,
        )
        .await?;

        utils::transfer_nft(tx_id, &self.nft_program, &msg_source, token_id).await?;

        token.pixel_price = None;
        token.owner = msg_source;

        Ok(NFTPixelboardEvent::Bought(token_id))
    }

    async fn change_sale_state(
        &mut self,
        tx_id: TransactionId,
        token_id: TokenId,
        pixel_price: Option<u128>,
    ) -> Result<NFTPixelboardEvent, NFTPixelboardError> {
        let msg_source = msg::source();
        let (_, token) = get_mut_token(
            &self.rectangles_by_token_ids,
            &mut self.tokens_by_rectangles,
            token_id,
        )?;
        if token.owner != msg_source {
            return Err(NFTPixelboardError::NotOwner);
        }

        if let Some(price) = pixel_price {
            check_pixel_price(price)?;
            if token.pixel_price.is_none() {
                utils::transfer_nft(tx_id, &self.nft_program, &exec::program_id(), token_id)
                    .await?;
            }
        } else if token.pixel_price.is_some() {
            utils::transfer_nft(tx_id, &self.nft_program, &msg_source, token_id).await?;
        }
        token.pixel_price = pixel_price;

        Ok(NFTPixelboardEvent::SaleStateChanged(token_id))
    }

    fn paint(
        &mut self,
        token_id: TokenId,
        painting: Vec<Color>,
    ) -> Result<NFTPixelboardEvent, NFTPixelboardError> {
        let (rectangle, token) = get_mut_token(
            &self.rectangles_by_token_ids,
            &mut self.tokens_by_rectangles,
            token_id,
        )?;
        if token.owner != msg::source() {
            return Err(NFTPixelboardError::NotOwner);
        }

        let rectangle_width = rectangle.width() as usize;
        let rectangle_height = rectangle.height() as usize;
        let pixel_count = get_pixel_count(rectangle_width, rectangle_height)?;
        check_painting(&painting, pixel_count)?;

        paint(
            self.resolution,
            rectangle,
            rectangle_width,
            rectangle_height,
            &mut self.painting,
            painting,
        );

        Ok(NFTPixelboardEvent::Painted(token_id))
    }

    fn coordinates_check(
        &self,
        rectangle: Rectangle,
        painting: Vec<Color>,
    ) -> Result<(), NFTPixelboardError> {
        if rectangle.top_left_corner.x % self.block_side_length != 0
            || rectangle.top_left_corner.y % self.block_side_length != 0
            || rectangle.bottom_right_corner.x % self.block_side_length != 0
            || rectangle.bottom_right_corner.y % self.block_side_length != 0
        {
            return Err(NFTPixelboardError::CoordinatesNotObserveBlockLayout);
        }

        if rectangle.top_left_corner.x > rectangle.bottom_right_corner.x
            || rectangle.top_left_corner.y > rectangle.bottom_right_corner.y
        {
            return Err(NFTPixelboardError::CoordinatesWithWrongCorners);
        }

        if rectangle.bottom_right_corner.x > self.resolution.width
            || rectangle.bottom_right_corner.y > self.resolution.height
        {
            return Err(NFTPixelboardError::CoordinatesOutOfCanvas);
        }

        if self.tokens_by_rectangles.keys().any(|existing_rectangle| {
            existing_rectangle.top_left_corner.x < rectangle.bottom_right_corner.x
                && existing_rectangle.bottom_right_corner.x > rectangle.top_left_corner.x
                && existing_rectangle.top_left_corner.y < rectangle.bottom_right_corner.y
                && existing_rectangle.bottom_right_corner.y > rectangle.top_left_corner.y
        }) {
            return Err(NFTPixelboardError::CoordinatesCollision);
        }

        let rectangle_width = rectangle.width() as usize;
        let rectangle_height = rectangle.height() as usize;
        let rectangle_pixel_count = get_pixel_count(rectangle_width, rectangle_height)?;

        check_painting(&painting, rectangle_pixel_count)
    }
}

static mut PROGRAM: Option<NFTPixelboard> = None;

#[no_mangle]
extern "C" fn init() {
    let result = process_init();
    let is_err = result.is_err();

    reply(result).expect("Failed to encode or reply with `Result<(), Error>` from `init()`");

    if is_err {
        exec::exit(ActorId::zero());
    }   
}

fn process_init() -> Result<(), NFTPixelboardError> {
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
        return Err(NFTPixelboardError::ZeroAddress);
    }

    if ft_program == ActorId::zero() {
        return Err(NFTPixelboardError::ZeroAddress);
    }

    if nft_program == ActorId::zero() {
        return Err(NFTPixelboardError::ZeroAddress);
    }

    if block_side_length == 0 {
        return Err(NFTPixelboardError::ZeroBlockSideLength);
    }

    let pixel_count = resolution.width as usize * resolution.height as usize;
    if pixel_count == 0 {
        return Err(NFTPixelboardError::ZeroWidthOrHeight);
    };

    if painting.len() != pixel_count {
       return Err(NFTPixelboardError::WrongPaintingLength);
    }

    if resolution.width % block_side_length != 0 || resolution.height % block_side_length != 0 {
        return Err(NFTPixelboardError::WrongResolution);
    }

    if commission_percentage > 100 {
        return Err(NFTPixelboardError::WrongCommissionPercentage);
    }

    if pixel_price > MAX_PIXEL_PRICE {
        return Err(NFTPixelboardError::PixelPriceExceeded);
    }
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
    Ok(())
}
#[async_main]
async fn main() {
    let action: NFTPixelboardAction = msg::load().expect("Unable to decode `NFTPixelboardAction`");
    let program = unsafe { PROGRAM.get_or_insert(Default::default()) };
    let msg_source = msg::source();

    let _reply: Result<NFTPixelboardEvent, NFTPixelboardError> =
        Err(NFTPixelboardError::PreviousTxMustBeCompleted);
    let tx_id = if let Some((tx_id, pend_action)) = program.txs.get(&msg_source) {
        if action != *pend_action {
            reply(_reply).expect(
                "Failed to encode or reply with `Result<NFTPixelboardEvent, NFTPixelboardError>`",
            );
            return;
        }
        *tx_id
    } else {
        let tx_id = program.tx_id;
        program.tx_id = program.tx_id.wrapping_add(MIN_STEP_FOR_TX);
        program.txs.insert(msg_source, (tx_id, action.clone()));
        tx_id
    };

    let result = match action.clone() {
        NFTPixelboardAction::Mint {
            rectangle,
            token_metadata,
            painting,
        } => {
            let reply = program
                .mint(tx_id, rectangle, token_metadata, painting)
                .await;
            program.txs.remove(&msg_source);
            reply
        }
        NFTPixelboardAction::Buy(token_id) => {
            let reply = program.buy(tx_id, token_id).await;
            program.txs.remove(&msg_source);
            reply
        }
        NFTPixelboardAction::ChangeSaleState {
            token_id,
            pixel_price,
        } => {
            let reply = program
                .change_sale_state(tx_id, token_id, pixel_price)
                .await;
            program.txs.remove(&msg_source);
            reply
        }
        NFTPixelboardAction::Paint { token_id, painting } => program.paint(token_id, painting),
    };
    reply(result)
        .expect("Failed to encode or reply with `Result<NFTPixelboardEvent, NFTPixelboardError>`");
}

fn reply(payload: impl Encode) -> GstdResult<MessageId> {
    msg::reply(payload, 0)
}

#[no_mangle]
extern "C" fn state() {
    let nft_pixelboard = unsafe { PROGRAM.as_ref().expect("Program is not initialized") };
    let nft_pixelboard_state: NFTPixelboardState = nft_pixelboard.into();
    msg::reply(nft_pixelboard_state, 0).expect("Failed to share state");
}

#[no_mangle]
extern "C" fn metahash() {
    let metahash: [u8; 32] = include!("../.metahash");
    msg::reply(metahash, 0).expect("Failed to share metahash");
}
