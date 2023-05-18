use dex_pair::WASM_BINARY_OPT;
use dex_pair_io::*;
use fmt::Debug;
use ft_main_io::{FTokenAction, FTokenEvent, InitFToken, LogicAction};
use gclient::{Error as GclientError, EventListener, EventProcessor, GearApi, Result};
use gear_core::ids::CodeId;
use gstd::{prelude::*, ActorId};
use pretty_assertions::assert_eq;
use primitive_types::H256;
use subxt::{
    error::{DispatchError, ModuleError, ModuleErrorData},
    Error as SubxtError,
};

const ALICE: [u8; 32] = [
    212, 53, 147, 199, 21, 253, 211, 28, 97, 20, 26, 189, 4, 169, 159, 214, 130, 44, 133, 88, 133,
    76, 205, 227, 154, 86, 132, 231, 165, 109, 162, 125,
];
const FT_MAIN: &str = "../target/ft_main.wasm";
const FT_STORAGE: &str = "../target/ft_storage.wasm";
const FT_LOGIC: &str = "../target/ft_logic.wasm";

fn decode<T: Decode>(payload: Vec<u8>) -> Result<T> {
    Ok(T::decode(&mut payload.as_slice())?)
}

async fn upload_code_common(
    result: Result<(CodeId, H256)>,
    get_code: impl FnOnce() -> Result<Vec<u8>>,
) -> Result<H256> {
    let code_id = match result {
        Ok((code_id, _)) => code_id.into(),
        Err(GclientError::Subxt(SubxtError::Runtime(DispatchError::Module(ModuleError {
            error_data:
                ModuleErrorData {
                    pallet_index: 104,
                    error: [6, 0, 0, 0],
                },
            ..
        })))) => sp_core_hashing::blake2_256(&get_code()?),
        Err(other_error) => return Err(other_error),
    };

    Ok(code_id.into())
}

async fn upload_code(client: &GearApi, code: &[u8]) -> Result<H256> {
    upload_code_common(client.upload_code(code).await, || Ok(code.to_vec())).await
}

async fn upload_code_by_path(client: &GearApi, path: &str) -> Result<H256> {
    let r = upload_code_common(client.upload_code_by_path(path).await, || {
        gclient::code_from_os(path)
    })
    .await;

    println!("Uploaded `{path}`.");

    r
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
        send_message_with_custom_limit(client, listener, destination, payload, |gas| gas * 2)
            .await?
            .expect("action failed, received an error message instead of a reply"),
    )
}

async fn send_message_for_pair(
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
    let client = GearApi::dev_from_path(env!("GEAR_NODE_PATH"))
        .await
        .unwrap();
    let mut listener = client.subscribe().await?;

    let storage_code_hash = upload_code_by_path(&client, FT_STORAGE).await?;
    let ft_logic_code_hash = upload_code_by_path(&client, FT_LOGIC).await?;

    let mut ft_actor_id_a = upload_program(
        &client,
        &mut listener,
        FT_MAIN,
        InitFToken {
            storage_code_hash,
            ft_logic_code_hash,
        },
    )
    .await?;
    let mut ft_actor_id_b = upload_program(
        &client,
        &mut listener,
        FT_MAIN,
        InitFToken {
            storage_code_hash,
            ft_logic_code_hash,
        },
    )
    .await?;

    if ft_actor_id_a < ft_actor_id_b {
        (ft_actor_id_a, ft_actor_id_b) = (ft_actor_id_b, ft_actor_id_a)
    };

    let pair_code_hash = upload_code(&client, WASM_BINARY_OPT).await?;
    let (factory_actor_id, reply) =
        upload_program_and_wait_reply::<Result<(), dex_factory_io::Error>>(
            &client,
            &mut listener,
            dex_factory::WASM_BINARY_OPT.into(),
            dex_factory_io::Initialize {
                fee_to: ActorId::zero(),
                fee_to_setter: ActorId::zero(),
                pair: pair_code_hash.into(),
            },
        )
        .await?;
    assert_eq!(reply, Ok(()));

    let reply: Result<dex_factory_io::Event, dex_factory_io::Error> = send_message(
        &client,
        &mut listener,
        factory_actor_id,
        dex_factory_io::Action::CreatePair(ft_actor_id_b.into(), ft_actor_id_a.into()),
    )
    .await?;
    let pair_actor_id = if let dex_factory_io::Event::PairCreated {
        token_pair: _,
        pair_actor,
        pair_number,
    } = reply.unwrap()
    {
        assert_eq!(pair_number, 1);

        pair_actor
    } else {
        unreachable!()
    };

    let amount = 100000;
    let liquidity = amount / 2;

    assert_eq!(
        FTokenEvent::Ok,
        send_message(
            &client,
            &mut listener,
            ft_actor_id_a,
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
    assert_eq!(
        FTokenEvent::Ok,
        send_message(
            &client,
            &mut listener,
            ft_actor_id_b,
            FTokenAction::Message {
                transaction_id: 0,
                payload: LogicAction::Mint {
                    recipient: ALICE.into(),
                    amount: liquidity,
                },
            },
        )
        .await?
    );

    assert_eq!(
        FTokenEvent::Ok,
        send_message(
            &client,
            &mut listener,
            ft_actor_id_a,
            FTokenAction::Message {
                transaction_id: 1,
                payload: LogicAction::Approve {
                    approved_account: pair_actor_id,
                    amount,
                },
            },
        )
        .await?
    );
    assert_eq!(
        FTokenEvent::Ok,
        send_message(
            &client,
            &mut listener,
            ft_actor_id_b,
            FTokenAction::Message {
                transaction_id: 1,
                payload: LogicAction::Approve {
                    approved_account: pair_actor_id,
                    amount: liquidity,
                },
            },
        )
        .await?
    );

    let true_liq = (liquidity - MINIMUM_LIQUIDITY as u128).into();
    let deadline = 999999999999999999;
    let mut action = Action::new(InnerAction::AddLiquidity {
        amount_a_desired: liquidity,
        amount_b_desired: liquidity,
        amount_a_min: 0,
        amount_b_min: 0,
        to: ALICE.into(),
        deadline,
    });

    println!(
        "{}",
        send_message_with_insufficient_gas(&client, &mut listener, pair_actor_id.into(), action)
            .await?
    );
    assert_eq!(
        send_message_for_pair(
            &client,
            &mut listener,
            pair_actor_id.into(),
            action.to_retry(),
        )
        .await?,
        Ok(Event::AddedLiquidity {
            sender: ALICE.into(),
            amount_a: liquidity,
            amount_b: liquidity,
            liquidity: true_liq
        }),
    );

    action.action = InnerAction::SwapExactTokensForTokens {
        amount_in: liquidity,
        amount_out_min: 0,
        to: ALICE.into(),
        deadline,
        swap_kind: SwapKind::AForB,
    };

    println!(
        "{}",
        send_message_with_insufficient_gas(&client, &mut listener, pair_actor_id.into(), action)
            .await?
    );
    assert_eq!(
        send_message_for_pair(
            &client,
            &mut listener,
            pair_actor_id.into(),
            action.to_retry(),
        )
        .await?,
        Ok(Event::Swap {
            sender: ALICE.into(),
            in_amount: liquidity,
            out_amount: 24962,
            to: ALICE.into(),
            kind: SwapKind::AForB
        }),
    );

    action.action = InnerAction::RemoveLiquidity {
        liquidity: true_liq,
        amount_a_min: 0,
        amount_b_min: 0,
        to: ALICE.into(),
        deadline,
    };

    println!(
        "{:?}",
        send_message_with_insufficient_gas(&client, &mut listener, pair_actor_id.into(), action)
            .await?
    );
    assert_eq!(
        send_message_for_pair(
            &client,
            &mut listener,
            pair_actor_id.into(),
            action.to_retry()
        )
        .await?,
        Ok(Event::RemovedLiquidity {
            sender: ALICE.into(),
            amount_a: 98000,
            amount_b: 24537,
            to: ALICE.into()
        }),
    );

    Ok(())
}
