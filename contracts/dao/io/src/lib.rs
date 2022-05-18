#![no_std]

use codec::{Decode, Encode};
use gstd::{ActorId, String};
use scale_info::TypeInfo;

#[derive(Debug, Decode, Encode, TypeInfo)]
pub enum DaoAction {
    AddToWhiteList(ActorId),
    SubmitMembershipProposal {
        applicant: ActorId,
        token_tribute: u128,
        shares_requested: u128,
        quorum: u128,
        details: String,
    },
    SubmitFundingProposal {
        applicant: ActorId,
        amount: u128,
        quorum: u128,
        details: String,
    },
    ProcessProposal(u128),
    SubmitVote {
        proposal_id: u128,
        vote: Vote,
    },
    RageQuit(u128),
    Abort(u128),
    CancelProposal(u128),
    UpdateDelegateKey(ActorId),
    SetAdmin(ActorId),
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum DaoEvent {
    MemberAddedToWhitelist(ActorId),
    SubmitMembershipProposal {
        proposer: ActorId,
        applicant: ActorId,
        proposal_id: u128,
        token_tribute: u128,
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
    Abort {
        member: ActorId,
        proposal_id: u128,
        amount: u128,
    },
    Cancel {
        member: ActorId,
        proposal_id: u128,
    },
    AdminUpdated(ActorId),
    DelegateKeyUpdated {
        member: ActorId,
        delegate: ActorId,
    },
}

#[derive(Debug, Decode, Encode, TypeInfo)]
pub struct InitDao {
    pub admin: ActorId,
    pub approved_token_program_id: ActorId,
    pub period_duration: u64,
    pub voting_period_length: u64,
    pub grace_period_length: u64,
    pub dilution_bound: u128,
    pub abort_window: u64,
}

#[derive(Debug, Encode, Decode, Clone, TypeInfo)]
pub enum Vote {
    Yes,
    No,
}
