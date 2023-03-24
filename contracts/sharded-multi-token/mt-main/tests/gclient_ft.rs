mod utils_gclient;

use gstd::{prelude::*, ActorId};
use mt_logic_io::TokenId;
use std::mem;
use utils_gclient::*;

#[tokio::test]
#[ignore]
async fn success_create_ft_gclient() -> gclient::Result<()> {
    let (api, program_id) = setup_gclient().await?;

    let mut tx_id = 0;
    let initial_amount = 1000000;

    let api = api.with(USER_ACCOUNTS[0])?;
    let mut listener = api.subscribe().await?;
    let user_account_0 = ActorId::new(api.account_id().clone().into());
    let token_id: TokenId = 1 << (mem::size_of::<TokenId>() * 8 / 2);
    mtoken_create(
        &api,
        &mut listener,
        &program_id,
        tx_id,
        initial_amount,
        String::from("https://example.com"),
        false,
    )
    .await?;
    assert_eq!(
        mtoken_get_balance(&api, &mut listener, &program_id, token_id, user_account_0).await?,
        initial_amount
    );
    tx_id += 1;

    let api = api.with(USER_ACCOUNTS[1])?;
    let mut listener = api.subscribe().await?;
    let user_account_1 = ActorId::new(api.account_id().clone().into());
    let token_id: TokenId = 2 << (mem::size_of::<TokenId>() * 8 / 2);
    mtoken_create(
        &api,
        &mut listener,
        &program_id,
        tx_id,
        initial_amount * 2,
        String::from("https://example.com"),
        false,
    )
    .await?;
    assert_eq!(
        mtoken_get_balance(&api, &mut listener, &program_id, token_id, user_account_1).await?,
        initial_amount * 2
    );
    tx_id += 1;

    let api = api.with(USER_ACCOUNTS[0])?;
    let mut listener = api.subscribe().await?;
    let token_id: TokenId = 3 << (mem::size_of::<TokenId>() * 8 / 2);
    mtoken_create(
        &api,
        &mut listener,
        &program_id,
        tx_id,
        initial_amount / 10000,
        String::from("https://example.com"),
        false,
    )
    .await?;
    assert_eq!(
        mtoken_get_balance(&api, &mut listener, &program_id, token_id, user_account_0).await?,
        initial_amount / 10000
    );

    Ok(())
}

#[tokio::test]
#[ignore]
async fn success_mint_batch_ft_gclient() -> gclient::Result<()> {
    let (api, program_id) = setup_gclient().await?;

    let user_account_1 = get_actor_id(&api, USER_ACCOUNTS[1])?;

    let mut tx_id = 0;
    let initial_amount = 1000000;
    let base_amount = 133700;
    let token_id: TokenId = 1 << (mem::size_of::<TokenId>() * 8 / 2);

    let (api, user_account_0) = gclient_with_account(api, USER_ACCOUNTS[0])?;
    let mut listener = api.subscribe().await?;

    api.transfer(
        user_account_1
            .encode()
            .as_slice()
            .try_into()
            .expect("Unexpected invalid user account id."),
        10000,
    )
    .await?;

    mtoken_create(
        &api,
        &mut listener,
        &program_id,
        tx_id,
        initial_amount,
        String::from("https://example.com"),
        false,
    )
    .await?;
    tx_id += 1;

    mtoken_mint_batch_ft(
        &api,
        &mut listener,
        &program_id,
        tx_id,
        token_id,
        vec![user_account_1],
        vec![base_amount],
    )
    .await?;
    assert_eq!(
        mtoken_get_balance(&api, &mut listener, &program_id, token_id, user_account_1).await?,
        base_amount
    );
    tx_id += 1;

    mtoken_mint_batch_ft(
        &api,
        &mut listener,
        &program_id,
        tx_id,
        token_id,
        vec![user_account_1],
        vec![base_amount * 2],
    )
    .await?;
    assert_eq!(
        mtoken_get_balance(&api, &mut listener, &program_id, token_id, user_account_1).await?,
        base_amount + base_amount * 2
    );
    tx_id += 1;

    mtoken_mint_batch_ft(
        &api,
        &mut listener,
        &program_id,
        tx_id,
        token_id,
        vec![user_account_0],
        vec![base_amount],
    )
    .await?;
    assert_eq!(
        mtoken_get_balance(&api, &mut listener, &program_id, token_id, user_account_0).await?,
        initial_amount + base_amount
    );

    Ok(())
}

#[tokio::test]
#[ignore]
async fn success_burn_batch_ft_gclient() -> gclient::Result<()> {
    let (api, program_id) = setup_gclient().await?;

    let user_account_1 = get_actor_id(&api, USER_ACCOUNTS[1])?;

    let mut tx_id = 0;
    let initial_amount = 1000000;
    let base_amount = 133700;
    let burn_amount = 10000;
    let token_id: TokenId = 1 << (mem::size_of::<TokenId>() * 8 / 2);

    let (api, user_account_0) = gclient_with_account(api, USER_ACCOUNTS[0])?;
    let mut listener = api.subscribe().await?;

    api.transfer(
        user_account_1
            .encode()
            .as_slice()
            .try_into()
            .expect("Unexpected invalid user account id."),
        10000,
    )
    .await?;

    mtoken_create(
        &api,
        &mut listener,
        &program_id,
        tx_id,
        initial_amount,
        String::from("https://example.com"),
        false,
    )
    .await?;
    tx_id += 1;

    mtoken_mint_batch_ft(
        &api,
        &mut listener,
        &program_id,
        tx_id,
        token_id,
        vec![user_account_1],
        vec![base_amount],
    )
    .await?;
    tx_id += 1;

    let (api, _) = gclient_with_account(api, USER_ACCOUNTS[1])?;
    let mut listener = api.subscribe().await?;

    mtoken_burn_batch_ft(
        &api,
        &mut listener,
        &program_id,
        tx_id,
        token_id,
        vec![user_account_1],
        vec![burn_amount],
    )
    .await?;
    assert_eq!(
        mtoken_get_balance(&api, &mut listener, &program_id, token_id, user_account_1).await?,
        base_amount - burn_amount
    );
    tx_id += 1;

    let (api, _) = gclient_with_account(api, USER_ACCOUNTS[0])?;
    let mut listener = api.subscribe().await?;

    mtoken_burn_batch_ft(
        &api,
        &mut listener,
        &program_id,
        tx_id,
        token_id,
        vec![user_account_0],
        vec![burn_amount],
    )
    .await?;
    assert_eq!(
        mtoken_get_balance(&api, &mut listener, &program_id, token_id, user_account_0).await?,
        initial_amount - burn_amount
    );
    tx_id += 1;

    let (api, _) = gclient_with_account(api, USER_ACCOUNTS[1])?;
    let mut listener = api.subscribe().await?;

    mtoken_approve(
        &api,
        &mut listener,
        &program_id,
        tx_id,
        user_account_0,
        true,
    )
    .await?;
    tx_id += 1;

    let (api, _) = gclient_with_account(api, USER_ACCOUNTS[0])?;
    let mut listener = api.subscribe().await?;

    mtoken_burn_batch_ft(
        &api,
        &mut listener,
        &program_id,
        tx_id,
        token_id,
        vec![user_account_1],
        vec![burn_amount],
    )
    .await?;
    assert_eq!(
        mtoken_get_balance(&api, &mut listener, &program_id, token_id, user_account_1).await?,
        base_amount - burn_amount - burn_amount
    );

    Ok(())
}

#[tokio::test]
#[ignore]
async fn success_approve_ft_gclient() -> gclient::Result<()> {
    let (api, program_id) = setup_gclient().await?;

    let user_account_1 = get_actor_id(&api, USER_ACCOUNTS[1])?;

    let mut tx_id = 0;

    let (api, user_account_0) = gclient_with_account(api, USER_ACCOUNTS[0])?;
    let mut listener = api.subscribe().await?;

    assert!(
        !mtoken_get_approval(
            &api,
            &mut listener,
            &program_id,
            user_account_0,
            user_account_1
        )
        .await?
    );
    mtoken_approve(
        &api,
        &mut listener,
        &program_id,
        tx_id,
        user_account_1,
        true,
    )
    .await?;
    assert!(
        mtoken_get_approval(
            &api,
            &mut listener,
            &program_id,
            user_account_0,
            user_account_1
        )
        .await?
    );
    tx_id += 1;

    mtoken_approve(
        &api,
        &mut listener,
        &program_id,
        tx_id,
        user_account_1,
        false,
    )
    .await?;
    assert!(
        !mtoken_get_approval(
            &api,
            &mut listener,
            &program_id,
            user_account_0,
            user_account_1
        )
        .await?
    );
    tx_id += 1;

    let (api, _) = gclient_with_account(api, USER_ACCOUNTS[1])?;
    let mut listener = api.subscribe().await?;

    mtoken_approve(
        &api,
        &mut listener,
        &program_id,
        tx_id,
        user_account_0,
        true,
    )
    .await?;
    assert!(
        mtoken_get_approval(
            &api,
            &mut listener,
            &program_id,
            user_account_1,
            user_account_0
        )
        .await?
    );
    tx_id += 1;

    mtoken_approve(
        &api,
        &mut listener,
        &program_id,
        tx_id,
        user_account_0,
        false,
    )
    .await?;
    assert!(
        !mtoken_get_approval(
            &api,
            &mut listener,
            &program_id,
            user_account_1,
            user_account_0
        )
        .await?
    );

    Ok(())
}

#[tokio::test]
#[ignore]
async fn success_transfer_ft_gclient() -> gclient::Result<()> {
    let (api, program_id) = setup_gclient().await?;

    let user_account_1 = get_actor_id(&api, USER_ACCOUNTS[1])?;

    let mut tx_id = 0;
    let initial_amount = 1000000;
    let transfer_amount = 50000;
    let transfer_return_amount = transfer_amount / 2;
    let token_id: TokenId = 1 << (mem::size_of::<TokenId>() * 8 / 2);

    let (api, user_account_0) = gclient_with_account(api, USER_ACCOUNTS[0])?;
    let mut listener = api.subscribe().await?;

    api.transfer(
        user_account_1
            .encode()
            .as_slice()
            .try_into()
            .expect("Unexpected invalid user account id."),
        10000,
    )
    .await?;

    mtoken_create(
        &api,
        &mut listener,
        &program_id,
        tx_id,
        initial_amount,
        String::from("https://example.com"),
        false,
    )
    .await?;
    tx_id += 1;

    mtoken_transfer(
        &api,
        &mut listener,
        &program_id,
        tx_id,
        token_id,
        user_account_0,
        user_account_1,
        transfer_amount,
    )
    .await?;
    assert_eq!(
        mtoken_get_balance(&api, &mut listener, &program_id, token_id, user_account_0).await?,
        initial_amount - transfer_amount
    );
    assert_eq!(
        mtoken_get_balance(&api, &mut listener, &program_id, token_id, user_account_1).await?,
        transfer_amount
    );
    tx_id += 1;

    let (api, _) = gclient_with_account(api, USER_ACCOUNTS[1])?;
    let mut listener = api.subscribe().await?;

    mtoken_transfer(
        &api,
        &mut listener,
        &program_id,
        tx_id,
        token_id,
        user_account_1,
        user_account_0,
        transfer_return_amount,
    )
    .await?;
    assert_eq!(
        mtoken_get_balance(&api, &mut listener, &program_id, token_id, user_account_0).await?,
        initial_amount - transfer_amount + transfer_return_amount
    );
    assert_eq!(
        mtoken_get_balance(&api, &mut listener, &program_id, token_id, user_account_1).await?,
        transfer_return_amount
    );
    tx_id += 1;

    mtoken_transfer(
        &api,
        &mut listener,
        &program_id,
        tx_id,
        token_id,
        user_account_1,
        user_account_0,
        transfer_return_amount,
    )
    .await?;
    assert_eq!(
        mtoken_get_balance(&api, &mut listener, &program_id, token_id, user_account_0).await?,
        initial_amount
    );
    assert_eq!(
        mtoken_get_balance(&api, &mut listener, &program_id, token_id, user_account_1).await?,
        0
    );

    Ok(())
}
