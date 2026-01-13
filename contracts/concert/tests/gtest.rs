use concert_client::concert::Concert;
use concert_client::ConcertCtors;
use concert_client::{Concert as ConcertClient, TokenMetadata};
use extended_vmt_client::vmt::Vmt;
use extended_vmt_client::ExtendedVmtClient;
use extended_vmt_client::ExtendedVmtClientCtors;
use sails_rs::client::*;
use sails_rs::gtest::constants::DEFAULT_USERS_INITIAL_BALANCE;
use sails_rs::{gtest::System, ActorId, U256};

pub const USER_ID: u64 = 10;
pub const TOKEN_ID: U256 = U256::one();
pub const CONCERT_ID: U256 = U256::zero();
pub const AMOUNT: U256 = U256::one();
pub const DATE: u128 = 100000;

#[tokio::test]
async fn create_concert() {
    let system = System::new();
    system.init_logger_with_default_filter("gwasm=debug,gtest=info,sails_rs=debug");
    system.mint_to(USER_ID, DEFAULT_USERS_INITIAL_BALANCE);

    let program_code_id = system.submit_code(concert::WASM_BINARY);
    let env = GtestEnv::new(system, USER_ID.into());

    let vmt_code_id = env
        .system()
        .submit_code_file("../target/wasm32-gear/release/extended_vmt.opt.wasm");

    let vmt_program = env
        .deploy::<extended_vmt_client::ExtendedVmtClientProgram>(vmt_code_id, b"salt-vmt".to_vec())
        .new("Name".to_string(), "Symbol".to_string(), 10_u8)
        .await
        .unwrap();

    let vmt_id: ActorId = vmt_program.id();

    let program = env
        .deploy::<concert_client::ConcertProgram>(program_code_id, b"salt".to_vec())
        .new(USER_ID.into(), vmt_id)
        .await
        .unwrap();

    let mut client = program.concert();
    // create
    client
        .create(
            USER_ID.into(),
            String::from("Sum 41"),
            String::from("Sum 41 concert in Madrid. 26/08/2022"),
            U256::from(100),
            DATE,
            TOKEN_ID,
        )
        .await
        .unwrap();
    // check state
    let state = client.get_storage().await.unwrap();

    assert_eq!(state.name, "Sum 41".to_string());
    assert_eq!(
        state.description,
        "Sum 41 concert in Madrid. 26/08/2022".to_string()
    );
    assert_eq!(state.date, DATE);
    assert_eq!(state.tickets_left, U256::from(100));
}

#[tokio::test]
async fn buy_tickets() {
    let system = System::new();
    system.init_logger_with_default_filter("gwasm=debug,gtest=info,sails_rs=debug");
    system.mint_to(USER_ID, DEFAULT_USERS_INITIAL_BALANCE);

    let concert_code_id = system.submit_code(concert::WASM_BINARY);

    let env = GtestEnv::new(system, USER_ID.into());

    let vmt_code_id = env
        .system()
        .submit_code_file("../target/wasm32-gear/release/extended_vmt.opt.wasm");

    let vmt_program = env
        .deploy::<extended_vmt_client::ExtendedVmtClientProgram>(vmt_code_id, b"salt-vmt".to_vec())
        .new("Name".to_string(), "Symbol".to_string(), 10_u8)
        .await
        .unwrap();

    let vmt_id: ActorId = vmt_program.id();

    let concert_program = env
        .deploy::<concert_client::ConcertProgram>(concert_code_id, b"salt-concert".to_vec())
        .new(USER_ID.into(), vmt_id)
        .await
        .unwrap();

    let concert_id: ActorId = concert_program.id();

    let mut vmt = vmt_program.vmt();
    vmt.grant_minter_role(concert_id).await.unwrap();
    vmt.grant_burner_role(concert_id).await.unwrap();

    let mut concert = concert_program.concert();

    concert
        .create(
            USER_ID.into(),
            "Sum 41".to_string(),
            "Sum 41 concert in Madrid. 26/08/2022".to_string(),
            U256::from(100),
            DATE,
            TOKEN_ID,
        )
        .await
        .unwrap();

    let metadata = vec![Some(TokenMetadata {
        title: Some("Sum 41 concert in Madrid 26/08/2022".to_string()),
        description: Some("Sum 41 Madrid 26/08/2022 Ticket. Row 4. Seat 4.".to_string()),
        media: Some("sum41.com".to_string()),
        reference: Some("UNKNOWN".to_string()),
    })];

    concert.buy_tickets(AMOUNT, metadata).await.unwrap();

    let state = concert.get_storage().await.unwrap();
    assert_eq!(state.buyers, vec![USER_ID.into()]);
    assert_eq!(state.tickets_left, U256::from(99));
    assert_eq!(state.metadata[0].0, USER_ID.into());

    let balance = vmt.balance_of(USER_ID.into(), TOKEN_ID).await.unwrap();
    assert_eq!(balance, 1.into());
}

#[tokio::test]
async fn hold_concert() {
    let system = System::new();
    system.init_logger_with_default_filter("gwasm=debug,gtest=info,sails_rs=debug");
    system.mint_to(USER_ID, DEFAULT_USERS_INITIAL_BALANCE);

    let concert_code_id = system.submit_code(concert::WASM_BINARY);

    let env = GtestEnv::new(system, USER_ID.into());

    let vmt_code_id = env
        .system()
        .submit_code_file("../target/wasm32-gear/release/extended_vmt.opt.wasm");

    let vmt_program = env
        .deploy::<extended_vmt_client::ExtendedVmtClientProgram>(vmt_code_id, b"salt-vmt".to_vec())
        .new("Name".to_string(), "Symbol".to_string(), 10_u8)
        .await
        .unwrap();

    let vmt_id: ActorId = vmt_program.id();

    let concert_program = env
        .deploy::<concert_client::ConcertProgram>(concert_code_id, b"salt-concert".to_vec())
        .new(USER_ID.into(), vmt_id)
        .await
        .unwrap();

    let concert_id: ActorId = concert_program.id();

    let mut vmt = vmt_program.vmt();
    vmt.grant_minter_role(concert_id).await.unwrap();
    vmt.grant_burner_role(concert_id).await.unwrap();

    let mut concert = concert_program.concert();
    // create
    concert
        .create(
            USER_ID.into(),
            String::from("Sum 41"),
            String::from("Sum 41 concert in Madrid. 26/08/2022"),
            U256::from(100),
            DATE,
            TOKEN_ID,
        )
        .await
        .unwrap();

    let metadata = vec![Some(TokenMetadata {
        title: Some(String::from("Sum 41 concert in Madrid 26/08/2022")),
        description: Some(String::from(
            "Sum 41 Madrid 26/08/2022 Ticket. Row 4. Seat 4.",
        )),
        media: Some(String::from("sum41.com")),
        reference: Some(String::from("UNKNOWN")),
    })];
    // buy tickets
    concert.buy_tickets(AMOUNT, metadata).await.unwrap();

    // hold concert
    concert.hold_concert().await.unwrap();

    // check state
    let state = concert.get_storage().await.unwrap();

    assert!(!state.running);
    let balance = vmt_program
        .vmt()
        .balance_of(USER_ID.into(), TOKEN_ID)
        .await
        .unwrap();
    assert_eq!(balance, 0.into());
    let balance = vmt_program
        .vmt()
        .balance_of(USER_ID.into(), TOKEN_ID + 1)
        .await
        .unwrap();
    assert_eq!(balance, 1.into());
}
