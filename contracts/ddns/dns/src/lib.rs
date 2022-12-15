#![no_std]

use dns_io::*;
use gstd::{async_main, msg, prelude::*, util, ActorId, Vec};

static mut RECORDS: Vec<DnsRecord> = Vec::new();

pub trait Dns {
    fn get_by_id(&self, id: ActorId) -> Option<DnsRecord>;

    fn get_by_name(&self, name: String) -> Vec<DnsRecord>;

    fn get_by_description(&self, description: String) -> Vec<DnsRecord>;

    fn get_by_creator(&self, creator: ActorId) -> Vec<DnsRecord>;

    fn get_by_pattern(&self, pattern: String) -> Vec<DnsRecord>;
}

async unsafe fn add_record(id: ActorId) -> Option<DnsRecord> {
    if RECORDS.iter().find(|&r| r.id == id).is_some() {
        panic!("Program already registered");
    }

    let reply: GetDnsMeta = msg::send_bytes_for_reply_as(id, Vec::from([0]), 0)
        .expect("Error in async")
        .await
        .expect("Unable to get reply");

    match reply {
        GetDnsMeta::DnsMeta(meta) => {
            if let Some(dns_meta) = meta {
                if RECORDS
                    .iter()
                    .find(|&r| r.meta.name == dns_meta.name)
                    .is_some()
                {
                    panic!("Domain {} already registered", dns_meta.name);
                }

                let record = DnsRecord {
                    id,
                    meta: dns_meta,
                    created_by: msg::source(),
                };

                RECORDS.push(record.clone());
                Some(record)
            } else {
                None
            }
        }
    }
}

async unsafe fn update_record(id: ActorId) -> Option<DnsRecord> {
    if let Some(record) = RECORDS.iter_mut().find(|r| r.id == id) {
        let reply: GetDnsMeta = msg::send_bytes_for_reply_as(id, Vec::from([0]), 0)
            .expect("Error in async")
            .await
            .expect("Unable to get reply");

        let result = match reply {
            GetDnsMeta::DnsMeta(meta) => {
                if let Some(dns_meta) = meta {
                    record.meta = dns_meta;

                    Some(record.clone())
                } else {
                    None
                }
            }
        };
        result
    } else {
        None
    }
}

unsafe fn remove_record(id: ActorId) -> Option<DnsRecord> {
    if let Some((index, record)) = RECORDS.iter().enumerate().find(|(_, r)| r.id == id) {
        if record.created_by == msg::source() {
            Some(RECORDS.swap_remove(index))
        } else {
            None
        }
    } else {
        None
    }
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

#[async_main]
async unsafe fn main() {
    let action: DnsAction = msg::load().expect("Unable to decode message");

    unsafe {
        let result: DnsReply = match action {
            DnsAction::Register(id) => DnsReply::Record(add_record(id).await),
            DnsAction::Remove(id) => DnsReply::Record(remove_record(id)),
            DnsAction::Update(id) => DnsReply::Record(update_record(id).await),
            DnsAction::GetById(id) => DnsReply::Record(RECORDS.get_by_id(id)),
            DnsAction::GetByName(name) => DnsReply::Records(RECORDS.get_by_name(name)),
            DnsAction::GetByDescription(description) => {
                DnsReply::Records(RECORDS.get_by_description(description))
            }
        };
        msg::reply_with_gas(result, 0, 0).expect("Error in sending a reply");
    }
}

#[no_mangle]
pub unsafe extern "C" fn meta_state() -> *mut [i32; 2] {
    let action: QueryAction = msg::load().expect("Unable to decode message");

    let result: QueryResult = match action {
        QueryAction::GetAll => QueryResult::Records(RECORDS.clone()),
        QueryAction::GetById(id) => QueryResult::Record(RECORDS.get_by_id(id)),
        QueryAction::GetByName(name) => QueryResult::Records(RECORDS.get_by_name(name)),
        QueryAction::GetByCreator(actor) => QueryResult::Records(RECORDS.get_by_creator(actor)),
        QueryAction::GetByDescription(description) => {
            QueryResult::Records(RECORDS.get_by_description(description))
        }

        QueryAction::GetByPattern(pattern) => QueryResult::Records(RECORDS.get_by_pattern(pattern)),
    };

    util::to_leak_ptr(result.encode())
}

gstd::metadata! {
    title: "DNS contract",
    handle:
        input: DnsAction,
        output: DnsReply,
    state:
        input: QueryAction,
        output: QueryResult,
}
