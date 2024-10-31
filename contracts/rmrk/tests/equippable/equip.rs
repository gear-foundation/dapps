use super::utils::{
    add_gem_assets, add_kanaria_assets, compose, equip_gems, mint_tokens, setup_catalog,
};
use crate::utils::mint_value_to_users;
use gtest::System;
use rmrk_types::primitives::TokenId;

#[test]
#[ignore]
fn equip() {
    let system = System::new();
    system.init_logger();
    mint_value_to_users(&system);
    setup_catalog(&system);
    mint_tokens(&system);
    add_kanaria_assets(&system);
    add_gem_assets(&system);
    equip_gems(&system);
    let token_id: TokenId = 1.into();
    let asset_id = 2;
    compose(&system, token_id, asset_id);
}
