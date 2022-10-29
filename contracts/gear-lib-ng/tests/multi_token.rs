use gear_lib::{
    multi_token::*,
    testing::{msg, utils},
    Amount, StorageProvider,
};
use gstd::{prelude::*, ActorId};
use pretty_assertions::assert_eq;

const AMOUNT: u128 = 12345;
const REMAINDER: u128 = AMOUNT / 2;

#[derive(Default, StorageProvider)]
struct Contract {
    #[storage_field]
    state: MTState,
    #[storage_field]
    metadata: MTStateMeta,
}

impl MultiToken for Contract {
    fn reply_transfer(&self, transfer: MTTransfer) -> Result<(), MTError> {
        utils::set_last_reply(transfer);

        Ok(())
    }

    fn reply_approval(&self, approval: MTApproval) -> Result<(), MTError> {
        utils::set_last_reply(approval);

        Ok(())
    }

    fn reply_transfer_batch(&self, transfer_batch: MTTransferBatch) -> Result<(), MTError> {
        utils::set_last_reply(transfer_batch);

        Ok(())
    }
}

impl MultiTokenMeta for Contract {
    fn reply_set_attribute(&self, attribute: MTAttribute) -> Result<(), MTError> {
        utils::set_last_reply(attribute);

        Ok(())
    }
}

#[test]
fn meta() {
    let key: Vec<u8> = "Name".into();
    let data: Vec<u8> = "Nuclear Fish Tank".into();
    let mut contract = Contract::default();

    MultiTokenMeta::set_attribute(&mut contract, 0u8.into(), key.clone(), data.clone()).unwrap();

    assert_eq!(
        MTAttribute {
            id: 0u8.into(),
            key: key.clone(),
            data: data.clone()
        },
        utils::last_reply()
    );
    assert_eq!(
        MTMeta::get_attribute(&contract, 0u8.into(), key.clone()),
        Some(data)
    );
    assert_eq!(
        MTMeta::get_attribute(&contract, 0u8.into(), "Nonexistent attribute".into()),
        None
    );
    assert_eq!(MTMeta::get_attribute(&contract, 1u8.into(), key), None);
}

#[test]
fn mint() {
    let mut contract = Contract::default();

    MTMint::mint(&mut contract, 1.into(), 0u8.into(), AMOUNT.into()).unwrap();

    assert_eq!(
        MTTransfer {
            from: ActorId::zero(),
            to: 1.into(),
            id: 0u8.into(),
            amount: AMOUNT.into()
        },
        utils::last_reply()
    );
    assert_eq!(
        MTCore::balance_of(&contract, 1.into(), Some(0u8.into())),
        AMOUNT.into()
    );
    assert_eq!(MTCore::balance_of(&contract, 1.into(), None), AMOUNT.into());
    assert_eq!(
        MTCore::total_supply(&contract, Some(0u8.into())),
        AMOUNT.into()
    );
    assert_eq!(MTCore::total_supply(&contract, None), AMOUNT.into());
}

#[test]
fn mint_failures() {
    let mut contract = Contract::default();

    assert_eq!(
        MTMint::mint(
            &mut contract,
            ActorId::zero(),
            0u8.into(),
            Amount::default()
        ),
        Err(MTError::ZeroRecipientAddress)
    );

    MTMint::mint(&mut contract, 1.into(), 0u8.into(), 1.into()).unwrap();

    assert_eq!(
        MTMint::mint(&mut contract, 1.into(), 0u8.into(), Amount::MAX),
        Err(MTError::InsufficientAmount)
    );
}

#[test]
fn burn() {
    let mut contract = Contract::default();

    MTMint::mint(&mut contract, 1.into(), 0u8.into(), AMOUNT.into()).unwrap();
    MTBurn::burn(
        &mut contract,
        1.into(),
        0u8.into(),
        (AMOUNT - REMAINDER).into(),
    )
    .unwrap();

    assert_eq!(
        MTTransfer {
            from: 1.into(),
            to: ActorId::zero(),
            id: 0u8.into(),
            amount: (AMOUNT - REMAINDER).into()
        },
        utils::last_reply()
    );
    assert_eq!(
        MTCore::balance_of(&contract, 1.into(), Some(0u8.into())),
        REMAINDER.into()
    );
    assert_eq!(
        MTCore::balance_of(&contract, 1.into(), None),
        REMAINDER.into()
    );
    assert_eq!(
        MTCore::total_supply(&contract, Some(0u8.into())),
        REMAINDER.into()
    );
    assert_eq!(MTCore::total_supply(&contract, None), REMAINDER.into());
}

#[test]
fn burn_failures() {
    let mut contract = Contract::default();

    assert_eq!(
        MTBurn::burn(
            &mut contract,
            ActorId::zero(),
            0u8.into(),
            Amount::default()
        ),
        Err(MTError::ZeroSenderAddress)
    );

    MTMint::mint(&mut contract, 1.into(), 0u8.into(), AMOUNT.into()).unwrap();

    assert_eq!(
        MTBurn::burn(&mut contract, 1.into(), 0u8.into(), (AMOUNT + 1).into()),
        Err(MTError::InsufficientAmount)
    );
}

#[test]
fn transfer() {
    let mut contract = Contract::default();

    MTMint::mint(&mut contract, 1.into(), 0u8.into(), AMOUNT.into()).unwrap();
    msg::set_source(1.into());
    MTCore::transfer(&mut contract, 2.into(), 0u8.into(), REMAINDER.into()).unwrap();

    assert_eq!(
        MTTransfer {
            from: 1.into(),
            to: 2.into(),
            id: 0u8.into(),
            amount: REMAINDER.into()
        },
        utils::last_reply()
    );
    assert_eq!(
        MTCore::balance_of(&contract, 1.into(), Some(0u8.into())),
        (AMOUNT - REMAINDER).into()
    );
    assert_eq!(
        MTCore::balance_of(&contract, 1.into(), None),
        (AMOUNT - REMAINDER).into()
    );
    assert_eq!(
        MTCore::balance_of(&contract, 2.into(), Some(0u8.into())),
        REMAINDER.into()
    );
    assert_eq!(
        MTCore::balance_of(&contract, 2.into(), None),
        REMAINDER.into()
    );
}

#[test]
fn transfer_failures() {
    let mut contract = Contract::default();

    msg::set_source(1.into());

    assert_eq!(
        MTCore::transfer(
            &mut contract,
            ActorId::zero(),
            0u8.into(),
            Amount::default()
        ),
        Err(MTError::ZeroRecipientAddress)
    );

    MTMint::mint(&mut contract, 1.into(), 0u8.into(), AMOUNT.into()).unwrap();

    assert_eq!(
        MTCore::transfer(&mut contract, 2.into(), 0u8.into(), (AMOUNT + 1).into()),
        Err(MTError::InsufficientAmount)
    );
}

#[test]
fn approve() {
    let mut contract = Contract::default();

    msg::set_source(1.into());
    MTCore::approve(&mut contract, 2.into(), Some(0u8.into()), true).unwrap();

    assert_eq!(
        MTApproval {
            owner: 1.into(),
            operator: 2.into(),
            id: Some(0u8.into()),
            approved: true
        },
        utils::last_reply()
    );
    assert_eq!(
        MTCore::allowance(&contract, 1.into(), 2.into(), Some(0u8.into())),
        true
    );

    MTCore::approve(&mut contract, 2.into(), Some(0u8.into()), false).unwrap();

    assert_eq!(
        MTApproval {
            owner: 1.into(),
            operator: 2.into(),
            id: Some(0u8.into()),
            approved: false
        },
        utils::last_reply()
    );
    assert_eq!(
        MTCore::allowance(&contract, 1.into(), 2.into(), Some(0u8.into())),
        false
    );

    MTCore::approve(&mut contract, 2.into(), None, true).unwrap();

    assert_eq!(
        MTApproval {
            owner: 1.into(),
            operator: 2.into(),
            id: None,
            approved: true
        },
        utils::last_reply()
    );
    assert_eq!(MTCore::allowance(&contract, 1.into(), 2.into(), None), true);
    assert_eq!(
        MTCore::allowance(&contract, 1.into(), 2.into(), Some(0u8.into())),
        true
    );

    MTCore::approve(&mut contract, 2.into(), None, false).unwrap();

    assert_eq!(
        MTApproval {
            owner: 1.into(),
            operator: 2.into(),
            id: None,
            approved: false
        },
        utils::last_reply()
    );
    assert_eq!(
        MTCore::allowance(&contract, 1.into(), 2.into(), None),
        false
    );
    assert_eq!(
        MTCore::allowance(&contract, 1.into(), 2.into(), Some(0u8.into())),
        false
    );
}

#[test]
fn approve_failures() {
    let mut contract = Contract::default();

    assert_eq!(
        MTCore::approve(&mut contract, ActorId::zero(), None, true),
        Err(MTError::ZeroRecipientAddress)
    );
}

#[test]
fn transfer_from() {
    let mut contract = Contract::default();

    MTMint::mint(&mut contract, 1.into(), 0u8.into(), AMOUNT.into()).unwrap();
    msg::set_source(1.into());
    MTCore::approve(&mut contract, 2.into(), Some(0u8.into()), true).unwrap();
    msg::set_source(2.into());
    MTCore::transfer_from(&mut contract, 1.into(), 2.into(), 0u8.into(), AMOUNT.into()).unwrap();

    assert_eq!(
        MTTransfer {
            from: 1.into(),
            to: 2.into(),
            id: 0u8.into(),
            amount: AMOUNT.into()
        },
        utils::last_reply()
    );

    MTCore::approve(&mut contract, 1.into(), None, true).unwrap();
    msg::set_source(1.into());
    MTCore::transfer_from(
        &mut contract,
        2.into(),
        1.into(),
        0u8.into(),
        REMAINDER.into(),
    )
    .unwrap();

    assert_eq!(
        MTTransfer {
            from: 2.into(),
            to: 1.into(),
            id: 0u8.into(),
            amount: REMAINDER.into()
        },
        utils::last_reply()
    );
}

#[test]
fn transfer_from_failures() {
    let mut contract = Contract::default();

    assert_eq!(
        MTCore::transfer_from(
            &mut contract,
            ActorId::zero(),
            1.into(),
            0u8.into(),
            Amount::default()
        ),
        Err(MTError::ZeroSenderAddress)
    );
    assert_eq!(
        MTCore::transfer_from(
            &mut contract,
            1.into(),
            2.into(),
            0u8.into(),
            Amount::default()
        ),
        Err(MTError::NotApproved)
    );
}

#[test]
fn mint_batch() {
    let mut contract = Contract::default();

    MTBatch::mint_batch(
        &mut contract,
        1.into(),
        BTreeMap::from([(0u8.into(), AMOUNT.into()), (1u8.into(), REMAINDER.into())]),
    )
    .unwrap();

    assert_eq!(
        MTTransferBatch {
            from: ActorId::zero(),
            to: 1.into(),
            ids_for_amount: BTreeMap::from([
                (0u8.into(), AMOUNT.into()),
                (1u8.into(), REMAINDER.into())
            ])
        },
        utils::last_reply()
    );
    assert_eq!(
        MTBatch::balance_of_batch(
            &contract,
            BTreeMap::from([(
                1.into(),
                BTreeSet::from([Some(0u8.into()), Some(1u8.into()), None])
            )])
        ),
        BTreeMap::from([(
            1.into(),
            BTreeSet::from([
                (Some(0u8.into()), AMOUNT.into()),
                (Some(1u8.into()), REMAINDER.into()),
                (None, (AMOUNT + REMAINDER).into())
            ])
        )])
    );
    assert_eq!(
        MTCore::total_supply(&contract, Some(0u8.into())),
        AMOUNT.into()
    );
    assert_eq!(
        MTCore::total_supply(&contract, Some(1u8.into())),
        REMAINDER.into()
    );
    assert_eq!(
        MTCore::total_supply(&contract, None),
        (AMOUNT + REMAINDER).into()
    );
}

#[test]
fn burn_batch() {
    let mut contract = Contract::default();

    MTBatch::mint_batch(
        &mut contract,
        1.into(),
        BTreeMap::from([
            (0u8.into(), AMOUNT.into()),
            (1u8.into(), (AMOUNT * 2).into()),
        ]),
    )
    .unwrap();
    MTBatch::burn_batch(
        &mut contract,
        1.into(),
        BTreeMap::from([
            (0u8.into(), (AMOUNT - REMAINDER).into()),
            (1u8.into(), AMOUNT.into()),
        ]),
    )
    .unwrap();

    assert_eq!(
        MTTransferBatch {
            from: 1.into(),
            to: ActorId::zero(),
            ids_for_amount: BTreeMap::from([
                (0u8.into(), (AMOUNT - REMAINDER).into()),
                (1u8.into(), AMOUNT.into())
            ]),
        },
        utils::last_reply()
    );
    assert_eq!(
        MTBatch::balance_of_batch(
            &contract,
            BTreeMap::from([(
                1.into(),
                BTreeSet::from([Some(0u8.into()), Some(1u8.into()), None])
            )])
        ),
        BTreeMap::from([(
            1.into(),
            BTreeSet::from([
                (Some(0u8.into()), REMAINDER.into()),
                (Some(1u8.into()), AMOUNT.into()),
                (None, (AMOUNT + REMAINDER).into())
            ])
        )])
    );
    assert_eq!(
        MTCore::total_supply(&contract, Some(0u8.into())),
        REMAINDER.into()
    );
    assert_eq!(
        MTCore::total_supply(&contract, Some(1u8.into())),
        AMOUNT.into()
    );
    assert_eq!(
        MTCore::total_supply(&contract, None),
        (AMOUNT + REMAINDER).into()
    );
}

#[test]
fn transfer_batch() {
    let mut contract = Contract::default();

    MTBatch::mint_batch(
        &mut contract,
        1.into(),
        BTreeMap::from([
            (0u8.into(), AMOUNT.into()),
            (1u8.into(), (AMOUNT * 2).into()),
        ]),
    )
    .unwrap();
    msg::set_source(1.into());
    MTBatch::transfer_batch(
        &mut contract,
        2.into(),
        BTreeMap::from([
            (0u8.into(), (AMOUNT - REMAINDER).into()),
            (1u8.into(), AMOUNT.into()),
        ]),
    )
    .unwrap();

    assert_eq!(
        MTTransferBatch {
            from: 1.into(),
            to: 2.into(),
            ids_for_amount: BTreeMap::from([
                (0u8.into(), (AMOUNT - REMAINDER).into()),
                (1u8.into(), AMOUNT.into()),
            ])
        },
        utils::last_reply()
    );
    assert_eq!(
        MTBatch::balance_of_batch(
            &contract,
            BTreeMap::from([
                (
                    1.into(),
                    BTreeSet::from([Some(0u8.into()), Some(1u8.into()), None])
                ),
                (
                    2.into(),
                    BTreeSet::from([Some(0u8.into()), Some(1u8.into()), None])
                )
            ])
        ),
        BTreeMap::from([
            (
                1.into(),
                BTreeSet::from([
                    (Some(0u8.into()), REMAINDER.into()),
                    (Some(1u8.into()), AMOUNT.into()),
                    (None, (AMOUNT + REMAINDER).into())
                ])
            ),
            (
                2.into(),
                BTreeSet::from([
                    (Some(0u8.into()), (AMOUNT - REMAINDER).into()),
                    (Some(1u8.into()), AMOUNT.into()),
                    (None, (AMOUNT * 2 - REMAINDER).into())
                ])
            )
        ])
    );
}

#[test]
fn transfer_from_batch() {
    let mut contract = Contract::default();

    MTBatch::mint_batch(
        &mut contract,
        1.into(),
        BTreeMap::from([
            (0u8.into(), AMOUNT.into()),
            (1u8.into(), (AMOUNT * 2).into()),
        ]),
    )
    .unwrap();
    msg::set_source(1.into());
    MTCore::approve(&mut contract, 2.into(), None, true).unwrap();
    msg::set_source(2.into());
    MTBatch::transfer_from_batch(
        &mut contract,
        1.into(),
        2.into(),
        BTreeMap::from([
            (0u8.into(), (AMOUNT - REMAINDER).into()),
            (1u8.into(), AMOUNT.into()),
        ]),
    )
    .unwrap();

    assert_eq!(
        MTTransferBatch {
            from: 1.into(),
            to: 2.into(),
            ids_for_amount: BTreeMap::from([
                (0u8.into(), (AMOUNT - REMAINDER).into()),
                (1u8.into(), AMOUNT.into()),
            ])
        },
        utils::last_reply()
    );
    assert_eq!(
        MTBatch::balance_of_batch(
            &contract,
            BTreeMap::from([
                (
                    1.into(),
                    BTreeSet::from([Some(0u8.into()), Some(1u8.into()), None])
                ),
                (
                    2.into(),
                    BTreeSet::from([Some(0u8.into()), Some(1u8.into()), None])
                )
            ])
        ),
        BTreeMap::from([
            (
                1.into(),
                BTreeSet::from([
                    (Some(0u8.into()), REMAINDER.into()),
                    (Some(1u8.into()), AMOUNT.into()),
                    (None, (AMOUNT + REMAINDER).into())
                ])
            ),
            (
                2.into(),
                BTreeSet::from([
                    (Some(0u8.into()), (AMOUNT - REMAINDER).into()),
                    (Some(1u8.into()), AMOUNT.into()),
                    (None, (AMOUNT * 2 - REMAINDER).into())
                ])
            )
        ])
    );
}

#[test]
fn transfer_from_batch_failures() {
    let mut contract = Contract::default();

    msg::set_source(1.into());
    MTCore::approve(&mut contract, 2.into(), Some(0u8.into()), true).unwrap();
    msg::set_source(2.into());

    assert_eq!(
        MTBatch::transfer_from_batch(
            &mut contract,
            1.into(),
            2.into(),
            BTreeMap::from([
                (0u8.into(), Amount::default()),
                (1u8.into(), Amount::default()),
            ]),
        ),
        Err(MTError::NotApproved)
    );
}
