#![feature(const_btree_new)]

#[path = "../src/common.rs"]
mod common;
use common::*;

#[cfg(test)]
use codec::Encode;
use gstd::{ActorId, String};
use gtest::{Log, Program, System};

const ZERO_ID: ActorId = ActorId::new([0u8; 32]);
const USERS: &'static [u64] = &[3, 4, 5];
const TOKEN_ID: u128 = 1;
const BALANCE: u128 = 100;

fn init(sys: &System) -> Program {
    sys.init_logger();

    let ft = Program::current(&sys);

    let init_config = InitConfig {
        name: String::from("Gear"),
        symbol: String::from("Gear"),
        base_uri: String::from("ipfs://"),
    };

    ft.send(USERS[0], init_config);
    return ft;
}

fn init_with_mint(sys: &System) {
    sys.init_logger();

    let ft = Program::current(&sys);

    let init_config = InitConfig {
        name: String::from("Gear"),
        symbol: String::from("Gear"),
        base_uri: String::from("ipfs://"),
    };

    let res = ft.send(USERS[0], init_config);

    assert!(res.log().is_empty());

    let res = ft.send(USERS[0], Action::Mint(USERS[1].into(), TOKEN_ID, BALANCE));

    assert!(res.contains(&(
        USERS[0],
        Event::TransferSingle(TransferSingleReply {
            operator: USERS[0].into(),
            from: ZERO_ID,
            to: USERS[1].into(),
            id: TOKEN_ID,
            amount: BALANCE,
        })
        .encode()
    )));
}

#[test]
fn mint() {
    let sys = System::new();
    init_with_mint(&sys);
}

#[test]
fn balance() {
    let sys = System::new();
    let ft = init(&sys);

    ft.send(USERS[0], Action::Mint(USERS[1].into(), TOKEN_ID, BALANCE));

    let res = ft.send(USERS[1], Action::BalanceOf(USERS[1].into(), TOKEN_ID));

    assert!(res.contains(&(USERS[1], Event::Balance(BALANCE).encode())));
}

#[test]
fn balance_of_batch() {
    let sys = System::new();
    let ft = init(&sys);

    ft.send(USERS[0], Action::Mint(USERS[1].into(), 1, BALANCE));
    ft.send(USERS[0], Action::Mint(USERS[2].into(), 2, BALANCE));

    let accounts: Vec<ActorId> = vec![USERS[1].into(), USERS[2].into()];

    let res = ft.send(
        USERS[0],
        Action::BalanceOfBatch(accounts, vec![1u128, 2u128]),
    );

    let reply1 = BalanceOfBatchReply {
        account: USERS[1].into(),
        id: 1,
        amount: BALANCE,
    };

    let reply2 = BalanceOfBatchReply {
        account: USERS[2].into(),
        id: 2,
        amount: BALANCE,
    };

    let replies = vec![reply1, reply2];

    let codec = Event::BalanceOfBatch(replies).encode();

    assert!(res.contains(&(USERS[0], codec)));
}

#[test]
fn mint_batch() {
    let sys = System::new();
    let ft = init(&sys);

    let res = ft.send(
        USERS[0],
        Action::MintBatch(USERS[1].into(), vec![1u128, 2u128], vec![BALANCE, BALANCE]),
    );

    let codec = Event::TransferBatch {
        operator: USERS[0].into(),
        from: ZERO_ID,
        to: USERS[1].into(),
        ids: vec![1u128, 2u128],
        values: vec![BALANCE, BALANCE],
    }
    .encode();

    assert!(res.contains(&(USERS[0], codec)));
}

#[test]
fn safe_transfer_from() {
    let sys = System::new();
    let ft = init(&sys);

    ft.send(USERS[0], Action::Mint(USERS[1].into(), TOKEN_ID, BALANCE));

    let from = USERS[1];
    let to = USERS[2];

    let res = ft.send(
        from,
        Action::SafeTransferFrom(from.into(), to.into(), TOKEN_ID, 10),
    );

    let failed_res = ft.send(
        from,
        Action::SafeTransferFrom(from.into(), ZERO_ID.into(), TOKEN_ID, 10),
    );

    assert!(failed_res.main_failed());

    let reply = TransferSingleReply {
        operator: from.into(),
        from: from.into(),
        to: to.into(),
        id: TOKEN_ID,
        amount: 10,
    };

    let codec = Event::TransferSingle(reply).encode();
    assert!(res.contains(&(from, codec)));

    // check two accounts balance
    let accounts: Vec<ActorId> = vec![from.into(), to.into()];
    let ids: Vec<u128> = vec![1, 1];
    let res = ft.send(USERS[0], Action::BalanceOfBatch(accounts, ids));

    let reply1 = BalanceOfBatchReply {
        account: from.into(),
        id: TOKEN_ID,
        amount: BALANCE - 10,
    };
    let reply2 = BalanceOfBatchReply {
        account: to.into(),
        id: TOKEN_ID,
        amount: 10,
    };

    let replies = vec![reply1, reply2];
    let codec = Event::BalanceOfBatch(replies).encode();

    assert!(res.contains(&(USERS[0], codec)));
}

#[test]
fn safe_batch_transfer_from() {
    let sys = System::new();
    let ft = init(&sys);

    let from = USERS[0];
    let to = USERS[1];
    let newuser = USERS[2];

    ft.send(
        from,
        Action::MintBatch(to.into(), vec![1u128, 2u128], vec![BALANCE, BALANCE]),
    );

    let ret = ft.send(
        to,
        Action::SafeBatchTransferFrom(
            to.into(),
            newuser.into(),
            vec![1u128, 2u128],
            vec![5u128, 10u128],
        ),
    );

    let codec = Event::TransferBatch {
        operator: to.into(),
        from: to.into(),
        to: newuser.into(),
        ids: vec![1u128, 2u128],
        values: vec![5u128, 10u128],
    }
    .encode();

    assert!(ret.contains(&(to, codec)));
}

#[test]
fn set_approval_for_all() {
    let sys = System::new();
    let ft = init(&sys);

    let owner = USERS[0];
    let operator = USERS[1];
    let other = USERS[2];

    ft.send(owner, Action::Mint(owner.into(), TOKEN_ID, BALANCE));

    let failed_res = ft.send(
        operator,
        Action::SafeTransferFrom(owner.into(), other.into(), 1, 10),
    );

    assert!(failed_res.main_failed());

    let ret = ft.send(owner, Action::SetApprovalForAll(operator.into(), true));

    let codec = Event::ApprovalForAll {
        owner: owner.into(),
        operator: operator.into(),
        approved: true,
    }
    .encode();

    assert!(ret.contains(&(owner, codec)));

    ft.send(
        operator,
        Action::SafeTransferFrom(owner.into(), other.into(), 1, 30),
    );

    let ret3 = ft.send(other, Action::BalanceOf(other.into(), 1));

    assert!(ret3.contains(&(other, Event::Balance(30).encode())));
}

#[test]
fn is_approved_for_all() {
    let sys = System::new();
    let ft = init(&sys);

    let owner = USERS[0];
    let operator = USERS[1];

    ft.send(owner, Action::SetApprovalForAll(operator.into(), true));

    let ret = ft.send(
        owner,
        Action::IsApprovedForAll(owner.into(), operator.into()),
    );
    let codec = Event::ApprovalForAll {
        owner: owner.into(),
        operator: operator.into(),
        approved: true,
    }
    .encode();

    assert!(ret.contains(&(owner, codec)));

    let newuser = USERS[2];
    let ret = ft.send(
        owner,
        Action::IsApprovedForAll(owner.into(), newuser.into()),
    );
    let codec = Event::ApprovalForAll {
        owner: owner.into(),
        operator: newuser.into(),
        approved: false,
    }
    .encode();

    assert!(ret.contains(&(owner, codec)));
}

#[test]
fn burn_batch() {
    let sys = System::new();
    let ft = init(&sys);

    let from = USERS[0];
    let user1 = USERS[1];

    ft.send(
        from,
        Action::MintBatch(user1.into(), vec![1u128, 2u128], vec![BALANCE, BALANCE]),
    );

    ft.send(user1, Action::BurnBatch(vec![1u128, 2u128], vec![10, 20]));

    let accounts: Vec<ActorId> = vec![user1.into(), user1.into()];

    let res = ft.send(
        USERS[0],
        Action::BalanceOfBatch(accounts, vec![1u128, 2u128]),
    );

    let reply1 = BalanceOfBatchReply {
        account: user1.into(),
        id: 1,
        amount: BALANCE - 10,
    };

    let reply2 = BalanceOfBatchReply {
        account: user1.into(),
        id: 2,
        amount: BALANCE - 20,
    };

    let replies = vec![reply1, reply2];
    let codec = Event::BalanceOfBatch(replies).encode();
    assert!(res.contains(&(from, codec)));

    // check ownership of ids
    let failed_res = ft.send(user1, Action::BurnBatch(vec![1u128, 3u128], vec![10, 20]));
    print!("main_failed {:?}", failed_res.main_failed());

    assert!(failed_res.main_failed());
}

#[test]
fn owner_of() {
    let sys = System::new();
    let ft = init(&sys);

    let from = USERS[0];
    let user1 = USERS[1];

    ft.send(from, Action::Mint(user1.into(), 1u128, BALANCE));
    let res = ft.send(user1, Action::OwnerOf(1u128));
    let log = Log::builder().payload(true);
    assert!(res.contains(&log));

    let res = ft.send(user1, Action::OwnerOf(2u128));
    let log = Log::builder().payload(false);
    assert!(res.contains(&log));
}

#[test]
fn owner_of_batch() {
    let sys = System::new();
    let ft = init(&sys);

    let from = USERS[0];
    let user1 = USERS[1];

    ft.send(
        from,
        Action::MintBatch(user1.into(), vec![1u128, 2u128], vec![BALANCE, BALANCE]),
    );

    let res = ft.send(user1, Action::OwnerOfBatch(vec![1u128, 2u128]));
    let log = Log::builder().payload(true);
    assert!(res.contains(&log));

    let res = ft.send(user1, Action::OwnerOfBatch(vec![3u128]));
    let log = Log::builder().payload(false);
    assert!(res.contains(&log));
}
