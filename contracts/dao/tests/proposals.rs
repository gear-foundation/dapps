pub mod utils;

use crate::utils::*;
use dao_io::*;
use gtest::{Program, System};

#[test]
fn membership_proposals() {
    let system = System::new();
    system.init_logger();
    let ftoken = Program::ftoken(&system);
    let dao = Program::dao(&system);
    let token_tribute: u128 = 10_000;
    let shares_requested: u128 = 10_000;
    let applicant: u64 = 200;
    let quorum: u128 = 50;
    let mut proposal_id: u128 = 0;

    // add members to DAO
    for applicant in APPLICANTS {
        ftoken.mint(0, *applicant, *applicant, token_tribute);
        ftoken.approve(1, *applicant, DAO_ID, token_tribute);
        dao.add_member(
            &system,
            proposal_id,
            *applicant,
            token_tribute,
            shares_requested,
        );
        proposal_id += 1;
    }

    //membership proposal
    ftoken.mint(0, applicant, applicant, token_tribute);
    ftoken.approve(1, applicant, DAO_ID, token_tribute);

    dao.add_to_whitelist(ADMIN, applicant, false);
    dao.submit_membership_proposal(
        ADMIN,
        proposal_id,
        applicant,
        token_tribute,
        shares_requested,
        quorum,
        false,
    );

    // members of DAO vote
    for applicant in APPLICANTS {
        let vote: Vote = if applicant < &16 { Vote::Yes } else { Vote::No };
        dao.submit_vote(*applicant, proposal_id, vote, false);
    }

    system.spend_blocks(VOTING_PERIOD_LENGTH as u32 + 1);

    // proposal passed
    dao.process_proposal(proposal_id, true, false);

    // check balance of applicant
    ftoken.check_balance(applicant, 0);

    // new proposal
    ftoken.mint(2, applicant, applicant, token_tribute);
    ftoken.approve(3, applicant, DAO_ID, token_tribute);
    proposal_id += 1;
    dao.submit_membership_proposal(
        ADMIN,
        proposal_id,
        applicant,
        token_tribute,
        shares_requested,
        quorum,
        false,
    );

    // DAO members vote
    for applicant in APPLICANTS {
        let vote: Vote = if applicant < &16 { Vote::No } else { Vote::Yes };
        dao.submit_vote(*applicant, proposal_id, vote, false);
    }

    system.spend_blocks(VOTING_PERIOD_LENGTH as u32 + 1);

    // proposal didn't pass
    dao.process_proposal(proposal_id, false, false);

    // check balance of applicant (it must be equal to token tribute since proposal did not pass)
    ftoken.check_balance(applicant, token_tribute);
    // check balance of DAO
    ftoken.check_balance(DAO_ID, 11 * token_tribute);
}

#[test]
fn funding_proposals() {
    let system = System::new();
    system.init_logger();
    let ftoken = Program::ftoken(&system);
    let dao = Program::dao(&system);
    let amount = 30_000;
    let token_tribute: u128 = 10_000;
    let shares_requested: u128 = 10_000;
    let receiver: u64 = 100;
    let quorum: u128 = 50;
    let mut proposal_id: u128 = 0;

    // add members to DAO
    for applicant in APPLICANTS {
        ftoken.mint(0, *applicant, *applicant, token_tribute);
        ftoken.approve(1, *applicant, DAO_ID, token_tribute);
        dao.add_member(
            &system,
            proposal_id,
            *applicant,
            token_tribute,
            shares_requested,
        );
        proposal_id += 1;
    }

    //funding proposal
    dao.submit_funding_proposal(ADMIN, proposal_id, receiver, amount, quorum, false);

    // members of DAO vote
    for applicant in APPLICANTS {
        let vote: Vote = if applicant < &16 { Vote::Yes } else { Vote::No };
        dao.submit_vote(*applicant, proposal_id, vote, false);
    }

    system.spend_blocks(VOTING_PERIOD_LENGTH as u32 + 1);

    // proposal passed
    dao.process_proposal(proposal_id, true, false);

    // check balance of receiver
    ftoken.check_balance(receiver, amount);
    // check balance of DAO
    ftoken.check_balance(DAO_ID, 10 * token_tribute - amount);

    // new proposal
    proposal_id += 1;
    dao.submit_funding_proposal(ADMIN, proposal_id, receiver, amount, quorum, false);

    // DAO members vote
    for applicant in APPLICANTS {
        let vote: Vote = if applicant < &16 { Vote::No } else { Vote::Yes };
        dao.submit_vote(*applicant, proposal_id, vote, false);
    }

    system.spend_blocks(VOTING_PERIOD_LENGTH as u32 + 1);

    // proposal didn't pass
    dao.process_proposal(proposal_id, false, false);

    // check balance of applicant
    ftoken.check_balance(receiver, amount);
    // check balance of DAO
    ftoken.check_balance(DAO_ID, 10 * token_tribute - amount);
}
