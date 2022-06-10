use codec::Encode;
use gtest::{Program, System};
use multisig_wallet_io::*;
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

    wallet
}

#[test]
fn common() {
    let sys = System::new();
    let wallet = common_init(&sys, &USERS[0..3], 3);
    let res = wallet.send(USERS[1], MWAction::ConfirmTransaction(0.into()));

    let expect = MWEvent::Confirmation {
        sender: USERS[1].into(),
        transaction_id: 0.into(),
    };

    assert!(res.contains(&(USERS[1], expect.encode())));
}

#[test]
fn owner_doesnt_exist() {
    let sys = System::new();
    let wallet = common_init(&sys, &USERS[0..3], 3);
    let res = wallet.send(USERS[3], MWAction::ConfirmTransaction(0.into()));

    assert!(res.main_failed());
}

#[test]
fn transaction_doesnt_exist() {
    let sys = System::new();
    let wallet = common_init(&sys, &USERS[0..3], 3);
    let res = wallet.send(USERS[1], MWAction::ConfirmTransaction(1.into()));

    assert!(res.main_failed());
}

#[test]
fn already_confirmed() {
    let sys = System::new();
    let wallet = common_init(&sys, &USERS[0..3], 3);
    let res = wallet.send(USERS[1], MWAction::ConfirmTransaction(0.into()));

    let expect = MWEvent::Confirmation {
        sender: USERS[1].into(),
        transaction_id: 0.into(),
    };

    assert!(res.contains(&(USERS[1], expect.encode())));

    let res = wallet.send(USERS[1], MWAction::ConfirmTransaction(0.into()));

    assert!(res.main_failed());
}

#[test]
fn confirm_and_execute_automatically() {
    let sys = System::new();
    let wallet = common_init(&sys, &USERS[0..3], 3);
    let res = wallet.send(USERS[1], MWAction::ConfirmTransaction(0.into()));

    let expect = MWEvent::Confirmation {
        sender: USERS[1].into(),
        transaction_id: 0.into(),
    };

    assert!(res.contains(&(USERS[1], expect.encode())));

    let res = wallet.send(USERS[2], MWAction::ConfirmTransaction(0.into()));

    let expect = MWEvent::Confirmation {
        sender: USERS[2].into(),
        transaction_id: 0.into(),
    };

    assert!(res.contains(&(USERS[2], expect.encode())));
}

#[test]
fn confirm_after_execution() {
    let sys = System::new();
    let wallet = common_init(&sys, &USERS[0..3], 1);

    let res = wallet.send(USERS[1], MWAction::ConfirmTransaction(0.into()));

    assert!(res.main_failed());
}
