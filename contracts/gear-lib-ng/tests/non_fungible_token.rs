use gear_lib::{
    non_fungible_token::*,
    testing::{msg, utils},
    Amount, StorageProvider,
};
use gstd::{prelude::*, ActorId};
use pretty_assertions::assert_eq;

#[derive(Default, StorageProvider)]
struct Contract {
    #[storage_field]
    state: NFTState,
}

impl NonFungibleToken for Contract {
    fn reply_transfer(&self, transfer: NFTTransfer) -> Result<(), NFTError> {
        utils::set_last_reply(transfer);

        Ok(())
    }

    fn reply_approval(&self, approval: NFTApproval) -> Result<(), NFTError> {
        utils::set_last_reply(approval);

        Ok(())
    }

    fn reply_set_attribute(&self, set_attribute: NFTAttribute) -> Result<(), NFTError> {
        utils::set_last_reply(set_attribute);

        Ok(())
    }
}

#[test]
fn meta() {
    let key: Vec<u8> = "Name".into();
    let data: Vec<u8> = "Nuclear Fish Tank".into();
    let mut contract = Contract::default();

    NonFungibleToken::set_attribute(&mut contract, 0u8.into(), key.clone(), data.clone()).unwrap();

    assert_eq!(
        NFTAttribute {
            id: 0u8.into(),
            key: key.clone(),
            data: data.clone()
        },
        utils::last_reply()
    );
    assert_eq!(
        NFTMeta::get_attribute(&contract, 0u8.into(), key.clone()),
        Some(data)
    );
    assert_eq!(
        NFTMeta::get_attribute(&contract, 0u8.into(), "Nonexistent attribute".into()),
        None
    );
    assert_eq!(NFTMeta::get_attribute(&contract, 1u8.into(), key), None);
}

#[test]
fn mint() {
    let mut contract = Contract::default();

    NFTMint::mint(&mut contract, 1.into(), 0u8.into()).unwrap();

    assert_eq!(
        NFTTransfer {
            from: ActorId::zero(),
            to: 1.into(),
            id: 0u8.into()
        },
        utils::last_reply()
    );
    assert_eq!(NFTCore::owner_of(&contract, 0u8.into()), 1.into());
    assert_eq!(NFTCore::balance_of(&contract, 1.into()), 1.into());
    assert_eq!(NFTCore::total_supply(&contract), 1.into());
}

#[test]
fn mint_failures() {
    let mut contract = Contract::default();

    assert_eq!(
        NFTMint::mint(&mut contract, 0.into(), 0u8.into()),
        Err(NFTError::ZeroRecipientAddress)
    );

    NFTMint::mint(&mut contract, 1.into(), 0u8.into()).unwrap();

    assert_eq!(
        NFTMint::mint(&mut contract, 1.into(), 0u8.into()),
        Err(NFTError::TokenExists)
    );

    contract.state.total_supply = Amount::MAX;

    assert_eq!(
        NFTMint::mint(&mut contract, 1.into(), 1u8.into()),
        Err(NFTError::TokenNotExists)
    );
}

#[test]
fn burn() {
    let mut contract = Contract::default();

    NFTMint::mint(&mut contract, 1.into(), 0u8.into()).unwrap();
    NFTBurn::burn(&mut contract, 1.into(), 0u8.into()).unwrap();

    assert_eq!(
        NFTTransfer {
            from: 1.into(),
            to: ActorId::zero(),
            id: 0u8.into()
        },
        utils::last_reply(),
    );
    assert_eq!(NFTCore::owner_of(&contract, 0u8.into()), ActorId::zero());
    assert_eq!(NFTCore::balance_of(&contract, 1.into()), 0.into());
    assert_eq!(NFTCore::total_supply(&contract), 0.into());
}

#[test]
fn burn_failures() {
    let mut contract = Contract::default();

    assert_eq!(
        NFTBurn::burn(&mut contract, ActorId::zero(), 0u8.into()),
        Err(NFTError::ZeroSenderAddress)
    );

    assert_eq!(
        NFTBurn::burn(&mut contract, 1.into(), 0u8.into()),
        Err(NFTError::TokenNotExists)
    );
}

#[test]
fn transfer() {
    let mut contract = Contract::default();

    NFTMint::mint(&mut contract, 1.into(), 0u8.into()).unwrap();
    msg::set_source(1.into());
    NFTCore::transfer(&mut contract, 2.into(), 0u8.into()).unwrap();

    assert_eq!(
        NFTTransfer {
            from: 1.into(),
            to: 2.into(),
            id: 0u8.into()
        },
        utils::last_reply()
    );
    assert_eq!(NFTCore::balance_of(&contract, 1.into()), 0.into());
    assert_eq!(NFTCore::balance_of(&contract, 2.into()), 1.into());
    assert_eq!(NFTCore::owner_of(&contract, 0u8.into()), 2.into());
}

#[test]
fn transfer_failures() {
    let mut contract = Contract::default();

    assert_eq!(
        NFTCore::transfer(&mut contract, ActorId::zero(), 0u8.into()),
        Err(NFTError::ZeroRecipientAddress)
    );
    assert_eq!(
        NFTCore::transfer(&mut contract, 2.into(), 0u8.into()),
        Err(NFTError::TokenNotExists)
    );

    NFTMint::mint(&mut contract, 1.into(), 0u8.into()).unwrap();
    msg::set_source(2.into());

    assert_eq!(
        NFTCore::transfer(&mut contract, 2.into(), 0u8.into()),
        Err(NFTError::NotApproved)
    );
}

#[test]
fn approve() {
    let mut contract = Contract::default();

    NFTMint::mint(&mut contract, 1.into(), 0u8.into()).unwrap();
    msg::set_source(1.into());
    NFTCore::approve(&mut contract, 2.into(), Some(0u8.into()), true).unwrap();

    assert_eq!(
        NFTApproval {
            owner: 1.into(),
            operator: 2.into(),
            id: Some(0u8.into()),
            approved: true
        },
        utils::last_reply()
    );
    assert_eq!(
        NFTCore::allowance(&contract, 1.into(), 2.into(), Some(0u8.into())),
        true
    );

    NFTCore::approve(&mut contract, 2.into(), Some(0u8.into()), false).unwrap();

    assert_eq!(
        NFTApproval {
            owner: 1.into(),
            operator: 2.into(),
            id: Some(0u8.into()),
            approved: false
        },
        utils::last_reply()
    );
    assert_eq!(
        NFTCore::allowance(&contract, 1.into(), 2.into(), Some(0u8.into())),
        false
    );

    NFTCore::approve(&mut contract, 2.into(), None, true).unwrap();

    assert_eq!(
        NFTApproval {
            owner: 1.into(),
            operator: 2.into(),
            id: None,
            approved: true
        },
        utils::last_reply()
    );
    assert_eq!(
        NFTCore::allowance(&contract, 1.into(), 2.into(), None),
        true
    );
    assert_eq!(
        NFTCore::allowance(&contract, 1.into(), 2.into(), Some(0u8.into())),
        true
    );

    NFTCore::approve(&mut contract, 2.into(), None, false).unwrap();

    assert_eq!(
        NFTApproval {
            owner: 1.into(),
            operator: 2.into(),
            id: None,
            approved: false
        },
        utils::last_reply()
    );
    assert_eq!(
        NFTCore::allowance(&contract, 1.into(), 2.into(), None),
        false
    );
    assert_eq!(
        NFTCore::allowance(&contract, 1.into(), 2.into(), Some(0u8.into())),
        false
    );
}

#[test]
fn approved_transfer() {
    let mut contract = Contract::default();

    NFTMint::mint(&mut contract, 1.into(), 0u8.into()).unwrap();
    msg::set_source(1.into());
    NFTCore::approve(&mut contract, 2.into(), Some(0u8.into()), true).unwrap();
    msg::set_source(2.into());
    NFTCore::transfer(&mut contract, 2.into(), 0u8.into()).unwrap();

    assert_eq!(
        NFTCore::allowance(&contract, 1.into(), 2.into(), Some(0u8.into())),
        false
    );

    NFTCore::approve(&mut contract, 1.into(), None, true).unwrap();
    NFTMint::mint(&mut contract, 2.into(), 1u8.into()).unwrap();
    msg::set_source(1.into());
    NFTCore::transfer(&mut contract, 1.into(), 1u8.into()).unwrap();

    assert_eq!(
        NFTCore::allowance(&contract, 2.into(), 1.into(), None),
        true
    );
    assert_eq!(
        NFTCore::allowance(&contract, 2.into(), 1.into(), Some(0u8.into())),
        true
    );
}

#[test]
fn approve_failures() {
    let mut contract = Contract::default();

    assert_eq!(
        NFTCore::approve(&mut contract, ActorId::zero(), None, true),
        Err(NFTError::ZeroRecipientAddress)
    );
    assert_eq!(
        NFTCore::approve(&mut contract, 2.into(), Some(0u8.into()), true),
        Err(NFTError::TokenNotExists)
    );

    NFTMint::mint(&mut contract, 1.into(), 0u8.into()).unwrap();
    msg::set_source(2.into());

    assert_eq!(
        NFTCore::approve(&mut contract, 2.into(), Some(0u8.into()), true),
        Err(NFTError::NotApproved)
    );
}
