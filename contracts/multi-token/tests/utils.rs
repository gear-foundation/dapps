use codec::Encode;
pub use gear_lib::multitoken::{io::*, state::*};
use gstd::{ActorId, String};
pub use gtest::{Program, System};
use multitoken_io::*;

const ZERO_ID: ActorId = ActorId::new([0u8; 32]);

pub fn init_mtk(sys: &System, from: u64) {
    sys.init_logger();
    let mtk = Program::current(sys);

    let res = mtk.send(
        from,
        InitMTK {
            name: String::from("MTK Simple"),
            symbol: String::from("MTK"),
            base_uri: String::from("http://mtk.simple"),
        },
    );

    assert!(res.log().is_empty());
}

pub fn mint_internal(
    mtk: &Program,
    from: u64,
    amount: u128,
    token_id: u128,
    token_metadata: Option<TokenMetadata>,
    should_fail: bool,
) {
    let res = mtk.send(
        from,
        MyMTKAction::Mint {
            amount,
            token_metadata,
        },
    );
    if should_fail {
        assert!(res.main_failed());
    } else {
        assert!(res.contains(&(
            from,
            MTKEvent::Transfer {
                operator: from.into(),
                from: ZERO_ID,
                to: from.into(),
                ids: vec![token_id],
                amounts: vec![amount],
            }
            .encode()
        )));
    }
}

pub fn mint_batch_internal(
    mtk: &Program,
    from: u64,
    ids: Vec<u128>,
    amounts: Vec<u128>,
    tokens_metadata: Vec<Option<TokenMetadata>>,
) {
    let res = mtk.send(
        from,
        MyMTKAction::MintBatch {
            ids: ids.clone(),
            amounts: amounts.clone(),
            tokens_metadata,
        },
    );

    let codec = MTKEvent::Transfer {
        operator: from.into(),
        from: ZERO_ID,
        to: from.into(),
        ids,
        amounts,
    }
    .encode();
    assert!(res.contains(&(from, codec)));
}

pub fn burn_internal(mtk: &Program, from: u64, token_id: u128, amount: u128, should_fail: bool) {
    let res = mtk.send(
        from,
        MyMTKAction::Burn {
            id: token_id,
            amount,
        },
    );

    if should_fail {
        assert!(res.main_failed());
    } else {
        assert!(res.contains(&(
            from,
            MTKEvent::Transfer {
                operator: from.into(),
                from: from.into(),
                to: ZERO_ID,
                ids: vec![token_id],
                amounts: vec![amount],
            }
            .encode()
        )));
    }
}

pub fn burn_batch_internal(mtk: &Program, from: u64, ids: Vec<u128>, amounts: Vec<u128>) {
    let res = mtk.send(
        from,
        MyMTKAction::BurnBatch {
            ids: ids.clone(),
            amounts: amounts.clone(),
        },
    );

    let codec = MTKEvent::Transfer {
        operator: from.into(),
        from: from.into(),
        to: ZERO_ID,
        ids,
        amounts,
    }
    .encode();
    assert!(res.contains(&(from, codec)));
}

pub fn balance_internal(mtk: &Program, from: u64, token_id: u128, amount: u128) {
    let res = mtk.send(
        from,
        MyMTKAction::BalanceOf {
            account: from.into(),
            id: token_id,
        },
    );

    assert!(res.contains(&(
        from,
        MTKEvent::BalanceOf(vec![BalanceReply {
            account: from.into(),
            id: token_id,
            amount,
        }])
        .encode()
    )));
}

pub fn balance_of_batch_internal(
    mtk: &Program,
    from: u64,
    accounts: Vec<ActorId>,
    ids: Vec<u128>,
    amounts: Vec<u128>,
) {
    let res = mtk.send(
        from,
        MyMTKAction::BalanceOfBatch {
            accounts: accounts.clone(),
            ids: ids.clone(),
        },
    );
    let replies = accounts
        .iter()
        .zip(ids.iter())
        .zip(amounts.iter())
        .map(|((account, token_id), amount)| BalanceReply {
            account: *account,
            id: *token_id,
            amount: *amount,
        })
        .collect();

    let codec = MTKEvent::BalanceOf(replies).encode();

    assert!(res.contains(&(from, codec)));
}

pub fn transfer_internal(
    mtk: &Program,
    from: u64,
    to: u64,
    token_id: u128,
    amount: u128,
    should_fail: bool,
) {
    let res = mtk.send(
        from,
        MyMTKAction::TransferFrom {
            from: from.into(),
            to: to.into(),
            id: token_id,
            amount,
        },
    );

    let codec = MTKEvent::Transfer {
        operator: from.into(),
        from: from.into(),
        to: to.into(),
        ids: vec![token_id],
        amounts: vec![amount],
    }
    .encode();
    if should_fail {
        assert!(res.main_failed());
    } else {
        assert!(res.contains(&(from, codec)));
    }
}

pub fn transfer_batch_internal(
    mtk: &Program,
    from: u64,
    to: u64,
    ids: Vec<u128>,
    amounts: Vec<u128>,
) {
    let res = mtk.send(
        from,
        MyMTKAction::BatchTransferFrom {
            from: from.into(),
            to: to.into(),
            ids: ids.clone(),
            amounts: amounts.clone(),
        },
    );

    let codec = MTKEvent::Transfer {
        operator: from.into(),
        from: from.into(),
        to: to.into(),
        ids,
        amounts,
    }
    .encode();

    assert!(res.contains(&(from, codec)));
}

pub fn check_token_ids_for_owner(mtk: &Program, account: u64, ids: Vec<u128>) {
    match mtk.meta_state::<_, MTKQueryReply>(MTKQuery::TokensIDsForOwner(ActorId::from(account))) {
        MTKQueryReply::TokensIDsForOwner(true_ids) => {
            if true_ids != ids {
                panic!("Token ids for ({account:?}) differs. In tests: ({ids:?}), actually: ({true_ids:?}).");
            }
        }
        _ => {
            unreachable!(
                "Unreachable metastate reply for the MTKQuery::Balance payload has occured"
            );
        }
    }
}
pub fn check_balance(mtk: &Program, account: u64, token_id: u128, balance: u128) {
    match mtk.meta_state::<_, MTKQueryReply>(MTKQuery::BalanceOf(ActorId::from(account), token_id))
    {
        MTKQueryReply::Balance(true_balance) => {
            if balance != true_balance {
                panic!("Balance for ({token_id:?}) differs for ({account:?}). In tests: ({balance:?}), actually: ({true_balance:?})");
            }
        }
        _ => {
            unreachable!(
                "Unreachable metastate reply for the MTKQuery::Balance payload has occured"
            );
        }
    }
}
