use auction_io::auction::*;
use dutch_auction::WASM_BINARY_OPT;
use gclient::{EventProcessor, GearApi, Result};
use gear_lib::non_fungible_token::token::TokenMetadata;
use gstd::prelude::*;
use gstd::{ActorId, Encode};
use nft_io::*;

const NFT_PATH: &str = "target/nft-0.2.10.opt.wasm";

#[tokio::test]
#[ignore]
async fn create_and_stop() -> Result<()> {
    let api = GearApi::dev().await?;

    let mut listener = api.subscribe().await?; // Subscribing for events.

    let init_nft = InitNFT {
        name: String::from("MyToken"),
        symbol: String::from("MTK"),
        base_uri: String::from(""),
        royalties: None,
    }
    .encode();

    let gas_info = api
        .calculate_upload_gas(
            None,
            gclient::code_from_os(NFT_PATH)?,
            init_nft.clone(),
            0,
            true,
        )
        .await?;

    let (message_id, program_id, _hash) = api
        .upload_program_bytes_by_path(
            NFT_PATH,
            gclient::now_micros().to_le_bytes(),
            init_nft,
            gas_info.min_limit,
            0,
        )
        .await
        .unwrap();

    assert!(listener
        .message_processed(message_id)
        .await
        .unwrap()
        .succeed());

    println!("INIT DONE");

    let transaction_id: u64 = 0;

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

    let action = Action::Create(CreateConfig {
        nft_contract_actor_id: ActorId::from(2),
        starting_price: 1_000_000_000,
        discount_rate: 1_000,
        token_id: 0.into(),
        duration: Duration {
            hours: 168,
            minutes: 0,
            seconds: 0,
        },
    });

    let action_payload = action.encode();

    let gas_info = api
        .calculate_upload_gas(
            None,
            WASM_BINARY_OPT.into(),
            action_payload.clone(),
            0,
            true,
        )
        .await?;

    let (message_id, program_id, _hash) = api
        .upload_program(
            WASM_BINARY_OPT.to_vec(),
            gclient::now_micros().to_le_bytes(),
            action_payload.clone(),
            gas_info.min_limit,
            0,
        )
        .await?;

    let (_message_id, _) = api
        .send_message(program_id, action_payload, gas_info.min_limit, 0)
        .await?;

    assert!(listener.message_processed(message_id).await?.succeed());

    // Checking that blocks still running.
    assert!(listener.blocks_running().await?);

    Ok(())
}
