use dao_light_io::*;
use fungible_token_io::{FTAction, InitConfig};
use gtest::{Program, RunResult, System};

pub const MEMBERS: &[u64] = &[3, 4, 5, 6];
pub const ZERO_ID: u64 = 0;

pub fn init_fungible_token(sys: &System) {
    sys.init_logger();
    let ft = Program::from_file(
        sys,
        "../target/wasm32-unknown-unknown/debug/fungible_token.opt.wasm",
    );

    let res = ft.send(
        MEMBERS[0],
        InitConfig {
            name: String::from("MyToken"),
            symbol: String::from("MTK"),
            decimals: 18,
        },
    );

    assert!(!res.main_failed());
    MEMBERS.iter().for_each(|member| {
        let res = ft.send(*member, FTAction::Mint(10000000));
        assert!(!res.main_failed());
    });
}

pub fn init_dao(sys: &System) {
    sys.init_logger();
    let dao = Program::current_opt(sys);
    let res = dao.send(
        MEMBERS[0],
        InitDao {
            approved_token_program_id: 1.into(),
            period_duration: 100000,
            grace_period_length: 100000,
            voting_period_length: 1000000,
        },
    );
    assert!(!res.main_failed());
}

pub fn deposit(dao: &Program<'_>, member: u64, amount: u128) -> RunResult {
    dao.send(member, DaoAction::Deposit { amount })
}

pub fn approve(ft: &Program<'_>, member: u64, to: u64, amount: u128) -> RunResult {
    ft.send(
        member,
        FTAction::Approve {
            to: to.into(),
            amount,
        },
    )
}

pub fn proposal(dao: &Program<'_>, member: u64, applicant: u64, amount: u128) -> RunResult {
    dao.send(
        member,
        DaoAction::SubmitFundingProposal {
            applicant: applicant.into(),
            amount,
            quorum: 80,
            details: "Funding proposal".to_string(),
        },
    )
}

pub fn vote(dao: &Program<'_>, member: u64, proposal_id: u128, vote: Vote) -> RunResult {
    dao.send(member, DaoAction::SubmitVote { proposal_id, vote })
}

pub fn process(dao: &Program<'_>, member: u64, proposal_id: u128) -> RunResult {
    dao.send(member, DaoAction::ProcessProposal { proposal_id })
}

pub fn ragequit(dao: &Program<'_>, member: u64, amount: u128) -> RunResult {
    dao.send(member, DaoAction::RageQuit { amount })
}
