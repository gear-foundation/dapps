#![no_std]

use gear_lib_macros::*;

// NFT
declare_derive_storage_trait!(derive_nft_state, NFTStateKeeper, NFTStateField);
declare_impl_trait!(derive_nft_core, NFTCore);
declare_impl_trait!(derive_nft_metastate, NFTMetaState);

// MultiToken
declare_derive_storage_trait!(derive_mtk_state, StateKeeper, MTKStateKeeper);
declare_impl_trait!(derive_mtk_token_state, MTKTokenState);
declare_impl_trait!(derive_mtk_core, MTKCore);
