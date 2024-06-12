use battleship_io::*;
use gclient::{GearApi, Result, WSAddress};
use gear_core::ids::ProgramId;
use gstd::{prelude::*, ActorId};
use std::fs::read_to_string;
pub const GAME_ID: &str = "0fb8a4905b3973c2e93f97e2d36241a8f924e13a40b089c97883038a29329150";

pub trait ApiUtils {
    fn get_actor_id(&self) -> ActorId;
    fn get_specific_actor_id(&self, value: impl AsRef<str>) -> ActorId;
}

impl ApiUtils for GearApi {
    fn get_actor_id(&self) -> ActorId {
        ActorId::new(
            self.account_id()
                .encode()
                .try_into()
                .expect("Unexpected invalid account id length."),
        )
    }

    fn get_specific_actor_id(&self, value: impl AsRef<str>) -> ActorId {
        let api_temp = self
            .clone()
            .with(value)
            .expect("Unable to build `GearApi` instance with provided signer.");
        api_temp.get_actor_id()
    }
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

#[tokio::test]
#[ignore]
async fn transfer_balances() -> Result<()> {
    let users = read_lines("./participants_new");

    let n = 4;
    let i = 0;
    let res = tokio::join!(
        transfer_balances_to_account(&users[0..n], (i as u32) * 25),
        transfer_balances_to_account(&users[n..2 * n], 1 + (i as u32) * 25),
        transfer_balances_to_account(&users[2 * n..3 * n], 2 + (i as u32) * 25),
        transfer_balances_to_account(&users[3 * n..4 * n], 3 + (i as u32) * 25),
        transfer_balances_to_account(&users[4 * n..5 * n], 4 + (i as u32) * 25),
        transfer_balances_to_account(&users[5 * n..6 * n], 5 + (i as u32) * 25),
        transfer_balances_to_account(&users[6 * n..7 * n], 6 + (i as u32) * 25),
        transfer_balances_to_account(&users[7 * n..8 * n], 7 + (i as u32) * 25),
        transfer_balances_to_account(&users[8 * n..9 * n], 8 + (i as u32) * 25),
        transfer_balances_to_account(&users[9 * n..10 * n], 9 + (i as u32) * 25),
        transfer_balances_to_account(&users[10 * n..11 * n], 10 + (i as u32) * 25),
        transfer_balances_to_account(&users[11 * n..12 * n], 11 + (i as u32) * 25),
        transfer_balances_to_account(&users[12 * n..13 * n], 12 + (i as u32) * 25),
        transfer_balances_to_account(&users[13 * n..14 * n], 13 + (i as u32) * 25),
        transfer_balances_to_account(&users[14 * n..15 * n], 14 + (i as u32) * 25),
        transfer_balances_to_account(&users[15 * n..16 * n], 15 + (i as u32) * 25),
        transfer_balances_to_account(&users[16 * n..17 * n], 16 + (i as u32) * 25),
        transfer_balances_to_account(&users[17 * n..18 * n], 17 + (i as u32) * 25),
        transfer_balances_to_account(&users[18 * n..19 * n], 18 + (i as u32) * 25),
        transfer_balances_to_account(&users[19 * n..20 * n], 19 + (i as u32) * 25),
        transfer_balances_to_account(&users[20 * n..21 * n], 20 + (i as u32) * 25),
        transfer_balances_to_account(&users[21 * n..22 * n], 21 + (i as u32) * 25),
        transfer_balances_to_account(&users[22 * n..23 * n], 22 + (i as u32) * 25),
        transfer_balances_to_account(&users[23 * n..24 * n], 23 + (i as u32) * 25),
        transfer_balances_to_account(&users[24 * n..25 * n], 24 + (i as u32) * 25),
    );
    if let Err(error) = res.0 {
        println!("{:?}", error);
    }
    Ok(())
}

async fn transfer_balances_to_account(accounts: &[String], nonce: u32) -> Result<()> {
    // let mut api = GearApi::dev().await?;
    let mut api = GearApi::init(WSAddress::new("wss://testnet.vara.rs", 443)).await?;
    let accounts = accounts.to_vec();
    for account in accounts.iter() {
        let account = api.get_specific_actor_id(account);

        let account: [u8; 32] = account.into();
        let account: ProgramId = account.into();
        let rpc_nonce = api.rpc_nonce().await?;
        println!("RPC NONCE {:?}", rpc_nonce);
        api.set_nonce(rpc_nonce + nonce as u64);
        api.transfer_keep_alive(account, 1_000_000_000_000).await?;
        println!("Transferred");
        println!("{:?}", api.total_balance(account).await.expect(""));
    }

    Ok(())
}

#[tokio::test]
#[ignore]
async fn start_games() -> Result<()> {
    let game_pid = hex::decode(GAME_ID).unwrap();
    let game_pid = ProgramId::decode(&mut game_pid.as_slice()).unwrap();
    // let api = GearApi::dev_from_path(env!("GEAR_NODE_PATH")).await?;
    // let api = GearApi::init(WSAddress::new("wss://testnet.vara.rs", 443)).await?;

    let users = read_lines("./accounts_1k_3.txt");
    let n = 40;
    let res = tokio::join!(
        start_game_from_account(game_pid, &users[0..n]),
        start_game_from_account(game_pid, &users[n..2 * n]),
        start_game_from_account(game_pid, &users[2 * n..3 * n]),
        start_game_from_account(game_pid, &users[3 * n..4 * n]),
        start_game_from_account(game_pid, &users[4 * n..5 * n]),
        start_game_from_account(game_pid, &users[5 * n..6 * n]),
        start_game_from_account(game_pid, &users[6 * n..7 * n]),
        start_game_from_account(game_pid, &users[7 * n..8 * n]),
        start_game_from_account(game_pid, &users[8 * n..9 * n]),
        start_game_from_account(game_pid, &users[9 * n..10 * n]),
        start_game_from_account(game_pid, &users[10 * n..11 * n],),
        start_game_from_account(game_pid, &users[11 * n..12 * n],),
        start_game_from_account(game_pid, &users[12 * n..13 * n],),
        start_game_from_account(game_pid, &users[13 * n..14 * n],),
        start_game_from_account(game_pid, &users[14 * n..15 * n],),
        start_game_from_account(game_pid, &users[15 * n..16 * n],),
        start_game_from_account(game_pid, &users[16 * n..17 * n],),
        start_game_from_account(game_pid, &users[17 * n..18 * n],),
        start_game_from_account(game_pid, &users[18 * n..19 * n],),
        start_game_from_account(game_pid, &users[19 * n..20 * n],),
        start_game_from_account(game_pid, &users[20 * n..21 * n],),
        start_game_from_account(game_pid, &users[21 * n..22 * n],),
        start_game_from_account(game_pid, &users[22 * n..23 * n],),
        start_game_from_account(game_pid, &users[23 * n..24 * n],),
        start_game_from_account(game_pid, &users[24 * n..25 * n],),
    );

    if let Err(error) = res.0 {
        println!("Error {:?}", error);
    }

    Ok(())
}

async fn start_game_from_account(game_pid: ProgramId, accounts: &[String]) -> Result<()> {
    // let mut api = GearApi::init(WSAddress::new("wss://testnet.vara.rs", 443)).await?;
    let mut api = GearApi::init(WSAddress::new("wss://vit.vara-network.io", 443)).await?;
    let accounts = accounts.to_vec();
    for (i, account) in accounts.iter().enumerate() {
        api = api
            .clone()
            .with(account)
            .expect("Unable to log with indicated account");

        let _account = api.get_actor_id();

        let _account: [u8; 32] = _account.into();
        //  let hex_string = format!("{:x}", account.to_ascii_lowercase());
        let _account: ProgramId = _account.into();

        let ships = Ships {
            ship_1: vec![19],
            ship_2: vec![0, 1, 2],
            ship_3: vec![4, 9],
            ship_4: vec![16, 21],
        };

        let start_payload = BattleshipAction::StartGame {
            ships,
            session_for_account: None,
        };
        let gas_info = api
            .calculate_handle_gas(None, game_pid, start_payload.encode(), 0, true)
            .await?;
        println!("GAS INFO {:?}", gas_info);
        println!("Balance {:?}", api.total_balance(_account).await.expect(""));

        if i > 0 {
            match api.total_balance(_account).await {
                Ok(balance) => {
                    println!("i {} balance {} ", i, balance);
                    api.send_message(game_pid, start_payload, 500_000_000_000, 0)
                        .await?;
                    println!("STARTED");
                }
                Err(error) => {
                    println!("Error {:?}", error);
                }
            }
        }
    }

    Ok(())
}
