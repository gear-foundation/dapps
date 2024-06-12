#![no_std]

use gear_lib_old::non_fungible_token::token::{TokenId, TokenMetadata};
use gmeta::{InOut, Metadata, Out};
use gstd::{prelude::*, ActorId};

pub struct ContractMetadata;

impl Metadata for ContractMetadata {
    type Init = InOut<InitNFTPixelboard, Result<(), NFTPixelboardError>>;
    type Handle = InOut<NFTPixelboardAction, Result<NFTPixelboardEvent, NFTPixelboardError>>;
    type Reply = ();
    type Others = ();
    type Signal = ();
    type State = Out<NFTPixelboardState>;
}

#[derive(Default, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct NFTPixelboardState {
    pub owner: ActorId,
    pub block_side_length: BlockSideLength,
    pub pixel_price: u128,
    pub resolution: Resolution,
    pub commission_percentage: u8,
    pub painting: Vec<Color>,

    pub rectangles_by_token_ids: Vec<(TokenId, Rectangle)>,
    pub tokens_by_rectangles: Vec<(Rectangle, TokenInfo)>,

    pub ft_program: ActorId,
    pub nft_program: ActorId,

    pub txs: Vec<(ActorId, (TransactionId, NFTPixelboardAction))>,
    pub tx_id: TransactionId,
}
/// The maximum price that can be set to a pixel.
///
/// This number is calculated to avoid an overflow and precisely calculate a
/// resale commission. Here's an explanation.
///
/// The maximum number of pixels that a canvas can contain is
/// [`BlockSideLength::MAX`]² = 2³². So the maximum price that each pixel can
/// have is [`u128::MAX`] / [`BlockSideLength::MAX`]² = 2⁹⁶.
///
/// To calculate the commission, the number can be multiplied by 100, so, to
/// avoid an overflow, the number must be divided by 100. Hence 2⁹⁶ / 100.
pub const MAX_PIXEL_PRICE: u128 = 2u128.pow(96) / 100;

/// A block side length.
///
/// It's also used to store pixel [`Coordinates`], [`Resolution`] of a canvas,
/// and NFT [`Rectangle`]s.
pub type BlockSideLength = u16;
/// A pixel color.
pub type Color = u8;
/// A transaction id for tracking transactions in the fungible token contract.
pub type TransactionId = u64;

/// Coordinates of the corners of an NFT rectangle on a canvas.
#[derive(Decode, Encode, TypeInfo, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, Default)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct Rectangle {
    pub top_left_corner: Coordinates,
    pub bottom_right_corner: Coordinates,
}

impl Rectangle {
    pub fn width(&self) -> BlockSideLength {
        self.bottom_right_corner.x - self.top_left_corner.x
    }

    pub fn height(&self) -> BlockSideLength {
        self.bottom_right_corner.y - self.top_left_corner.y
    }
}

impl
    From<(
        (BlockSideLength, BlockSideLength),
        (BlockSideLength, BlockSideLength),
    )> for Rectangle
{
    fn from(
        rectangle: (
            (BlockSideLength, BlockSideLength),
            (BlockSideLength, BlockSideLength),
        ),
    ) -> Self {
        Self {
            top_left_corner: rectangle.0.into(),
            bottom_right_corner: rectangle.1.into(),
        }
    }
}

/// Coordinates of some pixel on a canvas.
#[derive(Decode, Encode, TypeInfo, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, Default)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct Coordinates {
    pub x: BlockSideLength,
    pub y: BlockSideLength,
}

impl From<(BlockSideLength, BlockSideLength)> for Coordinates {
    fn from((x, y): (BlockSideLength, BlockSideLength)) -> Self {
        Self { x, y }
    }
}

/// A resolution of a canvas.
#[derive(Decode, Encode, Default, Clone, Copy, TypeInfo, Debug, PartialEq, Eq)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct Resolution {
    pub width: BlockSideLength,
    pub height: BlockSideLength,
}

impl From<(BlockSideLength, BlockSideLength)> for Resolution {
    fn from((width, height): (BlockSideLength, BlockSideLength)) -> Self {
        Self { width, height }
    }
}

/// An NFT with its [`Rectangle`] and [`TokenInfo`].
#[derive(Decode, Encode, TypeInfo, Clone, Copy, Debug, Default, PartialEq, Eq)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct Token(pub Rectangle, pub TokenInfo);

/// NFT info.
#[derive(Decode, Encode, TypeInfo, Clone, Copy, Debug, Default, PartialEq, Eq)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct TokenInfo {
    pub token_id: Option<TokenId>,
    pub owner: ActorId,
    /// If this field is [`None`], then this NFT isn't for sale, and vice versa.
    ///
    /// To calculate a price of the entire NFT, its area must be calculated and
    /// multiplied by `pixel_price`. The area can be calculated by multiplying a
    /// [width](`Rectangle::width`) & [height](`Rectangle::height`) from NFT
    /// [`Rectangle`]. NFT [`Rectangle`] can be obtained by
    /// [`token_info()`](../nft_pixelboard_state/metafns/fn.token_info.html) using `token_id` from this
    /// struct.
    pub pixel_price: Option<u128>,
}

/// Initializes the NFT pixelboard program.
///
/// # Requirements
/// * `owner` address mustn't be [`ActorId::zero()`].
/// * `block_side_length` must be more than 0.
/// * `pixel_price` mustn't be more than [`MAX_PIXEL_PRICE`].
/// * A [width](`Resolution#structfield.width`) &
///   [height](`Resolution#structfield.height`) (`resolution`) of a canvas must be
///   more than 0.
/// * Each side of `resolution` must be a multiple of `block_side_length`.
/// * `painting` length must equal a pixel count in a canvas (which can be
///   calculated by multiplying a [width](`Resolution#structfield.width`) &
///   [height](`Resolution#structfield.height`) from `resolution`).
/// * `commission_percentage` mustn't be more than 100.
/// * `ft_program` address mustn't be [`ActorId::zero()`].
/// * `nft_program` address mustn't be [`ActorId::zero()`].
#[derive(Decode, Encode, TypeInfo, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct InitNFTPixelboard {
    /// An address of a pixelboard owner to which minting fees and commissions
    /// on resales will be transferred.
    pub owner: ActorId,
    /// A block side length.
    ///
    /// To avoid a canvas clogging with one pixel NFTs, blocks are used instead
    /// of pixels to set NFT [`Rectangle`]s. This parameter is used to set a
    /// side length of these pixel blocks. If blocks aren't needed, then this
    /// parameter can be set to 1, so the block side length will equal a pixel.
    pub block_side_length: BlockSideLength,
    /// The price of a free pixel. It'll be used to calculate a minting price.
    pub pixel_price: u128,
    /// A canvas (pixelboard) [width](`Resolution#structfield.width`) &
    /// [height](`Resolution#structfield.height`).
    pub resolution: Resolution,
    /// A commission percentage that'll be included in each NFT resale.
    pub commission_percentage: u8,
    /// A painting that'll be displayed on the free territory of a pixelboard.
    pub painting: Vec<Color>,

    /// A FT program address.
    pub ft_program: ActorId,
    /// An NFT program address.
    pub nft_program: ActorId,
}

/// Sends a program info about what it should do.
#[derive(Decode, Encode, TypeInfo, Clone, PartialEq, Eq)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum NFTPixelboardAction {
    /// Mints one NFT on a pixelboard with given `token_metadata` & `painting`.
    ///
    /// Transfers a minted NFT to [`msg::source()`].
    ///
    /// # Requirements
    /// * `rectangle` coordinates mustn't be out of a canvas.
    /// * `rectangle` coordinates mustn't be mixed up or belong to wrong
    ///   corners.
    /// * `rectangle` coordinates must observe a block layout. In other words,
    ///   each `rectangle` coordinate must be a multiple of a block side length in
    ///   the canvas. The block side length can be obtained by
    ///   [`block_side_length()`](../nft_pixelboard_state/metafns/fn.block_side_length.html).
    /// * NFT `rectangle` mustn't collide with already minted one.
    /// * `painting` length must equal a pixel count in an NFT
    ///   (which can be calculated by multiplying a [width](`Rectangle::width`) &
    ///   [height](`Rectangle::height`) from `rectangle`).
    /// * [`msg::source()`] must have enough fungible tokens to buy all free
    ///   pixels that `rectangle` will occupy. An enough number of tokens can be
    ///   calculated by multiplying a `rectangle` area and the price of a free
    ///   pixel. The area can be calculated by multiplying a
    ///   [width](`Rectangle::width`) & [height](`Rectangle::height`) from
    ///   `rectangle`. The price of a free pixel can be obtained by
    ///   [`pixel_price()`](../nft_pixelboard_state/metafns/fn.pixel_price.html).
    ///
    /// On success, returns [`NFTPixelboardEvent::Minted`].
    ///
    /// [`msg::source()`]: gstd::msg::source
    Mint {
        rectangle: Rectangle,
        token_metadata: TokenMetadata,
        /// A painting that'll be displayed in a place of an NFT on a pixelboard
        /// after a successful minting.
        painting: Vec<Color>,
    },

    /// Buys an NFT minted on a pixelboard.
    ///
    /// Transfers a purchased NFT from a pixelboard program to
    /// [`msg::source()`].
    ///
    /// **Note:** If [`msg::source()`] has enough fungible tokens to pay a
    /// resale commission but not the entire NFT, then the commission will still
    /// be withdrawn from its account.
    ///
    /// # Requirements
    /// * An NFT must be minted on a pixelboard.
    /// * An NFT must be for sale. This can be found out by
    ///   [`token_info()`]. See also the documentation of
    ///   [`TokenInfo#structfield.pixel_price`].
    /// * [`msg::source()`] must have enough fungible tokens to buy all pixels
    ///   that an NFT occupies. This can be found out by
    ///   [`token_info()`]. See also the documentation of
    ///   [`TokenInfo#structfield.pixel_price`].
    ///
    /// On success, returns [`NFTPixelboardEvent::Bought`].
    ///
    /// [`msg::source()`]: gstd::msg::source
    /// [`token_info()`]: ../nft_pixelboard_state/metafns/fn.token_info.html
    Buy(TokenId),

    /// Changes a sale state of an NFT minted on a pixelboard.
    ///
    /// There are 3 options of a sale state change:
    /// * Putting up for sale\
    ///   If an NFT is **not** for sale, then assigning `pixel_price` to [`Some`]
    ///   price will transfer it to a pixelboard program & put it up for sale.
    /// * Updating a pixel price\
    ///   If an NFT is for sale, then assigning `pixel_price` to [`Some`] price
    ///   will update its pixel price.
    /// * Removing from sale\
    ///   Assigning the `pixel_price` to [`None`] will transfer an NFT back to its
    ///   owner & remove an NFT from sale.
    ///
    /// **Note:** A commission is included in each NFT resale, so a seller
    /// will receive not all fungible tokens but tokens with a commission
    /// deduction. A commission percentage can be obtained by
    /// [`commission_percentage()`](../nft_pixelboard_state/metafns/fn.commission_percentage.html).
    ///
    /// # Requirements
    /// * An NFT must be minted on a pixelboard.
    /// * [`msg::source()`](gstd::msg::source) must be the owner of an NFT.
    /// * `pixel_price` mustn't be more than [`MAX_PIXEL_PRICE`].
    ///
    /// On success, returns [`NFTPixelboardEvent::SaleStateChanged`].
    ChangeSaleState {
        token_id: TokenId,
        /// A price of each pixel that an NFT occupies. To calculate a price of
        /// the entire NFT, see the documentation of
        /// [`TokenInfo#structfield.pixel_price`].
        pixel_price: Option<u128>,
    },

    /// Paints with `painting` an NFT minted on a pixelboard.
    ///
    /// # Requirements
    /// * An NFT must be minted on a pixelboard.
    /// * [`msg::source()`](gstd::msg::source) must be the owner of an NFT.
    /// * `painting` length must equal a pixel count in an NFT. The count can be
    ///   calculated by multiplying a [width](`Rectangle::width`) &
    ///   [height](`Rectangle::height`) from a rectangle of the NFT. The NFT
    ///   rectangle can be obtained by [`token_info()`](../nft_pixelboard_state/metafns/fn.token_info.html).
    ///
    /// On success, returns [`NFTPixelboardEvent::Painted`].
    Paint {
        token_id: TokenId,
        painting: Vec<Color>,
    },
}

/// A result of processed [`NFTPixelboardAction`] in case of successfull execution.
#[derive(Decode, Encode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum NFTPixelboardEvent {
    /// Should be returned from [`NFTPixelboardAction::Mint`].
    Minted(TokenId),
    /// Should be returned from [`NFTPixelboardAction::Buy`].
    Bought(TokenId),
    /// Should be returned from [`NFTPixelboardAction::ChangeSaleState`].
    SaleStateChanged(TokenId),
    /// Should be returned from [`NFTPixelboardAction::Paint`].
    Painted(TokenId),
}

/// A result of processed [`NFTPixelboardAction`] in case of failure.
#[derive(Decode, Encode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum NFTPixelboardError {
    ZeroWidthOrHeight,
    ZeroAddress,
    ZeroBlockSideLength,
    WrongResolution,
    WrongCommissionPercentage,
    WrongPaintingLength,
    PixelPriceExceeded,
    NFTNotFoundById,
    NFTNotFountByRectangle,
    NFTIsNotOnSale,
    NotOwner,
    CoordinatesNotObserveBlockLayout,
    CoordinatesWithWrongCorners,
    CoordinatesOutOfCanvas,
    CoordinatesCollision,
    PreviousTxMustBeCompleted,
    NFTTransferFailed,
    FTokensTransferFailed,
    NFTMintFailed,
}
