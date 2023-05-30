use fmt::Debug;
use ft_main_io::{FTokenAction, FTokenEvent, InitFToken, LogicAction};
use game_of_chance::WASM_BINARY_OPT;
use game_of_chance_io::*;
use gclient::{Error as GclientError, EventListener, EventProcessor, GearApi, Result};
use gstd::prelude::*;
use primitive_types::H256;
use subxt::{
    error::{DispatchError, ModuleError, ModuleErrorData},
    Error as SubxtError,
};

const ALICE: [u8; 32] = [
    212, 53, 147, 199, 21, 253, 211, 28, 97, 20, 26, 189, 4, 169, 159, 214, 130, 44, 133, 88, 133,
    76, 205, 227, 154, 86, 132, 231, 165, 109, 162, 125,
];

fn decode<T: Decode>(payload: Vec<u8>) -> Result<T> {
    Ok(T::decode(&mut payload.as_slice())?)
}

async fn upload_code(client: &GearApi, path: &str) -> Result<H256> {
    let code_id = match client.upload_code_by_path(path).await {
        Ok((code_id, _)) => code_id.into(),
        Err(GclientError::Subxt(SubxtError::Runtime(DispatchError::Module(ModuleError {
            error_data:
                ModuleErrorData {
                    pallet_index: 14,
                    error: [6, 0, 0, 0],
                },
            ..
        })))) => sp_core_hashing::blake2_256(&gclient::code_from_os(path)?),
        Err(other_error) => return Err(other_error),
    };

    println!("Uploaded `{path}`.");

    Ok(code_id.into())
}

async fn upload_program_and_wait_reply<T: Decode>(
    client: &GearApi,
    listener: &mut EventListener,
    code: Vec<u8>,
    payload: impl Encode,
) -> Result<([u8; 32], T)> {
    let (message_id, program_id) = common_upload_program(client, code, payload).await?;
    let (_, raw_reply, _) = listener.reply_bytes_on(message_id.into()).await?;
    let reply = decode(
        raw_reply.expect("initialization failed, received an error message instead of a reply"),
    )?;

    Ok((program_id, reply))
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
    println!("Initialized `{path}`.");

    Ok(program_id)
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
            gclient::now_micros().to_le_bytes(),
            payload,
            gas_limit,
            0,
        )
        .await?;

    Ok((message_id.into(), program_id.into()))
}

async fn send_message_with_custom_limit<T: Decode>(
    client: &GearApi,
    listener: &mut EventListener,
    destination: [u8; 32],
    payload: impl Encode + Debug,
    modify_gas_limit: fn(u64) -> u64,
) -> Result<Result<T, String>> {
    let encoded_payload = payload.encode();
    let destination = destination.into();

    let gas_limit = client
        .calculate_handle_gas(None, destination, encoded_payload, 0, true)
        .await?
        .min_limit;
    let modified_gas_limit = modify_gas_limit(gas_limit);

    println!("Sending a payload: `{payload:?}`.");
    println!("Calculated gas limit: {gas_limit}.");
    println!("Modified gas limit: {modified_gas_limit}.");

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

async fn send_message<T: Decode>(
    client: &GearApi,
    listener: &mut EventListener,
    destination: [u8; 32],
    payload: impl Encode + Debug,
) -> Result<T> {
    Ok(
        send_message_with_custom_limit(client, listener, destination, payload, |gas| gas * 10)
            .await?
            .expect("action failed, received an error message instead of a reply"),
    )
}

async fn send_message_for_goc(
    client: &GearApi,
    listener: &mut EventListener,
    destination: [u8; 32],
    payload: impl Encode + Debug,
) -> Result<Result<Event, Error>> {
    send_message(client, listener, destination, payload).await
}

async fn send_message_with_insufficient_gas(
    client: &GearApi,
    listener: &mut EventListener,
    destination: [u8; 32],
    payload: impl Encode + Debug,
) -> Result<String> {
    Ok(
        send_message_with_custom_limit::<()>(client, listener, destination, payload, |gas| {
            gas - gas / 100
        })
        .await?
        .expect_err("received a reply instead of an error message"),
    )
}

#[tokio::test]
#[ignore]
async fn state_consistency() -> Result<()> {
    let client = GearApi::dev_from_path(env!("GEAR_NODE_PATH")).await?;
    let mut listener = client.subscribe().await?;

    let storage_code_hash = upload_code(&client, "target/ft_storage.wasm").await?;
    let ft_logic_code_hash = upload_code(&client, "target/ft_logic.wasm").await?;

    let ft_actor_id = upload_program(
        &client,
        &mut listener,
        "target/ft_main.wasm",
        InitFToken {
            storage_code_hash,
            ft_logic_code_hash,
        },
    )
    .await?;

    let (goc_actor_id, reply) = upload_program_and_wait_reply::<Result<(), Error>>(
        &client,
        &mut listener,
        WASM_BINARY_OPT.into(),
        Initialize {
            admin: ALICE.into(),
        },
    )
    .await?;
    assert_eq!(reply, Ok(()));

    let amount = 12345;

    assert!(
        FTokenEvent::Ok
            == send_message(
                &client,
                &mut listener,
                ft_actor_id,
                FTokenAction::Message {
                    transaction_id: 0,
                    payload: LogicAction::Mint {
                        recipient: ALICE.into(),
                        amount,
                    },
                },
            )
            .await?
    );
    assert!(
        FTokenEvent::Ok
            == send_message(
                &client,
                &mut listener,
                ft_actor_id,
                FTokenAction::Message {
                    transaction_id: 1,
                    payload: LogicAction::Approve {
                        approved_account: goc_actor_id.into(),
                        amount,
                    },
                },
            )
            .await?
    );

    println!(
        "{:?}",
        send_message_for_goc(
            &client,
            &mut listener,
            goc_actor_id,
            Action::Start {
                duration: 15000,
                participation_cost: 10000,
                fungible_token: Some(ft_actor_id.into())
            }
        )
        .await?
    );

    let mut payload = Action::Enter;

    println!(
        "{}",
        send_message_with_insufficient_gas(&client, &mut listener, goc_actor_id, payload).await?
    );
    assert_eq!(
        send_message_for_goc(&client, &mut listener, goc_actor_id, payload).await?,
        Ok(Event::PlayerAdded(ALICE.into()))
    );

    payload = Action::PickWinner;

    println!(
        "{}",
        send_message_with_insufficient_gas(&client, &mut listener, goc_actor_id, payload).await?
    );
    assert_eq!(
        Ok(Event::Winner(ALICE.into())),
        send_message_for_goc(&client, &mut listener, goc_actor_id, payload).await?
    );

    println!(
        "{:?}",
        client.read_state::<State>(goc_actor_id.into()).await?
    );

    Ok(())
}
