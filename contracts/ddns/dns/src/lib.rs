#![no_std]

use dns_io::*;
use gstd::{async_main, msg, prelude::*, ActorId, Vec};

static mut RECORDS: Vec<DnsRecord> = Vec::new();

async unsafe fn add_record(id: ActorId) -> Option<DnsRecord> {
    if RECORDS.iter().any(|r| r.id == id) {
        panic!("Program already registered");
    }

    let reply: GetDnsMeta = msg::send_bytes_for_reply_as(id, Vec::from([0]), 0)
        .expect("Error in async")
        .await
        .expect("Unable to get reply");

    match reply {
        GetDnsMeta::DnsMeta(meta) => {
            if let Some(dns_meta) = meta {
                if RECORDS.iter().any(|r| r.meta.name == dns_meta.name) {
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

        match reply {
            GetDnsMeta::DnsMeta(meta) => {
                if let Some(dns_meta) = meta {
                    record.meta = dns_meta;

                    Some(record.clone())
                } else {
                    None
                }
            }
        }
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
unsafe extern "C" fn state() {
    msg::reply(RECORDS.clone(), 0).expect("failed to reply");
}
