use gstd::{prelude::*, ActorId, String};

#[derive(Debug, Decode, Encode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum DaoAction {
    Deposit {
        amount: u128,
    },
    SubmitFundingProposal {
        applicant: ActorId,
        amount: u128,
        quorum: u128,
        details: String,
    },
    ProcessProposal {
        proposal_id: u128,
    },
    SubmitVote {
        proposal_id: u128,
        vote: Vote,
    },
    RageQuit {
        amount: u128,
    },
}

#[derive(Debug, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum DaoEvent {
    Deposit {
        member: ActorId,
        share: u128,
    },
    SubmitFundingProposal {
        proposer: ActorId,
        applicant: ActorId,
        proposal_id: u128,
        amount: u128,
    },
    SubmitVote {
        account: ActorId,
        proposal_id: u128,
        vote: Vote,
    },
    ProcessProposal {
        applicant: ActorId,
        proposal_id: u128,
        did_pass: bool,
    },
    RageQuit {
        member: ActorId,
        amount: u128,
    },
}

#[derive(Debug, Decode, Encode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct InitDao {
    pub approved_token_program_id: ActorId,
    pub voting_period_length: u64,
    pub period_duration: u64,
    pub grace_period_length: u64,
}

#[derive(Debug, Encode, Decode, Clone, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum Vote {
    Yes,
    No,
}
