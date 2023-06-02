use auto_changed_nft::WASM_BINARY_OPT;
use auto_changed_nft_io::*;
use gclient::{EventProcessor, GearApi, Result};
use gear_lib::non_fungible_token::token::TokenId;
use gstd::Encode;

// #[tokio::test]
// #[ignore]
// async fn mint_test() -> Result<()> {
//     let api = GearApi::dev_from_path(env!("GEAR_NODE_PATH")).await?;

//     let mut listener = api.subscribe().await?; // Subscribing for events.

//     // Checking that blocks still running.
//     assert!(listener.blocks_running().await?);

//     let init_nft = InitNFT {
//         name: String::from("MyToken"),
//         symbol: String::from("MTK"),
//         base_uri: String::from(""),
//     }
//     .encode();
//     let gas_info = api
//         .calculate_upload_gas(None, WASM_BINARY_OPT.to_vec(), init_nft.clone(), 0, true)
//         .await?;

//     let (message_id, program_id, _hash) = api
//         .upload_program_bytes(
//             WASM_BINARY_OPT.to_vec(),
//             gclient::now_micros().to_le_bytes(),
//             init_nft,
//             gas_info.min_limit,
//             0,
//         )
//         .await?;

//     assert!(listener.message_processed(message_id).await?.succeed());

//     let transaction_id: u64 = 0;
//     use gear_lib::non_fungible_token::token::TokenMetadata;
//     let token_metadata = TokenMetadata {
//         name: "CryptoKitty".to_string(),
//         description: "Description".to_string(),
//         media: "http://".to_string(),
//         reference: "http://".to_string(),
//     };

//     let mint_payload = NFTAction::Mint {
//         transaction_id,
//         token_metadata,
//     };

//     let gas_info = api
//         .calculate_handle_gas(None, program_id, mint_payload.encode(), 0, true)
//         .await?;

//     let (message_id, _) = api
//         .send_message(program_id, mint_payload, gas_info.min_limit, 0)
//         .await?;

//     assert!(listener.message_processed(message_id).await?.succeed());

//     assert!(listener.blocks_running().await?);

//     Ok(())
// }

// #[tokio::test]
// #[ignore]
// async fn burn_test() -> Result<()> {
//     let api = GearApi::dev_from_path(env!("GEAR_NODE_PATH")).await?;

//     let mut listener = api.subscribe().await?; // Subscribing for events.

//     // Checking that blocks still running.
//     assert!(listener.blocks_running().await?);

//     let init_nft = InitNFT {
//         name: String::from("MyToken"),
//         symbol: String::from("MTK"),
//         base_uri: String::from(""),
//     }
//     .encode();
//     let gas_info = api
//         .calculate_upload_gas(None, WASM_BINARY_OPT.to_vec(), init_nft.clone(), 0, true)
//         .await?;

//     let (message_id, program_id, _hash) = api
//         .upload_program_bytes(
//             WASM_BINARY_OPT.to_vec(),
//             gclient::now_micros().to_le_bytes(),
//             init_nft,
//             gas_info.min_limit,
//             0,
//         )
//         .await?;

//     assert!(listener.message_processed(message_id).await?.succeed());

//     let transaction_id: u64 = 0;
//     use gear_lib::non_fungible_token::token::TokenMetadata;
//     let token_metadata = TokenMetadata {
//         name: "CryptoKitty".to_string(),
//         description: "Description".to_string(),
//         media: "http://".to_string(),
//         reference: "http://".to_string(),
//     };

//     let mint_payload = NFTAction::Mint {
//         transaction_id,
//         token_metadata,
//     };

//     let gas_info = api
//         .calculate_handle_gas(None, program_id, mint_payload.encode(), 0, true)
//         .await?;

//     let (message_id, _) = api
//         .send_message(program_id, mint_payload, gas_info.min_limit, 0)
//         .await?;

//     assert!(listener.message_processed(message_id).await?.succeed());

//     assert!(listener.blocks_running().await?);

//     let transaction_id = transaction_id + 1;

//     let burn_payload = NFTAction::Burn {
//         transaction_id,
//         token_id: 0.into(),
//     };

//     let gas_info = api
//         .calculate_handle_gas(None, program_id, burn_payload.encode(), 0, true)
//         .await?;

//     let (message_id, _) = api
//         .send_message(program_id, burn_payload, gas_info.min_limit, 0)
//         .await?;

//     assert!(listener.message_processed(message_id).await?.succeed());

//     assert!(listener.blocks_running().await?);

//     // failures
//     let burn_payload = NFTAction::Burn {
//         transaction_id,
//         token_id: 666.into(),
//     };

//     let gas_info = api
//         .calculate_handle_gas(None, program_id, burn_payload.encode(), 0, true)
//         .await?;

//     let (message_id, _) = api
//         .send_message(program_id, burn_payload, gas_info.min_limit, 0)
//         .await?;

//     assert!(listener.message_processed(message_id).await?.succeed());

//     assert!(listener.blocks_running().await?);

//     Ok(())
// }

// #[tokio::test]
// #[ignore]
// async fn transfer_test() -> Result<()> {
//     let api = GearApi::dev_from_path(env!("GEAR_NODE_PATH")).await?;

//     let mut listener = api.subscribe().await?; // Subscribing for events.

//     // Checking that blocks still running.
//     assert!(listener.blocks_running().await?);

//     let init_nft = InitNFT {
//         name: String::from("MyToken"),
//         symbol: String::from("MTK"),
//         base_uri: String::from(""),
//     }
//     .encode();
//     let gas_info = api
//         .calculate_upload_gas(None, WASM_BINARY_OPT.to_vec(), init_nft.clone(), 0, true)
//         .await?;

//     let (message_id, program_id, _hash) = api
//         .upload_program_bytes(
//             WASM_BINARY_OPT.to_vec(),
//             gclient::now_micros().to_le_bytes(),
//             init_nft,
//             gas_info.min_limit,
//             0,
//         )
//         .await?;

//     assert!(listener.message_processed(message_id).await?.succeed());

//     let transaction_id: u64 = 0;
//     use gear_lib::non_fungible_token::token::TokenMetadata;
//     let token_metadata = TokenMetadata {
//         name: "CryptoKitty".to_string(),
//         description: "Description".to_string(),
//         media: "http://".to_string(),
//         reference: "http://".to_string(),
//     };

//     let mint_payload = NFTAction::Mint {
//         transaction_id,
//         token_metadata,
//     };

//     let gas_info = api
//         .calculate_handle_gas(None, program_id, mint_payload.encode(), 0, true)
//         .await?;

//     let (message_id, _) = api
//         .send_message(program_id, mint_payload, gas_info.min_limit, 0)
//         .await?;

//     assert!(listener.message_processed(message_id).await?.succeed());

//     assert!(listener.blocks_running().await?);

//     let transaction_id = transaction_id + 1;

//     let transfer_payload = NFTAction::Transfer {
//         transaction_id,
//         to: ActorId::from(4u64),
//         token_id: 0.into(),
//     };

//     let gas_info = api
//         .calculate_handle_gas(None, program_id, transfer_payload.encode(), 0, true)
//         .await?;

//     let (message_id, _) = api
//         .send_message(program_id, transfer_payload, gas_info.min_limit, 0)
//         .await?;

//     assert!(listener.message_processed(message_id).await?.succeed());

//     assert!(listener.blocks_running().await?);

//     Ok(())
// }

// #[tokio::test]
// #[ignore]
// async fn owner_test() -> Result<()> {
//     let api = GearApi::dev_from_path(env!("GEAR_NODE_PATH")).await?;

//     let mut listener = api.subscribe().await?; // Subscribing for events.

//     // Checking that blocks still running.
//     assert!(listener.blocks_running().await?);

//     let init_nft = InitNFT {
//         name: String::from("MyToken"),
//         symbol: String::from("MTK"),
//         base_uri: String::from(""),
//     }
//     .encode();
//     let gas_info = api
//         .calculate_upload_gas(None, WASM_BINARY_OPT.to_vec(), init_nft.clone(), 0, true)
//         .await?;

//     let (message_id, program_id, _hash) = api
//         .upload_program_bytes(
//             WASM_BINARY_OPT.to_vec(),
//             gclient::now_micros().to_le_bytes(),
//             init_nft,
//             gas_info.min_limit,
//             0,
//         )
//         .await?;

//     assert!(listener.message_processed(message_id).await?.succeed());

//     let transaction_id: u64 = 0;
//     use gear_lib::non_fungible_token::token::TokenMetadata;
//     let token_metadata = TokenMetadata {
//         name: "CryptoKitty".to_string(),
//         description: "Description".to_string(),
//         media: "http://".to_string(),
//         reference: "http://".to_string(),
//     };

//     let mint_payload = NFTAction::Mint {
//         transaction_id,
//         token_metadata,
//     };

//     let gas_info = api
//         .calculate_handle_gas(None, program_id, mint_payload.encode(), 0, true)
//         .await?;

//     let (message_id, _) = api
//         .send_message(program_id, mint_payload, gas_info.min_limit, 0)
//         .await?;

//     assert!(listener.message_processed(message_id).await?.succeed());

//     assert!(listener.blocks_running().await?);

//     let owner_payload = NFTAction::Owner { token_id: 0.into() };

//     let gas_info = api
//         .calculate_handle_gas(None, program_id, owner_payload.encode(), 0, true)
//         .await?;

//     let (message_id, _) = api
//         .send_message(program_id, owner_payload, gas_info.min_limit, 0)
//         .await?;

//     assert!(listener.message_processed(message_id).await?.succeed());

//     assert!(listener.blocks_running().await?);

//     Ok(())
// }

// #[tokio::test]
// #[ignore]
// async fn approved() -> Result<()> {
//     let api = GearApi::dev_from_path(env!("GEAR_NODE_PATH")).await?;

//     let mut listener = api.subscribe().await?; // Subscribing for events.

//     // Checking that blocks still running.
//     assert!(listener.blocks_running().await?);

//     let init_nft = InitNFT {
//         name: String::from("MyToken"),
//         symbol: String::from("MTK"),
//         base_uri: String::from(""),
//     }
//     .encode();
//     let gas_info = api
//         .calculate_upload_gas(None, WASM_BINARY_OPT.to_vec(), init_nft.clone(), 0, true)
//         .await?;

//     let (message_id, program_id, _hash) = api
//         .upload_program_bytes(
//             WASM_BINARY_OPT.to_vec(),
//             gclient::now_micros().to_le_bytes(),
//             init_nft,
//             gas_info.min_limit,
//             0,
//         )
//         .await?;

//     assert!(listener.message_processed(message_id).await?.succeed());

//     let transaction_id: u64 = 0;
//     use gear_lib::non_fungible_token::token::TokenMetadata;
//     let token_metadata = TokenMetadata {
//         name: "CryptoKitty".to_string(),
//         description: "Description".to_string(),
//         media: "http://".to_string(),
//         reference: "http://".to_string(),
//     };

//     let mint_payload = NFTAction::Mint {
//         transaction_id,
//         token_metadata,
//     };

//     let gas_info = api
//         .calculate_handle_gas(None, program_id, mint_payload.encode(), 0, true)
//         .await?;

//     let (message_id, _) = api
//         .send_message(program_id, mint_payload, gas_info.min_limit, 0)
//         .await?;

//     assert!(listener.message_processed(message_id).await?.succeed());

//     assert!(listener.blocks_running().await?);

//     let transaction_id = transaction_id + 1;
//     let approve_payload = NFTAction::Approve {
//         transaction_id,
//         to: ActorId::from(3),
//         token_id: 0.into(),
//     };

//     let gas_info = api
//         .calculate_handle_gas(None, program_id, approve_payload.encode(), 0, true)
//         .await?;

//     let (message_id, _) = api
//         .send_message(program_id, approve_payload, gas_info.min_limit, 0)
//         .await?;

//     let processed = listener.message_processed(message_id).await?;
//     assert!(processed.succeed());

//     assert!(listener.blocks_running().await?);

//     Ok(())
// }

#[tokio::test]
#[ignore]
async fn auto_changed() -> Result<()> {
    // let api = GearApi::dev_from_path(env!("GEAR_NODE_PATH")).await?;
    let api = GearApi::dev().await?;
    let mut listener = api.subscribe().await?; // Subscribing for events.

    // Checking that blocks still running.
    assert!(listener.blocks_running().await?);

    let init_nft = InitNFT {
        name: String::from("MyToken"),
        symbol: String::from("MTK"),
        base_uri: String::from(""),
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

    // Add auto-changed media
    let token_id = TokenId::default();
    let links = vec!["link 1", "link 2", "link 3", "link 4"];
    for link in links.iter() {
        let payload = NFTAction::AddMedia {
            token_id,
            media: link.to_string(),
        };

        let gas_info = api
            .calculate_handle_gas(None, program_id, payload.encode(), 0, true)
            .await?;
        let (message_id, _) = api
            .send_message(program_id, payload, gas_info.min_limit, 0)
            .await?;
        assert!(listener.message_processed(message_id).await?.succeed());
        assert!(listener.blocks_running().await?);
    }

    let update_period = 5;
    // Start auto changing
    let payload = NFTAction::StartAutoChanging {
        token_ids: vec![token_id],
        updates_count: 8,
        update_period,
    };
    let (message_id, _) = api
        .send_message(program_id, payload, 250_000_000_000, 0)
        .await?;
    assert!(listener.message_processed(message_id).await?.succeed());
    assert!(listener.blocks_running().await?);

    // Start update
    assert_eq!(
        current_media(&api, program_id.into_bytes(), token_id).await,
        links[3]
    );

    std::thread::sleep(std::time::Duration::from_secs(5));
    assert_eq!(
        current_media(&api, program_id.into_bytes(), token_id).await,
        links[2]
    );

    std::thread::sleep(std::time::Duration::from_secs(5));
    assert_eq!(
        current_media(&api, program_id.into_bytes(), token_id).await,
        links[1]
    );
    std::thread::sleep(std::time::Duration::from_secs(5));
    assert_eq!(
        dbg!(current_media(&api, program_id.into_bytes(), token_id).await),
        dbg!(links[0])
    );

    // Media rotation happens
    std::thread::sleep(std::time::Duration::from_secs(5));
    assert_eq!(
        current_media(&api, program_id.into_bytes(), token_id).await,
        links[3]
    );

    std::thread::sleep(std::time::Duration::from_secs(5));
    assert_eq!(
        current_media(&api, program_id.into_bytes(), token_id).await,
        links[2]
    );

    std::thread::sleep(std::time::Duration::from_secs(5));
    assert_eq!(
        current_media(&api, program_id.into_bytes(), token_id).await,
        links[1]
    );

    std::thread::sleep(std::time::Duration::from_secs(5));
    assert_eq!(
        current_media(&api, program_id.into_bytes(), token_id).await,
        links[0]
    );
    
    Ok(())
}

pub async fn current_media(api: &GearApi, program_id: [u8; 32], token_id: TokenId) -> String {
    let state: IoNFT = api.read_state(program_id.into()).await.unwrap();
    let (_token_id, metadata) = state
        .token
        .token_metadata_by_id
        .iter()
        .find(|(id, _meta)| token_id.eq(id))
        .unwrap();
    match metadata {
        Some(metadata) => metadata.media.clone(),
        None => unreachable!(),
    }
}
