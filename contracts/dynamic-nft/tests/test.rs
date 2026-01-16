use dynamic_nft_client::DynamicNft as ClientDynamicNft;
use dynamic_nft_client::TokenMetadata;
use dynamic_nft_client::dynamic_nft::DynamicNft;

use sails_rs::client::*;
use sails_rs::gtest::System;
use sails_rs::gtest::constants::DEFAULT_USERS_INITIAL_BALANCE;

use dynamic_nft_client::DynamicNftCtors;

pub const ADMIN_ID: u64 = 10;
pub const USER_ID: [u64; 2] = [11, 12];

#[tokio::test]
async fn test_basic_function() {
    let system = System::new();
    system.init_logger_with_default_filter("gwasm=debug,gtest=info,sails_rs=debug");

    system.mint_to(ADMIN_ID, DEFAULT_USERS_INITIAL_BALANCE);
    system.mint_to(USER_ID[0], DEFAULT_USERS_INITIAL_BALANCE);
    system.mint_to(USER_ID[1], DEFAULT_USERS_INITIAL_BALANCE);

    let env = GtestEnv::new(system, ADMIN_ID.into());

    let code_id = env
        .system()
        .submit_code_file("../target/wasm32-gear/release/dynamic_nft.opt.wasm");

    let program = env
        .deploy::<dynamic_nft_client::DynamicNftProgram>(code_id, b"salt".to_vec())
        .new(
            "collection_name".to_string(),
            "collection_symbol".to_string(),
            5_000_000_000,
        )
        .await
        .unwrap();

    let mut client = program.dynamic_nft();

    // mint
    let metadata = TokenMetadata {
        name: "token_name".to_string(),
        description: "token_description".to_string(),
        current_media_index: 0,
        media: vec!["token_media".to_string()],
        reference: "token_reference".to_string(),
    };

    client.mint(ADMIN_ID.into(), metadata).await.unwrap();

    // check balance
    let balance = client.balance_of(ADMIN_ID.into()).await.unwrap();
    assert_eq!(balance, 1.into());

    // check token_id
    let token_id = client.token_id().await.unwrap();
    assert_eq!(token_id, 1.into());

    // check owner
    let actor_id = client.owner_of(0.into()).await.unwrap();
    assert_eq!(actor_id, ADMIN_ID.into());

    // transfer
    client.transfer(USER_ID[0].into(), 0.into()).await.unwrap();

    // check owner
    let actor_id = client.owner_of(0.into()).await.unwrap();
    assert_eq!(actor_id, USER_ID[0].into());

    // approve
    client
        .approve(USER_ID[1].into(), 0.into())
        .with_actor_id(USER_ID[0].into())
        .await
        .unwrap();

    // transfer_from
    client
        .transfer_from(USER_ID[0].into(), ADMIN_ID.into(), 0.into())
        .with_actor_id(USER_ID[1].into())
        .await
        .unwrap();

    // check owner
    let actor_id = client.owner_of(0.into()).await.unwrap();
    assert_eq!(actor_id, ADMIN_ID.into());

    // burn
    client.burn(ADMIN_ID.into(), 0.into()).await.unwrap();

    // check balance
    let balance = client.balance_of(ADMIN_ID.into()).await.unwrap();
    assert_eq!(balance, 0.into());

    // check owner
    let actor_id = client.owner_of(0.into()).await.unwrap();
    assert_eq!(actor_id, 0.into());
}

#[tokio::test]
async fn test_grant_role() {
    let system = System::new();
    system.init_logger_with_default_filter("gwasm=debug,gtest=info,sails_rs=debug");

    system.mint_to(ADMIN_ID, DEFAULT_USERS_INITIAL_BALANCE);
    system.mint_to(USER_ID[0], DEFAULT_USERS_INITIAL_BALANCE);
    system.mint_to(USER_ID[1], DEFAULT_USERS_INITIAL_BALANCE);

    let env = GtestEnv::new(system, ADMIN_ID.into());

    let code_id = env
        .system()
        .submit_code_file("../target/wasm32-gear/release/dynamic_nft.opt.wasm");

    let program = env
        .deploy::<dynamic_nft_client::DynamicNftProgram>(code_id, b"salt".to_vec())
        .new("name".to_string(), "symbol".to_string(), 5_000_000_000)
        .await
        .unwrap();

    let mut client = program.dynamic_nft();

    let metadata = TokenMetadata {
        name: "token_name".to_string(),
        description: "token_description".to_string(),
        current_media_index: 0,
        media: vec!["token_media".to_string()],
        reference: "token_reference".to_string(),
    };

    // try minter role
    let res = client
        .mint(USER_ID[0].into(), metadata)
        .with_actor_id(USER_ID[0].into())
        .await;
    assert!(res.is_err());

    // grant mint role
    client.grant_minter_role(USER_ID[0].into()).await.unwrap();

    let minters = client.minters().await.unwrap();
    assert!(minters.contains(&ADMIN_ID.into()));
    assert!(minters.contains(&USER_ID[0].into()));

    // mint
    client
        .mint(
            USER_ID[0].into(),
            TokenMetadata {
                name: "token_name".to_string(),
                description: "token_description".to_string(),
                current_media_index: 0,
                media: vec!["token_media".to_string()],
                reference: "token_reference".to_string(),
            },
        )
        .with_actor_id(USER_ID[0].into())
        .await
        .unwrap();

    let balance = client.balance_of(USER_ID[0].into()).await.unwrap();
    assert_eq!(balance, 1.into());

    // try burner role
    let res = client
        .burn(USER_ID[0].into(), 0.into())
        .with_actor_id(USER_ID[0].into())
        .await;
    assert!(res.is_err());

    // grant burn role
    client.grant_burner_role(USER_ID[0].into()).await.unwrap();

    let burners = client.burners().await.unwrap();
    assert!(burners.contains(&ADMIN_ID.into()));
    assert!(burners.contains(&USER_ID[0].into()));

    // burn
    client
        .burn(USER_ID[0].into(), 0.into())
        .with_actor_id(USER_ID[0].into())
        .await
        .unwrap();

    let balance = client.balance_of(USER_ID[0].into()).await.unwrap();
    assert_eq!(balance, 0.into());

    // grant admin role
    client.grant_admin_role(USER_ID[0].into()).await.unwrap();

    let admins = client.admins().await.unwrap();
    assert!(admins.contains(&ADMIN_ID.into()));
    assert!(admins.contains(&USER_ID[0].into()));

    // revoke roles
    client.revoke_admin_role(USER_ID[0].into()).await.unwrap();
    let admins = client.admins().await.unwrap();
    assert_eq!(admins, vec![ADMIN_ID.into()]);

    client.revoke_minter_role(USER_ID[0].into()).await.unwrap();
    let minters = client.minters().await.unwrap();
    assert_eq!(minters, vec![ADMIN_ID.into()]);

    client.revoke_burner_role(USER_ID[0].into()).await.unwrap();
    let burners = client.burners().await.unwrap();
    assert_eq!(burners, vec![ADMIN_ID.into()]);
}

#[tokio::test]
async fn test_metadata_update() {
    let system = System::new();
    system.init_logger_with_default_filter("gwasm=debug,gtest=info,sails_rs=debug");

    system.mint_to(ADMIN_ID, DEFAULT_USERS_INITIAL_BALANCE);
    system.mint_to(USER_ID[0], DEFAULT_USERS_INITIAL_BALANCE);
    system.mint_to(USER_ID[1], DEFAULT_USERS_INITIAL_BALANCE);

    let env = GtestEnv::new(system, ADMIN_ID.into());

    let code_id = env
        .system()
        .submit_code_file("../target/wasm32-gear/release/dynamic_nft.opt.wasm");

    let program = env
        .deploy::<dynamic_nft_client::DynamicNftProgram>(code_id, b"salt".to_vec())
        .new(
            "collection_name".to_string(),
            "collection_symbol".to_string(),
            5_000_000_000,
        )
        .await
        .unwrap();

    let mut client = program.dynamic_nft();

    // mint
    let metadata = TokenMetadata {
        name: "token_name".to_string(),
        description: "token_description".to_string(),
        current_media_index: 0,
        media: vec![
            "token_media 1".to_string(),
            "token_media 2".to_string(),
            "token_media 3".to_string(),
        ],
        reference: "token_reference".to_string(),
    };

    client.mint(ADMIN_ID.into(), metadata).await.unwrap();

    // sanity checks
    assert_eq!(client.balance_of(ADMIN_ID.into()).await.unwrap(), 1.into());
    assert_eq!(client.token_id().await.unwrap(), 1.into());
    assert_eq!(client.owner_of(0.into()).await.unwrap(), ADMIN_ID.into());

    // start metadata update
    client.start_metadata_update(3, 5, 0.into()).await.unwrap();

    // check metadata -> should move to index 1 immediately
    let meta = client
        .token_metadata_by_id(0.into())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(meta.current_media_index, 1);

    // advance blocks
    for _ in 0..5 {
        env.system().run_next_block();
    }

    let meta = client
        .token_metadata_by_id(0.into())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(meta.current_media_index, 2);

    for _ in 0..5 {
        env.system().run_next_block();
    }

    let meta = client
        .token_metadata_by_id(0.into())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(meta.current_media_index, 0);
}
