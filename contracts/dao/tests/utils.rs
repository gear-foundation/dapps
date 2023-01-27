use dao_io::{DaoAction, DaoEvent, InitDao, Vote};
use ft_logic_io::Action;
use ft_main_io::{FTokenAction, FTokenEvent, InitFToken};

use gstd::prelude::*;
use gtest::{Program, System};
pub const ADMIN: u64 = 100;
const TOKEN_ID: u64 = 1;
pub const DAO_ID: u64 = 2;
pub const PERIOD_DURATION: u64 = 10000000;
pub const VOTING_PERIOD_LENGTH: u64 = 100000000;
pub const GRACE_PERIOD_LENGTH: u64 = 10000000;
const DILUTION_BOUND: u8 = 3;
pub const ABORT_WINDOW: u64 = 10000000;
pub const APPLICANTS: &[u64] = &[10, 11, 12, 13, 14, 15, 16, 17, 18, 19];

pub trait Dao {
    fn dao(system: &System) -> Program;
    fn add_to_whitelist(&self, from: u64, account: u64, error: bool);
    #[allow(clippy::too_many_arguments)]
    fn submit_membership_proposal(
        &self,
        from: u64,
        proposal_id: u128,
        applicant: u64,
        token_tribute: u128,
        shares_requested: u128,
        quorum: u128,
        error: bool,
    );
    fn submit_funding_proposal(
        &self,
        from: u64,
        proposal_id: u128,
        applicant: u64,
        amount: u128,
        quorum: u128,
        error: bool,
    );
    fn process_proposal(&self, proposal_id: u128, passed: bool, error: bool);
    fn submit_vote(&self, from: u64, proposal_id: u128, vote: Vote, error: bool);
    fn ragequit(&self, from: u64, amount: u128, funds: u128, error: bool);
    fn abort(&self, from: u64, proposal_id: u128, error: bool);
    fn update_delegate_key(&self, from: u64, account: u64, error: bool);
    fn add_member(
        &self,
        system: &System,
        proposal_id: u128,
        applicant: u64,
        token_tribute: u128,
        shares_requested: u128,
    );
}

impl Dao for Program<'_> {
    fn dao(system: &System) -> Program {
        let dao = Program::current(system);
        assert!(!dao
            .send(
                ADMIN,
                InitDao {
                    admin: ADMIN.into(),
                    approved_token_program_id: TOKEN_ID.into(),
                    period_duration: PERIOD_DURATION,
                    voting_period_length: VOTING_PERIOD_LENGTH,
                    grace_period_length: GRACE_PERIOD_LENGTH,
                    dilution_bound: DILUTION_BOUND,
                    abort_window: ABORT_WINDOW,
                },
            )
            .main_failed());
        dao
    }

    fn add_to_whitelist(&self, from: u64, account: u64, error: bool) {
        let res = self.send(from, DaoAction::AddToWhiteList(account.into()));
        let reply = DaoEvent::MemberAddedToWhitelist(account.into()).encode();
        if error {
            assert!(res.main_failed());
        } else {
            assert!(res.contains(&(from, reply)));
        }
    }

    fn submit_membership_proposal(
        &self,
        from: u64,
        proposal_id: u128,
        applicant: u64,
        token_tribute: u128,
        shares_requested: u128,
        quorum: u128,
        error: bool,
    ) {
        let res = self.send(
            from,
            DaoAction::SubmitMembershipProposal {
                applicant: applicant.into(),
                token_tribute,
                shares_requested,
                quorum,
                details: String::from(""),
            },
        );
        let reply = DaoEvent::SubmitMembershipProposal {
            proposer: from.into(),
            applicant: applicant.into(),
            proposal_id,
            token_tribute,
        }
        .encode();
        if error {
            assert!(res.main_failed());
        } else {
            assert!(res.contains(&(from, reply)));
        }
    }
    fn submit_funding_proposal(
        &self,
        from: u64,
        proposal_id: u128,
        applicant: u64,
        amount: u128,
        quorum: u128,
        error: bool,
    ) {
        let res = self.send(
            from,
            DaoAction::SubmitFundingProposal {
                applicant: applicant.into(),
                amount,
                quorum,
                details: String::from(""),
            },
        );
        let reply = DaoEvent::SubmitFundingProposal {
            proposer: from.into(),
            applicant: applicant.into(),
            proposal_id,
            amount,
        }
        .encode();
        if error {
            assert!(res.main_failed());
        } else {
            assert!(res.contains(&(from, reply)));
        }
    }

    fn process_proposal(&self, proposal_id: u128, passed: bool, error: bool) {
        let res = self.send(ADMIN, DaoAction::ProcessProposal(proposal_id));
        let reply = DaoEvent::ProcessProposal {
            proposal_id,
            passed,
        }
        .encode();
        if error {
            assert!(res.main_failed());
        } else {
            assert!(res.contains(&(ADMIN, reply)));
        }
    }
    fn submit_vote(&self, from: u64, proposal_id: u128, vote: Vote, error: bool) {
        let res = self.send(
            from,
            DaoAction::SubmitVote {
                proposal_id,
                vote: vote.clone(),
            },
        );
        let reply = DaoEvent::SubmitVote {
            account: from.into(),
            proposal_id,
            vote,
        }
        .encode();
        if error {
            assert!(res.main_failed());
        } else {
            assert!(res.contains(&(from, reply)));
        }
    }
    fn ragequit(&self, from: u64, amount: u128, funds: u128, error: bool) {
        let res = self.send(from, DaoAction::RageQuit(amount));
        let reply = DaoEvent::RageQuit {
            member: from.into(),
            amount: funds,
        }
        .encode();
        if error {
            assert!(res.main_failed());
        } else {
            assert!(res.contains(&(from, reply)));
        }
    }
    fn abort(&self, from: u64, proposal_id: u128, error: bool) {
        let res = self.send(from, DaoAction::Abort(proposal_id));
        let reply = DaoEvent::Abort(proposal_id).encode();
        if error {
            assert!(res.main_failed());
        } else {
            assert!(res.contains(&(from, reply)));
        }
    }
    fn update_delegate_key(&self, from: u64, account: u64, error: bool) {
        let res = self.send(from, DaoAction::UpdateDelegateKey(account.into()));
        let reply = DaoEvent::DelegateKeyUpdated {
            member: from.into(),
            delegate: account.into(),
        }
        .encode();
        if error {
            assert!(res.main_failed());
        } else {
            assert!(res.contains(&(from, reply)));
        }
    }

    fn add_member(
        &self,
        system: &System,
        proposal_id: u128,
        applicant: u64,
        token_tribute: u128,
        shares_requested: u128,
    ) {
        self.add_to_whitelist(ADMIN, applicant, false);
        self.submit_membership_proposal(
            ADMIN,
            proposal_id,
            applicant,
            token_tribute,
            shares_requested,
            0,
            false,
        );
        self.submit_vote(ADMIN, proposal_id, Vote::Yes, false);
        system.spend_blocks(VOTING_PERIOD_LENGTH as u32 + 1);
        self.process_proposal(proposal_id, true, false);
    }
}

pub trait FToken {
    fn ftoken(system: &System) -> Program;
    fn mint(&self, transaction_id: u64, from: u64, account: u64, amount: u128);
    fn check_balance(&self, account: u64, expected_amount: u128);
    fn approve(&self, transaction_id: u64, from: u64, approved_account: u64, amount: u128);
    fn send_message_and_check_res(&self, from: u64, payload: FTokenAction);
}

impl FToken for Program<'_> {
    fn ftoken(system: &System) -> Program {
        let ftoken = Program::from_file(system, "./target/ft_main.wasm");
        let storage_code_hash: [u8; 32] = system.submit_code("./target/ft_storage.wasm").into();
        let ft_logic_code_hash: [u8; 32] = system.submit_code("./target/ft_logic.wasm").into();

        let res = ftoken.send(
            100,
            InitFToken {
                storage_code_hash: storage_code_hash.into(),
                ft_logic_code_hash: ft_logic_code_hash.into(),
            },
        );
        assert!(!res.main_failed());
        ftoken
    }

    fn mint(&self, transaction_id: u64, from: u64, account: u64, amount: u128) {
        let payload = Action::Mint {
            recipient: account.into(),
            amount,
        }
        .encode();
        self.send_message_and_check_res(
            from,
            FTokenAction::Message {
                transaction_id,
                payload,
            },
        );
    }

    fn approve(&self, transaction_id: u64, from: u64, approved_account: u64, amount: u128) {
        let payload = Action::Approve {
            approved_account: approved_account.into(),
            amount,
        }
        .encode();
        self.send_message_and_check_res(
            from,
            FTokenAction::Message {
                transaction_id,
                payload,
            },
        );
    }

    fn check_balance(&self, account: u64, expected_amount: u128) {
        let res = self.send(100, FTokenAction::GetBalance(account.into()));
        let reply = FTokenEvent::Balance(expected_amount).encode();
        assert!(res.contains(&(100, reply)));
    }

    fn send_message_and_check_res(&self, from: u64, payload: FTokenAction) {
        let res = self.send(from, payload);
        assert!(res.contains(&(from, FTokenEvent::Ok.encode())));
    }
}
