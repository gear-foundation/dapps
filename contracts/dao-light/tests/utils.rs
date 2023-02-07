use dao_light_io::*;
use ft_io::*;
use gtest::{Program, RunResult, System};

pub const MEMBERS: &[u64] = &[3, 4, 5, 6];
pub const ZERO_ID: u64 = 0;

pub fn init_fungible_token(sys: &System) {
    sys.init_logger();
    let ft = Program::from_file(sys, "./target/ft.wasm");

    let res = ft.send(
        MEMBERS[0],
        InitConfig {
            name: String::from("MyToken"),
            symbol: String::from("MTK"),
            decimals: 18,
        },
    );

    assert!(res.log().is_empty());
    MEMBERS.iter().for_each(|member| {
        let res = ft.send(*member, FTAction::Mint(10000000));
        assert!(!res.main_failed());
    });
}

pub fn init_dao(sys: &System) {
    sys.init_logger();
    let dao = Program::current(sys);
    let res = dao.send(
        MEMBERS[0],
        InitDao {
            approved_token_program_id: 1.into(),
            period_duration: 10000000,
            grace_period_length: 10000000,
            voting_period_length: 100000000,
        },
    );
    assert!(res.log().is_empty());
}

pub fn deposit(dao: &Program, member: u64, amount: u128) -> RunResult {
    dao.send(member, DaoAction::Deposit { amount })
}

pub fn proposal(dao: &Program, member: u64, applicant: u64, amount: u128) -> RunResult {
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

pub fn vote(dao: &Program, member: u64, proposal_id: u128, vote: Vote) -> RunResult {
    dao.send(member, DaoAction::SubmitVote { proposal_id, vote })
}

pub fn process(dao: &Program, member: u64, proposal_id: u128) -> RunResult {
    dao.send(member, DaoAction::ProcessProposal { proposal_id })
}

pub fn ragequit(dao: &Program, member: u64, amount: u128) -> RunResult {
    dao.send(member, DaoAction::RageQuit { amount })
}
