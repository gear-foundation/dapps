use codec::Encode;
use dao_io::*;
use ft_io::*;
use gtest::{Program, System};

fn init_fungible_token(sys: &System) {
    sys.init_logger();
    let ft = Program::from_file(
        &sys,
        "../target/wasm32-unknown-unknown/release/fungible_token.wasm",
    );

    let res = ft.send(
        100001,
        InitConfig {
            name: String::from("MyToken"),
            symbol: String::from("MTK"),
        },
    );

    assert!(res.log().is_empty());

    let res = ft.send(4, FTAction::Mint(10000000));
    assert!(!res.main_failed());
    let res = ft.send(
        4,
        FTAction::Approve {
            to: 2.into(),
            amount: 10000000,
        },
    );
    assert!(!res.main_failed());
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

    let res = dao.send(3, DaoAction::AddToWhiteList(4.into()));
    assert!(res.contains(&(3, DaoEvent::MemberAddedToWhitelist(4.into()).encode())));
}

fn create_membership_proposal(dao: &Program, proposal_id: u128) {
    let res = dao.send(
        3,
        DaoAction::SubmitMembershipProposal {
            applicant: 4.into(),
            token_tribute: 1000,
            shares_requested: 1000,
            quorum: 80,
            details: "First membership proposal".to_string(),
        },
    );
    assert!(res.contains(&(
        3,
        DaoEvent::SubmitMembershipProposal {
            proposer: 3.into(),
            applicant: 4.into(),
            proposal_id: proposal_id.clone(),
            token_tribute: 1000
        }
        .encode()
    )));
}

fn vote(dao: &Program, proposal_id: u128, vote: Vote) {
    let res = dao.send(
        3,
        DaoAction::SubmitVote {
            proposal_id: proposal_id.clone(),
            vote: vote.clone(),
        },
    );
    assert!(res.contains(&(
        3,
        DaoEvent::SubmitVote {
            account: 3.into(),
            proposal_id: proposal_id.clone(),
            vote: vote.clone(),
        }
        .encode()
    )));
}

#[test]
fn create_proposal() {
    let sys = System::new();
    init_fungible_token(&sys);
    init_dao(&sys);
    let dao = sys.get_program(2);
    create_membership_proposal(&dao, 0);
}

#[test]
fn create_proposal_failures() {
    let sys = System::new();
    init_fungible_token(&sys);
    init_dao(&sys);
    let dao = sys.get_program(2);

    // must fail since account is not a delegate
    let res = dao.send(
        4,
        DaoAction::SubmitMembershipProposal {
            applicant: 4.into(),
            token_tribute: 1000,
            shares_requested: 1000,
            quorum: 80,
            details: "First membership proposal".to_string(),
        },
    );
    assert!(res.main_failed());

    // must fail since applicant is not in whitelist
    let res = dao.send(
        3,
        DaoAction::SubmitMembershipProposal {
            applicant: 5.into(),
            token_tribute: 1000,
            shares_requested: 1000,
            quorum: 80,
            details: "First membership proposal".to_string(),
        },
    );
    assert!(res.main_failed());
}

#[test]
fn vote_on_proposal_failures() {
    let sys = System::new();
    init_fungible_token(&sys);
    init_dao(&sys);

    let dao = sys.get_program(2);
    create_membership_proposal(&dao, 0);
    // must fail since the proposal does not exist
    let res = dao.send(
        3,
        DaoAction::SubmitVote {
            proposal_id: 1,
            vote: Vote::Yes,
        },
    );
    assert!(res.main_failed());

    //must fail since the account is not delegate
    let res = dao.send(
        4,
        DaoAction::SubmitVote {
            proposal_id: 0,
            vote: Vote::Yes,
        },
    );
    assert!(res.main_failed());

    sys.spend_blocks(1000000001);
    // must fail since the voting period has expired
    let res = dao.send(
        3,
        DaoAction::SubmitVote {
            proposal_id: 0,
            vote: Vote::Yes,
        },
    );
    assert!(res.main_failed());

    create_membership_proposal(&dao, 1);
    create_membership_proposal(&dao, 2);
    // must fail since the voting period has not started
    let res = dao.send(
        3,
        DaoAction::SubmitVote {
            proposal_id: 2,
            vote: Vote::Yes,
        },
    );
    assert!(res.main_failed());
}

#[test]
fn vote_on_proposal() {
    let sys = System::new();
    init_fungible_token(&sys);
    init_dao(&sys);

    let dao = sys.get_program(2);
    create_membership_proposal(&dao, 0);
    vote(&dao, 0, Vote::Yes);

    // must fail since the account has already voted
    let res = dao.send(
        3,
        DaoAction::SubmitVote {
            proposal_id: 0,
            vote: Vote::Yes,
        },
    );
    assert!(res.main_failed());
}

#[test]
fn process_proposal() {
    let sys = System::new();
    init_fungible_token(&sys);
    init_dao(&sys);

    let dao = sys.get_program(2);
    create_membership_proposal(&dao, 0);
    vote(&dao, 0, Vote::Yes);

    let dao = sys.get_program(2);
    // must fail since voting period is not over
    let res = dao.send(3, DaoAction::ProcessProposal(0));
    assert!(res.main_failed());

    sys.spend_blocks(1000000001);
    let res = dao.send(3, DaoAction::ProcessProposal(0));
    assert!(res.contains(&(
        3,
        DaoEvent::ProcessProposal {
            applicant: 4.into(),
            proposal_id: 0,
            did_pass: true,
        }
        .encode()
    )));

    create_membership_proposal(&dao, 1);
    vote(&dao, 1, Vote::No);
    sys.spend_blocks(1000000001);
    let res = dao.send(3, DaoAction::ProcessProposal(1));
    assert!(res.contains(&(
        3,
        DaoEvent::ProcessProposal {
            applicant: 4.into(),
            proposal_id: 1,
            did_pass: false,
        }
        .encode()
    )));
}

#[test]
fn abort_proposal_failures() {
    let sys = System::new();
    init_fungible_token(&sys);
    init_dao(&sys);

    let dao = sys.get_program(2);
    create_membership_proposal(&dao, 0);
    vote(&dao, 0, Vote::Yes);
    // must fail since proposal doesn't exist
    let res = dao.send(4, DaoAction::Abort(1));
    assert!(res.main_failed());

    // must fail the caller must be the applicant
    let res = dao.send(3, DaoAction::Abort(0));
    assert!(res.main_failed());

    sys.spend_blocks(100000001);
    // must fail sincle the abort window is over
    let res = dao.send(4, DaoAction::Abort(0));
    assert!(res.main_failed());
}

#[test]
fn abort_proposal() {
    let sys = System::new();
    init_fungible_token(&sys);
    init_dao(&sys);

    let ft = sys.get_program(1);
    let dao = sys.get_program(2);
    create_membership_proposal(&dao, 0);
    vote(&dao, 0, Vote::Yes);

    let res = dao.send(4, DaoAction::Abort(0));
    assert!(res.contains(&(
        4,
        DaoEvent::Abort {
            member: 4.into(),
            proposal_id: 0,
            amount: 1000,
        }
        .encode()
    )));

    // must fail since the proposal has already been aborted
    let res = dao.send(4, DaoAction::Abort(0));
    assert!(res.main_failed());

    let res = ft.send(3, FTAction::BalanceOf(4.into()));
    assert!(res.contains(&(3, FTEvent::Balance(10000000).encode())));
}

#[test]
fn cancel_proposal_failures() {
    let sys = System::new();
    sys.init_logger();
    init_fungible_token(&sys);
    init_dao(&sys);

    let dao = sys.get_program(2);
    create_membership_proposal(&dao, 0);

    // must fail since the proposal doesnt exist
    let res = dao.send(3, DaoAction::CancelProposal(1));
    assert!(res.main_failed());

    // must fail since the caller isn't the proposer
    let res = dao.send(4, DaoAction::CancelProposal(0));
    assert!(res.main_failed());

    // must fail since the voting period isnt over
    let res = dao.send(3, DaoAction::CancelProposal(0));
    assert!(res.main_failed());

    let res = dao.send(4, DaoAction::Abort(0));
    assert!(!res.main_failed());

    sys.spend_blocks(1000000001);
    // must fail since the proposal has been aborted
    let res = dao.send(3, DaoAction::CancelProposal(0));
    assert!(res.main_failed());

    create_membership_proposal(&dao, 1);
    vote(&dao, 1, Vote::Yes);
    sys.spend_blocks(1000000001);
    // must fail since YES votes > NO votes
    let res = dao.send(3, DaoAction::CancelProposal(1));
    assert!(res.main_failed());
}

#[test]
fn cancel_proposal() {
    let sys = System::new();
    init_fungible_token(&sys);
    init_dao(&sys);

    let ft = sys.get_program(1);
    let dao = sys.get_program(2);
    create_membership_proposal(&dao, 0);
    vote(&dao, 0, Vote::No);
    sys.spend_blocks(1000000001);

    let res = dao.send(3, DaoAction::CancelProposal(0));

    assert!(res.contains(&(
        3,
        DaoEvent::Cancel {
            member: 3.into(),
            proposal_id: 0,
        }
        .encode()
    )));
    // must fail since the proposal has already been cancelled
    let res = dao.send(3, DaoAction::CancelProposal(0));
    assert!(res.main_failed());

    let res = ft.send(3, FTAction::BalanceOf(4.into()));
    assert!(res.contains(&(3, FTEvent::Balance(10000000).encode())));
}

#[test]
fn ragequit_failures() {
    let sys = System::new();
    sys.init_logger();
    init_fungible_token(&sys);
    init_dao(&sys);

    let dao = sys.get_program(2);
    create_membership_proposal(&dao, 0);
    vote(&dao, 0, Vote::Yes);
    sys.spend_blocks(1000000001);
    let res = dao.send(3, DaoAction::ProcessProposal(0));
    assert!(!res.main_failed());
    // must fail since admin can not ragequit
    let res = dao.send(3, DaoAction::RageQuit(1000));
    assert!(res.main_failed());

    // must fail since account is not a DAO member
    let res = dao.send(5, DaoAction::RageQuit(1000));
    assert!(res.main_failed());

    // must fail since account has unsufficient shares
    let res = dao.send(4, DaoAction::RageQuit(1001));
    assert!(res.main_failed());

    create_membership_proposal(&dao, 1);
    let res = dao.send(
        4,
        DaoAction::SubmitVote {
            proposal_id: 1,
            vote: Vote::Yes,
        },
    );
    assert!(!res.main_failed());

    // must fail since account cant ragequit until highest index proposal member voted YES on is processed
    let res = dao.send(4, DaoAction::RageQuit(1001));
    assert!(res.main_failed());
}

#[test]
fn ragequit() {
    let sys = System::new();
    sys.init_logger();
    init_fungible_token(&sys);
    init_dao(&sys);

    let ft = sys.get_program(1);
    let dao = sys.get_program(2);
    create_membership_proposal(&dao, 0);
    vote(&dao, 0, Vote::Yes);
    sys.spend_blocks(1000000001);
    let res = dao.send(3, DaoAction::ProcessProposal(0));
    assert!(!res.main_failed());
    // must fail since admin can not ragequit
    let res = dao.send(4, DaoAction::RageQuit(1000));
    assert!(res.contains(&(
        4,
        DaoEvent::RageQuit {
            member: 4.into(),
            amount: 999,
        }
        .encode()
    )));

    let res = ft.send(3, FTAction::BalanceOf(4.into()));
    assert!(res.contains(&(3, FTEvent::Balance(9999999).encode())));
}

#[test]
fn delegate_failures() {
    let sys = System::new();
    sys.init_logger();
    init_fungible_token(&sys);
    init_dao(&sys);
    let dao = sys.get_program(2);
    // must fail since account is not a DAO member
    let res = dao.send(4, DaoAction::UpdateDelegateKey(5.into()));
    assert!(res.main_failed());

    create_membership_proposal(&dao, 0);
    vote(&dao, 0, Vote::Yes);
    sys.spend_blocks(1000000001);
    let res = dao.send(3, DaoAction::ProcessProposal(0));
    assert!(!res.main_failed());

    let res = dao.send(3, DaoAction::UpdateDelegateKey(5.into()));
    assert!(!res.main_failed());

    // must fail since the delegate address is already used
    let res = dao.send(4, DaoAction::UpdateDelegateKey(5.into()));
    assert!(res.main_failed());
}

#[test]
fn delegate() {
    let sys = System::new();
    sys.init_logger();
    init_fungible_token(&sys);
    init_dao(&sys);
    let dao = sys.get_program(2);
    // must fail since account is not a DAO member
    let res = dao.send(3, DaoAction::UpdateDelegateKey(5.into()));
    assert!(res.contains(&(
        3,
        DaoEvent::DelegateKeyUpdated {
            member: 3.into(),
            delegate: 5.into(),
        }
        .encode()
    )));

    create_membership_proposal(&dao, 0);
    // voting from the delegate key
    let res = dao.send(
        5,
        DaoAction::SubmitVote {
            proposal_id: 0,
            vote: Vote::Yes,
        },
    );
    assert!(!res.main_failed());
}
