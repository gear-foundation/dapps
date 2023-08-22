mod pixelboard;
pub use pixelboard::*;

mod ftoken;
pub use ftoken::*;

mod nftoken;
pub use nftoken::*;

mod common;
pub use common::*;

pub mod prelude;

pub const FOREIGN_USER: u64 = 12345;
pub const OWNER: u64 = 54321;
pub const USER: [u64; 2] = [3746287346, 13856289765];
