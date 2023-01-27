#![no_std]

use core::cmp::min;
use gstd::{errors::Result, exec, msg, prelude::*, ActorId, MessageId};
use hashbrown::{HashMap, HashSet};
use multisig_wallet_io::*;
use primitive_types::U256;

const MAX_OWNERS_COUNT: u32 = 50;
const ZERO_ID: ActorId = ActorId::new([0u8; 32]);

#[derive(Default)]
pub struct MultisigWallet {
    pub transactions: HashMap<TransactionId, Transaction>,
    pub confirmations: HashMap<TransactionId, HashSet<ActorId>>,
    pub owners: HashSet<ActorId>,
    pub required: u32,
    pub transaction_count: U256,
}

static mut WALLET: Option<MultisigWallet> = None;

fn validate_requirement(owners_count: usize, required: u32) {
    if (owners_count as u32) > MAX_OWNERS_COUNT {
        panic!("Too much owners");
    }

    if (owners_count as u32) < required {
        panic!("Required count more than owners count");
    }

    if required < 1 {
        panic!("Required quantity must be greater than zero");
    }
}

fn validate_not_null_address(actor_id: &ActorId) {
    if *actor_id == ZERO_ID {
        panic!("actor_id can not be zero");
    }
}

impl MultisigWallet {
    fn validate_only_wallet(&self) {
        if msg::source() != exec::program_id() {
            panic!("Only wallet can call it")
        }
    }

    fn validate_owner_doesnt_exist(&self, owner: &ActorId) {
        if self.has_owner(owner) {
            panic!("Owner already exists")
        }
    }

    fn validate_owner_exists(&self, owner: &ActorId) {
        if !self.has_owner(owner) {
            panic!("Owner doesn't exists")
        }
    }

    fn validate_transaction_exists(&self, transaction_id: &TransactionId) {
        if !self.transactions.contains_key(transaction_id) {
            panic!("Transaction with this ID doesn't exists")
        }
    }

    fn validate_confirmed(&self, transaction_id: &TransactionId, owner: &ActorId) {
        if !self
            .confirmations
            .get(transaction_id)
            .map(|confirmations| confirmations.contains(owner))
            .unwrap_or(false)
        {
            panic!("There is no confirmation of this owner")
        }
    }

    fn validate_not_confirmed(&self, transaction_id: &TransactionId, owner: &ActorId) {
        if self
            .confirmations
            .get(transaction_id)
            .map(|confirmations| confirmations.contains(owner))
            .unwrap_or(false)
        {
            panic!("There is confirmation of this owner")
        }
    }

    fn validate_not_executed(&self, transaction_id: &TransactionId) {
        if matches!(self.transactions.get(transaction_id), Some(t) if t.executed) {
            panic!("Transaction has been already executed")
        }
    }

    fn has_owner(&self, owner: &ActorId) -> bool {
        self.owners.contains(owner)
    }

    /// Allows to add a new owner. Transaction has to be sent by wallet.
    /// `owner` - Address of new owner.
    fn add_owner(&mut self, owner: &ActorId) {
        self.validate_only_wallet();
        self.validate_owner_doesnt_exist(owner);
        validate_requirement(self.owners.len() + 1, self.required);

        self.owners.insert(*owner);

        msg::reply(MWEvent::OwnerAddition { owner: *owner }, 0).unwrap();
    }

    /// Allows to remove an owner. Transaction has to be sent by wallet.
    /// `owner` Address of owner.
    fn remove_owner(&mut self, owner: &ActorId) {
        self.validate_only_wallet();
        self.validate_owner_exists(owner);
        let next_owners_count = self.owners.len() - 1;
        validate_requirement(
            next_owners_count,
            min(next_owners_count as u32, self.required),
        );

        self.owners.remove(owner);

        if (next_owners_count as u32) < self.required {
            self.change_requirement(next_owners_count as _);
        }

        msg::reply(MWEvent::OwnerRemoval { owner: *owner }, 0).unwrap();
    }

    /// Allows to replace an owner with a new owner. Transaction has to be sent by wallet.
    /// `owner` Address of owner to be replaced.
    /// `newOwner` Address of new owner.
    fn replace_owner(&mut self, old_owner: &ActorId, new_owner: &ActorId) {
        self.validate_only_wallet();
        self.validate_owner_exists(old_owner);
        self.validate_owner_doesnt_exist(new_owner);

        self.owners.insert(*new_owner);
        self.owners.remove(old_owner);

        msg::reply(
            MWEvent::OwnerReplace {
                old_owner: *old_owner,
                new_owner: *new_owner,
            },
            0,
        )
        .unwrap();
    }

    /// Allows to change the number of required confirmations. Transaction has to be sent by wallet.
    /// `required` Number of required confirmations.
    fn change_requirement(&mut self, required: u32) {
        self.validate_only_wallet();
        validate_requirement(self.owners.len(), required);

        self.required = required;

        msg::reply(MWEvent::RequirementChange(required.into()), 0).unwrap();
    }

    ///  Allows an owner to submit and confirm a transaction.
    ///  `destination` Transaction target address.
    ///  `value` Transaction value.
    ///  `data` Transaction data payload.
    ///  `description` Transaction description.
    ///  Returns transaction ID.
    fn submit_transaction(
        &mut self,
        destination: &ActorId,
        data: Vec<u8>,
        value: u128,
        description: Option<String>,
    ) {
        let transaction_id = self.add_transaction(destination, data, value, description);
        self.confirm_transaction(&transaction_id);

        msg::reply(MWEvent::Submission { transaction_id }, 0).unwrap();
    }

    /// Allows an owner to confirm a transaction.
    /// `transaction_id` Transaction ID.
    fn confirm_transaction(&mut self, transaction_id: &TransactionId) {
        self.validate_owner_exists(&msg::source());
        self.validate_transaction_exists(transaction_id);
        self.validate_not_confirmed(transaction_id, &msg::source());

        self.confirmations
            .entry(*transaction_id)
            .or_default()
            .insert(msg::source());

        self.execute_transaction(transaction_id, None::<fn()>);
    }

    fn external_confirm_transaction(&mut self, transaction_id: &TransactionId) {
        self.confirm_transaction(transaction_id);

        msg::reply(
            MWEvent::Confirmation {
                sender: msg::source(),
                transaction_id: *transaction_id,
            },
            0,
        )
        .unwrap();
    }

    /// Allows an owner to revoke a confirmation for a transaction.
    /// `transaction_id` Transaction ID.
    fn revoke_confirmation(&mut self, transaction_id: &TransactionId) {
        self.validate_owner_exists(&msg::source());
        self.validate_confirmed(transaction_id, &msg::source());
        self.validate_not_executed(transaction_id);

        self.confirmations
            .entry(*transaction_id)
            .or_default()
            .remove(&msg::source());

        msg::reply(
            MWEvent::Revocation {
                sender: msg::source(),
                transaction_id: *transaction_id,
            },
            0,
        )
        .unwrap();
    }

    /// Allows anyone to execute a confirmed transaction.
    /// `transaction_id` Transaction ID.
    fn execute_transaction<F>(&mut self, transaction_id: &TransactionId, completion: Option<F>)
    where
        F: Fn(),
    {
        let sender = msg::source();
        self.validate_owner_exists(&sender);
        self.validate_confirmed(transaction_id, &sender);
        self.validate_not_executed(transaction_id);

        if if let Some(confirmations) = self.confirmations.get(transaction_id) {
            (confirmations.intersection(&self.owners).count() as u32) < self.required
        } else {
            true
        } {
            return;
        }

        let txn = self.transactions.get_mut(transaction_id).unwrap();

        if exec::value_available() < txn.value {
            panic!("Insufficient amount of money in your wallet")
        }

        msg::send_bytes(txn.destination, txn.payload.clone(), txn.value)
            .expect("Sending message failed");

        txn.executed = true;

        if let Some(completion) = completion {
            completion();
        }
    }

    fn external_execute_transaction(&mut self, transaction_id: &TransactionId) {
        let completion = || {
            let payload = MWEvent::Execution {
                transaction_id: *transaction_id,
            };

            msg::reply(payload, 0).unwrap();
        };

        self.execute_transaction(transaction_id, Some(completion));
    }

    /*
     * Internal functions
     */

    /// Adds a new transaction to the transaction mapping, if transaction does not exist yet.
    /// `destination` Transaction target address.
    /// `value` Transaction value.
    /// `data` Transaction data payload.
    /// `description` Transaction description.
    /// Returns transaction ID.
    fn add_transaction(
        &mut self,
        destination: &ActorId,
        data: Vec<u8>,
        value: u128,
        description: Option<String>,
    ) -> TransactionId {
        validate_not_null_address(destination);
        let transaction_id = self.transaction_count;
        let transaction = Transaction {
            destination: *destination,
            payload: data,
            value,
            description,
            executed: false,
        };

        self.transactions.insert(transaction_id, transaction);
        self.transaction_count += 1.into();

        transaction_id
    }
}

#[no_mangle]
extern "C" fn init() {
    let config: MWInitConfig = msg::load().expect("Unable to decode MWInitConfig");

    let owners_count = config.owners.len();

    validate_requirement(owners_count, config.required);

    let mut wallet = MultisigWallet::default();

    for owner in &config.owners {
        if wallet.owners.contains(owner) {
            panic!("The same owner contained twice")
        } else {
            wallet.owners.insert(*owner);
        }
    }

    wallet.required = config.required;

    unsafe { WALLET = Some(wallet) };
}

#[gstd::async_main]
async unsafe fn main() {
    let action: MWAction = msg::load().expect("Could not load MWAction");

    let wallet: &mut MultisigWallet = unsafe { WALLET.get_or_insert(MultisigWallet::default()) };
    match action {
        MWAction::AddOwner(owner) => wallet.add_owner(&owner),
        MWAction::RemoveOwner(owner) => wallet.remove_owner(&owner),
        MWAction::ReplaceOwner {
            old_owner,
            new_owner,
        } => wallet.replace_owner(&old_owner, &new_owner),
        MWAction::ChangeRequiredConfirmationsCount(count) => wallet.change_requirement(count),
        MWAction::SubmitTransaction {
            destination,
            data,
            value,
            description,
        } => {
            wallet.submit_transaction(&destination, data, value, description);
        }
        MWAction::ConfirmTransaction(transaction_id) => {
            wallet.external_confirm_transaction(&transaction_id)
        }
        MWAction::RevokeConfirmation(transaction_id) => wallet.revoke_confirmation(&transaction_id),
        MWAction::ExecuteTransaction(transaction_id) => {
            wallet.external_execute_transaction(&transaction_id)
        }
    }
}

#[no_mangle]
extern "C" fn state() {
    let MultisigWallet {
        transactions,
        confirmations,
        owners,
        required,
        transaction_count,
    } = unsafe { WALLET.get_or_insert(Default::default()) };

    let state = State {
        transactions: transactions.iter().map(|(k, v)| (*k, v.clone())).collect(),
        confirmations: confirmations
            .iter()
            .map(|(k, v)| (*k, v.iter().copied().collect()))
            .collect(),
        owners: owners.iter().copied().collect(),
        required: *required,
        transaction_count: *transaction_count,
    };

    reply(state).expect(
        "Failed to encode or reply with `<ContractMetadata as Metadata>::State` from `state()`",
    );
}

#[no_mangle]
extern "C" fn metahash() {
    let metahash: [u8; 32] = include!("../.metahash");

    reply(metahash).expect("Failed to encode or reply with `[u8; 32]` from `metahash()`");
}

fn reply(payload: impl Encode) -> Result<MessageId> {
    msg::reply(payload, 0)
}
