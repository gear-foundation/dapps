use codec::Encode;
use gstd::ActorId;
use gtest::{Program, System};
use multisig_wallet_io::*;
const USERS: &[u64] = &[3, 4, 5, 6];
const ZERO_ID: ActorId = ActorId::new([0u8; 32]);

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
    let wallet = common_init(&sys, &USERS[0..3], 3);
    let res = wallet.send(
        USERS[0],
        MWAction::SubmitTransaction {
            destination: USERS[3].into(),
            data: vec![],
            value: 1000,
            description: Some("test".to_string()),
        },
    );

    let expect = MWEvent::Submission {
        transaction_id: 0.into(),
    };

    assert!(res.contains(&(USERS[0], expect.encode())));
}

#[test]
fn submit_several_transactions() {
    let sys = System::new();
    let wallet = common_init(&sys, &USERS[0..3], 3);
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
        transaction_id: 0.into(),
    };

    assert!(res.contains(&(USERS[0], expect.encode())));

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
}

#[test]
fn submit_and_execute_automatically() {
    let sys = System::new();
    let wallet = common_init(&sys, &USERS[0..3], 1);
    sys.mint_to(USERS[0], 1_000_000_000);
    let res = wallet.send_with_value(
        USERS[0],
        MWAction::SubmitTransaction {
            destination: USERS[3].into(),
            data: vec![],
            value: 10000,
            description: None,
        },
        1_000_000_000,
    );

    let expect = MWEvent::Submission {
        transaction_id: 0.into(),
    };

    assert!(res.contains(&(USERS[0], expect.encode())));
}

#[test]
fn submit_transaction_with_zero_destination() {
    let sys = System::new();
    let wallet = common_init(&sys, &USERS[0..3], 2);
    sys.mint_to(USERS[0], 10000);
    let res = wallet.send_with_value(
        USERS[0],
        MWAction::SubmitTransaction {
            destination: ZERO_ID,
            data: vec![],
            value: 1000,
            description: None,
        },
        10000,
    );

    assert!(res.main_failed());
}
