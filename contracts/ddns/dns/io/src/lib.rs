#![no_std]

use codec::{Decode, Encode};
use gstd::{prelude::*, ActorId, Clone, Vec};
use scale_info::TypeInfo;

#[derive(Encode, Decode, TypeInfo, Clone)]
pub struct DnsRecord {
    pub id: ActorId,
    pub meta: DnsMeta,
    pub created_by: ActorId,
}

#[derive(Encode, Decode, TypeInfo, Clone)]
pub struct DnsMeta {
    pub name: String,
    pub link: String,
    pub description: String,
}

#[derive(Decode, Clone)]
pub enum GetDnsMeta {
    DnsMeta(Option<DnsMeta>),
}

#[derive(Encode, Decode, TypeInfo)]
pub enum DnsAction {
    Register(ActorId),
    Remove(ActorId),
    Update(ActorId),
    GetById(ActorId),
    GetByName(String),
    GetByDescription(String),
}

#[derive(Encode, Decode, TypeInfo)]
pub enum DnsReply {
    Record(Option<DnsRecord>),
    Records(Vec<DnsRecord>),
}

#[derive(Encode, Decode, TypeInfo)]
pub enum QueryAction {
    GetAll,
    GetById(ActorId),
    GetByName(String),
    GetByCreator(ActorId),
    GetByDescription(String),
    GetByPattern(String),
}

#[derive(Encode, Decode, TypeInfo)]
pub enum QueryResult {
    Record(Option<DnsRecord>),
    Records(Vec<DnsRecord>),
}
