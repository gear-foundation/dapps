use gear_lib::{
    fungible_token::*,
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
    state: FTState,
    #[storage_field]
    metadata: FTStateMeta,
}

impl FungibleToken for Contract {
    fn reply_transfer(&self, transfer: FTTransfer) -> Result<(), FTError> {
        utils::set_last_reply(transfer);

        Ok(())
    }

    fn reply_approval(&self, approval: FTApproval) -> Result<(), FTError> {
        utils::set_last_reply(approval);

        Ok(())
    }
}

impl FungibleTokenMeta for Contract {}

#[test]
fn meta() {
    let name = Some("Name".into());
    let symbol = Some("Symbol".into());
    let decimals = 123;

    let mut contract = Contract::default();

    assert_eq!(FTMeta::name(&contract), None);
    assert_eq!(FTMeta::symbol(&contract), None);
    assert_eq!(FTMeta::decimals(&contract), 0);

    contract.metadata.name = name.clone();
    contract.metadata.symbol = symbol.clone();
    contract.metadata.decimals = decimals;

    assert_eq!(FTMeta::name(&contract), name);
    assert_eq!(FTMeta::symbol(&contract), symbol);
    assert_eq!(FTMeta::decimals(&contract), decimals);
}

#[test]
fn mint() {
    let mut contract = Contract::default();

    FTMint::mint(&mut contract, 1.into(), AMOUNT.into()).unwrap();

    assert_eq!(
        FTTransfer {
            from: ActorId::zero(),
            to: 1.into(),
            amount: AMOUNT.into()
        },
        utils::last_reply()
    );
    assert_eq!(FTCore::balance_of(&contract, 1.into()), AMOUNT.into());
    assert_eq!(FTCore::total_supply(&contract), AMOUNT.into());
}

#[test]
fn mint_failures() {
    let mut contract = Contract::default();

    assert_eq!(
        FTMint::mint(&mut contract, ActorId::zero(), Amount::default()),
        Err(FTError::ZeroRecipientAddress)
    );

    FTMint::mint(&mut contract, 1.into(), 1.into()).unwrap();

    assert_eq!(
        FTMint::mint(&mut contract, 1.into(), Amount::MAX),
        Err(FTError::InsufficientAmount)
    );
}

#[test]
fn burn() {
    let mut contract = Contract::default();

    FTMint::mint(&mut contract, 1.into(), AMOUNT.into()).unwrap();
    FTBurn::burn(&mut contract, 1.into(), (AMOUNT - REMAINDER).into()).unwrap();

    assert_eq!(
        FTTransfer {
            from: 1.into(),
            to: ActorId::zero(),
            amount: (AMOUNT - REMAINDER).into()
        },
        utils::last_reply()
    );
    assert_eq!(FTCore::balance_of(&contract, 1.into()), REMAINDER.into());
    assert_eq!(FTCore::total_supply(&contract), REMAINDER.into());
}

#[test]
fn burn_failures() {
    let mut contract = Contract::default();

    assert_eq!(
        FTBurn::burn(&mut contract, ActorId::zero(), Amount::default()),
        Err(FTError::ZeroSenderAddress)
    );

    FTMint::mint(&mut contract, 1.into(), AMOUNT.into()).unwrap();

    assert_eq!(
        FTBurn::burn(&mut contract, 1.into(), (AMOUNT + 1).into()),
        Err(FTError::InsufficientAmount)
    );
}

#[test]
fn transfer() {
    let mut contract = Contract::default();

    FTMint::mint(&mut contract, 1.into(), AMOUNT.into()).unwrap();
    msg::set_source(1.into());
    FTCore::transfer(&mut contract, 2.into(), REMAINDER.into()).unwrap();

    assert_eq!(
        FTTransfer {
            from: 1.into(),
            to: 2.into(),
            amount: REMAINDER.into()
        },
        utils::last_reply()
    );
    assert_eq!(
        FTCore::balance_of(&contract, 1.into()),
        (AMOUNT - REMAINDER).into()
    );
    assert_eq!(FTCore::balance_of(&contract, 2.into()), REMAINDER.into());
}

#[test]
fn transfer_failures() {
    let mut contract = Contract::default();

    msg::set_source(1.into());

    assert_eq!(
        FTCore::transfer(&mut contract, ActorId::zero(), Amount::default()),
        Err(FTError::ZeroRecipientAddress)
    );

    FTMint::mint(&mut contract, 1.into(), AMOUNT.into()).unwrap();

    assert_eq!(
        FTCore::transfer(&mut contract, 2.into(), (AMOUNT + 1).into()),
        Err(FTError::InsufficientAmount)
    );
}

#[test]
fn approve() {
    let mut contract = Contract::default();

    msg::set_source(1.into());
    FTCore::approve(&mut contract, 2.into(), AMOUNT.into()).unwrap();

    assert_eq!(
        FTApproval {
            owner: 1.into(),
            operator: 2.into(),
            amount: AMOUNT.into()
        },
        utils::last_reply()
    );
    assert_eq!(
        FTCore::allowance(&contract, 1.into(), 2.into()),
        AMOUNT.into()
    );

    FTCore::increase_allowance(&mut contract, 2.into(), AMOUNT.into()).unwrap();

    assert_eq!(
        FTApproval {
            owner: 1.into(),
            operator: 2.into(),
            amount: (AMOUNT * 2).into()
        },
        utils::last_reply()
    );
    assert_eq!(
        FTCore::allowance(&contract, 1.into(), 2.into()),
        (AMOUNT * 2).into()
    );

    FTCore::decrease_allowance(&mut contract, 2.into(), REMAINDER.into()).unwrap();

    assert_eq!(
        FTApproval {
            owner: 1.into(),
            operator: 2.into(),
            amount: (AMOUNT * 2 - REMAINDER).into()
        },
        utils::last_reply()
    );
    assert_eq!(
        FTCore::allowance(&contract, 1.into(), 2.into()),
        (AMOUNT * 2 - REMAINDER).into()
    );
}

#[test]
fn approve_failures() {
    let mut contract = Contract::default();

    assert_eq!(
        FTCore::approve(&mut contract, ActorId::zero(), Amount::default()),
        Err(FTError::ZeroRecipientAddress)
    );

    FTCore::approve(&mut contract, 2.into(), 1.into()).unwrap();

    assert_eq!(
        FTCore::decrease_allowance(&mut contract, 2.into(), 2.into()),
        Err(FTError::InsufficientAllowance)
    );
    assert_eq!(
        FTCore::increase_allowance(&mut contract, 2.into(), Amount::MAX),
        Err(FTError::InsufficientAllowance)
    );
}

#[test]
fn transfer_from() {
    let mut contract = Contract::default();

    FTMint::mint(&mut contract, 1.into(), AMOUNT.into()).unwrap();
    msg::set_source(1.into());
    FTCore::approve(&mut contract, 3.into(), AMOUNT.into()).unwrap();
    msg::set_source(3.into());
    FTCore::transfer_from(&mut contract, 1.into(), 2.into(), REMAINDER.into()).unwrap();

    assert_eq!(
        FTCore::allowance(&contract, 1.into(), 3.into()),
        (AMOUNT - REMAINDER).into()
    );
    assert_eq!(
        FTTransfer {
            from: 1.into(),
            to: 2.into(),
            amount: REMAINDER.into()
        },
        utils::last_reply()
    );
}

#[test]
fn transfer_from_failures() {
    let mut contract = Contract::default();

    assert_eq!(
        FTCore::transfer_from(&mut contract, ActorId::zero(), 2.into(), Amount::default()),
        Err(FTError::ZeroSenderAddress)
    );

    msg::set_source(1.into());
    FTCore::approve(&mut contract, 2.into(), AMOUNT.into()).unwrap();
    msg::set_source(2.into());

    assert_eq!(
        FTCore::transfer_from(&mut contract, 1.into(), 2.into(), (AMOUNT + 1).into()),
        Err(FTError::InsufficientAllowance)
    );
}
