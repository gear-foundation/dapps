use codec::Encode;
use dao_io::*;
use ft_io::*;
use gtest::{Program, System};

fn init_fungible_token(sys: &System) {
    let ft = Program::from_file(sys, "./target/fungible_token.wasm");

    let res = ft.send(
        100001,
        InitConfig {
            name: String::from("MyToken"),
            symbol: String::from("MTK"),
        },
    );

    assert!(res.log().is_empty());
}

fn init_dao(sys: &System) {
    let dao = Program::current(sys);
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

fn mint_tokens(ft: &Program, user: u64) {
    let res = ft.send(user, FTAction::Mint(10000));
    assert!(!res.main_failed());

    let res = ft.send(
        user,
        FTAction::Approve {
            to: 2.into(),
            amount: 10000,
        },
    );
    assert!(!res.main_failed());
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
            token_tribute,
            shares_requested,
            quorum,
            details: "".to_string(),
        },
    );
    assert!(!res.main_failed());

    let res = dao.send(
        3,
        DaoAction::SubmitVote {
            proposal_id,
            vote: Vote::Yes,
        },
    );
    assert!(!res.main_failed());

    sys.spend_blocks(1000000001);

    let res = dao.send(3, DaoAction::ProcessProposal(proposal_id));
    assert!(!res.main_failed());
}

#[test]
fn funding_proposal() {
    let sys = System::new();
    let users: Vec<u64> = (3..13).collect();
    init_fungible_token(&sys);
    init_dao(&sys);
    sys.init_logger();
    let ft = sys.get_program(1);
    let dao = sys.get_program(2);
    users.iter().enumerate().for_each(|(i, user)| {
        mint_tokens(&ft, *user);
        add_member(&sys, &dao, i.try_into().unwrap(), *user, 1000, 1000, 0);
    });

    let res = dao.send(
        3,
        DaoAction::SubmitFundingProposal {
            applicant: 20.into(),
            amount: 8000,
            quorum: 80,
            details: "First funding proposal".to_string(),
        },
    );
    assert!(res.contains(&(
        3,
        DaoEvent::SubmitFundingProposal {
            proposer: 3.into(),
            applicant: 20.into(),
            proposal_id: 10,
            amount: 8000,
        }
        .encode()
    )));
    for user in users {
        let res = dao.send(
            user,
            DaoAction::SubmitVote {
                proposal_id: 10,
                vote: Vote::Yes,
            },
        );
        assert!(!res.main_failed());
    }

    sys.spend_blocks(1000000001);

    let res = dao.send(3, DaoAction::ProcessProposal(10));
    assert!(!res.main_failed());

    let res = ft.send(3, FTAction::BalanceOf(20.into()));
    assert!(res.contains(&(3, FTEvent::Balance(8000).encode())));
    let res = dao.send(4, DaoAction::RageQuit(500));
    println!("{:?}", res.decoded_log::<DaoEvent>());
    assert!(res.contains(&(
        4,
        DaoEvent::RageQuit {
            member: 4.into(),
            amount: 99,
        }
        .encode()
    )));

    // must fail since DAO has no enough funds
    let res = dao.send(
        3,
        DaoAction::SubmitFundingProposal {
            applicant: 20.into(),
            amount: 8000,
            quorum: 80,
            details: "First funding proposal".to_string(),
        },
    );
    assert!(res.main_failed());
}
