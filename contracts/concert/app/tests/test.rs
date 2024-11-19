use concert::{
    traits::{Concert, ConcertFactory},
    Concert as ConcertClient, ConcertFactory as Factory, TokenMetadata,
};
use sails_rs::gtest::{calls::*, System};
use sails_rs::{calls::*, gtest::Program, ActorId, Decode, Encode, U256};

pub const USER_ID: u64 = 10;
pub const TOKEN_ID: U256 = U256::one();
pub const CONCERT_ID: U256 = U256::zero();
pub const AMOUNT: U256 = U256::one();
pub const DATE: u128 = 100000;

fn init_multitoken(sys: &System) -> (ActorId, Program<'_>) {
    let vmt = Program::from_file(
        sys,
        "../../target/wasm32-unknown-unknown/release/extended_vmt.opt.wasm",
    );
    let payload = ("Name".to_string(), "Symbol".to_string(), 10_u8);
    let encoded_request = ["New".encode(), payload.encode()].concat();
    let mid = vmt.send_bytes(USER_ID, encoded_request);
    let res = sys.run_next_block();
    assert!(res.succeed.contains(&mid));

    (vmt.id(), vmt)
}
fn grant_roles(sys: &System, vmt: &Program, concert_id: ActorId) {
    let encoded_request = [
        "Vmt".encode(),
        "GrantMinterRole".encode(),
        (concert_id).encode(),
    ]
    .concat();
    let mid = vmt.send_bytes(USER_ID, encoded_request);
    let res = sys.run_next_block();
    assert!(res.succeed.contains(&mid));

    let encoded_request = [
        "Vmt".encode(),
        "GrantBurnerRole".encode(),
        (concert_id).encode(),
    ]
    .concat();
    let mid = vmt.send_bytes(USER_ID, encoded_request);
    let res = sys.run_next_block();
    assert!(res.succeed.contains(&mid));
}

fn get_balance(sys: &System, vmt: &Program, account: ActorId, id: U256) -> U256 {
    let encoded_request = ["Vmt".encode(), "BalanceOf".encode(), (account, id).encode()].concat();
    let mid = vmt.send_bytes(USER_ID, encoded_request);
    let res = sys.run_next_block();
    assert!(res.succeed.contains(&mid));

    let (_, _, balance) = <(String, String, U256)>::decode(&mut res.log[0].payload())
        .expect("Unable to decode reply");
    balance
}
#[tokio::test]
async fn create_concert() {
    let system = System::new();
    system.init_logger();
    system.mint_to(USER_ID, 100_000_000_000_000);
    let program_space = GTestRemoting::new(system, USER_ID.into());
    let code_id = program_space
        .system()
        .submit_code_file("../../target/wasm32-unknown-unknown/release/concert.opt.wasm");

    let concert_factory = Factory::new(program_space.clone());
    let (vmt_id, _vmt_program) = init_multitoken(program_space.system());
    let concert_id = concert_factory
        .new(USER_ID.into(), vmt_id)
        .send_recv(code_id, "123")
        .await
        .unwrap();

    let mut client = ConcertClient::new(program_space);
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
        .send_recv(concert_id)
        .await
        .unwrap();
    // check state
    let state = client.get_storage().recv(concert_id).await.unwrap();

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
    system.init_logger();
    system.mint_to(USER_ID, 100_000_000_000_000);
    let program_space = GTestRemoting::new(system, USER_ID.into());
    let code_id = program_space
        .system()
        .submit_code_file("../../target/wasm32-unknown-unknown/release/concert.opt.wasm");

    let concert_factory = Factory::new(program_space.clone());
    let (vmt_id, vmt_program) = init_multitoken(program_space.system());
    let concert_id = concert_factory
        .new(USER_ID.into(), vmt_id)
        .send_recv(code_id, "123")
        .await
        .unwrap();
    grant_roles(program_space.system(), &vmt_program, concert_id);
    let mut client = ConcertClient::new(program_space.clone());
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
        .send_recv(concert_id)
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
    client
        .buy_tickets(AMOUNT, metadata)
        .send_recv(concert_id)
        .await
        .unwrap();

    // check state
    let state = client.get_storage().recv(concert_id).await.unwrap();

    assert_eq!(state.buyers, vec![USER_ID.into()]);
    assert_eq!(state.tickets_left, U256::from(99));
    assert_eq!(state.metadata[0].0, USER_ID.into());
    let balance = get_balance(
        program_space.system(),
        &vmt_program,
        USER_ID.into(),
        TOKEN_ID,
    );
    assert_eq!(balance, 1.into());
}

#[tokio::test]
async fn hold_concert() {
    let system = System::new();
    system.init_logger();
    system.mint_to(USER_ID, 100_000_000_000_000);
    let program_space = GTestRemoting::new(system, USER_ID.into());
    let code_id = program_space
        .system()
        .submit_code_file("../../target/wasm32-unknown-unknown/release/concert.opt.wasm");

    let concert_factory = Factory::new(program_space.clone());
    let (vmt_id, vmt_program) = init_multitoken(program_space.system());
    let concert_id = concert_factory
        .new(USER_ID.into(), vmt_id)
        .send_recv(code_id, "123")
        .await
        .unwrap();
    grant_roles(program_space.system(), &vmt_program, concert_id);
    let mut client = ConcertClient::new(program_space.clone());
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
        .send_recv(concert_id)
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
    client
        .buy_tickets(AMOUNT, metadata)
        .send_recv(concert_id)
        .await
        .unwrap();

    // hold concert
    client.hold_concert().send_recv(concert_id).await.unwrap();

    // check state
    let state = client.get_storage().recv(concert_id).await.unwrap();

    assert!(!state.running);
    let balance = get_balance(
        program_space.system(),
        &vmt_program,
        USER_ID.into(),
        TOKEN_ID,
    );
    assert_eq!(balance, 0.into());
    let balance = get_balance(
        program_space.system(),
        &vmt_program,
        USER_ID.into(),
        TOKEN_ID + 1,
    );
    assert_eq!(balance, 1.into());
}
