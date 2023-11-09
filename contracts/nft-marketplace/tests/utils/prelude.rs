pub use super::{Market, Program};
pub use gstd::prelude::*;
pub use nft_marketplace::*;

pub const BUYER: u64 = 100;
pub const SELLER: u64 = 101;
pub const NFT_PRICE: u128 = 1_000_000_000_000_000;
pub const ADMIN: u64 = 200;
pub const TREASURY_ID: u64 = 300;
pub const TREASURY_FEE: u16 = 3;
pub const TOKEN_ID: u64 = 0;
pub const BID_PERIOD: u64 = 3_600_000;
pub const DURATION: u64 = 86_400_000;
pub const PARTICIPANTS: &[u64] = &[500, 501, 502, 503, 504];
pub const MARKET_ID: u64 = 3;
pub const MIN_BID_PERIOD: u64 = 60_000;
