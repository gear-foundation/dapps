#![no_std]

extern crate alloc;
extern crate proc_macro;

#[allow(unused_imports)]
use gear_lib_macros::*;

declare_derive_storage_trait!(derive_mtk_state, StateKeeper, MTKStateKeeper);
declare_impl_trait!(derive_mtk_token_state, MTKTokenState);
declare_impl_trait!(derive_mtk_core, MTKCore);
