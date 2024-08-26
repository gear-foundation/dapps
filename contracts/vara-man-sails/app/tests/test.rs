use sails_rs::calls::*;
use sails_rs::gtest::calls::*;
use vara_man_wasm::{
    traits::{VaraMan, VaraManFactory},
    Config, VaraMan as VaraManClient, VaraManFactory as Factory,
    Status, Level
};
use gtest::{Log, Program, System};
use sails_rs::ActorId;
use sails_rs::Encode;
use sails_rs::U256;

fn init_fungible_token(sys: &System, vara_man_id: ActorId) -> (ActorId, Program) {
    let vft = Program::from_file(
        sys,
        "../../target/wasm32-unknown-unknown/debug/extended_vft_wasm.opt.wasm",
    );
    let payload = ("Name".to_string(), "Symbol".to_string(), 10_u8);
    let encoded_request = ["New".encode(), payload.encode()].concat();
    let res = vft.send_bytes(100_u64, encoded_request);
    assert!(!res.main_failed());

    let encoded_request = ["Vft".encode(), "GrantMinterRole".encode(), vara_man_id.encode()].concat();
    let res = vft.send_bytes(100_u64, encoded_request);
    assert!(!res.main_failed());

    (vft.id(), vft)
}

fn ft_balance_of(program: Program, account: ActorId) {
    let encoded_request = ["Vft".encode(), "BalanceOf".encode(), account.encode()].concat();
    let res = program.send_bytes(100_u64, encoded_request);
    assert!(!res.main_failed());
    let state = &res.decoded_log::<(String, String, U256)>();
    println!("STATE {:?}", state)
}

#[tokio::test]
async fn test_play_game() {
    let program_space = GTestRemoting::new(100.into());

    let cloned_program_space = program_space.clone();
    let system = cloned_program_space.system();
    system.init_logger();

    let code_id = system
        .submit_code_file("../../target/wasm32-unknown-unknown/debug/vara_man_wasm.opt.wasm");

    let vara_man_factory = Factory::new(program_space.clone());
    let config = Config {
        one_point_in_value: 10_000_000_000_000,
        max_number_gold_coins: 2,
        max_number_silver_coins: 82,
        points_per_gold_coin_easy: 5,
        points_per_silver_coin_easy: 1,
        points_per_gold_coin_medium: 8,
        points_per_silver_coin_medium: 2,
        points_per_gold_coin_hard: 10,
        points_per_silver_coin_hard: 3,
        gas_for_finish_tournament: 10_000_000_000,
        gas_for_mint_fungible_token: 10_000_000_000,
        time_for_single_round: 180_000,
    };
    let vara_man_id = vara_man_factory
        .new(config)
        .send_recv(code_id, "123")
        .await
        .unwrap();

    let mut client = VaraManClient::new(program_space.clone());
    // change status
    client.change_status(Status::StartedWithNativeToken).send_recv(vara_man_id).await.unwrap();
    
    // check game status
    let status = client.status().recv(vara_man_id).await.unwrap();
    assert_eq!(status, Status::StartedWithNativeToken);
    system.mint_to(vara_man_id, 1_000_000_000_000_000);

    let old_balance = system.balance_of(  program_space.actor_id());
    client.finish_single_game(1, 5, Level::Easy).send_recv(vara_man_id).await.unwrap();

    let mailbox = system.get_mailbox(program_space.actor_id());

    let log = Log::builder().dest(program_space.actor_id());
    assert!(mailbox.contains(&log));
    assert!(mailbox.claim_value(log).is_ok());

    let new_balance = system.balance_of(  program_space.actor_id());
    println!("new balance: {:?}", new_balance);
    assert_eq!(new_balance - old_balance, 100_000_000_000_000);

}


#[tokio::test]
async fn test_play_game_with_fungible_token() {
    let program_space = GTestRemoting::new(100.into());

    let cloned_program_space = program_space.clone();
    let system = cloned_program_space.system();
    system.init_logger();

    let code_id = system
        .submit_code_file("../../target/wasm32-unknown-unknown/debug/vara_man_wasm.opt.wasm");

    let vara_man_factory = Factory::new(program_space.clone());
    let config = Config {
        one_point_in_value: 10_000_000_000_000,
        max_number_gold_coins: 2,
        max_number_silver_coins: 82,
        points_per_gold_coin_easy: 5,
        points_per_silver_coin_easy: 1,
        points_per_gold_coin_medium: 8,
        points_per_silver_coin_medium: 2,
        points_per_gold_coin_hard: 10,
        points_per_silver_coin_hard: 3,
        gas_for_finish_tournament: 10_000_000_000,
        gas_for_mint_fungible_token: 10_000_000_000,
        time_for_single_round: 180_000,
    };
    let vara_man_id = vara_man_factory
        .new(config)
        .send_recv(code_id, "123")
        .await
        .unwrap();

    let mut client = VaraManClient::new(program_space.clone());

    let (ft_address, ft_program) = init_fungible_token(system, vara_man_id);
    // change status
    client.change_status(Status::StartedWithFungibleToken{ft_address}).send_recv(vara_man_id).await.unwrap();
    
    // check game status
    let status = client.status().recv(vara_man_id).await.unwrap();
    assert_eq!(status, Status::StartedWithFungibleToken{ft_address});
    system.mint_to(vara_man_id, 1_000_000_000_000_000);

    let old_balance = system.balance_of(program_space.actor_id());
    client.finish_single_game(1, 5, Level::Easy).send_recv(vara_man_id).await.unwrap();

    ft_balance_of(ft_program, program_space.actor_id());

}
