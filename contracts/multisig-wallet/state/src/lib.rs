#![no_std]

use gmeta::{metawasm, Metadata};
use gstd::{prelude::*, ActorId};
use multisig_wallet_io::*;

fn common_confirmations_count(
    state: &<ContractMetadata as Metadata>::State,
    transaction_id: TransactionId,
) -> Option<u32> {
    state
        .confirmations
        .iter()
        .find_map(|(tx_id, confirmations)| {
            (tx_id == &transaction_id).then_some(
                confirmations
                    .iter()
                    .filter(|confirmation| state.owners.contains(confirmation))
                    .count() as _,
            )
        })
}

#[metawasm]
pub mod metafns {
    pub type State = <ContractMetadata as Metadata>::State;

    /// Returns number of confirmations of a transaction.
    /// `transaction_id` Transaction ID.
    /// Number of confirmations.
    pub fn confirmations_count(state: State, transaction_id: TransactionId) -> Option<u32> {
        common_confirmations_count(&state, transaction_id)
    }

    /// Returns total number of transactions after filers are applied.
    /// `pending` Include pending transactions.
    /// `executed` Include executed transactions.
    /// Total number of transactions after filters are applied.
    pub fn transactions_count(state: State, pending: bool, executed: bool) -> u32 {
        state
            .transactions
            .into_iter()
            .filter(|(_, tx)| (pending && !tx.executed) || (executed && tx.executed))
            .count() as _
    }

    /// Returns list of owners.
    /// List of owner addresses.
    pub fn owners(state: State) -> Vec<ActorId> {
        state.owners
    }

    /// Returns array with owner addresses, which confirmed transaction.
    /// `transaction_id` Transaction ID.
    /// Returns array of owner addresses.
    pub fn confirmations(state: State, transaction_id: TransactionId) -> Option<Vec<ActorId>> {
        state
            .confirmations
            .into_iter()
            .find_map(|(tx_id, confirmations)| (tx_id == transaction_id).then_some(confirmations))
    }

    /// Returns list of transaction IDs in defined range.
    /// `from` Index start position of transaction array.
    /// `to` Index end position of transaction array(not included).
    /// `pending` Include pending transactions.
    /// `executed` Include executed transactions.
    /// `Returns` array of transaction IDs.
    pub fn transaction_ids(
        state: State,
        from: u32,
        to: u32,
        pending: bool,
        executed: bool,
    ) -> Vec<TransactionId> {
        state
            .transactions
            .into_iter()
            .filter(|(_, tx)| (pending && !tx.executed) || (executed && tx.executed))
            .map(|(id, _)| id)
            .take(to as _)
            .skip(from as _)
            .collect()
    }

    /// Returns the confirmation status of a transaction.
    /// `transaction_id` Transaction ID.
    pub fn is_confirmed(state: State, transaction_id: TransactionId) -> bool {
        let required = state.required;

        if let Some(count) = common_confirmations_count(&state, transaction_id) {
            count >= required
        } else {
            false
        }
    }

    /// Returns the description of a transaction.
    /// `transaction_id` Transaction ID.
    pub fn transaction_description(
        state: State,
        transaction_id: TransactionId,
    ) -> Option<Option<String>> {
        state
            .transactions
            .into_iter()
            .find_map(|(tx_id, tx)| (tx_id == transaction_id).then_some(tx.description))
    }
}
