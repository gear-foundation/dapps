#![no_std]
#![allow(static_mut_refs)]
use sails_rs::{
    collections::{HashMap, HashSet},
    gstd::{exec, msg},
    prelude::*,
};

struct PtsService(());

#[derive(Debug, Clone)]
struct Storage {
    balances: HashMap<ActorId, (u128, u64)>,
    admins: HashSet<ActorId>,
    accrual: u128,
    time_ms_between_balance_receipt: u64,
}

static mut STORAGE: Option<Storage> = None;

#[event]
#[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum Event {
    NewAdminAdded(ActorId),
    AdminDeleted(ActorId),
    AccrualChanged(u128),
    TimeBetweenBalanceReceiptChanged(u64),
    AccrualReceived {
        id: ActorId,
        accrual: u128,
    },
    SubtractionIsDone {
        id: ActorId,
        amount: u128,
    },
    AdditionIsDone {
        id: ActorId,
        amount: u128,
    },
    Transfered {
        from: ActorId,
        to: ActorId,
        amount: u128,
    },
    BatchTransfered {
        from: ActorId,
        to_ids: Vec<ActorId>,
        amounts: Vec<u128>,
    },
}

impl PtsService {
    pub fn new() -> Self {
        Self(())
    }
    pub fn init(accrual: u128, time_ms_between_balance_receipt: u64) -> Self {
        unsafe {
            STORAGE = Some(Storage {
                admins: HashSet::from([msg::source()]),
                balances: HashMap::new(),
                accrual,
                time_ms_between_balance_receipt,
            });
        }
        Self(())
    }
    fn get_mut(&mut self) -> &'static mut Storage {
        unsafe { STORAGE.as_mut().expect("Storage is not initialized") }
    }
    fn get(&self) -> &'static Storage {
        unsafe { STORAGE.as_ref().expect("Storage is not initialized") }
    }
}

#[sails_rs::service(events = Event)]
impl PtsService {
    #[export]
    pub fn get_accural(&mut self) {
        let storage = self.get_mut();
        let msg_src = msg::source();
        let (balance, last_time) = storage.balances.entry(msg_src).or_insert((0, 0));
        let current_time = exec::block_timestamp();
        if current_time - *last_time < storage.time_ms_between_balance_receipt {
            panic!("Time has not yet expired");
        }
        *balance += storage.accrual;
        *last_time = current_time;
        self.emit_event(Event::AccrualReceived {
            id: msg_src,
            accrual: storage.accrual,
        })
        .expect("Notification Error");
    }

    #[export]
    pub fn transfer(&mut self, from: ActorId, to: ActorId, amount: u128) {
        let storage = self.get_mut();
        let msg_src = msg::source();
        if !storage.admins.contains(&msg_src) && from != msg_src {
            panic!("Access denied");
        }
        if from == to {
            panic!("Cannot transfer to self");
        }
        let (from_balance, _last_time) = storage
            .balances
            .get_mut(&from)
            .expect("Actor id must be exist");

        *from_balance = from_balance.checked_sub(amount).expect("Low balance");

        let (to_balance, _last_time) = storage.balances.entry(to).or_insert((0, 0));
        *to_balance = to_balance.checked_add(amount).unwrap_or(u128::MAX);

        self.emit_event(Event::Transfered { from, to, amount })
            .expect("Notification Error");
    }

    #[export]
    pub fn batch_transfer(&mut self, from: ActorId, to_ids: Vec<ActorId>, amounts: Vec<u128>) {
        let storage = self.get_mut();
        let msg_src = msg::source();
        if !storage.admins.contains(&msg_src) && from != msg_src {
            panic!("Access denied");
        }

        let (from_balance, _last_time) = storage
            .balances
            .get_mut(&from)
            .expect("Actor id must be exist");

        let total_amount = amounts.iter().sum();
        *from_balance = from_balance.checked_sub(total_amount).expect("Low balance");

        for (id, amount) in to_ids.clone().into_iter().zip(amounts.clone()) {
            let (to_balance, _last_time) = storage.balances.entry(id).or_insert((0, 0));
            *to_balance = to_balance.checked_add(amount).unwrap_or(u128::MAX);
        }

        self.emit_event(Event::BatchTransfered {
            from,
            to_ids,
            amounts,
        })
        .expect("Notification Error");
    }

    #[export]
    pub fn add_admin(&mut self, new_admin: ActorId) {
        let storage = self.get_mut();
        if !storage.admins.contains(&msg::source()) {
            panic!("Access denied");
        }
        storage.admins.insert(new_admin);
        self.emit_event(Event::NewAdminAdded(new_admin))
            .expect("Notification Error");
    }

    #[export]
    pub fn delete_admin(&mut self, admin: ActorId) {
        let storage = self.get_mut();
        if !storage.admins.contains(&msg::source()) {
            panic!("Access denied");
        }
        storage.admins.remove(&admin);
        self.emit_event(Event::AdminDeleted(admin))
            .expect("Notification Error");
    }

    #[export]
    pub fn change_accrual(&mut self, new_accrual: u128) {
        let storage = self.get_mut();
        if !storage.admins.contains(&msg::source()) {
            panic!("Access denied");
        }
        storage.accrual = new_accrual;
        self.emit_event(Event::AccrualChanged(new_accrual))
            .expect("Notification Error");
    }

    #[export]
    pub fn change_time_between_balance_receipt(&mut self, new_time_between_balance_receipt: u64) {
        let storage = self.get_mut();
        if !storage.admins.contains(&msg::source()) {
            panic!("Access denied");
        }
        storage.time_ms_between_balance_receipt = new_time_between_balance_receipt;
        self.emit_event(Event::TimeBetweenBalanceReceiptChanged(
            new_time_between_balance_receipt,
        ))
        .expect("Notification Error");
    }

    #[export]
    pub fn admins(&self) -> Vec<ActorId> {
        self.get().admins.clone().into_iter().collect()
    }
    #[export]
    pub fn accrual(&self) -> u128 {
        self.get().accrual
    }
    #[export]
    pub fn time_ms_between_balance_receipt(&self) -> u64 {
        self.get().time_ms_between_balance_receipt
    }

    #[export]
    pub fn get_balance(&self, id: ActorId) -> u128 {
        let (balance, _) = self
            .get()
            .balances
            .get(&id)
            .expect("Actor id must be exist");
        *balance
    }
    #[export]
    pub fn get_remaining_time_ms(&self, id: ActorId) -> Option<u64> {
        let storage = self.get();
        let (_, last_time) = storage.balances.get(&id).expect("Actor id must be exist");
        storage
            .time_ms_between_balance_receipt
            .checked_sub(exec::block_timestamp() - last_time)
    }
}

pub struct PtsProgram(());

#[sails_rs::program]
impl PtsProgram {
    // Program's constructor
    pub fn new(accrual: u128, time_ms_between_balance_receipt: u64) -> Self {
        PtsService::init(accrual, time_ms_between_balance_receipt);
        Self(())
    }

    // Exposed service
    pub fn pts(&self) -> PtsService {
        PtsService::new()
    }
}
