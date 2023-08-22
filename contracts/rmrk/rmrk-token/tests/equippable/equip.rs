use gtest::System;
use types::primitives::TokenId;

use super::utils::{
    add_gem_assets, add_kanaria_assets, compose, equip_gems, mint_tokens, setup_catalog,
};

#[test]
fn equip() {
    let system = System::new();
    system.init_logger();
    setup_catalog(&system);
    mint_tokens(&system);
    add_kanaria_assets(&system);
    add_gem_assets(&system);
    equip_gems(&system);
    let token_id: TokenId = 1.into();
    let asset_id = 2;
    compose(&system, token_id, asset_id);
}
