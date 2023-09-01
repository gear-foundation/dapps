#![no_std]

use gmeta::{In, InOut, Metadata, Out};
use gstd::{prelude::*, ActorId, Decode, Encode, TypeInfo};

pub struct DaoLightMetadata;

impl Metadata for DaoLightMetadata {
    type Init = In<InitDao>;
    type Handle = InOut<DaoAction, DaoEvent>;
    type Others = ();
    type Reply = ();
    type Signal = ();
    type State = Out<DaoState>;
}

#[derive(Debug, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct DaoState {
    pub approved_token_program_id: ActorId,
    pub period_duration: u64,
    pub voting_period_length: u64,
    pub grace_period_length: u64,
    pub total_shares: u128,
    pub members: Vec<(ActorId, Member)>,
    pub proposal_id: u128,
    pub locked_funds: u128,
    pub proposals: Vec<(u128, Proposal)>,
}

impl DaoState {
    pub fn is_member(&self, account: &ActorId) -> bool {
        self.members
            .iter()
            .any(|(id, member)| id == account && member.shares != 0)
    }
}

#[derive(Debug, Default, Clone, Decode, Encode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct Proposal {
    pub proposer: ActorId,
    pub applicant: ActorId,
    pub yes_votes: u128,
    pub no_votes: u128,
    pub quorum: u128,
    pub amount: u128,
    pub processed: bool,
    pub did_pass: bool,
    pub details: String,
    pub starting_period: u64,
    pub ended_at: u64,
    pub votes_by_member: Vec<(ActorId, Vote)>,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct Member {
    pub shares: u128,
    pub highest_index_yes_vote: Option<u128>,
}

#[derive(Debug, Decode, Encode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum Role {
    Admin,
    Member,
    None,
}

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
