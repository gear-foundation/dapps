pub mod utils;

use crate::utils::*;
use dao_io::*;
use gtest::{Program, System};

#[test]
fn submit_membership_proposal() {
    let system = System::new();
    system.init_logger();
    let _ftoken = Program::ftoken(&system);
    let dao = Program::dao(&system);
    let token_tribute: u128 = 10_000;
    let shares_requested: u128 = 10_000;
    let applicant: u64 = 200;
    let quorum: u128 = 50;
    let proposal_id: u128 = 0;

    let user = 1000;
    // must fail since account is neither a member nor a delegate
    dao.submit_membership_proposal(
        user,
        proposal_id,
        applicant,
        token_tribute,
        shares_requested,
        quorum,
        true,
    );

    // must fail since an applicant is not in the whitelist
    dao.submit_membership_proposal(
        ADMIN,
        proposal_id,
        applicant,
        token_tribute,
        shares_requested,
        quorum,
        true,
    );
}

#[test]
fn submit_funding_proposal() {
    let system = System::new();
    system.init_logger();
    let _ftoken = Program::ftoken(&system);
    let dao = Program::dao(&system);
    let applicant: u64 = 200;
    let amount: u128 = 10_000;
    let quorum: u128 = 50;
    let proposal_id: u128 = 0;

    let user = 1000;
    // must fail since account is neither a member nor a delegate
    dao.submit_funding_proposal(user, proposal_id, applicant, amount, quorum, true);
}

#[test]
fn submit_vote() {
    let system = System::new();
    system.init_logger();
    let ftoken = Program::ftoken(&system);
    let dao = Program::dao(&system);
    let token_tribute: u128 = 10_000;
    let shares_requested: u128 = 10_000;
    let applicant: u64 = 200;
    let quorum: u128 = 50;
    let proposal_id: u128 = 0;
    let user = 1000;

    dao.add_to_whitelist(ADMIN, applicant, false);
    ftoken.mint(0, applicant, applicant, 2 * token_tribute);
    ftoken.approve(1, applicant, DAO_ID, 2 * token_tribute);
    dao.submit_membership_proposal(
        ADMIN,
        proposal_id,
        applicant,
        token_tribute,
        shares_requested,
        quorum,
        false,
    );

    dao.submit_membership_proposal(
        ADMIN,
        proposal_id + 1,
        applicant,
        token_tribute,
        shares_requested,
        quorum,
        false,
    );

    // must fail since the the account is neither a member nor a delegate
    dao.submit_vote(user, proposal_id, Vote::Yes, true);

    // must fail since the voting period has not started
    dao.submit_vote(ADMIN, proposal_id + 1, Vote::Yes, true);

    system.spend_blocks((PERIOD_DURATION / 1000) as u32);

    dao.submit_vote(ADMIN, proposal_id + 1, Vote::Yes, false);

    // must fail since the account has already voted on this proposal
    dao.submit_vote(ADMIN, proposal_id + 1, Vote::Yes, true);

    system.spend_blocks(((VOTING_PERIOD_LENGTH + 1000) / 1000) as u32);

    // must fail since the proposal voting period has expired
    dao.submit_vote(ADMIN, proposal_id + 1, Vote::Yes, true);

    // must fail since the proposal does not exist
    dao.submit_vote(ADMIN, proposal_id + 2, Vote::Yes, true);

    ftoken.mint(2, applicant, applicant, 2 * token_tribute);
    ftoken.approve(3, applicant, DAO_ID, token_tribute);
    dao.submit_membership_proposal(
        ADMIN,
        proposal_id + 2,
        applicant,
        token_tribute,
        shares_requested,
        quorum,
        false,
    );

    dao.abort(applicant, proposal_id + 2, false);
    // must fail since the proposal has been aborted
    dao.submit_vote(ADMIN, proposal_id + 2, Vote::Yes, true);
}

#[test]
fn process_proposal() {
    let system = System::new();
    system.init_logger();
    let ftoken = Program::ftoken(&system);
    let dao = Program::dao(&system);
    let token_tribute: u128 = 10_000;
    let shares_requested: u128 = 10_000;
    let applicant: u64 = 200;
    let quorum: u128 = 50;
    let proposal_id: u128 = 0;

    dao.add_to_whitelist(ADMIN, applicant, false);
    ftoken.mint(0, applicant, applicant, 2 * token_tribute);
    ftoken.approve(1, applicant, DAO_ID, 2 * token_tribute);

    // must fail since proposal does not exist
    dao.process_proposal(proposal_id, true, true);

    dao.submit_membership_proposal(
        ADMIN,
        proposal_id,
        applicant,
        token_tribute,
        shares_requested,
        quorum,
        false,
    );

    dao.submit_membership_proposal(
        ADMIN,
        proposal_id + 1,
        applicant,
        token_tribute,
        shares_requested,
        quorum,
        false,
    );

    // must fail since previous proposal must be processed
    dao.process_proposal(proposal_id, true, true);

    // must fail since the proposal is not ready to be processed
    dao.process_proposal(proposal_id + 1, true, true);
    dao.abort(applicant, proposal_id + 1, false);
    system.spend_blocks(((VOTING_PERIOD_LENGTH + GRACE_PERIOD_LENGTH) / 1000) as u32);

    dao.process_proposal(proposal_id, false, false);

    // must fail since the proposal has already been processed
    dao.process_proposal(proposal_id, true, true);

    // must fail since the proposal has been aborted
    dao.process_proposal(proposal_id + 1, true, true);
}

#[test]
fn abort() {
    let system = System::new();
    system.init_logger();
    let ftoken = Program::ftoken(&system);
    let dao = Program::dao(&system);
    let token_tribute: u128 = 10_000;
    let shares_requested: u128 = 10_000;
    let applicant: u64 = 200;
    let quorum: u128 = 50;
    let proposal_id: u128 = 0;

    dao.add_to_whitelist(ADMIN, applicant, false);
    ftoken.mint(0, applicant, applicant, token_tribute);
    ftoken.approve(1, applicant, DAO_ID, token_tribute);

    // must fail since proposal does not exist
    dao.abort(applicant, proposal_id, true);

    dao.submit_membership_proposal(
        ADMIN,
        proposal_id,
        applicant,
        token_tribute,
        shares_requested,
        quorum,
        false,
    );

    // must fail since the caller must be the applicant
    dao.abort(ADMIN, proposal_id, true);

    ftoken.check_balance(applicant, 0);
    dao.abort(applicant, proposal_id, false);

    ftoken.check_balance(applicant, token_tribute);

    ftoken.approve(2, applicant, DAO_ID, token_tribute);
    dao.submit_membership_proposal(
        ADMIN,
        proposal_id + 1,
        applicant,
        token_tribute,
        shares_requested,
        quorum,
        false,
    );

    system.spend_blocks((ABORT_WINDOW / 1000) as u32);

    // must fail since the the abort window is over
    dao.abort(ADMIN, proposal_id + 1, true);

    dao.submit_funding_proposal(
        ADMIN,
        proposal_id + 2,
        applicant,
        token_tribute,
        quorum,
        false,
    );

    // must fail since the the proposal must be membership
    dao.abort(ADMIN, proposal_id + 2, true);
}

#[test]
fn ragequit() {
    let system = System::new();
    system.init_logger();
    let ftoken = Program::ftoken(&system);
    let dao = Program::dao(&system);
    let applicant: u64 = 200;
    let token_tribute: u128 = 10_000;
    let shares_requested: u128 = 10_000;
    let mut total_shares = 10 * shares_requested + 1;
    let mut balance = 10 * token_tribute;
    let ragequit_amount: u128 = 6_000;
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

    //must fail since the applicant voted YES and the proposal has not been processed
    dao.ragequit(14, ragequit_amount, 0, true);

    //must fail since an account is not a DAO member
    dao.ragequit(300, ragequit_amount, 0, true);

    //must fail since a memeber has unsufficient shares
    dao.ragequit(17, 2 * ragequit_amount, 0, true);

    // successfull ragequit
    ftoken.check_balance(17, 0);
    let funds = (balance * ragequit_amount) / (total_shares);
    dao.ragequit(17, ragequit_amount, funds, false);
    total_shares -= ragequit_amount;
    balance -= funds;
    ftoken.check_balance(17, funds);
    ftoken.check_balance(DAO_ID, balance + token_tribute);

    // successfull ragequit
    ftoken.check_balance(18, 0);
    let funds = (balance * ragequit_amount) / (total_shares);
    dao.ragequit(18, ragequit_amount, funds, false);
    balance -= funds;
    ftoken.check_balance(18, funds);
    ftoken.check_balance(DAO_ID, balance + token_tribute);
}
