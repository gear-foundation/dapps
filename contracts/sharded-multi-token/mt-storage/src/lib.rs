#![no_std]

use gstd::{msg, prelude::*, ActorId};
use hashbrown::HashMap;
use mt_storage_io::*;
use primitive_types::H256;

#[derive(Default)]
struct MTStorage {
    mt_logic_id: ActorId,
    transaction_status: HashMap<H256, bool>,
    balances: HashMap<TokenId, HashMap<ActorId, u128>>,
    approvals: HashMap<ActorId, HashMap<ActorId, bool>>,
}

static mut MT_STORAGE: Option<MTStorage> = None;

impl MTStorage {
    fn get_balance(&self, token_id: TokenId, account: &ActorId) -> u128 {
        let token = self
            .balances
            .get(&token_id)
            .expect("Unable to locate token.");

        *token.get(account).unwrap_or(&0)
    }

    fn get_approval(&self, account: &ActorId, approval_target: &ActorId) -> bool {
        if account == approval_target {
            return true;
        }

        let account_approvals = self
            .approvals
            .get(account)
            .expect("Unable to locate account approvals.");

        *account_approvals.get(approval_target).unwrap_or(&false)
    }

    fn assert_mt_contract(&self) {
        assert!(
            msg::source() == self.mt_logic_id,
            "Only multitoken logic contract is allowed to call that action."
        )
    }

    fn clear_transaction(&mut self, transaction_hash: H256) {
        self.transaction_status.remove(&transaction_hash);
    }

    fn transfer(
        &mut self,
        transaction_hash: H256,
        token_id: TokenId,
        msg_source: &ActorId,
        sender: &ActorId,
        recipient: &ActorId,
        amount: u128,
    ) {
        self.assert_mt_contract();

        if let Some(status) = self.transaction_status.get(&transaction_hash) {
            match status {
                true => reply_ok(),
                false => reply_err(),
            };
            return;
        }

        match self.decrease(token_id, msg_source, sender, amount) {
            true => {
                let token_balances = self
                    .balances
                    .get_mut(&token_id)
                    .expect("Unable to locate token.");

                token_balances
                    .entry(*recipient)
                    .and_modify(|balance| {
                        *balance = balance.checked_add(amount).expect("Math overflow.");
                    })
                    .or_insert_with(|| amount);

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
        approve: bool,
    ) {
        self.assert_mt_contract();

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
                    .and_modify(|allowed| *allowed = approve)
                    .or_insert_with(|| approve);
            })
            .or_insert_with(|| [(*account, approve)].into());

        reply_ok();
    }

    fn decrease(
        &mut self,
        token_id: TokenId,
        msg_source: &ActorId,
        sender: &ActorId,
        amount: u128,
    ) -> bool {
        // Save flag before mutable borrowing
        let approved = self.get_approval(sender, msg_source);

        if let Some(token) = self.balances.get_mut(&token_id) {
            if let Some(balance) = token.get_mut(sender) {
                if *balance >= amount && (msg_source == sender || approved) {
                    *balance = balance.checked_sub(amount).expect("Math overflow.");
                    return true;
                }
            }
        }

        false
    }

    fn increase_balance(
        &mut self,
        transaction_hash: H256,
        token_id: TokenId,
        account: &ActorId,
        amount: u128,
    ) {
        self.assert_mt_contract();

        if let Some(status) = self.transaction_status.get(&transaction_hash) {
            match status {
                true => reply_ok(),
                false => reply_err(),
            };
            return;
        }

        self.balances
            .entry(token_id)
            .and_modify(|token_balances| {
                token_balances
                    .entry(*account)
                    .and_modify(|balance| {
                        *balance = (*balance).checked_add(amount).expect("Math overflow.")
                    })
                    .or_insert(amount);
            })
            .or_insert_with(|| {
                let mut token_balances = HashMap::new();
                token_balances.insert(*account, amount);
                token_balances
            });

        self.transaction_status.insert(transaction_hash, true);
        reply_ok();
    }

    fn decrease_balance(
        &mut self,
        transaction_hash: H256,
        token_id: TokenId,
        msg_source: &ActorId,
        account: &ActorId,
        amount: u128,
    ) {
        self.assert_mt_contract();

        if let Some(status) = self.transaction_status.get(&transaction_hash) {
            match status {
                true => reply_ok(),
                false => reply_err(),
            };
            return;
        }

        match self.decrease(token_id, msg_source, account, amount) {
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
}

#[no_mangle]
extern "C" fn handle() {
    let action: MTStorageAction = msg::load().expect("Unable to load `MTStorageAction`.");
    let storage: &mut MTStorage = unsafe { MT_STORAGE.get_or_insert(Default::default()) };

    match action {
        MTStorageAction::GetBalance { token_id, account } => {
            msg::reply(
                MTStorageEvent::Balance(storage.get_balance(token_id, &account)),
                0,
            )
            .expect("Unable to reply.");
        }
        MTStorageAction::GetApproval {
            account,
            approval_target,
        } => {
            msg::reply(
                MTStorageEvent::Approval(storage.get_approval(&account, &approval_target)),
                0,
            )
            .expect("Unable to reply.");
        }
        MTStorageAction::Transfer {
            transaction_hash,
            token_id,
            msg_source,
            sender,
            recipient,
            amount,
        } => {
            storage.transfer(
                transaction_hash,
                token_id,
                &msg_source,
                &sender,
                &recipient,
                amount,
            );
        }
        MTStorageAction::Approve {
            transaction_hash,
            msg_source,
            account,
            approve,
        } => {
            storage.approve(transaction_hash, &msg_source, &account, approve);
        }
        MTStorageAction::ClearTransaction(transaction_hash) => {
            storage.clear_transaction(transaction_hash);
        }
        MTStorageAction::IncreaseBalance {
            transaction_hash,
            token_id,
            account,
            amount,
        } => {
            storage.increase_balance(transaction_hash, token_id, &account, amount);
        }
        MTStorageAction::DecreaseBalance {
            transaction_hash,
            token_id,
            msg_source,
            account,
            amount,
        } => {
            storage.decrease_balance(transaction_hash, token_id, &msg_source, &account, amount);
        }
    }
}

#[no_mangle]
extern "C" fn init() {
    let storage = MTStorage {
        mt_logic_id: msg::source(),
        ..Default::default()
    };
    unsafe { MT_STORAGE = Some(storage) };
}

#[no_mangle]
extern "C" fn state() {
    let storage = unsafe { MT_STORAGE.as_ref().expect("Storage is not initialized.") };
    let storage_state = MTStorageState {
        mt_logic_id: storage.mt_logic_id,
        transaction_status: storage
            .transaction_status
            .iter()
            .map(|(key, value)| (*key, *value))
            .collect(),
        balances: storage
            .balances
            .iter()
            .map(|(key, value)| (*key, value.iter().map(|(a, b)| (*a, *b)).collect()))
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
    };

    msg::reply(storage_state, 0).expect("Failed to share state.");
}

#[no_mangle]
extern "C" fn metahash() {
    let metahash: [u8; 32] = include!("../.metahash");
    msg::reply(metahash, 0).expect("Failed to share metahash.");
}

fn reply_ok() {
    msg::reply(MTStorageEvent::Ok, 0).expect("error in sending a reply `MTStorageEvent::Ok`.");
}

fn reply_err() {
    msg::reply(MTStorageEvent::Err, 0).expect("error in sending a reply `MTStorageEvent::Err`.");
}
