#![no_std]

use codec::{Decode, Encode};
use gmeta::{InOut, Metadata, Out};
use gstd::{prelude::*, ActorId, Clone, Vec};
use scale_info::TypeInfo;

pub struct ContractMetadata;

impl Metadata for ContractMetadata {
    type Init = ();
    type Handle = InOut<DnsAction, DnsReply>;
    type Reply = ();
    type Others = ();
    type Signal = ();
    type State = Out<Vec<DnsRecord>>;
}

#[derive(Encode, Decode, TypeInfo, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct DnsRecord {
    pub id: ActorId,
    pub meta: DnsMeta,
    pub created_by: ActorId,
}

#[derive(Encode, Decode, TypeInfo, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct DnsMeta {
    pub name: String,
    pub link: String,
    pub description: String,
}

#[derive(Decode, Clone)]
#[codec(crate = gstd::codec)]
pub enum GetDnsMeta {
    DnsMeta(Option<DnsMeta>),
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum DnsAction {
    Register(ActorId),
    Remove(ActorId),
    Update(ActorId),
    GetById(ActorId),
    GetByName(String),
    GetByDescription(String),
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum DnsReply {
    Record(Option<DnsRecord>),
    Records(Vec<DnsRecord>),
}

pub trait Dns {
    fn get_by_id(&self, id: ActorId) -> Option<DnsRecord>;

    fn get_by_name(&self, name: String) -> Vec<DnsRecord>;

    fn get_by_description(&self, description: String) -> Vec<DnsRecord>;

    fn get_by_creator(&self, creator: ActorId) -> Vec<DnsRecord>;

    fn get_by_pattern(&self, pattern: String) -> Vec<DnsRecord>;
}

impl Dns for Vec<DnsRecord> {
    fn get_by_id(&self, id: ActorId) -> Option<DnsRecord> {
        self.iter().find(|&r| r.id == id).cloned()
    }

    fn get_by_name(&self, name: String) -> Vec<DnsRecord> {
        self.iter()
            .filter(|r| r.meta.name == name)
            .cloned()
            .collect()
    }

    fn get_by_description(&self, description: String) -> Vec<DnsRecord> {
        self.iter()
            .filter(|&r| r.meta.description.as_str().contains(description.as_str()))
            .cloned()
            .collect()
    }

    fn get_by_creator(&self, creator: ActorId) -> Vec<DnsRecord> {
        self.iter()
            .filter(|&r| r.created_by == creator)
            .cloned()
            .collect()
    }

    fn get_by_pattern(&self, pattern: String) -> Vec<DnsRecord> {
        self.iter()
            .filter(|&r| {
                r.meta.name.as_str().contains(pattern.as_str())
                    || r.meta.description.as_str().contains(pattern.as_str())
            })
            .cloned()
            .collect()
    }
}
