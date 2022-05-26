use codec::Encode;
use gear_lib::multitoken::io::*;
use gstd::{ActorId, String};
use gtest::{Program, System};
use multitoken_io::*;

const USERS: &[u64] = &[3, 4, 5, 0];
const ZERO_ID: ActorId = ActorId::new([0u8; 32]);
const TOKEN_AMOUNT: u128 = 100;
const TOKENS_TO_BURN: u128 = 50;
const TOKEN_ID: u128 = 0;

fn init(sys: &System) -> Program {
    sys.init_logger();

    let mtk = Program::current(sys);
    let res = mtk.send(
        USERS[0],
        InitMTK {
            name: String::from("MTK Simple"),
            symbol: String::from("MTK"),
            base_uri: String::from("http://mtk.simple"),
        },
    );

    assert!(res.log().is_empty());
    mtk
}

fn init_with_mint(sys: &System) {
    sys.init_logger();
    let mtk = Program::current(sys);
    let res = mtk.send(
        USERS[0],
        InitMTK {
            name: String::from("MTK Simple"),
            symbol: String::from("MTK"),
            base_uri: String::from("http://mtk.simple"),
        },
    );

    assert!(res.log().is_empty());

    let res = mtk.send(
        USERS[0],
        MyMTKAction::Mint {
            amount: TOKEN_AMOUNT,
            token_metadata: None,
        },
    );

    assert!(res.contains(&(
        USERS[0],
        MTKEvent::Transfer {
            operator: USERS[0].into(),
            from: ZERO_ID,
            to: USERS[0].into(),
            ids: vec![TOKEN_ID],
            amounts: vec![TOKEN_AMOUNT],
        }
        .encode()
    )));
}

#[test]
fn mint() {
    let sys = System::new();
    init_with_mint(&sys);
}

#[test]
fn mint_failures() {
    let sys = System::new();
    let mtk = init(&sys);

    // Must fail since minting to ZERO_ID
    let res = mtk.send(
        0,
        MyMTKAction::Mint {
            amount: TOKEN_AMOUNT,
            token_metadata: None,
        },
    );
    assert!(res.main_failed());

    // Must fail since provided meta for amount > 1
    let meta = TokenMetadata {
        title: Some(String::from("Kitty")),
        description: Some(String::from("Just a test kitty")),
        media: Some(String::from("www.example.com/erc1155/kitty.png")),
        reference: Some(String::from("www.example.com/erc1155/kitty")),
    };
    let res = mtk.send(
        0,
        MyMTKAction::Mint {
            amount: TOKEN_AMOUNT,
            token_metadata: Some(meta),
        },
    );
    assert!(res.main_failed());
}

#[test]
fn mint_batch() {
    let sys = System::new();
    let mtk = init(&sys);

    let res = mtk.send(
        USERS[0],
        MyMTKAction::MintBatch {
            ids: vec![1u128, 2u128],
            amounts: vec![TOKEN_AMOUNT, TOKEN_AMOUNT],
            tokens_metadata: vec![None, None],
        },
    );

    let codec = MTKEvent::Transfer {
        operator: USERS[0].into(),
        from: ZERO_ID,
        to: USERS[0].into(),
        ids: vec![1u128, 2u128],
        amounts: vec![TOKEN_AMOUNT, TOKEN_AMOUNT],
    }
    .encode();

    assert!(res.contains(&(USERS[0], codec)));
}

#[test]
fn burn() {
    let sys = System::new();
    let mtk = init(&sys);

    let res = mtk.send(
        USERS[0],
        MyMTKAction::Mint {
            amount: TOKEN_AMOUNT,
            token_metadata: None,
        },
    );

    assert!(res.contains(&(
        USERS[0],
        MTKEvent::Transfer {
            operator: USERS[0].into(),
            from: ZERO_ID,
            to: USERS[0].into(),
            ids: vec![TOKEN_ID],
            amounts: vec![TOKEN_AMOUNT],
        }
        .encode()
    )));

    let res = mtk.send(
        USERS[0],
        MyMTKAction::Burn {
            id: TOKEN_ID,
            amount: TOKEN_AMOUNT,
        },
    );

    assert!(res.contains(&(
        USERS[0],
        MTKEvent::Transfer {
            operator: USERS[0].into(),
            from: USERS[0].into(),
            to: ZERO_ID,
            ids: vec![TOKEN_ID],
            amounts: vec![TOKEN_AMOUNT],
        }
        .encode()
    )));
}

#[test]
fn burn_failures() {
    let sys = System::new();
    let mtk = init(&sys);

    let res = mtk.send(
        USERS[0],
        MyMTKAction::Mint {
            amount: TOKEN_AMOUNT,
            token_metadata: None,
        },
    );

    assert!(res.contains(&(
        USERS[0],
        MTKEvent::Transfer {
            operator: USERS[0].into(),
            from: ZERO_ID,
            to: USERS[0].into(),
            ids: vec![TOKEN_ID],
            amounts: vec![TOKEN_AMOUNT],
        }
        .encode()
    )));

    // Must fail since we do not have enough tokens
    let res = mtk.send(
        USERS[0],
        MyMTKAction::Burn {
            id: TOKEN_ID,
            amount: TOKEN_AMOUNT + 1,
        },
    );

    assert!(res.main_failed());
}

#[test]
fn burn_batch() {
    let sys = System::new();
    let mtk = init(&sys);

    let res = mtk.send(
        USERS[0],
        MyMTKAction::MintBatch {
            ids: vec![1u128, 2u128],
            amounts: vec![TOKEN_AMOUNT, TOKEN_AMOUNT],
            tokens_metadata: vec![None, None],
        },
    );

    let codec = MTKEvent::Transfer {
        operator: USERS[0].into(),
        from: ZERO_ID,
        to: USERS[0].into(),
        ids: vec![1u128, 2u128],
        amounts: vec![TOKEN_AMOUNT, TOKEN_AMOUNT],
    }
    .encode();

    assert!(res.contains(&(USERS[0], codec)));

    let res = mtk.send(
        USERS[0],
        MyMTKAction::BurnBatch {
            ids: vec![1u128, 2u128],
            amounts: vec![TOKENS_TO_BURN, TOKENS_TO_BURN],
        },
    );

    let codec = MTKEvent::Transfer {
        operator: USERS[0].into(),
        from: USERS[0].into(),
        to: ZERO_ID,
        ids: vec![1u128, 2u128],
        amounts: vec![TOKENS_TO_BURN, TOKENS_TO_BURN],
    }
    .encode();
    assert!(res.contains(&(USERS[0], codec)));
}

#[test]
fn balance() {
    let sys = System::new();
    let mtk = init(&sys);
    let res = mtk.send(
        USERS[0],
        MyMTKAction::Mint {
            amount: TOKEN_AMOUNT,
            token_metadata: None,
        },
    );

    assert!(res.contains(&(
        USERS[0],
        MTKEvent::Transfer {
            operator: USERS[0].into(),
            from: ZERO_ID,
            to: USERS[0].into(),
            ids: vec![TOKEN_ID],
            amounts: vec![TOKEN_AMOUNT],
        }
        .encode()
    )));

    let res = mtk.send(
        USERS[0],
        MyMTKAction::BalanceOf {
            account: USERS[0].into(),
            id: TOKEN_ID,
        },
    );

    assert!(res.contains(&(
        USERS[0],
        MTKEvent::BalanceOf(vec![BalanceReply {
            account: USERS[0].into(),
            id: TOKEN_ID,
            amount: TOKEN_AMOUNT,
        }])
        .encode()
    )));
}

#[test]
fn balance_of_batch() {
    let sys = System::new();
    let mtk = init(&sys);

    let res = mtk.send(
        USERS[0],
        MyMTKAction::MintBatch {
            ids: vec![1u128, 2u128],
            amounts: vec![TOKEN_AMOUNT, TOKEN_AMOUNT],
            tokens_metadata: vec![None, None],
        },
    );

    let codec = MTKEvent::Transfer {
        operator: USERS[0].into(),
        from: ZERO_ID,
        to: USERS[0].into(),
        ids: vec![1u128, 2u128],
        amounts: vec![TOKEN_AMOUNT, TOKEN_AMOUNT],
    }
    .encode();

    assert!(res.contains(&(USERS[0], codec)));

    let accounts: Vec<ActorId> = vec![USERS[0].into(), USERS[0].into()];

    let res = mtk.send(
        USERS[0],
        MyMTKAction::BalanceOfBatch {
            accounts,
            ids: vec![1u128, 2u128],
        },
    );

    let reply1 = BalanceReply {
        account: USERS[0].into(),
        id: 1,
        amount: TOKEN_AMOUNT,
    };

    let reply2 = BalanceReply {
        account: USERS[0].into(),
        id: 2,
        amount: TOKEN_AMOUNT,
    };

    let replies = vec![reply1, reply2];

    let codec = MTKEvent::BalanceOf(replies).encode();

    assert!(res.contains(&(USERS[0], codec)));
}

#[test]
fn transfer_from() {
    let sys = System::new();
    let mtk = init(&sys);

    let res = mtk.send(
        USERS[1],
        MyMTKAction::Mint {
            amount: TOKEN_AMOUNT,
            token_metadata: None,
        },
    );

    assert!(res.contains(&(
        USERS[1],
        MTKEvent::Transfer {
            operator: USERS[1].into(),
            from: ZERO_ID,
            to: USERS[1].into(),
            ids: vec![TOKEN_ID],
            amounts: vec![TOKEN_AMOUNT],
        }
        .encode()
    )));

    let from = USERS[1];
    let to = USERS[2];

    let res = mtk.send(
        from,
        MyMTKAction::TransferFrom {
            from: from.into(),
            to: to.into(),
            id: TOKEN_ID,
            amount: 10,
        },
    );

    let codec = MTKEvent::Transfer {
        operator: from.into(),
        from: from.into(),
        to: to.into(),
        ids: vec![TOKEN_ID],
        amounts: vec![10],
    }
    .encode();
    assert!(res.contains(&(from, codec)));
}

#[test]
fn transfer_from_failures() {
    let sys = System::new();
    let mtk = init(&sys);

    let res = mtk.send(
        USERS[1],
        MyMTKAction::Mint {
            amount: TOKEN_AMOUNT,
            token_metadata: None,
        },
    );

    assert!(res.contains(&(
        USERS[1],
        MTKEvent::Transfer {
            operator: USERS[1].into(),
            from: ZERO_ID,
            to: USERS[1].into(),
            ids: vec![TOKEN_ID],
            amounts: vec![TOKEN_AMOUNT],
        }
        .encode()
    )));

    let from = USERS[1];

    let failed_res = mtk.send(
        from,
        MyMTKAction::TransferFrom {
            from: from.into(),
            to: ZERO_ID,
            id: TOKEN_ID,
            amount: 10,
        },
    );
    // must fail since we're sending to ZERO_ID
    assert!(failed_res.main_failed());

    let failed_res = mtk.send(
        from,
        MyMTKAction::TransferFrom {
            from: from.into(),
            to: ZERO_ID,
            id: TOKEN_ID,
            amount: TOKEN_AMOUNT + 100,
        },
    );
    // must fail since we're sending > balance
    assert!(failed_res.main_failed());

    let failed_res = mtk.send(
        from,
        MyMTKAction::TransferFrom {
            from: from.into(),
            to: from.into(),
            id: TOKEN_ID,
            amount: TOKEN_AMOUNT + 100,
        },
    );
    // must fail since same addresses
    assert!(failed_res.main_failed());
}

#[test]
fn batch_transfer_from() {
    let sys = System::new();
    let mtk = init(&sys);

    let to = USERS[1];
    let newuser = USERS[2];

    let res = mtk.send(
        to,
        MyMTKAction::MintBatch {
            ids: vec![1u128, 2u128],
            amounts: vec![TOKEN_AMOUNT, TOKEN_AMOUNT],
            tokens_metadata: vec![None, None],
        },
    );

    let codec = MTKEvent::Transfer {
        operator: USERS[1].into(),
        from: ZERO_ID,
        to: USERS[1].into(),
        ids: vec![1u128, 2u128],
        amounts: vec![TOKEN_AMOUNT, TOKEN_AMOUNT],
    }
    .encode();

    assert!(res.contains(&(USERS[1], codec)));

    let ret = mtk.send(
        to,
        MyMTKAction::BatchTransferFrom {
            from: to.into(),
            to: newuser.into(),
            ids: vec![1u128, 2u128],
            amounts: vec![5u128, 10u128],
        },
    );

    let codec = MTKEvent::Transfer {
        operator: to.into(),
        from: to.into(),
        to: newuser.into(),
        ids: vec![1u128, 2u128],
        amounts: vec![5u128, 10u128],
    }
    .encode();

    assert!(ret.contains(&(to, codec)));
}
