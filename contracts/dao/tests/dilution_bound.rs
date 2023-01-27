pub mod utils;

use crate::utils::*;
use dao_io::*;
use gtest::{Program, System};

#[test]
fn dilution_bound() {
    let system = System::new();
    system.init_logger();
    let ftoken = Program::ftoken(&system);
    let dao = Program::dao(&system);
    let applicant: u64 = 200;
    let token_tribute: u128 = 10_000;
    let shares_requested: u128 = 10_000;
    let mut total_shares = 10 * shares_requested + 1;
    let mut balance = 10 * token_tribute;
    let ragequit_amount: u128 = 9_000;
    let quorum: u128 = 10;
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

    // APPLICANT[0] votes YES
    dao.submit_vote(APPLICANTS[0], proposal_id, Vote::Yes, false);
    // APPLICANT[1] votes YES
    dao.submit_vote(APPLICANTS[1], proposal_id, Vote::Yes, false);

    // APPLICANT[2]-APPLICANT[9] ragequit
    for applicant in APPLICANTS.iter().take(10).skip(2) {
        let funds = (balance * ragequit_amount) / (total_shares);
        dao.ragequit(*applicant, ragequit_amount, funds, false);
        total_shares -= ragequit_amount;
        balance -= funds;
    }

    // quorum is achieved and number of YES votes > NO votes
    // but max_total_shares_at_yes_vote > total_shares * dilution_bound
    // proposal is not passed
    system.spend_blocks(((VOTING_PERIOD_LENGTH + GRACE_PERIOD_LENGTH) / 1000) as u32);
    dao.process_proposal(proposal_id, false, false);
}
