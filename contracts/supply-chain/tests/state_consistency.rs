use deploy::*;
use ft_logic_io::Action as FTAction;
use ft_main_io::{FTokenAction, FTokenEvent, InitFToken};
use gclient::Result;
use gear_lib::non_fungible_token::token::TokenMetadata;
use gstd::prelude::*;
use nft_io::InitNFT;
use supply_chain::WASM_BINARY_OPT;
use supply_chain_io::*;

#[tokio::test]
#[ignore]
async fn state_consistency() -> Result<()> {
    let node = Client::node();
    let mut client = Client::local(&node).await?;

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

    let nft_actor_id = client
        .upload_program(
            NFT_BINARY,
            InitNFT {
                name: Default::default(),
                symbol: Default::default(),
                base_uri: Default::default(),
                royalties: Default::default(),
            },
        )
        .await?;

    let (supply_chain_actor_id, reply) = client
        .upload_program_and_wait_reply::<Result<(), Error>>(
            WASM_BINARY_OPT.into(),
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
            == client
                .send_message(
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
            == client
                .send_message(
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
        client
            .send_message_with_insufficient_gas(supply_chain_actor_id, payload.clone(),)
            .await?
    );
    assert!(
        client
            .is_action_cached(supply_chain_actor_id, payload.clone())
            .await?
    );
    assert_eq!(
        client
            .send_message_for_sc(supply_chain_actor_id, payload.to_retry())
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
        client
            .send_message_with_insufficient_gas(supply_chain_actor_id, payload.clone(),)
            .await?
    );
    assert!(
        client
            .is_action_cached(supply_chain_actor_id, payload.clone())
            .await?
    );
    assert_eq!(
        client
            .send_message_for_sc(supply_chain_actor_id, payload.to_retry())
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
        client
            .send_message_with_insufficient_gas(supply_chain_actor_id, payload.clone(),)
            .await?
    );
    assert!(
        client
            .is_action_cached(supply_chain_actor_id, payload.clone())
            .await?
    );
    assert_eq!(
        client
            .send_message_for_sc(supply_chain_actor_id, payload.to_retry())
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
        client
            .send_message_for_sc(supply_chain_actor_id, payload.clone())
            .await?,
        Ok(Event {
            item_id,
            item_state: ItemState {
                state: ItemEventState::Approved,
                by: Role::Producer
            }
        })
    );
    assert!(
        !client
            .is_action_cached(supply_chain_actor_id, payload)
            .await?
    );

    // InnerAction::Producer(ProducerAction::Ship)

    assert_eq!(
        client
            .send_message_for_sc(
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
        client
            .send_message_with_insufficient_gas(supply_chain_actor_id, payload.clone(),)
            .await?
    );
    assert!(
        client
            .is_action_cached(supply_chain_actor_id, payload.clone())
            .await?
    );
    assert_eq!(
        client
            .send_message_for_sc(supply_chain_actor_id, payload.to_retry())
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
        client
            .send_message_for_sc(
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
        client
            .send_message_for_sc(
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
        client
            .send_message_with_insufficient_gas(supply_chain_actor_id, payload.clone(),)
            .await?
    );
    assert!(
        client
            .is_action_cached(supply_chain_actor_id, payload.clone())
            .await?
    );
    assert_eq!(
        client
            .send_message_for_sc(supply_chain_actor_id, payload.to_retry())
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
        client
            .send_message_with_insufficient_gas(supply_chain_actor_id, payload.clone(),)
            .await?
    );
    assert!(
        client
            .is_action_cached(supply_chain_actor_id, payload.clone())
            .await?
    );
    assert_eq!(
        client
            .send_message_for_sc(supply_chain_actor_id, payload.to_retry())
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
        client
            .send_message_for_sc(supply_chain_actor_id, payload.clone())
            .await?,
        Ok(Event {
            item_id,
            item_state: ItemState {
                state: ItemEventState::Approved,
                by: Role::Distributor
            }
        }),
    );
    assert!(
        !client
            .is_action_cached(supply_chain_actor_id, payload)
            .await?
    );

    // InnerAction::Distributor(DistributorAction::Ship)

    assert_eq!(
        client
            .send_message_for_sc(
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
        client
            .send_message_with_insufficient_gas(supply_chain_actor_id, payload.clone(),)
            .await?
    );
    assert!(
        client
            .is_action_cached(supply_chain_actor_id, payload.clone())
            .await?
    );
    assert_eq!(
        client
            .send_message_for_sc(supply_chain_actor_id, payload.to_retry())
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
        client
            .send_message_with_insufficient_gas(supply_chain_actor_id, payload.clone(),)
            .await?
    );
    assert!(
        client
            .is_action_cached(supply_chain_actor_id, payload.clone())
            .await?
    );
    assert_eq!(
        client
            .send_message_for_sc(supply_chain_actor_id, payload.to_retry())
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
        client
            .send_message_with_insufficient_gas(supply_chain_actor_id, payload.clone(),)
            .await?
    );
    assert!(
        client
            .is_action_cached(supply_chain_actor_id, payload.clone())
            .await?
    );
    assert_eq!(
        client
            .send_message_for_sc(supply_chain_actor_id, payload.to_retry())
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
