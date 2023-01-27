use fmt::Debug;
use ft_logic_io::Action as FTAction;
use ft_main_io::{FTokenAction, FTokenEvent, InitFToken};
use gclient::{Error as GclientError, EventListener, EventProcessor, GearApi, Result};
use gear_lib::non_fungible_token::token::TokenMetadata;
use gstd::{prelude::*, ActorId};
use nft_io::InitNFT;
use pretty_assertions::assert_eq;
use primitive_types::H256;
use subxt::{
    error::{DispatchError, ModuleError, ModuleErrorData},
    Error as SubxtError,
};
use supply_chain::WASM_BINARY_OPT as WASM;
use supply_chain_io::*;
use supply_chain_state::{WASM_BINARY as META_WASM, WASM_EXPORTS as META_WASM_FNS};

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
    let reply = decode(raw_reply.expect("Received an error message instead of a reply"))?;

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
            gclient::now_in_micros().to_le_bytes(),
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
            .expect("Received an error message instead of a reply"),
    )
}

async fn send_message_for_sc(
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
        .expect_err("Received a reply instead of an error message"),
    )
}

async fn is_action_cached(
    client: &GearApi,
    supply_chain_actor_id: [u8; 32],
    action: Action,
) -> Result<bool> {
    client
        .read_state_using_wasm::<_, bool>(
            supply_chain_actor_id.into(),
            META_WASM_FNS[7],
            META_WASM.into(),
            Some((ActorId::from(ALICE), action.clone().action)),
        )
        .await
}

#[tokio::test]
#[ignore]
async fn state_consistency() -> Result<()> {
    let client = GearApi::dev()
        .await
        .expect("The node must be running for a gclient test");
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

    let nft_actor_id = upload_program(
        &client,
        &mut listener,
        "target/nft.opt.wasm",
        InitNFT {
            name: Default::default(),
            symbol: Default::default(),
            base_uri: Default::default(),
            royalties: Default::default(),
        },
    )
    .await?;

    let (supply_chain_actor_id, reply) = upload_program_and_wait_reply::<Result<(), Error>>(
        &client,
        &mut listener,
        WASM.into(),
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

    let item_id = 0.into();
    let price = 123456;
    let delivery_time = 600000;
    let approve = true;
    let mut payload = Action::new(InnerAction::Producer(ProducerAction::Produce {
        token_metadata: TokenMetadata::default(),
    }));

    assert!(
        FTokenEvent::Ok
            == send_message(
                &client,
                &mut listener,
                ft_actor_id,
                FTokenAction::Message {
                    transaction_id: 0,
                    payload: FTAction::Mint {
                        recipient: ALICE.into(),
                        amount: price
                    }
                    .encode(),
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
                    payload: FTAction::Approve {
                        approved_account: supply_chain_actor_id.into(),
                        amount: price * 3,
                    }
                    .encode(),
                },
            )
            .await?
    );

    // InnerAction::Producer(ProducerAction::Produce)

    println!(
        "{}",
        send_message_with_insufficient_gas(
            &client,
            &mut listener,
            supply_chain_actor_id,
            payload.clone(),
        )
        .await?
    );
    assert!(is_action_cached(&client, supply_chain_actor_id, payload.clone()).await?);
    assert_eq!(
        send_message_for_sc(
            &client,
            &mut listener,
            supply_chain_actor_id,
            payload.to_retry()
        )
        .await?,
        Ok(Event {
            item_id,
            item_state: ItemState {
                state: ItemEventState::Produced,
                by: Role::Producer
            }
        })
    );

    // InnerAction::Producer(ProducerAction::PutUpForSale)

    payload = Action::new(InnerAction::Producer(ProducerAction::PutUpForSale {
        item_id,
        price,
    }));

    println!(
        "{}",
        send_message_with_insufficient_gas(
            &client,
            &mut listener,
            supply_chain_actor_id,
            payload.clone(),
        )
        .await?
    );
    assert!(is_action_cached(&client, supply_chain_actor_id, payload.clone()).await?);
    assert_eq!(
        send_message_for_sc(
            &client,
            &mut listener,
            supply_chain_actor_id,
            payload.to_retry()
        )
        .await?,
        Ok(Event {
            item_id,
            item_state: ItemState {
                state: ItemEventState::ForSale,
                by: Role::Producer
            }
        }),
    );

    // InnerAction::Distributor(DistributorAction::Purchase)

    payload = Action::new(InnerAction::Distributor(DistributorAction::Purchase {
        item_id,
        delivery_time,
    }));

    println!(
        "{}",
        send_message_with_insufficient_gas(
            &client,
            &mut listener,
            supply_chain_actor_id,
            payload.clone(),
        )
        .await?
    );
    assert!(is_action_cached(&client, supply_chain_actor_id, payload.clone()).await?);
    assert_eq!(
        send_message_for_sc(
            &client,
            &mut listener,
            supply_chain_actor_id,
            payload.to_retry()
        )
        .await?,
        Ok(Event {
            item_id,
            item_state: ItemState {
                state: ItemEventState::Purchased,
                by: Role::Distributor
            }
        }),
    );

    // InnerAction::Producer(ProducerAction::Approve)

    payload = Action::new(InnerAction::Producer(ProducerAction::Approve {
        item_id,
        approve,
    }));

    assert_eq!(
        send_message_for_sc(
            &client,
            &mut listener,
            supply_chain_actor_id,
            payload.clone()
        )
        .await?,
        Ok(Event {
            item_id,
            item_state: ItemState {
                state: ItemEventState::Approved,
                by: Role::Producer
            }
        })
    );
    assert!(!is_action_cached(&client, supply_chain_actor_id, payload).await?);

    // InnerAction::Producer(ProducerAction::Ship)

    assert_eq!(
        send_message_for_sc(
            &client,
            &mut listener,
            supply_chain_actor_id,
            Action::new(InnerAction::Producer(ProducerAction::Ship(item_id)))
        )
        .await?,
        Ok(Event {
            item_id,
            item_state: ItemState {
                state: ItemEventState::Shipped,
                by: Role::Producer
            }
        }),
    );

    // InnerAction::Distributor(DistributorAction::Receive)

    payload = Action::new(InnerAction::Distributor(DistributorAction::Receive(
        item_id,
    )));

    println!(
        "{}",
        send_message_with_insufficient_gas(
            &client,
            &mut listener,
            supply_chain_actor_id,
            payload.clone(),
        )
        .await?
    );
    assert!(is_action_cached(&client, supply_chain_actor_id, payload.clone()).await?);
    assert_eq!(
        send_message_for_sc(
            &client,
            &mut listener,
            supply_chain_actor_id,
            payload.to_retry()
        )
        .await?,
        Ok(Event {
            item_id,
            item_state: ItemState {
                state: ItemEventState::Received,
                by: Role::Distributor
            }
        }),
    );

    // InnerAction::Distributor(DistributorAction::Process)

    assert_eq!(
        send_message_for_sc(
            &client,
            &mut listener,
            supply_chain_actor_id,
            Action::new(InnerAction::Distributor(DistributorAction::Process(
                item_id
            )))
        )
        .await?,
        Ok(Event {
            item_id,
            item_state: ItemState {
                state: ItemEventState::Processed,
                by: Role::Distributor
            }
        }),
    );

    // InnerAction::Distributor(DistributorAction::Package)

    assert_eq!(
        send_message_for_sc(
            &client,
            &mut listener,
            supply_chain_actor_id,
            Action::new(InnerAction::Distributor(DistributorAction::Package(
                item_id
            )))
        )
        .await?,
        Ok(Event {
            item_id,
            item_state: ItemState {
                state: ItemEventState::Packaged,
                by: Role::Distributor
            }
        }),
    );

    // InnerAction::Distributor(DistributorAction::PutUpForSale)

    payload = Action::new(InnerAction::Distributor(DistributorAction::PutUpForSale {
        item_id,
        price,
    }));

    println!(
        "{}",
        send_message_with_insufficient_gas(
            &client,
            &mut listener,
            supply_chain_actor_id,
            payload.clone(),
        )
        .await?
    );
    assert!(is_action_cached(&client, supply_chain_actor_id, payload.clone()).await?);
    assert_eq!(
        send_message_for_sc(
            &client,
            &mut listener,
            supply_chain_actor_id,
            payload.to_retry()
        )
        .await?,
        Ok(Event {
            item_id,
            item_state: ItemState {
                state: ItemEventState::ForSale,
                by: Role::Distributor
            }
        }),
    );

    // InnerAction::Retailer(RetailerAction::Purchase)

    payload = Action::new(InnerAction::Retailer(RetailerAction::Purchase {
        item_id,
        delivery_time,
    }));

    println!(
        "{}",
        send_message_with_insufficient_gas(
            &client,
            &mut listener,
            supply_chain_actor_id,
            payload.clone(),
        )
        .await?
    );
    assert!(is_action_cached(&client, supply_chain_actor_id, payload.clone()).await?);
    assert_eq!(
        send_message_for_sc(
            &client,
            &mut listener,
            supply_chain_actor_id,
            payload.to_retry()
        )
        .await?,
        Ok(Event {
            item_id,
            item_state: ItemState {
                state: ItemEventState::Purchased,
                by: Role::Retailer
            }
        })
    );

    // InnerAction::Distributor(DistributorAction::Approve)

    payload = Action::new(InnerAction::Distributor(DistributorAction::Approve {
        item_id,
        approve,
    }));

    assert_eq!(
        send_message_for_sc(
            &client,
            &mut listener,
            supply_chain_actor_id,
            payload.clone()
        )
        .await?,
        Ok(Event {
            item_id,
            item_state: ItemState {
                state: ItemEventState::Approved,
                by: Role::Distributor
            }
        }),
    );
    assert!(!is_action_cached(&client, supply_chain_actor_id, payload).await?);

    // InnerAction::Distributor(DistributorAction::Ship)

    assert_eq!(
        send_message_for_sc(
            &client,
            &mut listener,
            supply_chain_actor_id,
            Action::new(InnerAction::Distributor(DistributorAction::Ship(item_id)))
        )
        .await?,
        Ok(Event {
            item_id,
            item_state: ItemState {
                state: ItemEventState::Shipped,
                by: Role::Distributor
            }
        }),
    );

    // InnerAction::Retailer(RetailerAction::Receive)

    payload = Action::new(InnerAction::Retailer(RetailerAction::Receive(item_id)));

    println!(
        "{}",
        send_message_with_insufficient_gas(
            &client,
            &mut listener,
            supply_chain_actor_id,
            payload.clone(),
        )
        .await?
    );
    assert!(is_action_cached(&client, supply_chain_actor_id, payload.clone()).await?);
    assert_eq!(
        send_message_for_sc(
            &client,
            &mut listener,
            supply_chain_actor_id,
            payload.to_retry()
        )
        .await?,
        Ok(Event {
            item_id,
            item_state: ItemState {
                state: ItemEventState::Received,
                by: Role::Retailer
            }
        }),
    );

    // InnerAction::Retailer(RetailerAction::PutUpForSale)

    payload = Action::new(InnerAction::Retailer(RetailerAction::PutUpForSale {
        item_id,
        price,
    }));

    println!(
        "{}",
        send_message_with_insufficient_gas(
            &client,
            &mut listener,
            supply_chain_actor_id,
            payload.clone(),
        )
        .await?
    );
    assert!(is_action_cached(&client, supply_chain_actor_id, payload.clone()).await?);
    assert_eq!(
        send_message_for_sc(
            &client,
            &mut listener,
            supply_chain_actor_id,
            payload.to_retry()
        )
        .await?,
        Ok(Event {
            item_id,
            item_state: ItemState {
                state: ItemEventState::ForSale,
                by: Role::Retailer
            }
        }),
    );

    // InnerAction::Consumer(ConsumerAction::Purchase)

    payload = Action::new(InnerAction::Consumer(ConsumerAction::Purchase(item_id)));

    println!(
        "{}",
        send_message_with_insufficient_gas(
            &client,
            &mut listener,
            supply_chain_actor_id,
            payload.clone(),
        )
        .await?
    );
    assert!(is_action_cached(&client, supply_chain_actor_id, payload.clone()).await?);
    assert_eq!(
        send_message_for_sc(
            &client,
            &mut listener,
            supply_chain_actor_id,
            payload.to_retry()
        )
        .await?,
        Ok(Event {
            item_id,
            item_state: ItemState {
                state: ItemEventState::Purchased,
                by: Role::Consumer
            }
        }),
    );

    Ok(())
}
