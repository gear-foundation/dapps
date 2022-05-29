use codec::Encode;
use gtest::{Program, System};
use nft_example_io::*;
const USERS: &[u64] = &[3, 4, 5];

fn init_with_mint(sys: &System) {
    sys.init_logger();

    let nft = Program::current(sys);
    let res = nft.send(
        USERS[0],
        InitConfig {
            name: String::from("MyToken"),
            symbol: String::from("MTK"),
            base_uri: String::from(""),
        },
    );

    assert!(res.log().is_empty());

    let res = nft.send(USERS[0], Action::Mint);
    assert!(res.contains(&(
        USERS[0],
        Event::Transfer {
            from: 0.into(),
            to: USERS[0].into(),
            token_id: 0.into(),
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
fn burn() {
    let sys = System::new();
    init_with_mint(&sys);
    let nft = sys.get_program(1);
    let res = nft.send(USERS[0], Action::Burn(0.into()));
    assert!(res.contains(&(
        USERS[0],
        Event::Transfer {
            from: USERS[0].into(),
            to: 0.into(),
            token_id: 0.into(),
        }
        .encode()
    )));
}

#[test]
fn burn_failures() {
    let sys = System::new();
    init_with_mint(&sys);
    let nft = sys.get_program(1);
    // must fail since the token doesn't exist
    let res = nft.send(USERS[0], Action::Burn(1.into()));
    assert!(res.main_failed());

    // must fail since the caller isn't the token owner
    let res = nft.send(USERS[1], Action::Burn(0.into()));
    assert!(res.main_failed());
}

#[test]
fn owner_of() {
    let sys = System::new();
    init_with_mint(&sys);
    let nft = sys.get_program(1);
    let res = nft.send(USERS[0], Action::OwnerOf(0.into()));
    assert!(res.contains(&(USERS[0], Event::OwnerOf(USERS[0].into()).encode())));

    // must return zero address since the token doesn't exist
    let res = nft.send(USERS[0], Action::OwnerOf(100.into()));
    assert!(res.contains(&(USERS[0], Event::OwnerOf(0.into()).encode())));
}

#[test]
fn balance_of() {
    let sys = System::new();
    init_with_mint(&sys);
    let nft = sys.get_program(1);
    let res = nft.send(USERS[0], Action::Mint);
    assert!(!res.main_failed());
    let res = nft.send(USERS[0], Action::Mint);
    assert!(!res.main_failed());

    let res = nft.send(USERS[0], Action::BalanceOf(USERS[0].into()));
    assert!(res.contains(&(USERS[0], Event::BalanceOf(3.into()).encode())));
}

#[test]
fn transfer() {
    let sys = System::new();
    init_with_mint(&sys);
    let nft = sys.get_program(1);
    let res = nft.send(
        USERS[0],
        Action::Transfer {
            to: USERS[1].into(),
            token_id: 0.into(),
        },
    );

    assert!(res.contains(&(
        USERS[0],
        Event::Transfer {
            from: USERS[0].into(),
            to: USERS[1].into(),
            token_id: 0.into(),
        }
        .encode()
    )));

    // check that the balance of `USER[0]` is zero, the balance of `USER[1]` is now 1
    let res = nft.send(USERS[0], Action::BalanceOf(USERS[0].into()));
    assert!(res.contains(&(USERS[0], Event::BalanceOf(0.into()).encode())));
    let res = nft.send(USERS[0], Action::BalanceOf(USERS[1].into()));
    assert!(res.contains(&(USERS[0], Event::BalanceOf(1.into()).encode())));

    // check that `USER[1]` is now the owner of the token with `0` id
    let res = nft.send(USERS[0], Action::OwnerOf(0.into()));
    assert!(res.contains(&(USERS[0], Event::OwnerOf(USERS[1].into()).encode())));
}

#[test]
fn transfer_failures() {
    let sys = System::new();
    init_with_mint(&sys);
    let nft = sys.get_program(1);
    //must fail since the tokens doesn't exist
    let res = nft.send(
        USERS[0],
        Action::Transfer {
            to: USERS[1].into(),
            token_id: 100.into(),
        },
    );
    assert!(res.main_failed());

    //must fail since the caller isn't the is not an authorized source
    let res = nft.send(
        USERS[2],
        Action::Transfer {
            to: USERS[1].into(),
            token_id: 0.into(),
        },
    );
    assert!(res.main_failed());

    //must fail since the `to` is the zero address
    let res = nft.send(
        USERS[0],
        Action::Transfer {
            to: 0.into(),
            token_id: 0.into(),
        },
    );
    assert!(res.main_failed());
}

#[test]
fn approve_and_transfer() {
    let sys = System::new();
    init_with_mint(&sys);
    let nft = sys.get_program(1);

    let res = nft.send(
        USERS[0],
        Action::Approve {
            to: USERS[1].into(),
            token_id: 0.into(),
        },
    );
    assert!(res.contains(&(
        USERS[0],
        Event::Approval {
            owner: USERS[0].into(),
            spender: USERS[1].into(),
            token_id: 0.into(),
        }
        .encode()
    )));

    let res = nft.send(
        USERS[1],
        Action::Transfer {
            to: USERS[2].into(),
            token_id: 0.into(),
        },
    );

    assert!(res.contains(&(
        USERS[1],
        Event::Transfer {
            from: USERS[0].into(),
            to: USERS[2].into(),
            token_id: 0.into(),
        }
        .encode()
    )));
}

#[test]
fn approve_for_all() {
    let sys = System::new();
    init_with_mint(&sys);
    let nft = sys.get_program(1);

    let res = nft.send(USERS[0], Action::Mint);
    assert!(!res.main_failed());
    let res = nft.send(USERS[0], Action::Mint);
    assert!(!res.main_failed());

    let res = nft.send(
        USERS[0],
        Action::ApproveForAll {
            to: USERS[1].into(),
            approved: true,
        },
    );
    assert!(res.contains(&(
        USERS[0],
        Event::ApprovalForAll {
            owner: USERS[0].into(),
            operator: USERS[1].into(),
            approved: true,
        }
        .encode()
    )));

    let res = nft.send(
        USERS[1],
        Action::Transfer {
            to: USERS[2].into(),
            token_id: 0.into(),
        },
    );
    assert!(!res.main_failed());
    let res = nft.send(
        USERS[1],
        Action::Transfer {
            to: USERS[2].into(),
            token_id: 1.into(),
        },
    );
    assert!(!res.main_failed());

    let res = nft.send(
        USERS[0],
        Action::ApproveForAll {
            to: USERS[1].into(),
            approved: false,
        },
    );
    assert!(res.contains(&(
        USERS[0],
        Event::ApprovalForAll {
            owner: USERS[0].into(),
            operator: USERS[1].into(),
            approved: false,
        }
        .encode()
    )));

    // must fail since the `USERS[1]` can no longer send tokens
    let res = nft.send(
        USERS[1],
        Action::Transfer {
            to: USERS[2].into(),
            token_id: 2.into(),
        },
    );
    assert!(res.main_failed());
}
