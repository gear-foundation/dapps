use gstd::{ActorId, Encode};
mod utils;
use multi_token_io::*;
use utils::*;

const USERS: &[u64] = &[3, 4, 5, 0];
const TOKEN_AMOUNT: u128 = 100;
const TOKEN_AMOUNT_NFT: u128 = 1;
const TOKENS_TO_BURN: u128 = 50;
const TOKEN_ID: u128 = 0;
const TOKENS_TO_TRANSFORM: u128 = 2;
const NFT_1_ID: u128 = 100001;
const NFT_2_ID: u128 = 100002;

#[test]
fn mint_success() {
    let sys = System::new();
    init_mtk(&sys, USERS[0]);
    let mtk = sys.get_program(1);
    // USERS[0] should have no token_ids before
    check_token_ids_for_owner(&mtk, USERS[0], vec![]);
    mint_internal(&mtk, USERS[0], TOKEN_ID, TOKEN_AMOUNT, None, None);
    check_balance(&mtk, USERS[0], TOKEN_ID, TOKEN_AMOUNT);
    // USERS[0] should have token_ids after minting
    check_token_ids_for_owner(&mtk, USERS[0], vec![TOKEN_ID]);

    mint_internal(&mtk, USERS[0], TOKEN_ID + 1, TOKEN_AMOUNT, None, None);
    mint_batch_internal(
        &mtk,
        USERS[0],
        vec![0u128, 1u128],
        vec![TOKEN_AMOUNT, TOKEN_AMOUNT],
        vec![None, None],
        None,
    );
    check_balance(&mtk, USERS[0], 0u128, 2 * TOKEN_AMOUNT);
    check_balance(&mtk, USERS[0], 1u128, 2 * TOKEN_AMOUNT);
}

#[test]
fn mint_failures() {
    let sys = System::new();
    init_mtk(&sys, USERS[0]);
    let mtk = sys.get_program(1);
    // MUST fail since we are minting to ZERO_ID
    mint_internal(
        &mtk,
        0,
        TOKEN_ID,
        TOKEN_AMOUNT,
        None,
        Some(MtkError::ZeroAddress),
    );
    // MUST fail since different lengths ids and amounts/tokens_metadata
    mint_batch_internal(
        &mtk,
        USERS[0],
        vec![0u128, 1u128, 2u128],
        vec![TOKEN_AMOUNT, TOKEN_AMOUNT],
        vec![None, None],
        Some(MtkError::LengthMismatch),
    );
    // MUST fail since ids not unique
    mint_batch_internal(
        &mtk,
        USERS[0],
        vec![0u128, 0u128],
        vec![TOKEN_AMOUNT, TOKEN_AMOUNT],
        vec![None, None],
        Some(MtkError::IdIsNotUnique),
    );

    // Success mint
    let meta = TokenMetadata {
        title: Some(String::from("Kitty")),
        description: Some(String::from("Just a test kitty")),
        media: Some(String::from("www.example.com/erc1155/kitty.png")),
        reference: Some(String::from("www.example.com/erc1155/kitty")),
    };
    mint_internal(
        &mtk,
        USERS[0],
        TOKEN_ID,
        TOKEN_AMOUNT_NFT,
        Some(meta.clone()),
        None,
    );

    // MUST fail since ids not unique
    mint_batch_internal(
        &mtk,
        USERS[0],
        vec![0u128, 1u128],
        vec![TOKEN_AMOUNT, TOKEN_AMOUNT],
        vec![None, None],
        Some(MtkError::TokenAlreadyExists),
    );

    // MUST fail since we are providing meta for amount > 1 (meta for FT)
    mint_internal(
        &mtk,
        USERS[0],
        TOKEN_ID + 1,
        TOKEN_AMOUNT,
        Some(meta),
        Some(MtkError::MintMetadataToFungibleToken),
    );
}

#[test]
fn burn() {
    let sys = System::new();
    init_mtk(&sys, USERS[0]);
    let mtk = sys.get_program(1);
    mint_internal(&mtk, USERS[0], TOKEN_ID, TOKEN_AMOUNT, None, None);
    check_balance(&mtk, USERS[0], TOKEN_ID, TOKEN_AMOUNT);
    burn_internal(&mtk, USERS[0], TOKEN_ID, TOKENS_TO_BURN, None);
    check_balance(&mtk, USERS[0], TOKEN_ID, TOKEN_AMOUNT - TOKENS_TO_BURN);

    mint_batch_internal(
        &mtk,
        USERS[0],
        vec![1u128, 2u128],
        vec![TOKEN_AMOUNT, TOKEN_AMOUNT],
        vec![None, None],
        None,
    );
    check_balance(&mtk, USERS[0], 1u128, TOKEN_AMOUNT);
    check_balance(&mtk, USERS[0], 2u128, TOKEN_AMOUNT);
    burn_batch_internal(
        &mtk,
        USERS[0],
        vec![1u128, 2u128],
        vec![TOKENS_TO_BURN, TOKENS_TO_BURN],
        None,
    );
    check_balance(&mtk, USERS[0], 1u128, TOKEN_AMOUNT - TOKENS_TO_BURN);
    check_balance(&mtk, USERS[0], 2u128, TOKEN_AMOUNT - TOKENS_TO_BURN);
}

#[test]
fn burn_failures() {
    let sys = System::new();
    init_mtk(&sys, USERS[0]);
    let mtk = sys.get_program(1);
    mint_internal(&mtk, USERS[0], TOKEN_ID, TOKEN_AMOUNT, None, None);
    // MUST fail since we do not have enough tokens
    burn_internal(
        &mtk,
        USERS[0],
        TOKEN_ID,
        TOKEN_AMOUNT + 1,
        Some(MtkError::NotEnoughBalance),
    );
    let meta = TokenMetadata {
        title: Some(String::from("Kitty")),
        description: Some(String::from("Just a test kitty")),
        media: Some(String::from("www.example.com/erc1155/kitty.png")),
        reference: Some(String::from("www.example.com/erc1155/kitty")),
    };
    mint_internal(
        &mtk,
        USERS[0],
        TOKEN_ID + 1,
        TOKEN_AMOUNT_NFT,
        Some(meta.clone()),
        None,
    );
    // MUST fail since amount greater than one for nft
    burn_internal(
        &mtk,
        USERS[0],
        TOKEN_ID + 1,
        TOKEN_AMOUNT,
        Some(MtkError::AmountGreaterThanOneForNft),
    );
    // MUST fail since different lengths ids and amounts
    burn_batch_internal(
        &mtk,
        USERS[0],
        vec![1u128, 2u128, 3u128],
        vec![TOKENS_TO_BURN, TOKENS_TO_BURN],
        Some(MtkError::LengthMismatch),
    );
}

#[test]
fn balance() {
    let sys = System::new();
    init_mtk(&sys, USERS[0]);
    let mtk = sys.get_program(1);
    mint_internal(&mtk, USERS[0], TOKEN_ID, TOKEN_AMOUNT, None, None);
    check_balance(&mtk, USERS[0], TOKEN_ID, TOKEN_AMOUNT);
    balance_internal(&mtk, USERS[0], TOKEN_ID, TOKEN_AMOUNT);
}

#[test]
fn balance_of_batch() {
    let sys = System::new();
    init_mtk(&sys, USERS[0]);
    let mtk = sys.get_program(1);
    mint_internal(&mtk, USERS[0], TOKEN_ID, TOKEN_AMOUNT, None, None);
    check_balance(&mtk, USERS[0], TOKEN_ID, TOKEN_AMOUNT);
    mint_internal(&mtk, USERS[1], TOKEN_ID + 1, TOKEN_AMOUNT, None, None);
    check_balance(&mtk, USERS[1], TOKEN_ID + 1, TOKEN_AMOUNT);
    balance_of_batch_internal(
        &mtk,
        USERS[0],
        vec![USERS[0].into(), USERS[1].into()],
        vec![TOKEN_ID, TOKEN_ID + 1],
        vec![TOKEN_AMOUNT, TOKEN_AMOUNT],
    );
}

#[test]
fn transfer_from() {
    let sys = System::new();
    init_mtk(&sys, USERS[0]);
    let mtk = sys.get_program(1);
    mint_internal(&mtk, USERS[0], TOKEN_ID, TOKEN_AMOUNT, None, None);
    check_balance(&mtk, USERS[0], TOKEN_ID, TOKEN_AMOUNT);
    // USERS[1] should have no token_ids before
    check_token_ids_for_owner(&mtk, USERS[1], vec![]);
    transfer_internal(&mtk, USERS[0], USERS[1], TOKEN_ID, TOKEN_AMOUNT, None);
    // check that the first user's balance decreased and the second's one increased
    check_balance(&mtk, USERS[0], TOKEN_ID, 0);
    check_balance(&mtk, USERS[1], TOKEN_ID, TOKEN_AMOUNT);
    // USERS[1] should have token_ids after
    check_token_ids_for_owner(&mtk, USERS[1], vec![TOKEN_ID]);
}

#[test]
fn transfer_from_failures() {
    let sys = System::new();
    init_mtk(&sys, USERS[0]);
    let mtk = sys.get_program(1);
    mint_internal(&mtk, USERS[0], TOKEN_ID, TOKEN_AMOUNT, None, None);
    check_balance(&mtk, USERS[0], TOKEN_ID, TOKEN_AMOUNT);
    // MUST fail since we are sending to a ZERO account
    transfer_internal(
        &mtk,
        USERS[0],
        0,
        TOKEN_ID,
        TOKEN_AMOUNT,
        Some(MtkError::ZeroAddress),
    );
    // MUST fail since caller is not owner or approved
    let res = mtk.send(
        USERS[1],
        MtkAction::TransferFrom {
            from: USERS[0].into(),
            to: USERS[1].into(),
            id: TOKEN_ID,
            amount: TOKEN_AMOUNT,
        },
    );
    assert!(res.contains(&(
        USERS[1],
        Err::<MtkEvent, MtkError>(MtkError::CallerIsNotOwnerOrApproved).encode()
    )));

    // MUST fail since we are sending more than we have
    transfer_internal(
        &mtk,
        USERS[0],
        USERS[1],
        TOKEN_ID,
        TOKEN_AMOUNT + 1,
        Some(MtkError::InsufficientBalanceForTransfer),
    );
    // MUST fail since we are sending to the same account
    transfer_internal(
        &mtk,
        USERS[0],
        USERS[0],
        TOKEN_ID,
        TOKEN_AMOUNT,
        Some(MtkError::SenderAndRecipientAddressesAreSame),
    );
    // MUST fail since different lengths ids and amounts
    transfer_batch_internal(
        &mtk,
        USERS[0],
        USERS[1],
        vec![1u128, 2u128, 3u128],
        vec![TOKEN_AMOUNT, TOKEN_AMOUNT],
        Some(MtkError::LengthMismatch),
    );
}

#[test]
fn transfer_from_batch() {
    let sys = System::new();
    init_mtk(&sys, USERS[0]);
    let mtk = sys.get_program(1);
    mint_batch_internal(
        &mtk,
        USERS[0],
        vec![1u128, 2u128],
        vec![TOKEN_AMOUNT, TOKEN_AMOUNT],
        vec![None, None],
        None,
    );
    check_balance(&mtk, USERS[0], 1u128, TOKEN_AMOUNT);
    check_balance(&mtk, USERS[0], 2u128, TOKEN_AMOUNT);
    transfer_batch_internal(
        &mtk,
        USERS[0],
        USERS[1],
        vec![1u128, 2u128],
        vec![TOKEN_AMOUNT, TOKEN_AMOUNT],
        None,
    );

    // check that the first user's balance decreased and the second's one increased
    check_balance(&mtk, USERS[0], 1u128, 0);
    check_balance(&mtk, USERS[0], 2u128, 0);

    check_balance(&mtk, USERS[1], 1u128, TOKEN_AMOUNT);
    check_balance(&mtk, USERS[1], 2u128, TOKEN_AMOUNT);
}

#[test]
fn test_approve_and_revoke() {
    let sys = System::new();
    init_mtk(&sys, USERS[0]);
    let mtk = sys.get_program(1);
    approve(&mtk, USERS[0], USERS[1], None);
    mint_internal(&mtk, USERS[0], TOKEN_ID, TOKEN_AMOUNT, None, None);
    check_balance(&mtk, USERS[0], TOKEN_ID, TOKEN_AMOUNT);
    // USERS[1] should have no token_ids before
    check_token_ids_for_owner(&mtk, USERS[1], vec![]);
    mtk.send(
        USERS[1],
        MtkAction::TransferFrom {
            from: USERS[0].into(),
            to: USERS[1].into(),
            id: TOKEN_ID,
            amount: TOKEN_AMOUNT,
        },
    );
    // check that the first user's balance decreased and the second's one increased
    check_balance(&mtk, USERS[0], TOKEN_ID, 0);
    check_balance(&mtk, USERS[1], TOKEN_ID, TOKEN_AMOUNT);
    // USERS[1] should have token_ids after
    check_token_ids_for_owner(&mtk, USERS[1], vec![TOKEN_ID]);

    revoke_approval(&mtk, USERS[0], USERS[1], None);
    let res = mtk.send(
        USERS[1],
        MtkAction::TransferFrom {
            from: USERS[0].into(),
            to: USERS[1].into(),
            id: TOKEN_ID,
            amount: TOKEN_AMOUNT,
        },
    );
    assert!(res.contains(&(
        USERS[1],
        Err::<MtkEvent, MtkError>(MtkError::CallerIsNotOwnerOrApproved).encode()
    )));
}

#[test]
fn transform() {
    let sys = System::new();
    init_mtk(&sys, USERS[0]);
    let mtk = sys.get_program(1);
    mint_internal(&mtk, USERS[0], TOKEN_ID, TOKEN_AMOUNT, None, None);
    check_balance(&mtk, USERS[0], TOKEN_ID, TOKEN_AMOUNT);
    let nfts = vec![BurnToNFT {
        account: ActorId::from(USERS[1]),
        nfts_ids: vec![NFT_1_ID, NFT_2_ID],
        nfts_metadata: vec![
            Some(TokenMetadata {
                title: Some(String::from("Kitty")),
                description: Some(String::from("Just a test kitty #1")),
                media: Some(String::from("www.example.com/erc1155/kitty.png")),
                reference: Some(String::from("www.example.com/erc1155/kitty")),
            }),
            Some(TokenMetadata {
                title: Some(String::from("Kitty")),
                description: Some(String::from("Just a test kitty #2")),
                media: Some(String::from("www.example.com/erc1155/kitty.png")),
                reference: Some(String::from("www.example.com/erc1155/kitty")),
            }),
        ],
    }];
    transform_internal(&mtk, USERS[0], TOKEN_ID, TOKENS_TO_TRANSFORM, nfts);
    // check that user actually has an NFT now
    check_balance(&mtk, USERS[1], NFT_1_ID, 1);
}
