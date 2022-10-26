use crate::{Member, Proposal};
use gstd::{prelude::*, ActorId};

#[derive(Debug, Decode, Encode, TypeInfo)]
pub enum State {
    IsMember(ActorId),
    IsInWhitelist(ActorId),
    ProposalId,
    ProposalInfo(u128),
    MemberInfo(ActorId),
}

#[derive(Debug, Encode, TypeInfo)]
pub enum StateReply {
    IsMember(bool),
    IsInWhitelist(bool),
    ProposalId(u128),
    ProposalInfo(Proposal),
    MemberInfo(Member),
}
