use gstd::Encode;
use gtest::{Program, System};
use multisig_wallet_io::*;

const USERS: &[u64] = &[3, 4, 5, 6];

// We can test execute transaction only in case when we change a required confirmations count
fn common_init<'a>(sys: &'a System, users: &[u64]) -> Program<'a> {
    sys.init_logger();

    let wallet = Program::current(sys);

    wallet.send(
        users[0],
        MWInitConfig {
            owners: users.iter().copied().map(|x| x.into()).collect(),
            required: 2,
        },
    );

    sys.mint_to(USERS[0], 2_000_000_000);
    let res = wallet.send_with_value(
        users[0],
        MWAction::SubmitTransaction {
            destination: USERS[3].into(),
            data: vec![],
            value: 1000,
            description: None,
        },
        1_000_000_000,
    );

    let expect = MWEvent::Submission {
        transaction_id: 0.into(),
    };

    assert!(res.contains(&(users[0], expect.encode())));

    let res = wallet.send_with_value(
        users[0],
        MWAction::SubmitTransaction {
            destination: 1.into(),
            data: MWAction::ChangeRequiredConfirmationsCount(1).encode(),
            value: 0,
            description: None,
        },
        1_000_000_000,
    );

    let expect = MWEvent::Submission {
        transaction_id: 1.into(),
    };

    assert!(res.contains(&(users[0], expect.encode())));

    let res = wallet.send(users[1], MWAction::ConfirmTransaction(1.into()));

    let expect = MWEvent::Confirmation {
        sender: users[1].into(),
        transaction_id: 1.into(),
    };

    assert!(res.contains(&(users[1], expect.encode())));

    wallet
}

#[test]
fn common() {
    let sys = System::new();
    let wallet = common_init(&sys, &USERS[0..3]);
    let res = wallet.send(USERS[0], MWAction::ExecuteTransaction(0.into()));

    let expect = MWEvent::Execution {
        transaction_id: 0.into(),
    };

    assert!(res.contains(&(USERS[0], expect.encode())));
}

#[test]
fn owner_doesnt_exist() {
    let sys = System::new();
    let wallet = common_init(&sys, &USERS[0..3]);
    let res = wallet.send(USERS[3], MWAction::ExecuteTransaction(0.into()));

    assert!(res.main_failed());
}

#[test]
fn owner_not_confirmed() {
    let sys = System::new();
    let wallet = common_init(&sys, &USERS[0..3]);
    let res = wallet.send(USERS[1], MWAction::ExecuteTransaction(0.into()));

    assert!(res.main_failed());
}

#[test]
fn already_executed() {
    let sys = System::new();
    let wallet = common_init(&sys, &USERS[0..3]);
    let res = wallet.send(USERS[0], MWAction::ExecuteTransaction(0.into()));

    let expect = MWEvent::Execution {
        transaction_id: 0.into(),
    };

    assert!(res.contains(&(USERS[0], expect.encode())));

    let res = wallet.send(USERS[0], MWAction::ExecuteTransaction(0.into()));

    assert!(res.main_failed());
}

#[test]
fn not_confirmed() {
    let sys = System::new();
    sys.init_logger();

    let wallet = Program::current(&sys);

    wallet.send(
        USERS[0],
        MWInitConfig {
            owners: USERS.iter().copied().map(|x| x.into()).collect(),
            required: 2,
        },
    );

    sys.mint_to(USERS[0], 1_000_000_000);
    let res = wallet.send_with_value(
        USERS[0],
        MWAction::SubmitTransaction {
            destination: USERS[3].into(),
            data: vec![],
            value: 1000,
            description: None,
        },
        1_000_000_000,
    );

    let expect = MWEvent::Submission {
        transaction_id: 0.into(),
    };

    assert!(res.contains(&(USERS[0], expect.encode())));

    let res = wallet.send(USERS[0], MWAction::ExecuteTransaction(0.into()));

    assert!(!res.main_failed());
}
