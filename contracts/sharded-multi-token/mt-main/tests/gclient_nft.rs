mod utils_gclient;

use gstd::{prelude::*, ActorId};
use mt_logic_io::{TokenId, NFT_BIT};
use std::mem;
use utils_gclient::*;

#[tokio::test]
#[ignore]
async fn success_create_and_mint_batch_nft_gclient() -> gclient::Result<()> {
    let (api, program_id) = setup_gclient().await?;

    let mut tx_id = 0;
    // Abstract `collection` id
    let token_id: TokenId = 1 << (mem::size_of::<TokenId>() * 8 / 2) | NFT_BIT;
    // Abstract `edition`(copy) id
    let minted_id_1: TokenId = token_id | 1;
    let minted_id_2: TokenId = token_id | 2;

    let api = api.with(USER_ACCOUNTS[0])?;
    let mut listener = api.subscribe().await?;

    let user_account_1 = {
        let api = api.clone().with(USER_ACCOUNTS[1])?;
        ActorId::new(api.account_id().clone().into())
    };
    let user_account_2 = {
        let api = api.clone().with(USER_ACCOUNTS[2])?;
        ActorId::new(api.account_id().clone().into())
    };

    mtoken_create(
        &api,
        &mut listener,
        &program_id,
        tx_id,
        0,
        String::from("https://example.com"),
        true,
    )
    .await?;
    tx_id += 1;

    mtoken_mint_batch_nft(
        &api,
        &mut listener,
        &program_id,
        tx_id,
        token_id,
        vec![user_account_1, user_account_2],
    )
    .await?;

    assert_eq!(
        mtoken_get_balance(
            &api,
            &mut listener,
            &program_id,
            minted_id_1,
            user_account_1
        )
        .await?,
        1
    );
    assert_eq!(
        mtoken_get_balance(
            &api,
            &mut listener,
            &program_id,
            minted_id_2,
            user_account_2
        )
        .await?,
        1
    );

    Ok(())
}

#[tokio::test]
#[ignore]
async fn success_transfer_nft_gclient() -> gclient::Result<()> {
    let (api, program_id) = setup_gclient().await?;

    let mut tx_id = 0;
    // Abstract `collection` id
    let token_id: TokenId = 1 << (mem::size_of::<TokenId>() * 8 / 2) | NFT_BIT;
    // Abstract `edition`(copy) id
    let minted_id_1: TokenId = token_id | 1;
    let minted_id_2: TokenId = token_id | 2;

    let api = api.with(USER_ACCOUNTS[0])?;
    let mut listener = api.subscribe().await?;

    let user_account_1 = {
        let api = api.clone().with(USER_ACCOUNTS[1])?;
        ActorId::new(api.account_id().clone().into())
    };
    let user_account_2 = {
        let api = api.clone().with(USER_ACCOUNTS[2])?;
        ActorId::new(api.account_id().clone().into())
    };

    mtoken_create(
        &api,
        &mut listener,
        &program_id,
        tx_id,
        0,
        String::from("https://example.com"),
        true,
    )
    .await?;
    tx_id += 1;

    mtoken_mint_batch_nft(
        &api,
        &mut listener,
        &program_id,
        tx_id,
        token_id,
        vec![user_account_1, user_account_2],
    )
    .await?;
    tx_id += 1;

    {
        let api = api.clone().with(USER_ACCOUNTS[2])?;

        mtoken_transfer(
            &api,
            &mut listener,
            &program_id,
            tx_id,
            minted_id_2,
            user_account_2,
            user_account_1,
            0,
        )
        .await?;

        assert_eq!(
            mtoken_get_balance(
                &api,
                &mut listener,
                &program_id,
                minted_id_2,
                user_account_2
            )
            .await?,
            0
        );
        assert_eq!(
            mtoken_get_balance(
                &api,
                &mut listener,
                &program_id,
                minted_id_2,
                user_account_1
            )
            .await?,
            1
        );
    }

    tx_id += 1;

    {
        let api = api.clone().with(USER_ACCOUNTS[1])?;

        mtoken_transfer(
            &api,
            &mut listener,
            &program_id,
            tx_id,
            minted_id_1,
            user_account_1,
            user_account_2,
            0,
        )
        .await?;

        assert_eq!(
            mtoken_get_balance(
                &api,
                &mut listener,
                &program_id,
                minted_id_1,
                user_account_1
            )
            .await?,
            0
        );
        assert_eq!(
            mtoken_get_balance(
                &api,
                &mut listener,
                &program_id,
                minted_id_1,
                user_account_2
            )
            .await?,
            1
        );
    }

    Ok(())
}

#[tokio::test]
#[ignore]
async fn success_burn_nft_gclient() -> gclient::Result<()> {
    let (api, program_id) = setup_gclient().await?;

    let mut tx_id = 0;
    // Abstract `collection` id
    let token_id: TokenId = 1 << (mem::size_of::<TokenId>() * 8 / 2) | NFT_BIT;
    // Abstract `edition`(copy) id
    let minted_id_1: TokenId = token_id | 1;
    let minted_id_2: TokenId = token_id | 2;

    let api = api.with(USER_ACCOUNTS[0])?;
    let mut listener = api.subscribe().await?;

    let user_account_1 = {
        let api = api.clone().with(USER_ACCOUNTS[1])?;
        ActorId::new(api.account_id().clone().into())
    };
    let user_account_2 = {
        let api = api.clone().with(USER_ACCOUNTS[2])?;
        ActorId::new(api.account_id().clone().into())
    };

    mtoken_create(
        &api,
        &mut listener,
        &program_id,
        tx_id,
        0,
        String::from("https://example.com"),
        true,
    )
    .await?;
    tx_id += 1;

    mtoken_mint_batch_nft(
        &api,
        &mut listener,
        &program_id,
        tx_id,
        token_id,
        vec![user_account_1, user_account_2],
    )
    .await?;
    tx_id += 1;

    {
        let api = api.clone().with(USER_ACCOUNTS[1])?;
        mtoken_burn_nft(
            &api,
            &mut listener,
            &program_id,
            tx_id,
            minted_id_1,
            user_account_1,
        )
        .await?;

        assert_eq!(
            mtoken_get_balance(
                &api,
                &mut listener,
                &program_id,
                minted_id_1,
                user_account_1
            )
            .await?,
            0
        );
    }

    tx_id += 1;

    {
        let api = api.clone().with(USER_ACCOUNTS[2])?;
        mtoken_burn_nft(
            &api,
            &mut listener,
            &program_id,
            tx_id,
            minted_id_2,
            user_account_2,
        )
        .await?;

        assert_eq!(
            mtoken_get_balance(
                &api,
                &mut listener,
                &program_id,
                minted_id_2,
                user_account_2
            )
            .await?,
            0
        );
    }

    Ok(())
}
