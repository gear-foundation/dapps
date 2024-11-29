use sails_rs::collections::{HashMap, HashSet};
use sails_rs::gstd::{exec, msg};
use sails_rs::prelude::*;

pub type TransactionId = U256;
const MAX_OWNERS_COUNT: u32 = 50;

#[derive(Default, Clone, Debug)]
pub struct MultisigWallet {
    pub transactions: HashMap<TransactionId, Transaction>,
    pub confirmations: HashMap<TransactionId, HashSet<ActorId>>,
    pub owners: HashSet<ActorId>,
    pub required: u32,
    pub transaction_count: U256,
}

impl MultisigWallet {
    pub fn validate_only_wallet(&self) {
        if msg::source() != exec::program_id() {
            panic!("Only wallet can call it")
        }
    }

    pub fn validate_owner_doesnt_exist(&self, owner: &ActorId) {
        if self.has_owner(owner) {
            panic!("Owner already exists")
        }
    }

    pub fn validate_owner_exists(&self, owner: &ActorId) {
        if !self.has_owner(owner) {
            panic!("Owner doesn't exists")
        }
    }

    fn validate_transaction_exists(&self, transaction_id: &TransactionId) {
        if !self.transactions.contains_key(transaction_id) {
            panic!("Transaction with this ID doesn't exists")
        }
    }

    pub fn validate_confirmed(&self, transaction_id: &TransactionId, owner: &ActorId) {
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

    pub fn validate_not_executed(&self, transaction_id: &TransactionId) {
        if matches!(self.transactions.get(transaction_id), Some(t) if t.executed) {
            panic!("Transaction has been already executed")
        }
    }

    fn validate_not_null_address(actor_id: &ActorId) {
        if *actor_id == ActorId::zero() {
            panic!("actor_id can not be zero");
        }
    }

    fn has_owner(&self, owner: &ActorId) -> bool {
        self.owners.contains(owner)
    }

    /// Allows to change the number of required confirmations. Transaction has to be sent by wallet.
    /// `required` Number of required confirmations.
    pub fn change_requirement(&mut self, required: u32) {
        self.validate_only_wallet();
        validate_requirement(self.owners.len(), required);

        self.required = required;
    }

    /// Allows an owner to confirm a transaction.
    /// `transaction_id` Transaction ID.
    pub fn confirm_transaction(&mut self, transaction_id: &TransactionId, msg_source: ActorId) {
        self.validate_owner_exists(&msg_source);
        self.validate_transaction_exists(transaction_id);
        self.validate_not_confirmed(transaction_id, &msg_source);

        self.confirmations
            .entry(*transaction_id)
            .or_default()
            .insert(msg_source);

        self.execute_transaction(transaction_id, None::<fn()>);
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
    pub fn add_transaction(
        &mut self,
        destination: &ActorId,
        data: Vec<u8>,
        value: u128,
        description: Option<String>,
    ) -> TransactionId {
        Self::validate_not_null_address(destination);
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

    /// Allows anyone to execute a confirmed transaction.
    /// `transaction_id` Transaction ID.
    pub fn execute_transaction<F>(&mut self, transaction_id: &TransactionId, completion: Option<F>)
    where
        F: FnMut(),
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

        if let Some(mut completion) = completion {
            completion();
        }
    }
}

#[derive(Debug, Default, Encode, Decode, TypeInfo, Hash, PartialEq, Eq, PartialOrd, Ord, Clone)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct State {
    pub transactions: Vec<(TransactionId, Transaction)>,
    pub confirmations: Vec<(TransactionId, Vec<ActorId>)>,
    pub owners: Vec<ActorId>,
    pub required: u32,
    pub transaction_count: U256,
}

#[derive(Debug, Default, Encode, Decode, TypeInfo, Hash, PartialEq, Eq, PartialOrd, Ord, Clone)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct Transaction {
    pub destination: ActorId,
    pub payload: Vec<u8>,
    pub value: u128,
    pub description: Option<String>,
    pub executed: bool,
}

pub fn validate_requirement(owners_count: usize, required: u32) {
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

impl From<MultisigWallet> for State {
    fn from(value: MultisigWallet) -> Self {
        let MultisigWallet {
            transactions,
            confirmations,
            owners,
            required,
            transaction_count,
        } = value;

        let transactions = transactions
            .iter()
            .map(|(tran_id, tran)| (*tran_id, tran.clone()))
            .collect();

        let confirmations = confirmations
            .iter()
            .map(|(tran_id, ids)| (*tran_id, ids.iter().copied().collect()))
            .collect();

        let owners = owners.iter().copied().collect();

        Self {
            transactions,
            confirmations,
            owners,
            required,
            transaction_count,
        }
    }
}
