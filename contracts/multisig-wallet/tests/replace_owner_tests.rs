use gstd::Encode;
use gtest::{Program, System};
use multisig_wallet_io::*;

const USERS: &[u64] = &[3, 4, 5, 6];

fn common_init<'a>(sys: &'a System, users: &[u64], required: u32) -> Program<'a> {
    sys.init_logger();

    let wallet = Program::current(sys);

    sys.mint_to(USERS[0], 1_000_000_000);
    wallet.send_with_value(
        USERS[0],
        MWInitConfig {
            owners: users.iter().copied().map(|x| x.into()).collect(),
            required,
        },
        1_000_000_000,
    );

    wallet
}

#[test]
fn common() {
    let sys = System::new();
    let wallet = common_init(&sys, &USERS[0..2], 1);

    let res = wallet.send(
        USERS[0],
        MWAction::SubmitTransaction {
            destination: 1.into(),
            data: MWAction::ReplaceOwner {
                old_owner: USERS[1].into(),
                new_owner: USERS[2].into(),
            }
            .encode(),
            value: 0,
            description: None,
        },
    );

    let expect = MWEvent::Submission {
        transaction_id: 0.into(),
    };

    assert!(res.contains(&(USERS[0], expect.encode())));
    assert!(!res.others_failed());

    let res = wallet.send(
        USERS[2],
        MWAction::SubmitTransaction {
            destination: USERS[3].into(),
            data: vec![],
            value: 0,
            description: None,
        },
    );

    let expect = MWEvent::Submission {
        transaction_id: 1.into(),
    };

    assert!(res.contains(&(USERS[2], expect.encode())));
    assert!(!res.others_failed());
}

#[test]
fn send_directly() {
    let sys = System::new();
    let wallet = common_init(&sys, &USERS[0..2], 1);

    sys.mint_to(USERS[0], 1_000_000_000);
    let res = wallet.send_with_value(
        USERS[0],
        MWAction::ReplaceOwner {
            old_owner: USERS[1].into(),
            new_owner: USERS[2].into(),
        }
        .encode(),
        1_000_000_000,
    );

    assert!(res.main_failed());
}

#[test]
fn new_owner_is_already_owner() {
    let sys = System::new();
    let wallet = common_init(&sys, &USERS[0..3], 1);

    let res = wallet.send(
        USERS[0],
        MWAction::SubmitTransaction {
            destination: 1.into(),
            data: MWAction::ReplaceOwner {
                old_owner: USERS[1].into(),
                new_owner: USERS[2].into(),
            }
            .encode(),
            value: 0,
            description: None,
        },
    );

    assert!(res.others_failed());
}

#[test]
fn old_owner_is_not_owner() {
    let sys = System::new();
    let wallet = common_init(&sys, &USERS[0..2], 1);

    let res = wallet.send(
        USERS[0],
        MWAction::SubmitTransaction {
            destination: 1.into(),
            data: MWAction::ReplaceOwner {
                old_owner: USERS[2].into(),
                new_owner: USERS[3].into(),
            }
            .encode(),
            value: 0,
            description: None,
        },
    );

    assert!(res.others_failed());
}

#[test]
fn replace_and_reverse() {
    let sys = System::new();
    let wallet = common_init(&sys, &USERS[0..2], 1);

    let res = wallet.send(
        USERS[0],
        MWAction::SubmitTransaction {
            destination: 1.into(),
            data: MWAction::ReplaceOwner {
                old_owner: USERS[1].into(),
                new_owner: USERS[2].into(),
            }
            .encode(),
            value: 0,
            description: None,
        },
    );

    assert!(!res.others_failed());

    let res = wallet.send(
        USERS[0],
        MWAction::SubmitTransaction {
            destination: 1.into(),
            data: MWAction::ReplaceOwner {
                old_owner: USERS[2].into(),
                new_owner: USERS[1].into(),
            }
            .encode(),
            value: 0,
            description: None,
        },
    );

    assert!(!res.others_failed());

    let res = wallet.send(
        USERS[1],
        MWAction::SubmitTransaction {
            destination: USERS[3].into(),
            data: vec![],
            value: 0,
            description: None,
        },
    );

    let expect = MWEvent::Submission {
        transaction_id: 2.into(),
    };

    assert!(res.contains(&(USERS[1], expect.encode())));
}

#[test]
fn remove_owner_then_replace() {
    let sys = System::new();
    let wallet = common_init(&sys, &USERS[0..3], 1);

    let res = wallet.send(
        USERS[0],
        MWAction::SubmitTransaction {
            destination: 1.into(),
            data: MWAction::RemoveOwner(USERS[2].into()).encode(),
            value: 0,
            description: None,
        },
    );

    assert!(!res.others_failed());

    let res = wallet.send(
        USERS[0],
        MWAction::SubmitTransaction {
            destination: 1.into(),
            data: MWAction::ReplaceOwner {
                old_owner: USERS[1].into(),
                new_owner: USERS[2].into(),
            }
            .encode(),
            value: 0,
            description: None,
        },
    );

    assert!(!res.others_failed());

    let res = wallet.send(
        USERS[2],
        MWAction::SubmitTransaction {
            destination: USERS[3].into(),
            data: vec![],
            value: 1000,
            description: None,
        },
    );

    let expect = MWEvent::Submission {
        transaction_id: 2.into(),
    };

    assert!(res.contains(&(USERS[2], expect.encode())));
}

#[test]
fn add_owner_then_replace() {
    let sys = System::new();
    let wallet = common_init(&sys, &USERS[0..2], 1);

    let res = wallet.send(
        USERS[0],
        MWAction::SubmitTransaction {
            destination: 1.into(),
            data: MWAction::AddOwner(USERS[2].into()).encode(),
            value: 0,
            description: None,
        },
    );

    let expect = MWEvent::Submission {
        transaction_id: 0.into(),
    };

    assert!(res.contains(&(USERS[0], expect.encode())));
    assert!(!res.others_failed());

    let res = wallet.send(
        USERS[0],
        MWAction::SubmitTransaction {
            destination: 1.into(),
            data: MWAction::ReplaceOwner {
                old_owner: USERS[2].into(),
                new_owner: USERS[3].into(),
            }
            .encode(),
            value: 0,
            description: None,
        },
    );

    let expect = MWEvent::Submission {
        transaction_id: 1.into(),
    };

    assert!(res.contains(&(USERS[0], expect.encode())));
    assert!(!res.others_failed());

    let res = wallet.send(
        USERS[3],
        MWAction::SubmitTransaction {
            destination: USERS[2].into(),
            data: vec![],
            value: 1000,
            description: None,
        },
    );

    let expect = MWEvent::Submission {
        transaction_id: 2.into(),
    };

    assert!(res.contains(&(USERS[3], expect.encode())));
}
