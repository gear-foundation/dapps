use gtest::{Program, RunResult, System};
use multisig_wallet_io::*;

const USERS: &[u64] = &[3, 4, 5, 6];

fn common_init(sys: &System, users: &[u64], required: u32) -> RunResult {
    sys.init_logger();

    let wallet = Program::current(sys);

    wallet.send(
        USERS[0],
        MWInitConfig {
            owners: users.iter().copied().map(|x| x.into()).collect(),
            required,
        },
    )
}

#[test]
fn required_equals_owners_count() {
    let sys = System::new();
    let res = common_init(&sys, &USERS[0..3], 3);

    assert!(res.log().is_empty())
}

#[test]
fn required_less_than_owners_count() {
    let sys = System::new();
    let res = common_init(&sys, &USERS[0..4], 3);

    assert!(res.log().is_empty())
}

#[test]
fn without_contract_owner() {
    let sys = System::new();
    let res = common_init(&sys, &USERS[1..4], 3);

    assert!(res.log().is_empty())
}

#[test]
fn required_more_than_owners_count() {
    let sys = System::new();
    let res = common_init(&sys, &USERS[0..2], 3);

    assert!(res.main_failed())
}

#[test]
fn too_much_owners() {
    let sys = System::new();
    let array: [u64; 51] = (3..=53).collect::<Vec<_>>().try_into().unwrap();
    let res = common_init(&sys, &array, 3);

    assert!(res.main_failed())
}

#[test]
fn zero_required() {
    let sys = System::new();
    let res = common_init(&sys, &USERS[0..2], 0);

    assert!(res.main_failed())
}

#[test]
fn contains_one_owner_two_times() {
    let sys = System::new();
    let res = common_init(&sys, &[USERS[1], USERS[0], USERS[3], USERS[1]], 2);

    assert!(res.main_failed())
}
