use gstd::{prelude::*, ActorId};

use crate::{Member, Proposal};

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
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum StateReply {
    UserStatus(Role),
    AllProposals(BTreeMap<u128, Proposal>),
    IsMember(bool),
    ProposalId(u128),
    ProposalInfo(Proposal),
    MemberInfo(Member),
    MemberPower(u128),
}
