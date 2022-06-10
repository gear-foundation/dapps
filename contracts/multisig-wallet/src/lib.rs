#![no_std]
#![feature(const_btree_new)]

extern crate alloc;

use core::cmp::min;
use gstd::{exec, msg, prelude::*, ActorId};
pub use multisig_wallet_io::*;
use primitive_types::U256;
pub mod state;
use state::*;

type TransactionId = U256;

const MAX_OWNERS_COUNT: u64 = 50;
const ZERO_ID: ActorId = ActorId::new([0u8; 32]);

pub struct Transaction {
    destination: ActorId,
    payload: Vec<u8>,
    value: u128,
    description: Option<String>,
    executed: bool,
}

#[derive(Default)]
pub struct MultisigWallet {
    pub transactions: BTreeMap<TransactionId, Transaction>,
    pub confirmations: BTreeMap<TransactionId, BTreeSet<ActorId>>,
    pub owners: BTreeSet<ActorId>,
    pub required: u64,
    pub transaction_count: U256,
}

static mut WALLET: Option<MultisigWallet> = None;

fn validate_requirement(owners_count: usize, required: u64) {
    if (owners_count as u64) > MAX_OWNERS_COUNT {
        panic!("Too much owners");
    }

    if (owners_count as u64) < required {
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
            min(next_owners_count as u64, self.required),
        );

        self.owners.remove(owner);

        if (next_owners_count as u64) < self.required {
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
    fn change_requirement(&mut self, required: u64) {
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

        if !self.is_confirmed(transaction_id) {
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

    /// Returns the confirmation status of a transaction.
    /// `transaction_id` Transaction ID.
    fn is_confirmed(&self, transaction_id: &TransactionId) -> bool {
        let count = self.get_confirmation_count(transaction_id);

        count >= self.required
    }

    /// Returns the description of a transaction.
    /// `transaction_id` Transaction ID.
    fn transaction_description(&self, transaction_id: &TransactionId) -> Option<String> {
        self.transactions
            .get(transaction_id)
            .expect("Transaction with this ID doesn't exists")
            .description
            .clone()
    }

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

    /*
     * State
     */

    /// Returns number of confirmations of a transaction.
    /// `transaction_id` Transaction ID.
    /// Number of confirmations.
    fn get_confirmation_count(&self, transaction_id: &TransactionId) -> u64 {
        self.confirmations
            .get(transaction_id)
            .expect("There is no such transaction or confirmations for it")
            .intersection(&self.owners)
            .count() as _
    }

    /// Returns total number of transactions after filers are applied.
    /// `pending` Include pending transactions.
    /// `executed` Include executed transactions.
    /// Total number of transactions after filters are applied.
    fn get_transaction_count(&self, pending: bool, executed: bool) -> u64 {
        self.transactions
            .values()
            .filter(|transaction| {
                (pending && !transaction.executed) || (executed && transaction.executed)
            })
            .count() as _
    }

    /// Returns list of owners.
    /// List of owner addresses.
    fn get_owners(&self) -> Vec<ActorId> {
        self.owners.iter().copied().collect()
    }

    /// Returns array with owner addresses, which confirmed transaction.
    /// `transaction_id` Transaction ID.
    /// Returns array of owner addresses.
    fn get_confirmations(&self, transaction_id: &TransactionId) -> Vec<ActorId> {
        self.confirmations
            .get(transaction_id)
            .expect("There is no transaction with this ID")
            .iter()
            .copied()
            .collect()
    }

    /// Returns list of transaction IDs in defined range.
    /// `from` Index start position of transaction array.
    /// `to` Index end position of transaction array(not included).
    /// `pending` Include pending transactions.
    /// `executed` Include executed transactions.
    /// `Returns` array of transaction IDs.
    fn get_transaction_ids(
        &self,
        from: u64,
        to: u64,
        pending: bool,
        executed: bool,
    ) -> Vec<TransactionId> {
        self.transactions
            .iter()
            .filter(|(_, txn)| (pending && !txn.executed) || (executed && txn.executed))
            .map(|(id, _)| *id)
            .take(to.try_into().unwrap())
            .skip(from.try_into().unwrap())
            .collect()
    }
}

gstd::metadata! {
    title: "MultisigWallet",
    init:
        input: MWInitConfig,
    handle:
        input: MWAction,
        output: MWEvent,
    state:
        input: State,
        output: StateReply,
}

#[no_mangle]
pub unsafe extern "C" fn init() {
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

    WALLET = Some(wallet);
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
pub unsafe extern "C" fn meta_state() -> *mut [i32; 2] {
    let state: State = msg::load().expect("failed to decode input argument");
    let wallet: &mut MultisigWallet = WALLET.get_or_insert(MultisigWallet::default());
    let encoded = match state {
        State::ConfirmationsCount(transaction_id) => {
            StateReply::ConfirmationCount(wallet.get_confirmation_count(&transaction_id))
        }
        State::TransactionsCount { pending, executed } => {
            StateReply::TransactionsCount(wallet.get_transaction_count(pending, executed))
        }
        State::Owners => StateReply::Owners(wallet.get_owners()),
        State::Confirmations(transaction_id) => {
            StateReply::Confirmations(wallet.get_confirmations(&transaction_id))
        }
        State::TransactionIds {
            from_index,
            to_index,
            pending,
            executed,
        } => StateReply::TransactionIds(
            wallet.get_transaction_ids(from_index, to_index, pending, executed),
        ),
        State::IsConfirmed(transaction_id) => {
            StateReply::IsConfirmed(wallet.is_confirmed(&transaction_id))
        }
        State::Description(transaction_id) => {
            StateReply::Description(wallet.transaction_description(&transaction_id))
        }
    }
    .encode();

    gstd::util::to_leak_ptr(encoded)
}
