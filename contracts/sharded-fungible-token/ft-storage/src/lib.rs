#![no_std]
use ft_storage_io::*;
use gstd::{msg, prelude::*, ActorId};
use hashbrown::HashMap;
use primitive_types::H256;

#[derive(Default)]
struct FTStorage {
    ft_logic_id: ActorId,
    transaction_status: HashMap<H256, bool>,
    balances: HashMap<ActorId, u128>,
    approvals: HashMap<ActorId, HashMap<ActorId, u128>>,
    permits: HashMap<ActorId, u128>,
}

static mut FT_STORAGE: Option<FTStorage> = None;

impl FTStorage {
    fn get_permit_id(&self, account: &ActorId) {
        let permit_id = self.permits.get(account).unwrap_or(&0);
        msg::reply(FTStorageEvent::PermitId(*permit_id), 0).expect("");
    }

    fn check_and_increment_permit_id(
        &mut self,
        transaction_hash: H256,
        account: &ActorId,
        signed_permit_id: &u128,
    ) {
        self.assert_ft_contract();

        // check transaction status
        if let Some(status) = self.transaction_status.get(&transaction_hash) {
            match status {
                true => reply_ok(),
                false => reply_err(),
            };
            return;
        }

        if self.permits.get(account).unwrap_or(&0) != signed_permit_id {
            reply_err();
            return;
        }

        self.permits
            .entry(*account)
            .and_modify(|id| *id += 1)
            .or_insert(1);
        reply_ok();
    }

    fn get_balance(&self, account: &ActorId) {
        let balance = self.balances.get(account).unwrap_or(&0);
        msg::reply(FTStorageEvent::Balance(*balance), 0).expect("");
    }

    fn decrease(&mut self, msg_source: &ActorId, sender: &ActorId, amount: u128) -> bool {
        if let Some(balance) = self.balances.get_mut(sender) {
            if *balance >= amount {
                if msg_source == sender {
                    *balance -= amount;
                    return true;
                } else if let Some(allowed_amount) = self
                    .approvals
                    .get_mut(sender)
                    .and_then(|m| m.get_mut(msg_source))
                {
                    if *allowed_amount >= amount {
                        *balance -= amount;
                        *allowed_amount -= amount;
                        return true;
                    }
                }
            }
        }
        false
    }
    fn transfer(
        &mut self,
        transaction_hash: H256,
        msg_source: &ActorId,
        sender: &ActorId,
        recipient: &ActorId,
        amount: u128,
    ) {
        self.assert_ft_contract();

        // check transaction status
        if let Some(status) = self.transaction_status.get(&transaction_hash) {
            match status {
                true => reply_ok(),
                false => reply_err(),
            };
            return;
        }

        match self.decrease(msg_source, sender, amount) {
            true => {
                self.balances
                    .entry(*recipient)
                    .and_modify(|balance| *balance = (*balance).saturating_add(amount))
                    .or_insert(amount);

                self.transaction_status.insert(transaction_hash, true);
                reply_ok();
            }
            false => {
                self.transaction_status.insert(transaction_hash, false);
                reply_err();
            }
        }
    }

    fn increase_balance(&mut self, transaction_hash: H256, account: &ActorId, amount: u128) {
        self.assert_ft_contract();

        // check transaction status
        if let Some(status) = self.transaction_status.get(&transaction_hash) {
            match status {
                true => reply_ok(),
                false => reply_err(),
            };
            return;
        }

        // increase balance
        self.balances
            .entry(*account)
            .and_modify(|balance| *balance = (*balance).saturating_add(amount))
            .or_insert(amount);

        self.transaction_status.insert(transaction_hash, true);
        reply_ok();
    }

    fn decrease_balance(
        &mut self,
        transaction_hash: H256,
        msg_source: &ActorId,
        account: &ActorId,
        amount: u128,
    ) {
        self.assert_ft_contract();
        // check transaction status
        if let Some(status) = self.transaction_status.get(&transaction_hash) {
            match status {
                true => reply_ok(),
                false => reply_err(),
            };
            return;
        }

        // decrease balance
        match self.decrease(msg_source, account, amount) {
            true => {
                self.transaction_status.insert(transaction_hash, true);
                reply_ok();
            }
            false => {
                self.transaction_status.insert(transaction_hash, false);
                reply_err();
            }
        }
    }

    fn approve(
        &mut self,
        transaction_hash: H256,
        msg_source: &ActorId,
        account: &ActorId,
        amount: u128,
    ) {
        self.assert_ft_contract();

        // check transaction status
        if let Some(status) = self.transaction_status.get(&transaction_hash) {
            match status {
                true => reply_ok(),
                false => reply_err(),
            };
            return;
        }

        self.approvals
            .entry(*msg_source)
            .and_modify(|accounts| {
                accounts
                    .entry(*account)
                    .and_modify(|allowed_amount| {
                        *allowed_amount = (*allowed_amount).saturating_add(amount)
                    })
                    .or_insert_with(|| amount);
            })
            .or_insert_with(|| [(*account, amount)].into());

        reply_ok();
    }

    fn assert_ft_contract(&self) {
        assert!(
            msg::source() == self.ft_logic_id,
            "Only fungible logic token contract is allowed to call that action"
        );
    }
}

#[no_mangle]
unsafe extern "C" fn handle() {
    let action: FTStorageAction = msg::load().expect("Error in loading `StorageAction`");
    let storage: &mut FTStorage = FT_STORAGE.get_or_insert(Default::default());
    match action {
        FTStorageAction::GetBalance(account) => storage.get_balance(&account),
        FTStorageAction::GetPermitId(account) => storage.get_permit_id(&account),
        FTStorageAction::IncrementPermitId {
            transaction_hash,
            account,
            expected_permit_id,
        } => storage.check_and_increment_permit_id(transaction_hash, &account, &expected_permit_id),
        FTStorageAction::IncreaseBalance {
            transaction_hash,
            account,
            amount,
        } => storage.increase_balance(transaction_hash, &account, amount),
        FTStorageAction::DecreaseBalance {
            transaction_hash,
            msg_source,
            account,
            amount,
        } => storage.decrease_balance(transaction_hash, &msg_source, &account, amount),
        FTStorageAction::Approve {
            transaction_hash,
            msg_source,
            account,
            amount,
        } => storage.approve(transaction_hash, &msg_source, &account, amount),
        FTStorageAction::Transfer {
            transaction_hash,
            msg_source,
            sender,
            recipient,
            amount,
        } => storage.transfer(transaction_hash, &msg_source, &sender, &recipient, amount),
    }
}

#[no_mangle]
unsafe extern "C" fn init() {
    let storage = FTStorage {
        ft_logic_id: msg::source(),
        ..Default::default()
    };
    FT_STORAGE = Some(storage);
}

fn reply_ok() {
    msg::reply(FTStorageEvent::Ok, 0).expect("error in sending a reply `FTStorageEvent::Ok");
}

fn reply_err() {
    msg::reply(FTStorageEvent::Err, 0).expect("error in sending a reply `FTStorageEvent::Err");
}

#[no_mangle]
extern "C" fn state() {
    let storage = unsafe { FT_STORAGE.as_ref().expect("Storage is not initialized") };
    let storage_state = FTStorageState {
        ft_logic_id: storage.ft_logic_id,
        transaction_status: storage
            .transaction_status
            .iter()
            .map(|(key, value)| (*key, *value))
            .collect(),
        balances: storage
            .balances
            .iter()
            .map(|(key, value)| (*key, *value))
            .collect(),
        approvals: storage
            .approvals
            .iter()
            .map(|(key, value)| {
                (
                    *key,
                    value.iter().map(|(key, value)| (*key, *value)).collect(),
                )
            })
            .collect(),
        permits: storage
            .permits
            .iter()
            .map(|(key, value)| (*key, *value))
            .collect(),
    };
    msg::reply(storage_state, 0).expect("Failed to share state");
}

#[no_mangle]
extern "C" fn metahash() {
    let metahash: [u8; 32] = include!("../.metahash");
    msg::reply(metahash, 0).expect("Failed to share metahash");
}
