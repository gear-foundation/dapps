use gclient::{EventProcessor, GearApi, Result};
use gstd::{ActorId, Encode};
use nft::WASM_BINARY_OPT;
use nft_io::*;

#[tokio::test]
#[ignore]
async fn mint_test() -> Result<()> {
    let api = GearApi::dev_from_path(env!("GEAR_NODE_PATH")).await?;

    let mut listener = api.subscribe().await?; // Subscribing for events.

    // Checking that blocks still running.
    assert!(listener.blocks_running().await?);

    let collection = Collection {
        name: String::from("MyToken"),
        description: String::from("My token"),
    };
    let actor_id = ActorId::from_slice(&api.account_id().encode()).unwrap();
    let init_nft = InitNFT {
        collection,
        royalties: None,
        constraints: Constraints {
            max_mint_count: Some(100),
            authorized_minters: vec![actor_id],
        },
    }
    .encode();
    let gas_info = api
        .calculate_upload_gas(None, WASM_BINARY_OPT.to_vec(), init_nft.clone(), 0, true)
        .await?;

    let (message_id, program_id, _hash) = api
        .upload_program_bytes(
            WASM_BINARY_OPT.to_vec(),
            gclient::now_micros().to_le_bytes(),
            init_nft,
            gas_info.min_limit,
            0,
        )
        .await?;

    assert!(listener.message_processed(message_id).await?.succeed());

    let transaction_id: u64 = 0;
    use gear_lib::non_fungible_token::token::TokenMetadata;
    let token_metadata = TokenMetadata {
        name: "CryptoKitty".to_string(),
        description: "Description".to_string(),
        media: "http://".to_string(),
        reference: "http://".to_string(),
    };

    let mint_payload = NFTAction::Mint {
        transaction_id,
        token_metadata,
    };

    let gas_info = api
        .calculate_handle_gas(None, program_id, mint_payload.encode(), 0, true)
        .await?;

    let (message_id, _) = api
        .send_message(program_id, mint_payload, gas_info.min_limit, 0)
        .await?;

    assert!(listener.message_processed(message_id).await?.succeed());

    assert!(listener.blocks_running().await?);

    Ok(())
}

#[tokio::test]
#[ignore]
async fn burn_test() -> Result<()> {
    let api = GearApi::dev_from_path(env!("GEAR_NODE_PATH")).await?;

    let mut listener = api.subscribe().await?; // Subscribing for events.

    // Checking that blocks still running.
    assert!(listener.blocks_running().await?);
    let collection = Collection {
        name: String::from("MyToken"),
        description: String::from("My token"),
    };

    let actor_id = ActorId::from_slice(&api.account_id().encode()).unwrap();
    let init_nft = InitNFT {
        collection,
        royalties: None,
        constraints: Constraints {
            max_mint_count: Some(100),
            authorized_minters: vec![actor_id],
        },
    }
    .encode();
    let gas_info = api
        .calculate_upload_gas(None, WASM_BINARY_OPT.to_vec(), init_nft.clone(), 0, true)
        .await?;

    let (message_id, program_id, _hash) = api
        .upload_program_bytes(
            WASM_BINARY_OPT.to_vec(),
            gclient::now_micros().to_le_bytes(),
            init_nft,
            gas_info.min_limit,
            0,
        )
        .await?;

    assert!(listener.message_processed(message_id).await?.succeed());

    let transaction_id: u64 = 0;
    use gear_lib::non_fungible_token::token::TokenMetadata;
    let token_metadata = TokenMetadata {
        name: "CryptoKitty".to_string(),
        description: "Description".to_string(),
        media: "http://".to_string(),
        reference: "http://".to_string(),
    };

    let mint_payload = NFTAction::Mint {
        transaction_id,
        token_metadata,
    };

    let gas_info = api
        .calculate_handle_gas(None, program_id, mint_payload.encode(), 0, true)
        .await?;

    let (message_id, _) = api
        .send_message(program_id, mint_payload, gas_info.min_limit, 0)
        .await?;

    assert!(listener.message_processed(message_id).await?.succeed());

    assert!(listener.blocks_running().await?);

    let transaction_id = transaction_id + 1;

    let burn_payload = NFTAction::Burn {
        transaction_id,
        token_id: 0.into(),
    };

    let gas_info = api
        .calculate_handle_gas(None, program_id, burn_payload.encode(), 0, true)
        .await?;

    let (message_id, _) = api
        .send_message(program_id, burn_payload, gas_info.min_limit, 0)
        .await?;

    assert!(listener.message_processed(message_id).await?.succeed());

    assert!(listener.blocks_running().await?);

    // failures
    let burn_payload = NFTAction::Burn {
        transaction_id,
        token_id: 666.into(),
    };

    let gas_info = api
        .calculate_handle_gas(None, program_id, burn_payload.encode(), 0, true)
        .await?;

    let (message_id, _) = api
        .send_message(program_id, burn_payload, gas_info.min_limit, 0)
        .await?;

    assert!(listener.message_processed(message_id).await?.succeed());

    assert!(listener.blocks_running().await?);

    Ok(())
}

#[tokio::test]
#[ignore]
async fn transfer_test() -> Result<()> {
    let api = GearApi::dev_from_path(env!("GEAR_NODE_PATH")).await?;

    let mut listener = api.subscribe().await?; // Subscribing for events.

    // Checking that blocks still running.
    assert!(listener.blocks_running().await?);

    let collection = Collection {
        name: String::from("MyToken"),
        description: String::from("My token"),
    };

    let actor_id = ActorId::from_slice(&api.account_id().encode()).unwrap();
    let init_nft = InitNFT {
        collection,
        royalties: None,
        constraints: Constraints {
            max_mint_count: Some(100),
            authorized_minters: vec![actor_id],
        },
    }
    .encode();
    let gas_info = api
        .calculate_upload_gas(None, WASM_BINARY_OPT.to_vec(), init_nft.clone(), 0, true)
        .await?;

    let (message_id, program_id, _hash) = api
        .upload_program_bytes(
            WASM_BINARY_OPT.to_vec(),
            gclient::now_micros().to_le_bytes(),
            init_nft,
            gas_info.min_limit,
            0,
        )
        .await?;

    assert!(listener.message_processed(message_id).await?.succeed());

    let transaction_id: u64 = 0;
    use gear_lib::non_fungible_token::token::TokenMetadata;
    let token_metadata = TokenMetadata {
        name: "CryptoKitty".to_string(),
        description: "Description".to_string(),
        media: "http://".to_string(),
        reference: "http://".to_string(),
    };

    let mint_payload = NFTAction::Mint {
        transaction_id,
        token_metadata,
    };

    let gas_info = api
        .calculate_handle_gas(None, program_id, mint_payload.encode(), 0, true)
        .await?;

    let (message_id, _) = api
        .send_message(program_id, mint_payload, gas_info.min_limit, 0)
        .await?;

    assert!(listener.message_processed(message_id).await?.succeed());

    assert!(listener.blocks_running().await?);

    let transaction_id = transaction_id + 1;

    let transfer_payload = NFTAction::Transfer {
        transaction_id,
        to: ActorId::from(4u64),
        token_id: 0.into(),
    };

    let gas_info = api
        .calculate_handle_gas(None, program_id, transfer_payload.encode(), 0, true)
        .await?;

    let (message_id, _) = api
        .send_message(program_id, transfer_payload, gas_info.min_limit, 0)
        .await?;

    assert!(listener.message_processed(message_id).await?.succeed());

    assert!(listener.blocks_running().await?);

    Ok(())
}

#[tokio::test]
#[ignore]
async fn owner_test() -> Result<()> {
    let api = GearApi::dev_from_path(env!("GEAR_NODE_PATH")).await?;

    let mut listener = api.subscribe().await?; // Subscribing for events.

    // Checking that blocks still running.
    assert!(listener.blocks_running().await?);

    let collection = Collection {
        name: String::from("MyToken"),
        description: String::from("My token"),
    };

    let actor_id = ActorId::from_slice(&api.account_id().encode()).unwrap();
    let init_nft = InitNFT {
        collection,
        royalties: None,
        constraints: Constraints {
            max_mint_count: Some(100),
            authorized_minters: vec![actor_id],
        },
    }
    .encode();
    let gas_info = api
        .calculate_upload_gas(None, WASM_BINARY_OPT.to_vec(), init_nft.clone(), 0, true)
        .await?;

    let (message_id, program_id, _hash) = api
        .upload_program_bytes(
            WASM_BINARY_OPT.to_vec(),
            gclient::now_micros().to_le_bytes(),
            init_nft,
            gas_info.min_limit,
            0,
        )
        .await?;

    assert!(listener.message_processed(message_id).await?.succeed());

    let transaction_id: u64 = 0;
    use gear_lib::non_fungible_token::token::TokenMetadata;
    let token_metadata = TokenMetadata {
        name: "CryptoKitty".to_string(),
        description: "Description".to_string(),
        media: "http://".to_string(),
        reference: "http://".to_string(),
    };

    let mint_payload = NFTAction::Mint {
        transaction_id,
        token_metadata,
    };

    let gas_info = api
        .calculate_handle_gas(None, program_id, mint_payload.encode(), 0, true)
        .await?;

    let (message_id, _) = api
        .send_message(program_id, mint_payload, gas_info.min_limit, 0)
        .await?;

    assert!(listener.message_processed(message_id).await?.succeed());

    assert!(listener.blocks_running().await?);

    let owner_payload = NFTAction::Owner { token_id: 0.into() };

    let gas_info = api
        .calculate_handle_gas(None, program_id, owner_payload.encode(), 0, true)
        .await?;

    let (message_id, _) = api
        .send_message(program_id, owner_payload, gas_info.min_limit, 0)
        .await?;

    assert!(listener.message_processed(message_id).await?.succeed());

    assert!(listener.blocks_running().await?);

    Ok(())
}

#[tokio::test]
#[ignore]
async fn approved() -> Result<()> {
    let api = GearApi::dev_from_path(env!("GEAR_NODE_PATH")).await?;

    let mut listener = api.subscribe().await?; // Subscribing for events.

    // Checking that blocks still running.
    assert!(listener.blocks_running().await?);

    let collection = Collection {
        name: String::from("MyToken"),
        description: String::from("My token"),
    };

    let actor_id = ActorId::from_slice(&api.account_id().encode()).unwrap();
    let init_nft = InitNFT {
        collection,
        royalties: None,
        constraints: Constraints {
            max_mint_count: Some(100),
            authorized_minters: vec![actor_id],
        },
    }
    .encode();
    let gas_info = api
        .calculate_upload_gas(None, WASM_BINARY_OPT.to_vec(), init_nft.clone(), 0, true)
        .await?;

    let (message_id, program_id, _hash) = api
        .upload_program_bytes(
            WASM_BINARY_OPT.to_vec(),
            gclient::now_micros().to_le_bytes(),
            init_nft,
            gas_info.min_limit,
            0,
        )
        .await?;

    assert!(listener.message_processed(message_id).await?.succeed());

    let transaction_id: u64 = 0;
    use gear_lib::non_fungible_token::token::TokenMetadata;
    let token_metadata = TokenMetadata {
        name: "CryptoKitty".to_string(),
        description: "Description".to_string(),
        media: "http://".to_string(),
        reference: "http://".to_string(),
    };

    let mint_payload = NFTAction::Mint {
        transaction_id,
        token_metadata,
    };

    let gas_info = api
        .calculate_handle_gas(None, program_id, mint_payload.encode(), 0, true)
        .await?;

    let (message_id, _) = api
        .send_message(program_id, mint_payload, gas_info.min_limit, 0)
        .await?;

    assert!(listener.message_processed(message_id).await?.succeed());

    assert!(listener.blocks_running().await?);

    let transaction_id = transaction_id + 1;
    let approve_payload = NFTAction::Approve {
        transaction_id,
        to: ActorId::from(3),
        token_id: 0.into(),
    };

    let gas_info = api
        .calculate_handle_gas(None, program_id, approve_payload.encode(), 0, true)
        .await?;

    let (message_id, _) = api
        .send_message(program_id, approve_payload, gas_info.min_limit, 0)
        .await?;

    let processed = listener.message_processed(message_id).await?;
    assert!(processed.succeed());

    assert!(listener.blocks_running().await?);

    Ok(())
}
