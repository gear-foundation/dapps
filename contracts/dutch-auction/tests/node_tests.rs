use auction_io::auction::*;
use dutch_auction::WASM_BINARY_OPT;
use gclient::{EventProcessor, GearApi, Result};
use gear_lib::non_fungible_token::token::{TokenId, TokenMetadata};
use gstd::prelude::*;
use gstd::{ActorId, Encode};
use nft_io::*;

const NFT_PATH: &str = "target/wasm32-unknown-unknown/debug/nft.opt.wasm";
pub const ALICE: [u8; 32] = [
    212, 53, 147, 199, 21, 253, 211, 28, 97, 20, 26, 189, 4, 169, 159, 214, 130, 44, 133, 88, 133,
    76, 205, 227, 154, 86, 132, 231, 165, 109, 162, 125,
];

#[tokio::test]
#[ignore]
async fn create_and_stop() -> Result<()> {
    let api = GearApi::dev_from_path(env!("GEAR_NODE_PATH")).await?;
    // let api = GearApi::dev().await?;
    let mut listener = api.subscribe().await?; // Subscribing for events.

    // Init NFT
    let init_nft = InitNFT {
        royalties: None,
        collection: Default::default(),
        constraints: Constraints {
            authorized_minters: vec![ALICE.into()],
            ..Default::default()
        },
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
    let (message_id, nft_program_id, _hash) = api
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

    // Mint nft
    let mut transaction_id: u64 = 0;
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
        .calculate_handle_gas(None, nft_program_id, mint_payload.encode(), 0, true)
        .await?;
    let (message_id, _) = api
        .send_message(nft_program_id, mint_payload, gas_info.min_limit, 0)
        .await?;
    assert!(listener.message_processed(message_id).await?.succeed());

    // Upload And Init Auction
    let payload: Vec<u8> = vec![];
    let gas_info = api
        .calculate_upload_gas(None, WASM_BINARY_OPT.into(), vec![], 0, true)
        .await?;
    let (message_id, auction_program_id, _hash) = api
        .upload_program(
            WASM_BINARY_OPT.to_vec(),
            gclient::now_micros().to_le_bytes(),
            payload,
            gas_info.min_limit,
            0,
        )
        .await?;
    assert!(listener.message_processed(message_id).await?.succeed());

    // Approve NFT to auction
    let to = ActorId::from_slice(&auction_program_id.into_bytes()).unwrap();
    println!("INIT DONE. Auction_contract_id: {:?}", to);

    transaction_id += 1;
    let approve_action = NFTAction::Approve {
        transaction_id,
        to,
        token_id: TokenId::default(),
    };
    let gas_info = api
        .calculate_handle_gas(None, nft_program_id, approve_action.encode(), 0, true)
        .await?;
    let (message_id, _hash) = api
        .send_message(nft_program_id, approve_action, gas_info.min_limit, 0)
        .await?;

    // Create Auction
    let starting_price = 1_000_000_000;
    let discount_rate = 2_000_000;
    let nft_contract_actor_id = ActorId::from_slice(&nft_program_id.into_bytes()).unwrap();
    println!(
        "Approve DONE. nft_contract_actor_id: {:?}",
        nft_contract_actor_id
    );
    let create = Action::Create(CreateConfig {
        nft_contract_actor_id,
        starting_price,
        discount_rate,
        token_id: TokenId::default(),
        duration: Duration {
            hours: 0,
            minutes: 5,
            seconds: 0,
        },
    });
    let gas_info = api
        .calculate_handle_gas(None, auction_program_id, create.encode(), 0, true)
        .await?;
    let (_message_id, _) = api
        .send_message(auction_program_id, create, gas_info.min_limit, 0)
        .await?;

    assert!(listener.message_processed(message_id).await?.succeed());
    assert!(listener.blocks_running().await?);

    std::thread::sleep(std::time::Duration::from_secs(10));

    let state: AuctionInfo = api.read_state(auction_program_id).await?;
    assert!(matches!(state.status, Status::IsRunning));

    // Buy
    let buy = Action::Buy;
    let value = 1_000_000_000;

    let (message_id, _) = api
        .send_message(auction_program_id, buy, 250_000_000_000, value)
        .await?;

    assert!(listener.message_processed(message_id).await?.succeed());
    assert!(listener.blocks_running().await?);

    let state: AuctionInfo = api.read_state(auction_program_id).await?;
    assert!(matches!(state.status, Status::Purchased { price: _ }));

    // ForceStop
    let force_stop = Action::ForceStop;
    let gas_info = api
        .calculate_handle_gas(None, auction_program_id, force_stop.encode(), 0, true)
        .await?;
    let (message_id, _) = api
        .send_message(auction_program_id, force_stop, gas_info.min_limit, 0)
        .await?;

    assert!(listener.message_processed(message_id).await?.succeed());
    assert!(listener.blocks_running().await?);

    let state: AuctionInfo = api.read_state(auction_program_id).await?;
    assert!(matches!(state.status, Status::Purchased { price: _ }));

    Ok(())
}

#[tokio::test]
#[ignore]
async fn create_buy_reward() -> Result<()> {
    let api = GearApi::dev_from_path(env!("GEAR_NODE_PATH")).await?;
    // let api = GearApi::dev().await?;
    let mut listener = api.subscribe().await?; // Subscribing for events.

    // Init NFT
    let init_nft = InitNFT {
        royalties: None,
        collection: Default::default(),
        constraints: Constraints {
            authorized_minters: vec![ALICE.into()],
            ..Default::default()
        },
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
    let (message_id, nft_program_id, _hash) = api
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

    // Mint nft
    let mut transaction_id: u64 = 0;
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
        .calculate_handle_gas(None, nft_program_id, mint_payload.encode(), 0, true)
        .await?;
    let (message_id, _) = api
        .send_message(nft_program_id, mint_payload, gas_info.min_limit, 0)
        .await?;
    assert!(listener.message_processed(message_id).await?.succeed());

    // Upload And Init Auction
    let payload: Vec<u8> = vec![];
    let gas_info = api
        .calculate_upload_gas(None, WASM_BINARY_OPT.into(), vec![], 0, true)
        .await?;
    let (message_id, auction_program_id, _hash) = api
        .upload_program(
            WASM_BINARY_OPT.to_vec(),
            gclient::now_micros().to_le_bytes(),
            payload,
            gas_info.min_limit,
            0,
        )
        .await?;
    assert!(listener.message_processed(message_id).await?.succeed());
    // Approve NFT to auction
    let to = ActorId::from_slice(&auction_program_id.into_bytes()).unwrap();
    println!("INIT DONE. Auction_contract_id: {:?}", to);

    transaction_id += 1;
    let approve_action = NFTAction::Approve {
        transaction_id,
        to,
        token_id: TokenId::default(),
    };
    let gas_info = api
        .calculate_handle_gas(None, nft_program_id, approve_action.encode(), 0, true)
        .await?;
    let (message_id, _hash) = api
        .send_message(nft_program_id, approve_action, gas_info.min_limit, 0)
        .await?;

    // Create Auction
    let starting_price = 1_000_000_000;
    let discount_rate = 2_000_000;
    let nft_contract_actor_id = ActorId::from_slice(&nft_program_id.into_bytes()).unwrap();
    println!(
        "Approve DONE. nft_contract_actor_id: {:?}",
        nft_contract_actor_id
    );
    let create = Action::Create(CreateConfig {
        nft_contract_actor_id,
        starting_price,
        discount_rate,
        token_id: TokenId::default(),
        duration: Duration {
            hours: 0,
            minutes: 5,
            seconds: 0,
        },
    });
    let gas_info = api
        .calculate_handle_gas(None, auction_program_id, create.encode(), 0, true)
        .await?;
    let (_message_id, _) = api
        .send_message(auction_program_id, create, gas_info.min_limit, 0)
        .await?;

    assert!(listener.message_processed(message_id).await?.succeed());
    assert!(listener.blocks_running().await?);

    std::thread::sleep(std::time::Duration::from_secs(10));

    let state: AuctionInfo = api.read_state(auction_program_id).await?;
    dbg!(state);

    // Buy
    let buy = Action::Buy;
    let buy_payload = buy.encode();
    let value = 1_000_000_000;
    let gas_info = api
        .calculate_handle_gas(None, auction_program_id, buy_payload, value, true)
        .await?;
    let (message_id, _) = api
        .send_message(auction_program_id, buy, gas_info.min_limit, value)
        .await?;

    assert!(listener.message_processed(message_id).await?.succeed());
    assert!(listener.blocks_running().await?);

    let state: AuctionInfo = api.read_state(auction_program_id).await?;
    assert!(matches!(state.status, Status::Purchased { price: _ }));

    // Reward
    let reward = Action::Reward;
    let reward_payload = reward.encode();
    let gas_info = api
        .calculate_handle_gas(None, auction_program_id, reward_payload, 0, true)
        .await?;
    let (message_id, _) = api
        .send_message(auction_program_id, reward, gas_info.min_limit, 0)
        .await?;

    assert!(listener.message_processed(message_id).await?.succeed());
    assert!(listener.blocks_running().await?);

    let state: AuctionInfo = api.read_state(auction_program_id).await?;
    assert!(matches!(state.status, Status::Rewarded { price: _ }));

    Ok(())
}
