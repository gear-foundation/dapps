use gclient::{EventProcessor, GearApi, Result};
use gstd::{ActorId, Encode};
use on_chain_nft::WASM_BINARY_OPT;
use on_chain_nft_io::*;

pub const TOKEN_ADDRESS: u64 = 1;
pub const ICO_CONTRACT_ID: u64 = 2;
pub const OWNER_ID: u64 = 100001;
pub const USER_ID: u64 = 12345;

pub const ZERO_ID: ActorId = ActorId::zero();

pub const TOKENS_CNT: u128 = 100;
pub const START_PRICE: u128 = 1000;
pub const PRICE_INCREASE_STEP: u128 = 100;
pub const TIME_INCREASE_STEP: u128 = 1000;

// const USERS: &[u64] = &[1, 2, 3, 4, 5, 6, 7, 8];

#[tokio::test]
async fn gclient_init() -> Result<()> {
    let api = GearApi::dev_from_path("../target/tmp/gear").await?;

    let mut listener = api.subscribe().await?; // Subscribing for events.

    // Checking that blocks still running.
    assert!(listener.blocks_running().await?);

    let mut layers = vec![];
    let first_layer = vec![
        String::from(
        "PHN2ZyBoZWlnaHQ9JzIxMCcgd2lkdGg9JzUwMCc+PHBvbHlnb24gcG9pbnRzPScxMDAsMTAgNDAsMTk4IDE5MCw3OCAxMCw3OCAxNjAsMTk4JyBzdHlsZT0nZmlsbDpsaW1lO3N0cm9rZTpwdXJwbGU7c3Ryb2tlLXdpZHRoOjU7ZmlsbC1ydWxlOm5vbnplcm87Jy8+PC9zdmc+",
        ),
        String::from(
            "PHN2ZyBoZWlnaHQ9JzIxMCcgd2lkdGg9JzUwMCc+PHBvbHlnb24gcG9pbnRzPScxMDAsMTAgNDAsMTk4IDE5MCw3OCAxMCw3OCAxNjAsMTk4JyBzdHlsZT0nZmlsbDpibHVlO3N0cm9rZTpyZWQ7c3Ryb2tlLXdpZHRoOjU7ZmlsbC1ydWxlOm5vbnplcm87Jy8+PC9zdmc+",
        )
    ];
    let second_layer = vec![
        String::from(
            "PHN2ZyBoZWlnaHQ9JzMwJyB3aWR0aD0nMjAwJz48dGV4dCB4PScwJyB5PScxNScgZmlsbD0ncmVkJz5PbiBDaGFpbiBORlQ8L3RleHQ+PC9zdmc+"
        ),
        String::from(
            "PHN2ZyBoZWlnaHQ9JzMwJyB3aWR0aD0nMjAwJz48dGV4dCB4PScwJyB5PScxNScgZmlsbD0nZ3JlZW4nPk9uIENoYWluIE5GVDwvdGV4dD48L3N2Zz4="
        )
    ];
    layers.push((0, first_layer));
    layers.push((1, second_layer));

    let init = InitOnChainNFT {
        name: String::from("OnChainToken"),
        symbol: String::from("OCT"),
        base_uri: String::from(""),
        royalties: None,
        base_image: String::from("<svg height='100' width='100'><circle cx='50' cy='50' r='40' stroke='black' stroke-width='3' fill='red' /></svg>"),
        layers,
    };

    let init_payload = init.encode();

    let gas_info = api
        .calculate_upload_gas(
            None,
            WASM_BINARY_OPT.to_vec(),
            init_payload.clone(),
            0,
            true,
        )
        .await?;

    let (message_id, _program_id, _hash) = api
        .upload_program_bytes(
            WASM_BINARY_OPT.to_vec(),
            gclient::now_micros().to_le_bytes(),
            init_payload,
            gas_info.burned * 2,
            0,
        )
        .await?;

    assert!(listener.message_processed(message_id).await?.succeed());

    Ok(())
}
