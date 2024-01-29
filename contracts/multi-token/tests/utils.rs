use gstd::{ActorId, Encode, String};
pub use gtest::{Program, System};
use multi_token_io::*;

const NFT_COUNT: u128 = 1;

pub fn init_mtk(sys: &System, from: u64) {
    sys.init_logger();
    let mtk = Program::current_opt(sys);

    let res = mtk.send(
        from,
        InitMtk {
            name: String::from("Mtk Simple"),
            symbol: String::from("Mtk"),
            base_uri: String::from("http://mtk.simple"),
        },
    );

    assert!(!res.main_failed());
}

pub fn mint_internal(
    mtk: &Program<'_>,
    from: u64,
    id: TokenId,
    amount: u128,
    token_metadata: Option<TokenMetadata>,
    error: Option<MtkError>,
) {
    let res = mtk.send(
        from,
        MtkAction::Mint {
            id,
            amount,
            token_metadata,
        },
    );

    if let Some(error) = error {
        assert!(res.contains(&(from, Err::<MtkEvent, MtkError>(error).encode())));
    } else {
        assert!(res.contains(&(
            from,
            Ok::<MtkEvent, MtkError>(MtkEvent::Transfer {
                from: ActorId::zero(),
                to: from.into(),
                ids: vec![id],
                amounts: vec![amount],
            })
            .encode()
        )));
    }
}

pub fn mint_batch_internal(
    mtk: &Program<'_>,
    from: u64,
    ids: Vec<u128>,
    amounts: Vec<u128>,
    tokens_metadata: Vec<Option<TokenMetadata>>,
    error: Option<MtkError>,
) {
    let res = mtk.send(
        from,
        MtkAction::MintBatch {
            ids: ids.clone(),
            amounts: amounts.clone(),
            tokens_metadata,
        },
    );

    let expected = if let Some(error) = error {
        Err(error)
    } else {
        Ok(MtkEvent::Transfer {
            from: ActorId::zero(),
            to: from.into(),
            ids,
            amounts,
        })
    };
    assert!(res.contains(&(from, expected.encode())));
}

pub fn burn_internal(
    mtk: &Program<'_>,
    from: u64,
    token_id: u128,
    amount: u128,
    error: Option<MtkError>,
) {
    let res = mtk.send(
        from,
        MtkAction::Burn {
            id: token_id,
            amount,
        },
    );

    if let Some(error) = error {
        assert!(res.contains(&(from, Err::<MtkEvent, MtkError>(error).encode())));
    } else {
        assert!(res.contains(&(
            from,
            Ok::<MtkEvent, MtkError>(MtkEvent::Transfer {
                from: from.into(),
                to: ActorId::zero(),
                ids: vec![token_id],
                amounts: vec![amount],
            })
            .encode()
        )));
    }
}

pub fn burn_batch_internal(
    mtk: &Program<'_>,
    from: u64,
    ids: Vec<u128>,
    amounts: Vec<u128>,
    error: Option<MtkError>,
) {
    let res = mtk.send(
        from,
        MtkAction::BurnBatch {
            ids: ids.clone(),
            amounts: amounts.clone(),
        },
    );

    let expected = if let Some(error) = error {
        Err(error)
    } else {
        Ok(MtkEvent::Transfer {
            from: from.into(),
            to: ActorId::zero(),
            ids,
            amounts,
        })
    };
    assert!(res.contains(&(from, expected.encode())));
}

pub fn balance_internal(mtk: &Program<'_>, from: u64, token_id: u128, amount: u128) {
    let res = mtk.send(
        from,
        MtkAction::BalanceOf {
            account: from.into(),
            id: token_id,
        },
    );

    assert!(res.contains(&(
        from,
        Ok::<MtkEvent, MtkError>(MtkEvent::BalanceOf(vec![BalanceReply {
            account: from.into(),
            id: token_id,
            amount,
        }]))
        .encode()
    )));
}

pub fn balance_of_batch_internal(
    mtk: &Program<'_>,
    from: u64,
    accounts: Vec<ActorId>,
    ids: Vec<u128>,
    amounts: Vec<u128>,
) {
    let res = mtk.send(
        from,
        MtkAction::BalanceOfBatch {
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

    let expected = Ok::<MtkEvent, MtkError>(MtkEvent::BalanceOf(replies)).encode();

    assert!(res.contains(&(from, expected)));
}

pub fn transfer_internal(
    mtk: &Program<'_>,
    from: u64,
    to: u64,
    token_id: u128,
    amount: u128,
    error: Option<MtkError>,
) {
    let res = mtk.send(
        from,
        MtkAction::TransferFrom {
            from: from.into(),
            to: to.into(),
            id: token_id,
            amount,
        },
    );

    let expected = if let Some(error) = error {
        Err(error)
    } else {
        Ok(MtkEvent::Transfer {
            from: from.into(),
            to: to.into(),
            ids: vec![token_id],
            amounts: vec![amount],
        })
    };
    assert!(res.contains(&(from, expected.encode())));
}

pub fn transfer_batch_internal(
    mtk: &Program<'_>,
    from: u64,
    to: u64,
    ids: Vec<u128>,
    amounts: Vec<u128>,
    error: Option<MtkError>,
) {
    let res = mtk.send(
        from,
        MtkAction::BatchTransferFrom {
            from: from.into(),
            to: to.into(),
            ids: ids.clone(),
            amounts: amounts.clone(),
        },
    );

    let expected = if let Some(error) = error {
        Err(error)
    } else {
        Ok(MtkEvent::Transfer {
            from: from.into(),
            to: to.into(),
            ids,
            amounts,
        })
    };
    assert!(res.contains(&(from, expected.encode())));
}

pub fn transform_internal(
    mtk: &Program<'_>,
    from: u64,
    token_id: u128,
    amount: u128,
    nfts: Vec<BurnToNFT>,
) {
    let mut ids = vec![];
    for burn_info in &nfts {
        for id in &burn_info.nfts_ids {
            ids.push(*id);
        }
    }
    let res = mtk.send(
        from,
        MtkAction::Transform {
            id: token_id,
            amount,
            nfts,
        },
    );

    let expected = Ok::<MtkEvent, MtkError>(MtkEvent::Transfer {
        from: ActorId::zero(),
        to: ActorId::zero(),
        ids,
        amounts: vec![NFT_COUNT; amount as usize],
    })
    .encode();
    assert!(res.contains(&(from, expected)));
}

pub fn approve(mtk: &Program<'_>, from: u64, to: u64, error: Option<MtkError>) {
    let res = mtk.send(from, MtkAction::Approve { account: to.into() });

    let expected = if let Some(error) = error {
        Err(error)
    } else {
        Ok(MtkEvent::Approval {
            from: from.into(),
            to: to.into(),
        })
    };
    assert!(res.contains(&(from, expected.encode())));
}

pub fn revoke_approval(mtk: &Program<'_>, from: u64, to: u64, error: Option<MtkError>) {
    let res = mtk.send(from, MtkAction::RevokeApproval { account: to.into() });
    let expected = if let Some(error) = error {
        Err(error)
    } else {
        Ok(MtkEvent::RevokeApproval {
            from: from.into(),
            to: to.into(),
        })
    };
    assert!(res.contains(&(from, expected.encode())));
}

pub fn check_token_ids_for_owner(mtk: &Program<'_>, account: u64, ids: Vec<u128>) {
    let state: State = mtk.read_state(0).expect("Can't read state");
    let true_ids = state.tokens_ids_for_owner(&ActorId::from(account));
    if true_ids != ids {
        panic!(
            "Token ids for ({account:?}) differs. In tests: ({ids:?}), actually: ({true_ids:?})."
        );
    }
}

pub fn check_balance(mtk: &Program<'_>, account: u64, token_id: u128, balance: u128) {
    let state: State = mtk.read_state(0).expect("Can't read state");
    let true_balance = state.get_balance(&ActorId::from(account), &token_id);

    if balance != true_balance {
        panic!("Balance for ({token_id:?}) differs for ({account:?}). In tests: ({balance:?}), actually: ({true_balance:?})");
    }
}
