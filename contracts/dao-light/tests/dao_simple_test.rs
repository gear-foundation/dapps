mod utils;

use dao_light_io::*;
use gstd::Encode;
use gtest::System;
use utils::*;

#[test]
fn deposit_tokens() {
    let sys = System::new();
    init_fungible_token(&sys);
    init_dao(&sys);
    let dao = sys.get_program(2);
    assert!(!approve(&sys.get_program(1), MEMBERS[0], 2, 1000).main_failed());
    let res = deposit(&dao, MEMBERS[0], 1000);
    assert!(res.contains(&(
        MEMBERS[0],
        DaoEvent::Deposit {
            member: MEMBERS[0].into(),
            share: 1000,
        }
        .encode()
    )));
}

#[test]
fn create_proposal() {
    let sys = System::new();
    init_fungible_token(&sys);
    init_dao(&sys);
    let ft = sys.get_program(1);
    let dao = sys.get_program(2);
    assert!(!approve(&ft, MEMBERS[0], 2, 1000).main_failed());
    assert!(!deposit(&dao, MEMBERS[0], 1000).main_failed());
    let res = proposal(&dao, MEMBERS[0], MEMBERS[2], 800);
    assert!(res.contains(&(
        MEMBERS[0],
        DaoEvent::SubmitFundingProposal {
            proposer: MEMBERS[0].into(),
            applicant: MEMBERS[2].into(),
            proposal_id: 0,
            amount: 800,
        }
        .encode()
    )));

    let res = proposal(&dao, MEMBERS[0], MEMBERS[2], 100);
    assert!(res.contains(&(
        MEMBERS[0],
        DaoEvent::SubmitFundingProposal {
            proposer: MEMBERS[0].into(),
            applicant: MEMBERS[2].into(),
            proposal_id: 1,
            amount: 100,
        }
        .encode()
    )));
}

#[test]
fn create_proposal_failures() {
    let sys = System::new();
    init_fungible_token(&sys);
    init_dao(&sys);
    let ft = sys.get_program(1);
    let dao = sys.get_program(2);
    assert!(!approve(&ft, MEMBERS[0], 2, 1000).main_failed());
    assert!(!deposit(&dao, MEMBERS[0], 1000).main_failed());
    // must fail since dao has not enough tokens for funding
    assert!(proposal(&dao, MEMBERS[0], MEMBERS[2], 1100).main_failed());
    // creates proposal for funding and locks tokens
    assert!(!proposal(&dao, MEMBERS[0], MEMBERS[2], 800).main_failed());
    // must fail since tokens of dao are locked because of previous proposal
    assert!(proposal(&dao, MEMBERS[0], MEMBERS[2], 300).main_failed());
    // must fail since proposal is made for the zero address
    assert!(proposal(&dao, MEMBERS[0], ZERO_ID, 100).main_failed());
    // must fail since `msg::source()` is not a dao member
    assert!(proposal(&dao, MEMBERS[1], MEMBERS[0], 100).main_failed());
}

#[test]
fn vote_on_proposal() {
    let sys = System::new();
    init_fungible_token(&sys);
    init_dao(&sys);
    let ft = sys.get_program(1);
    let dao = sys.get_program(2);
    assert!(!approve(&ft, MEMBERS[0], 2, 1000).main_failed());
    assert!(!approve(&ft, MEMBERS[1], 2, 1000).main_failed());
    assert!(!deposit(&dao, MEMBERS[0], 1000).main_failed());
    assert!(!deposit(&dao, MEMBERS[1], 1000).main_failed());
    //submit funding proposal
    assert!(!proposal(&dao, MEMBERS[0], MEMBERS[2], 800).main_failed());
    // vote YES
    let res = vote(&dao, MEMBERS[0], 0, Vote::Yes);
    assert!(res.contains(&(
        MEMBERS[0],
        DaoEvent::SubmitVote {
            account: MEMBERS[0].into(),
            proposal_id: 0,
            vote: Vote::Yes,
        }
        .encode()
    )));
    // vote NO
    let res = vote(&dao, MEMBERS[1], 0, Vote::No);
    assert!(res.contains(&(
        MEMBERS[1],
        DaoEvent::SubmitVote {
            account: MEMBERS[1].into(),
            proposal_id: 0,
            vote: Vote::No,
        }
        .encode()
    )));
}

#[test]
fn vote_on_proposal_failures() {
    let sys = System::new();
    init_fungible_token(&sys);
    init_dao(&sys);

    let ft = sys.get_program(1);
    let dao = sys.get_program(2);

    assert!(!approve(&ft, MEMBERS[0], 2, 1000).main_failed());
    assert!(!approve(&ft, MEMBERS[1], 2, 1000).main_failed());
    assert!(!deposit(&dao, MEMBERS[0], 1000).main_failed());
    assert!(!deposit(&dao, MEMBERS[1], 1000).main_failed());
    //submit funding proposal
    assert!(!proposal(&dao, MEMBERS[0], MEMBERS[2], 800).main_failed());
    // must fail since the proposal does not exist
    assert!(vote(&dao, MEMBERS[1], 1, Vote::No).main_failed());
    // must fail since `msg::source()` is not a dao member
    assert!(vote(&dao, MEMBERS[2], 0, Vote::No).main_failed());

    //submit one more funding proposal
    assert!(!proposal(&dao, MEMBERS[0], MEMBERS[2], 800).main_failed());
    // must fail since the voting period has not started
    assert!(vote(&dao, MEMBERS[1], 1, Vote::No).main_failed());

    // vote on the proposal
    assert!(!vote(&dao, MEMBERS[1], 0, Vote::No).main_failed());

    // must fail since account has already voted on that proposal
    assert!(vote(&dao, MEMBERS[1], 0, Vote::Yes).main_failed());

    sys.spend_blocks(1000001);

    // must fail since proposal voting period has expired
    assert!(vote(&dao, MEMBERS[0], 0, Vote::Yes).main_failed());
}

#[test]
fn process_proposal() {
    let sys = System::new();
    init_fungible_token(&sys);
    init_dao(&sys);

    let ft = sys.get_program(1);
    let dao = sys.get_program(2);

    assert!(!approve(&ft, MEMBERS[0], 2, 1000).main_failed());
    assert!(!approve(&ft, MEMBERS[1], 2, 2000).main_failed());
    assert!(!approve(&ft, MEMBERS[2], 2, 3000).main_failed());
    assert!(!approve(&ft, MEMBERS[3], 2, 4000).main_failed());

    assert!(!deposit(&dao, MEMBERS[0], 1000).main_failed());
    assert!(!deposit(&dao, MEMBERS[1], 2000).main_failed());
    assert!(!deposit(&dao, MEMBERS[2], 3000).main_failed());
    assert!(!deposit(&dao, MEMBERS[3], 4000).main_failed());

    //submit funding proposal
    assert!(!proposal(&dao, MEMBERS[0], MEMBERS[2], 800).main_failed());
    // votes YES
    assert!(!vote(&dao, MEMBERS[0], 0, Vote::Yes).main_failed());
    assert!(!vote(&dao, MEMBERS[2], 0, Vote::Yes).main_failed());
    assert!(!vote(&dao, MEMBERS[3], 0, Vote::Yes).main_failed());
    // votes NO
    assert!(!vote(&dao, MEMBERS[1], 0, Vote::No).main_failed());

    sys.spend_blocks(1000001);

    let res = process(&dao, MEMBERS[0], 0);
    assert!(res.contains(&(
        MEMBERS[0],
        DaoEvent::ProcessProposal {
            applicant: MEMBERS[2].into(),
            proposal_id: 0,
            did_pass: true,
        }
        .encode()
    )));

    //submit funding proposal
    assert!(!proposal(&dao, MEMBERS[0], MEMBERS[2], 800).main_failed());

    // votes NO
    assert!(!vote(&dao, MEMBERS[0], 1, Vote::No).main_failed());
    assert!(!vote(&dao, MEMBERS[2], 1, Vote::No).main_failed());
    // votes YES
    assert!(!vote(&dao, MEMBERS[3], 1, Vote::Yes).main_failed());
    assert!(!vote(&dao, MEMBERS[1], 1, Vote::Yes).main_failed());

    sys.spend_blocks(1000001);

    let res = process(&dao, MEMBERS[0], 1);
    assert!(res.contains(&(
        MEMBERS[0],
        DaoEvent::ProcessProposal {
            applicant: MEMBERS[2].into(),
            proposal_id: 1,
            did_pass: false,
        }
        .encode()
    )));
}

#[test]
fn process_proposal_failures() {
    let sys = System::new();
    init_fungible_token(&sys);
    init_dao(&sys);

    let ft = sys.get_program(1);
    let dao = sys.get_program(2);

    assert!(!approve(&ft, MEMBERS[0], 2, 1000).main_failed());
    assert!(!deposit(&dao, MEMBERS[0], 1000).main_failed());

    //submit funding proposal
    assert!(!proposal(&dao, MEMBERS[0], MEMBERS[2], 800).main_failed());
    //must fail since proposal is not ready to be processed
    assert!(process(&dao, MEMBERS[0], 0).main_failed());
    //must fail since previous proposal must be processed
    assert!(process(&dao, MEMBERS[0], 1).main_failed());

    sys.spend_blocks(1000001);

    assert!(!process(&dao, MEMBERS[0], 0).main_failed());
    //must fail since proposal does not exist
    assert!(process(&dao, MEMBERS[0], 1).main_failed());
    //must fail since proposal has already been processed
    assert!(process(&dao, MEMBERS[0], 0).main_failed());
}

#[test]
fn ragequit_dao() {
    let sys = System::new();
    init_fungible_token(&sys);
    init_dao(&sys);

    let ft = sys.get_program(1);
    let dao = sys.get_program(2);

    assert!(!approve(&ft, MEMBERS[1], 2, 1000).main_failed());
    assert!(!deposit(&dao, MEMBERS[1], 1000).main_failed());

    let res = ragequit(&dao, MEMBERS[1], 800);
    assert!(res.contains(&(
        MEMBERS[1],
        DaoEvent::RageQuit {
            member: MEMBERS[1].into(),
            amount: 800,
        }
        .encode()
    )));

    assert!(!approve(&ft, MEMBERS[0], 2, 1000).main_failed());
    assert!(!approve(&ft, MEMBERS[1], 2, 1000).main_failed());
    assert!(!approve(&ft, MEMBERS[2], 2, 1000).main_failed());
    assert!(!approve(&ft, MEMBERS[3], 2, 1000).main_failed());

    assert!(!deposit(&dao, MEMBERS[0], 1000).main_failed());
    assert!(!deposit(&dao, MEMBERS[1], 1000).main_failed());
    assert!(!deposit(&dao, MEMBERS[2], 1000).main_failed());
    assert!(!deposit(&dao, MEMBERS[3], 1000).main_failed());

    //submit funding proposal
    assert!(!proposal(&dao, MEMBERS[1], MEMBERS[2], 800).main_failed());

    // votes YES
    assert!(!vote(&dao, MEMBERS[0], 0, Vote::Yes).main_failed());
    assert!(!vote(&dao, MEMBERS[1], 0, Vote::Yes).main_failed());
    assert!(!vote(&dao, MEMBERS[2], 0, Vote::Yes).main_failed());
    assert!(!vote(&dao, MEMBERS[3], 0, Vote::Yes).main_failed());

    sys.spend_blocks(1000001);

    assert!(!process(&dao, MEMBERS[0], 0).main_failed());

    let res = ragequit(&dao, MEMBERS[1], 800);
    assert!(res.contains(&(
        MEMBERS[1],
        DaoEvent::RageQuit {
            member: MEMBERS[1].into(),
            amount: 647,
        }
        .encode()
    )));
}

#[test]
fn ragequit_failures() {
    let sys = System::new();
    init_fungible_token(&sys);
    init_dao(&sys);

    let ft = sys.get_program(1);
    let dao = sys.get_program(2);

    assert!(!approve(&ft, MEMBERS[0], 2, 1000).main_failed());
    assert!(!deposit(&dao, MEMBERS[0], 1000).main_failed());
    // must fail since the account is not a DAO member
    assert!(ragequit(&dao, MEMBERS[1], 800).main_failed());
    // must fail since the account has no sufficient shares
    assert!(ragequit(&dao, MEMBERS[0], 1100).main_failed());
    //submit funding proposal
    assert!(!proposal(&dao, MEMBERS[0], MEMBERS[2], 800).main_failed());
    assert!(!vote(&dao, MEMBERS[0], 0, Vote::Yes).main_failed());
    // must fail since cant ragequit until highest index proposal member voted YES on is processed
    assert!(ragequit(&dao, MEMBERS[0], 100).main_failed());
}
