use gclient::Result;
use gear_lib_old::non_fungible_token::{io::NFTApproval, token::TokenMetadata};
use gstd::prelude::*;
use non_fungible_token_io::{Constraints, InitNFT, NFTAction, NFTEvent};
use sharded_fungible_token_io::{FTokenAction, FTokenEvent, InitFToken, LogicAction};
use supply_chain::WASM_BINARY_OPT;
use supply_chain_deploy::*;
use supply_chain_io::*;

#[tokio::test]
#[ignore]
async fn state_consistency() -> Result<()> {
    let mut client = Client::local().await?;

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
                royalties: Default::default(),
                collection: Default::default(),
                constraints: Constraints {
                    authorized_minters: vec![ALICE.into()],
                    ..Default::default()
                },
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

    assert_eq!(
        NFTEvent::MinterAdded {
            minter_id: supply_chain_actor_id.into(),
        },
        client
            .send_message(
                nft_actor_id,
                NFTAction::AddMinter {
                    transaction_id: 0,
                    minter_id: supply_chain_actor_id.into(),
                },
            )
            .await?
    );

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
                        payload: LogicAction::Mint {
                            recipient: ALICE.into(),
                            amount: price
                        },
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
                        payload: LogicAction::Approve {
                            approved_account: supply_chain_actor_id.into(),
                            amount: price * 3,
                        },
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

    assert_eq!(
        NFTEvent::Approval(NFTApproval {
            owner: ALICE.into(),
            approved_account: supply_chain_actor_id.into(),
            token_id: item_id
        }),
        client
            .send_message(
                nft_actor_id,
                NFTAction::Approve {
                    transaction_id: 1,
                    to: supply_chain_actor_id.into(),
                    token_id: item_id
                },
            )
            .await?
    );

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

    assert_eq!(
        NFTEvent::Approval(NFTApproval {
            owner: ALICE.into(),
            approved_account: supply_chain_actor_id.into(),
            token_id: item_id
        }),
        client
            .send_message(
                nft_actor_id,
                NFTAction::Approve {
                    transaction_id: 2,
                    to: supply_chain_actor_id.into(),
                    token_id: item_id
                },
            )
            .await?
    );

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

    assert_eq!(
        NFTEvent::Approval(NFTApproval {
            owner: ALICE.into(),
            approved_account: supply_chain_actor_id.into(),
            token_id: item_id
        }),
        client
            .send_message(
                nft_actor_id,
                NFTAction::Approve {
                    transaction_id: 3,
                    to: supply_chain_actor_id.into(),
                    token_id: item_id
                },
            )
            .await?
    );

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
