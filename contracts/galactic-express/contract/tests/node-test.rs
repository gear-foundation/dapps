use gclient::{EventListener, EventProcessor, GearApi, Result, WSAddress};

use fmt::Debug;
use gstd::{prelude::*, ActorId};
use launch_io::*;
const PATH: &str = "./target/wasm32-unknown-unknown/release/launch_site.opt.wasm";

pub const PLAYERS: &[&str] = &[
    "//John", "//Mike", "//Dan", "//Bot", "//Jack", "//Mops", "//Alex",
];

fn decode<T: Decode>(payload: Vec<u8>) -> Result<T> {
    Ok(T::decode(&mut payload.as_slice())?)
}

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

async fn common_upload_program(
    client: &GearApi,
    code: Vec<u8>,
    payload: impl Encode,
) -> Result<([u8; 32], [u8; 32])> {
    let encoded_payload = payload.encode();
    let gas_limit = client
        .calculate_upload_gas(None, code.clone(), encoded_payload, 0, true)
        .await?
        .min_limit;
    let (message_id, program_id, _) = client
        .upload_program(
            code,
            gclient::now_in_micros().to_le_bytes(),
            payload,
            gas_limit,
            0,
        )
        .await?;

    Ok((message_id.into(), program_id.into()))
}
async fn upload_program(
    client: &GearApi,
    listener: &mut EventListener,
    path: &str,
    payload: impl Encode,
) -> Result<[u8; 32]> {
    let (message_id, program_id) =
        common_upload_program(client, gclient::code_from_os(path)?, payload).await?;

    assert!(listener
        .message_processed(message_id.into())
        .await?
        .succeed());

    Ok(program_id)
}

async fn send_message<T: Decode>(
    client: &GearApi,
    listener: &mut EventListener,
    destination: [u8; 32],
    payload: impl Encode + Debug,
    increase_gas: bool,
) -> Result<Result<T, String>> {
    let encoded_payload = payload.encode();
    let destination = destination.into();

    let gas_limit = client
        .calculate_handle_gas(None, destination, encoded_payload, 0, true)
        .await?
        .min_limit;

    let modified_gas_limit = if increase_gas {
        gas_limit + (gas_limit * 50) / 100
    } else {
        gas_limit
    };

    println!("Sending a payload: `{payload:?}`.");

    let (message_id, _) = client
        .send_message(destination, payload, modified_gas_limit, 0)
        .await?;

    println!("Sending completed.");

    let (_, raw_reply, _) = listener.reply_bytes_on(message_id).await?;

    Ok(match raw_reply {
        Ok(raw_reply) => Ok(decode(raw_reply)?),
        Err(error) => Err(error),
    })
}

#[tokio::test]
async fn laucnh() -> Result<()> {
    //let address = WSAddress::new("wss://node-workshop.gear.rs", 443);
    //let client = GearApi::init_with(address, "//Alice").await?;
    let client = GearApi::dev().await?.with("//Alice")?;

    // Fund players
    let alice_balance = client.total_balance(client.account_id()).await?;
    let amount = alice_balance / 20;

    for player in PLAYERS {
        client
            .transfer(
                client
                    .get_specific_actor_id(player)
                    .encode()
                    .as_slice()
                    .try_into()
                    .expect("Unexpected invalid `ProgramId`."),
                amount,
            )
            .await?;
    }

    let mut listener = client.subscribe().await?;

    //upload contract
    let launch_id = upload_program(&client, &mut listener, PATH, "").await?;

    // start session
    let event = send_message::<Event>(
        &client,
        &mut listener,
        launch_id.into(),
        Action::StartNewSession,
        true,
    )
    .await?;

    println!(" Session started with {:#?}", event);

    // register users
    for i in 0..PLAYERS.len() {
        let client = client
            .clone()
            .with(PLAYERS[i])
            .expect("Unable to change signer.");
        let event = send_message::<Event>(
            &client,
            &mut listener,
            launch_id,
            Action::RegisterParticipant(PLAYERS[i].to_string()),
            true,
        )
        .await?;
        println!("Player registered: {:#?}", event);
    }

    let client = client
        .clone()
        .with(PLAYERS[0])
        .expect("Unable to change signer.");
    let event = send_message::<Event>(
        &client,
        &mut listener,
        launch_id,
        Action::RegisterOnLaunch {
            fuel_amount: 95,
            payload_amount: 85,
        },
        true,
    )
    .await?;

    println!("Player registered on launch: {:#?}", event);

    let client = client
        .clone()
        .with(PLAYERS[1])
        .expect("Unable to change signer.");
    let event = send_message::<Event>(
        &client,
        &mut listener,
        launch_id,
        Action::RegisterOnLaunch {
            fuel_amount: 90,
            payload_amount: 82,
        },
        true,
    )
    .await?;

    println!("Player registered on launch: {:#?}", event);

    let client = client
        .clone()
        .with(PLAYERS[2])
        .expect("Unable to change signer.");
    let event = send_message::<Event>(
        &client,
        &mut listener,
        launch_id,
        Action::RegisterOnLaunch {
            fuel_amount: 75,
            payload_amount: 65,
        },
        true,
    )
    .await?;

    println!("Player registered on launch: {:#?}", event);

    let client = client
        .clone()
        .with(PLAYERS[3])
        .expect("Unable to change signer.");
    let event = send_message::<Event>(
        &client,
        &mut listener,
        launch_id,
        Action::RegisterOnLaunch {
            fuel_amount: 88,
            payload_amount: 82,
        },
        true,
    )
    .await?;

    println!("Player registered on launch: {:#?}", event);

    let client = client
        .clone()
        .with(PLAYERS[4])
        .expect("Unable to change signer.");
    let event = send_message::<Event>(
        &client,
        &mut listener,
        launch_id,
        Action::RegisterOnLaunch {
            fuel_amount: 85,
            payload_amount: 80,
        },
        true,
    )
    .await?;

    println!("Player registered on launch: {:#?}", event);

    let client = client
        .clone()
        .with(PLAYERS[5])
        .expect("Unable to change signer.");
    let event = send_message::<Event>(
        &client,
        &mut listener,
        launch_id,
        Action::RegisterOnLaunch {
            fuel_amount: 90,
            payload_amount: 80,
        },
        true,
    )
    .await?;

    println!("Player registered on launch: {:#?}", event);

    let client = client.with("//Alice").expect("Unable to change signer");

    let event = send_message::<Event>(
        &client,
        &mut listener,
        launch_id,
        Action::ExecuteSession,
        true,
    )
    .await?;

    println!("Session executed {:#?}", event);

    let state: LaunchSite = client
        .read_state(launch_id.into())
        .await
        .expect("Unable to read state");
    println!("Final state {:#?}", state);

    Ok(())
}
