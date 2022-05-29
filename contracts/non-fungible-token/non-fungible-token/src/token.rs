use codec::{Decode, Encode};
use gstd::prelude::*;
use scale_info::TypeInfo;

#[derive(Debug, Decode, Encode, TypeInfo)]
pub struct TokenMetadata {
    /// The title of NFT Item: for example "CryptoKitty #2505"
    pub title: Option<String>,
    /// The NFT item description
    pub description: Option<String>,
    /// URL to associated media
    pub media: Option<String>,
    /// URL to an off-chain JSON file with more info
    pub reference: Option<String>,
}
