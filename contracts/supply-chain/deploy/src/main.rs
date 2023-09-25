use clap::{Arg, ArgAction, Command};
use gclient::Result;
use non_fungible_token_io::InitNFT;
use primitive_types::U256;
use sharded_fungible_token_io::InitFToken;
use supply_chain::WASM_BINARY_OPT as WASM_BINARY;
use supply_chain_deploy::*;
use supply_chain_io::*;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let matches = Command::new(env!("CARGO_PKG_NAME"))
        .arg(Arg::new("local").short('l').action(ArgAction::SetTrue))
        .arg(Arg::new("login"))
        .arg(Arg::new("full").short('f').action(ArgAction::SetTrue))
        .get_matches();

    let mut client = if matches.get_flag("local") {
        Client::local().await
    } else {
        Client::global().await
    }?;

    if let Some(login) = matches.get_one::<String>("login") {
        client = client.login(login)?
    }

    let storage_code_hash = client.upload_code(FT_STORAGE).await?;
    let ft_logic_code_hash = client.upload_code(FT_LOGIC).await?;

    let ft_actor_id = client
        .upload_program(
            FT_MAIN,
            InitFToken {
                storage_code_hash,
                ft_logic_code_hash,
            },
        )
        .await?;

    println!(">>> 0x{:x} <<<", U256::from(ft_actor_id));

    let nft_actor_id = client
        .upload_program(
            NFT_BINARY,
            InitNFT {
                royalties: Default::default(),
                collection: Default::default(),
                config: Default::default(),
            },
        )
        .await?;

    println!(">>> 0x{:x} <<<", U256::from(nft_actor_id));

    if matches.get_flag("full") {
        let (supply_chain_actor_id, reply) = client
            .upload_program_and_wait_reply::<Result<(), Error>>(
                WASM_BINARY.into(),
                Initialize {
                    producers: vec![ALICE.into()],
                    distributors: vec![ALICE.into()],
                    retailers: vec![ALICE.into()],

                    fungible_token: ft_actor_id.into(),
                    non_fungible_token: nft_actor_id.into(),
                },
            )
            .await?;
        assert_eq!(reply, Ok(()));

        println!("Initialized the main contract.");
        println!(">>> 0x{:x} <<<", U256::from(supply_chain_actor_id));
    }

    Ok(())
}
