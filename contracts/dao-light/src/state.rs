use crate::{Member, Proposal};
use codec::{Decode, Encode};
use gstd::{prelude::*, ActorId};
use scale_info::TypeInfo;

#[derive(Debug, Decode, Encode, TypeInfo)]
pub enum Role {
    Admin,
    Member,
    None,
}

#[derive(Debug, Decode, Encode, TypeInfo)]
pub enum State {
    UserStatus(ActorId),
    AllProposals,
    IsMember(ActorId),
    ProposalId,
    ProposalInfo(u128),
    MemberInfo(ActorId),
    MemberPower(ActorId),
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum StateReply {
    UserStatus(Role),
    AllProposals(BTreeMap<u128, Proposal>),
    IsMember(bool),
    ProposalId(u128),
    ProposalInfo(Proposal),
    MemberInfo(Member),
    MemberPower(u128),
}
