use gstd::Encode;
use gtest::{Program, System};
use multisig_wallet::io::*;

const USERS: &[u64] = &[3, 4, 5, 6];

fn common_init<'a>(sys: &'a System, users: &[u64], required: u64) -> Program<'a> {
    sys.init_logger();

    let wallet = Program::current(sys);

    wallet.send(
        USERS[0],
        MWInitConfig {
            owners: users.iter().copied().map(|x| x.into()).collect(),
            required,
        },
    );

    wallet
}

#[test]
fn common() {
    let sys = System::new();
    let wallet = common_init(&sys, &USERS[0..3], 2);

    sys.mint_to(USERS[0], 1_000_000_000);
    let res = wallet.send_with_value(
        USERS[0],
        MWAction::SubmitTransaction {
            destination: 1.into(),
            data: MWAction::ChangeRequiredConfirmationsCount(1).encode(),
            value: 0,
            description: None,
        },
        1_000_000_000,
    );

    let expect = MWEvent::Submission {
        transaction_id: 0.into(),
    };

    assert!(res.contains(&(USERS[0], expect.encode())));

    let res = wallet.send(USERS[1], MWAction::ConfirmTransaction(0.into()));

    let expect = MWEvent::Confirmation {
        sender: USERS[1].into(),
        transaction_id: 0.into(),
    };

    assert!(res.contains(&(USERS[1], expect.encode())));

    let res = wallet.send(
        USERS[0],
        MWAction::SubmitTransaction {
            destination: USERS[3].into(),
            data: vec![],
            value: 1000,
            description: None,
        },
    );

    let expect = MWEvent::Submission {
        transaction_id: 1.into(),
    };

    assert!(res.contains(&(USERS[0], expect.encode())));

    let res = wallet.send(USERS[1], MWAction::ConfirmTransaction(1.into()));

    assert!(res.main_failed());
}

#[test]
fn make_greater() {
    let sys = System::new();
    let wallet = common_init(&sys, &USERS[0..3], 2);

    sys.mint_to(USERS[0], 1_000_000_000);
    let res = wallet.send_with_value(
        USERS[0],
        MWAction::SubmitTransaction {
            destination: 1.into(),
            data: MWAction::ChangeRequiredConfirmationsCount(3).encode(),
            value: 0,
            description: None,
        },
        1_000_000_000,
    );

    let expect = MWEvent::Submission {
        transaction_id: 0.into(),
    };

    assert!(res.contains(&(USERS[0], expect.encode())));

    let res = wallet.send(USERS[1], MWAction::ConfirmTransaction(0.into()));

    let expect = MWEvent::Confirmation {
        sender: USERS[1].into(),
        transaction_id: 0.into(),
    };

    assert!(res.contains(&(USERS[1], expect.encode())));

    let res = wallet.send(
        USERS[0],
        MWAction::SubmitTransaction {
            destination: USERS[3].into(),
            data: vec![],
            value: 1000,
            description: None,
        },
    );

    let expect = MWEvent::Submission {
        transaction_id: 1.into(),
    };

    assert!(res.contains(&(USERS[0], expect.encode())));

    let res = wallet.send(USERS[1], MWAction::ConfirmTransaction(1.into()));

    let expect = MWEvent::Confirmation {
        sender: USERS[1].into(),
        transaction_id: 1.into(),
    };

    assert!(res.contains(&(USERS[1], expect.encode())));

    let res = wallet.send(USERS[2], MWAction::ConfirmTransaction(1.into()));

    let expect = MWEvent::Confirmation {
        sender: USERS[2].into(),
        transaction_id: 1.into(),
    };

    assert!(res.contains(&(USERS[2], expect.encode())));
}

#[test]
fn try_to_send_directly() {
    let sys = System::new();
    let wallet = common_init(&sys, &USERS[0..3], 1);

    sys.mint_to(USERS[0], 1_000_000_000);
    let res = wallet.send_with_value(
        USERS[0],
        MWAction::ChangeRequiredConfirmationsCount(3).encode(),
        1_000_000_000,
    );

    assert!(res.main_failed());
}

#[test]
fn required_greater_than_owners_count() {
    let sys = System::new();
    let wallet = common_init(&sys, &USERS[0..3], 1);

    sys.mint_to(USERS[0], 1_000_000_000);
    let res = wallet.send_with_value(
        USERS[0],
        MWAction::SubmitTransaction {
            destination: 1.into(),
            data: MWAction::ChangeRequiredConfirmationsCount(4).encode(),
            value: 0,
            description: None,
        },
        1_000_000_000,
    );

    assert!(res.others_failed());
}

#[test]
fn zero_required_confirmations() {
    let sys = System::new();
    let wallet = common_init(&sys, &USERS[0..3], 1);

    sys.mint_to(USERS[0], 1_000_000_000);
    let res = wallet.send_with_value(
        USERS[0],
        MWAction::SubmitTransaction {
            destination: 1.into(),
            data: MWAction::ChangeRequiredConfirmationsCount(0).encode(),
            value: 0,
            description: None,
        },
        1_000_000_000,
    );

    assert!(res.others_failed());
}
