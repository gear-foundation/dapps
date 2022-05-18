use codec::{Decode, Encode};
use dao_io::*;
use ft_io::*;
use gtest::{Program, System, WasmProgram};

#[derive(Debug)]
struct FungibleToken;

impl WasmProgram for FungibleToken {
    fn init(&mut self, _: Vec<u8>) -> Result<Option<Vec<u8>>, &'static str> {
        Ok(Some(b"GOT IT".to_vec()))
    }

    fn handle(&mut self, payload: Vec<u8>) -> Result<Option<Vec<u8>>, &'static str> {
        let res = FTAction::decode(&mut &payload[..]).map_err(|_| "Can't decode")?;
        match res {
            FTAction::Transfer {
                from: _,
                to: _,
                amount: _,
            } => {
                return Ok(Some(
                    FTEvent::Transfer {
                        from: 3.into(),
                        to: 3.into(),
                        amount: 10000,
                    }
                    .encode(),
                ));
            }
            FTAction::BalanceOf(_) => {
                return Ok(Some(FTEvent::Balance(10000).encode()));
            }
            _ => return Ok(None),
        }
    }

    fn handle_reply(&mut self, _: Vec<u8>) -> Result<Option<Vec<u8>>, &'static str> {
        Ok(None)
    }
}

fn init_dao(sys: &System) {
    sys.init_logger();
    let dao = Program::current(&sys);
    let res = dao.send(
        100001,
        InitDao {
            admin: 3.into(),
            approved_token_program_id: 1.into(),
            period_duration: 10000000,
            voting_period_length: 100000000,
            grace_period_length: 10000000,
            dilution_bound: 3,
            abort_window: 10000000,
        },
    );

    assert!(res.log().is_empty());
}

fn add_member(
    sys: &System,
    dao: &Program,
    proposal_id: u128,
    applicant: u64,
    token_tribute: u128,
    shares_requested: u128,
    quorum: u128,
) {
    let res = dao.send(3, DaoAction::AddToWhiteList(applicant.into()));
    assert!(!res.main_failed());

    let res = dao.send(
        3,
        DaoAction::SubmitMembershipProposal {
            applicant: applicant.into(),
            token_tribute: token_tribute,
            shares_requested: shares_requested,
            quorum: quorum,
            details: "".to_string(),
        },
    );
    assert!(!res.main_failed());

    let res = dao.send(
        3,
        DaoAction::SubmitVote {
            proposal_id: proposal_id.clone(),
            vote: Vote::Yes,
        },
    );
    assert!(!res.main_failed());

    sys.spend_blocks(1000000001);

    let res = dao.send(3, DaoAction::ProcessProposal(proposal_id.clone()));
    assert!(!res.main_failed());
}

#[test]
fn proposal_passed() {
    let sys = System::new();
    sys.init_logger();
    let users: Vec<u64> = (3..13).collect();

    let ft = Program::mock(&sys, FungibleToken);

    let res = ft.send_bytes(100001, "INIT");
    assert!(!res.log().is_empty());

    init_dao(&sys);
    let dao = sys.get_program(2);
    users.iter().enumerate().for_each(|(i, user)| {
        add_member(&sys, &dao, i.try_into().unwrap(), *user, 1000, 1000, 0);
    });

    let res = dao.send(
        users[0],
        DaoAction::SubmitMembershipProposal {
            applicant: users[1].into(),
            token_tribute: 2000,
            shares_requested: 2000,
            quorum: 7000,
            details: "".to_string(),
        },
    );
    assert!(!res.main_failed());

    for i in 0..7 {
        let res = dao.send(
            users[i],
            DaoAction::SubmitVote {
                proposal_id: 10,
                vote: Vote::Yes,
            },
        );
        assert!(!res.main_failed());
    }
    for i in 8..10 {
        let res = dao.send(
            users[i],
            DaoAction::SubmitVote {
                proposal_id: 10,
                vote: Vote::No,
            },
        );
        assert!(!res.main_failed());
    }
    sys.spend_blocks(1000000001);

    let res = dao.send(3, DaoAction::ProcessProposal(10));
    assert!(res.contains(&(
        3,
        DaoEvent::ProcessProposal {
            applicant: users[1].into(),
            proposal_id: 10,
            did_pass: true,
        }
        .encode()
    )));
}

#[test]
fn proposal_did_not_pass() {
    let sys = System::new();
    sys.init_logger();
    let users: Vec<u64> = (3..13).collect();

    let ft = Program::mock(&sys, FungibleToken);

    let res = ft.send_bytes(100001, "INIT");
    assert!(!res.log().is_empty());

    init_dao(&sys);
    let dao = sys.get_program(2);
    users.iter().enumerate().for_each(|(i, user)| {
        add_member(&sys, &dao, i.try_into().unwrap(), *user, 1000, 1000, 0);
    });

    let res = dao.send(
        users[0],
        DaoAction::SubmitMembershipProposal {
            applicant: users[1].into(),
            token_tribute: 2000,
            shares_requested: 2000,
            quorum: 7000,
            details: "".to_string(),
        },
    );
    assert!(!res.main_failed());

    for i in 0..7 {
        println!("id {:?}", i);
        let res = dao.send(
            users[i],
            DaoAction::SubmitVote {
                proposal_id: 10,
                vote: Vote::No,
            },
        );
        assert!(!res.main_failed());
    }
    for i in 8..10 {
        let res = dao.send(
            users[i],
            DaoAction::SubmitVote {
                proposal_id: 10,
                vote: Vote::Yes,
            },
        );
        assert!(!res.main_failed());
    }
    sys.spend_blocks(1000000001);

    let res = dao.send(3, DaoAction::ProcessProposal(10));
    println!("{:?}", res.decoded_log::<DaoEvent>());
    assert!(res.contains(&(
        3,
        DaoEvent::ProcessProposal {
            applicant: users[1].into(),
            proposal_id: 10,
            did_pass: false,
        }
        .encode()
    )));
}

#[test]
fn quorum_is_not_reached() {
    let sys = System::new();
    sys.init_logger();
    let users: Vec<u64> = (3..13).collect();

    let ft = Program::mock(&sys, FungibleToken);

    let res = ft.send_bytes(100001, "INIT");
    assert!(!res.log().is_empty());

    init_dao(&sys);
    let dao = sys.get_program(2);
    users.iter().enumerate().for_each(|(i, user)| {
        add_member(&sys, &dao, i.try_into().unwrap(), *user, 1000, 1000, 0);
    });

    let res = dao.send(
        users[0],
        DaoAction::SubmitMembershipProposal {
            applicant: users[1].into(),
            token_tribute: 2000,
            shares_requested: 2000,
            quorum: 7500,
            details: "".to_string(),
        },
    );
    assert!(!res.main_failed());

    for i in 0..7 {
        println!("id {:?}", i);
        let res = dao.send(
            users[i],
            DaoAction::SubmitVote {
                proposal_id: 10,
                vote: Vote::Yes,
            },
        );
        assert!(!res.main_failed());
    }
    for i in 8..10 {
        let res = dao.send(
            users[i],
            DaoAction::SubmitVote {
                proposal_id: 10,
                vote: Vote::No,
            },
        );
        assert!(!res.main_failed());
    }
    sys.spend_blocks(1000000001);

    let res = dao.send(3, DaoAction::ProcessProposal(10));
    println!("{:?}", res.decoded_log::<DaoEvent>());
    assert!(res.contains(&(
        3,
        DaoEvent::ProcessProposal {
            applicant: users[1].into(),
            proposal_id: 10,
            did_pass: false,
        }
        .encode()
    )));
}

#[test]
fn ragequit() {
    let sys = System::new();
    sys.init_logger();
    let users: Vec<u64> = (3..13).collect();

    let ft = Program::mock(&sys, FungibleToken);

    let res = ft.send_bytes(100001, "INIT");
    assert!(!res.log().is_empty());

    init_dao(&sys);
    let dao = sys.get_program(2);
    users.iter().enumerate().for_each(|(i, user)| {
        add_member(&sys, &dao, i.try_into().unwrap(), *user, 1000, 1000, 0);
    });

    let res = dao.send(
        users[0],
        DaoAction::SubmitMembershipProposal {
            applicant: users[1].into(),
            token_tribute: 2000,
            shares_requested: 2000,
            quorum: 1000,
            details: "".to_string(),
        },
    );
    assert!(!res.main_failed());

    for i in 9..10 {
        let res = dao.send(
            users[i],
            DaoAction::SubmitVote {
                proposal_id: 10,
                vote: Vote::Yes,
            },
        );
        assert!(!res.main_failed());
    }
    sys.spend_blocks(1000000001);
    for i in 1..8 {
        println!("id {:?}", i);
        let res = dao.send(users[i], DaoAction::RageQuit(1000));
        assert!(!res.main_failed());
    }

    let res = dao.send(3, DaoAction::ProcessProposal(10));
    assert!(res.contains(&(
        3,
        DaoEvent::ProcessProposal {
            applicant: users[1].into(),
            proposal_id: 10,
            did_pass: false,
        }
        .encode()
    )));
}
